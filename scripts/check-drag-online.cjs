// Two browser clients and a disposable local server. No production requests or match files.
const {chromium}=require('playwright-core');
const {spawn,execFileSync}=require('node:child_process');
const fs=require('node:fs'),os=require('node:os'),path=require('node:path'),net=require('node:net');
const assert=require('node:assert/strict');
const delay=ms=>new Promise(r=>setTimeout(r,ms));
(async()=>{
 const root=path.resolve(__dirname,'..');
 execFileSync('cargo',['build','--locked','-p','server','--quiet'],{cwd:root,stdio:'ignore'});
 const port=await new Promise(resolve=>{const s=net.createServer();s.listen(0,'127.0.0.1',()=>{const p=s.address().port;s.close(()=>resolve(p));});});
 const directory=fs.mkdtempSync(path.join(os.tmpdir(),'maginet-drag-online-'));
 const server=spawn(path.join(root,'target/debug/server'),[],{cwd:directory,env:{...process.env,MAGINET_PORT:String(port)},stdio:'ignore'});
 let browser;
 try {
 const api=async(p,body)=>{const r=await fetch(`http://127.0.0.1:${port}${p}`,body===undefined?{}:{method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify(body)});return r.json()};
 for(let i=0;i<100;i++){try{await api('/session');break}catch{await delay(50)}}
 browser=await chromium.launch({executablePath:process.env.CHROME_PATH || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 const clients=[];let code;const errors=[];
 for(let i=0;i<2;i++){
   const page=await browser.newPage({viewport:{width:1000,height:700}});
   const client={page,actions:[],session:null,replacement:null};clients.push(client);
   page.on('pageerror',e=>errors.push(e.message));
   await page.route('https://tunnel.evrim.zone/**',async r=>{
     const p=new URL(r.request().url()).pathname;
     let result;
     if(p==='/lobby/create'&&code) result=await api(`/lobby/${code}/state`);
     else if(client.replacement && p.includes('/turns/')){result=client.replacement;client.replacement=null;}
     else result=await api(p,r.request().method()==='POST'?r.request().postDataJSON():undefined);
     if(p==='/session')client.session=result.session_id;
     if(p==='/lobby/create')code=result.Lobby.settings.lobby_sort.Online;
     if(p.endsWith('/act'))client.actions.push(r.request().postDataJSON());
     await r.fulfill({json:result});
   });
   await page.goto(process.env.DRAG_URL || 'http://127.0.0.1:8789/html/game.html');await page.waitForSelector('#game-canvas');
   const box=await page.locator('#game-canvas').boundingBox();
   client.point=(x,y)=>({x:box.x+(x+72)*box.width/400,y:box.y+(y+8)*box.height/272});
   client.click=async(x,y)=>{const p=client.point(x,y);await page.mouse.click(p.x,p.y,{delay:30});await delay(80)};
   await client.click(248,110);await client.click(208,80);
   if(process.env.PLAYER_TEAM === 'Blue') await client.click(172,160);
   await client.click(128,200);
   await delay(1300);
 }
 await delay(1300);
 const state=async()=>(await api(`/lobby/${code}/state`)).Lobby;
 let lobby=await state();assert.equal(Object.keys(lobby.players).length,2);
 assert.equal(lobby.players[clients[0].session].team,process.env.PLAYER_TEAM || 'Red');
 const assignments=structuredClone(lobby.players);
 const board=lobby.game.level.board;
 const point=(client,tile)=>client.point((8-board.width)*16+tile[0]*32+16,(8-board.height)*16+(assignments[client.session].team === 'Blue' ? board.height-1-tile[1] : tile[1])*32+16);
 const dragStart=async(c,turn)=>{const a=point(c,turn[0]),b=point(c,turn[1]);await c.page.mouse.move(a.x,a.y);await c.page.mouse.down();await c.page.mouse.move(b.x,b.y);await delay(80)};
 const red=clients.find(c=>lobby.players[c.session].team==='Red');
 const blue=clients.find(c=>c!==red);
 const turn=lobby.game.available_turns[0];
 // Illegal destination never sends a turn. The same selection can then be dragged legally.
 const a=point(red,turn[0]);const outside=red.point(0,0);
 await red.page.mouse.move(a.x,a.y);await red.page.mouse.down();await red.page.mouse.move(outside.x,outside.y);await red.page.mouse.up();await delay(80);
 assert.equal((await state()).game.turns.length,0);assert.equal(red.actions.length,0);
 await dragStart(red,turn);
 await delay(1400); // Empty polling responses must not cancel a held drag.
 await red.page.mouse.up();await delay(1400);
 assert.deepEqual((await state()).game.turns,[turn]);assert.equal(red.actions.length,1);
 assert.deepEqual(red.actions[0].message,{Turn:turn},'landing data stays client-only');
 lobby=await state();const reply=lobby.game.available_turns[0];
 // A remotely submitted turn arriving while this client drags cancels the local gesture.
 await dragStart(blue,reply);
 assert.equal(await api(`/lobby/${code}/act`,{session_id:blue.session,message:{Turn:reply}}),'Ok');
 await delay(1400);await blue.page.mouse.up();await delay(100);
 assert.equal((await state()).game.turns.length,2);assert.equal(blue.actions.length,0);
 // Replace history during another drag via the existing Lobby message path.
 lobby=await state();const next=lobby.game.available_turns[0];await dragStart(red,next);
 for(const c of clients)await api(`/lobby/${code}/rematch`,{session_id:c.session});
 const replacement=await api(`/lobby/${code}/state`);
 red.replacement=replacement;blue.replacement=replacement;
 await delay(1400);await red.page.mouse.up();await delay(100);
 assert.equal((await state()).game.turns.length,0);assert.equal(red.actions.length,1);
 for(const c of clients) assert.equal((await state()).players[c.session].team,assignments[c.session].team);
 await dragStart(red,(await state()).game.available_turns[0]);await red.page.mouse.up();await delay(1400);
 assert.equal((await state()).game.turns.length,1,'red can open rematch with the same view');
 const blueTurn=(await state()).game.available_turns[0];
 await dragStart(blue,blueTurn);await blue.page.mouse.up();await delay(1400);
 assert.equal((await state()).game.turns.length,2,'blue drag works in mirrored view');
 assert.deepEqual(blue.actions.at(-1).message,{Turn:blueTurn});
 assert.deepEqual(errors,[]);
 console.log('PASS: two online clients, invalid drop/no history, exactly one turn, held drag across polling, unchanged wire message, remote turn cancellation, history replacement');
 } finally {if(browser)await browser.close();server.kill();await new Promise(r=>server.once('exit',r));fs.rmSync(directory,{recursive:true,force:true});}
})().catch(e=>{console.error(e);process.exit(1)});
