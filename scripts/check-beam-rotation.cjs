// A disposable local lobby verifies the real Blue view and unchanged turn messages.
const {chromium}=require('playwright-core');
const {spawn,execFileSync}=require('node:child_process');
const fs=require('node:fs'),os=require('node:os'),path=require('node:path'),net=require('node:net');
const assert=require('node:assert/strict');
const delay=ms=>new Promise(r=>setTimeout(r,ms));
(async()=>{
 const root=path.resolve(__dirname,'..');execFileSync('cargo',['build','--locked','-p','server','--quiet'],{cwd:root,stdio:'ignore'});
 const port=await new Promise(resolve=>{const s=net.createServer();s.listen(0,'127.0.0.1',()=>{const p=s.address().port;s.close(()=>resolve(p))})});
 const directory=fs.mkdtempSync(path.join(os.tmpdir(),'maginet-beam-view-'));
 const server=spawn(path.join(root,'target/debug/server'),[],{cwd:directory,env:{...process.env,MAGINET_PORT:String(port)},stdio:'ignore'});
 let browser;
 try {
  const api=async(p,body)=>{const r=await fetch(`http://127.0.0.1:${port}${p}`,body===undefined?{}:{method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify(body)});return r.json()};
  for(let i=0;i<100;i++){try{await api('/session');break}catch{await delay(50)}}
  browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
  const page=await browser.newPage({viewport:{width:1000,height:700}}),errors=[],actions=[];let code,session;
  page.on('pageerror',e=>errors.push(e.message));
  await page.route('https://tunnel.evrim.zone/**',async r=>{
   const p=new URL(r.request().url()).pathname;let body=r.request().method()==='POST'?r.request().postDataJSON():undefined;
   if(p==='/lobby/create')body.lobby_settings.player_team='Blue';
   const result=await api(p,body);
   if(p==='/session')session=result.session_id;
   if(p==='/lobby/create')code=result.Lobby.settings.lobby_sort.Online;
   if(p.endsWith('/act'))actions.push(body);
   await r.fulfill({json:result});
  });
  await page.addInitScript(()=>{
   const mage=(index,team,position,powerup=null)=>({index,team,position,powerup,sort:'Plus',mana:[4,4],spell:{pattern:[[-2,0],[-1,0],[1,0],[2,0],[0,-2],[0,-1],[0,1],[0,2]]}});
   localStorage.setItem('levels',JSON.stringify({0:{board:{width:4,height:4,style:'Grass'},mages:[mage(0,'Blue',[0,0]),mage(1,'Red',[3,3],'Shield'),mage(2,'Blue',[0,3],'Shield')],mage_index:3,powerups:[[[1,0],'Beam']],starting_team:'Blue'}}));
   window.Worker=class{postMessage(){}terminate(){}};
   window.beams=[];window.shields=[];const draw=CanvasRenderingContext2D.prototype.drawImage;
   CanvasRenderingContext2D.prototype.drawImage=function(source,...a){const t=this.getTransform();if(a[1]===56&&a[0]>=120&&a[0]<=136&&a[2]===8)beams.push({x:t.e,y:t.f});if(a[1]===16&&a[2]===16&&a[3]===16&&t.a===1&&[32,48].includes(a[0]))shields.push({sprite:a[0],y:t.d});return draw.call(this,source,...a)};
  });
  await page.goto(process.env.DRAG_URL||'http://127.0.0.1:8798/html/game.html');await page.waitForSelector('#game-canvas');await delay(500);
  const box=await page.locator('#game-canvas').boundingBox();const click=async(x,y)=>{await page.mouse.click(box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272,{delay:30});await delay(400)};
  await click(248,144);await click(276,240);await click(276,224);
  for(let i=0;!code&&i<50;i++)await delay(100);assert(code);
  const opponent=(await api('/session')).session_id;await api(`/lobby/${code}/ready`,{session_id:opponent});await delay(1600);
  const lobby=(await api(`/lobby/${code}/state`)).Lobby;assert.equal(lobby.players[session].team,'Blue');
  await page.waitForFunction(()=>shields.some(s=>s.sprite===48&&s.y===1)&&shields.some(s=>s.sprite===32&&s.y===1));
  await page.evaluate(()=>beams=[]);await click(80,176);await click(112,176);await page.waitForFunction(()=>beams.length>500);
  const beam=await page.evaluate(()=>beams);assert(beam.filter(p=>p.y>160).length>beam.length*0.4,'horizontal beam follows the pickup at the bottom of Blue view');
  assert(Math.max(...beam.map(p=>p.y))-Math.min(...beam.map(p=>p.y))>110,'vertical particles reach the opposite edge');
  assert.deepEqual(actions.map(a=>a.message),[{Turn:[[0,0],[1,0]]}]);assert.deepEqual(errors,[]);
  await page.screenshot({path:path.join(root,'assessments/campaign-challenge/browser/beam-blue-view.png')});
  console.log('PASS: Blue-view beam origin and reach, untransformed 16x16 shields, unchanged wire Turn, local server');
 } finally {if(browser)await browser.close();server.kill();await new Promise(r=>server.once('exit',r));fs.rmSync(directory,{recursive:true,force:true})}
})().catch(e=>{console.error(e);process.exit(1)});
