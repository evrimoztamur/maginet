// Every revised battle must keep its old star and reproduce a recorded opening in the browser.
const {chromium}=require('playwright-core');
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path');
const root=path.resolve('assessments/campaign-challenge');
const graph=JSON.parse(require('node:child_process').execFileSync('cargo',['run','--quiet','-p','shared','--example','campaign_catalogue'],{encoding:'utf8',stdio:['ignore','pipe','inherit']}));
const alphabet='0123456789abcdefghjkmnpqrstvwxyz';
function canonical(code){let bits=[...code].map(c=>alphabet.indexOf(c).toString(2).padStart(5,'0')).join('');let data=Array.from({length:Math.floor(bits.length/8)},(_,i)=>parseInt(bits.slice(i*8,i*8+8),2));const start=3+3*data[1],props=[];for(let i=start;i<data.length;i+=2)props.push(data.slice(i,i+2));data=[...data.slice(0,start),...props.sort((a,b)=>a[0]-b[0]).flat()];bits=data.map(b=>b.toString(2).padStart(8,'0')).join('');bits+='0'.repeat((5-bits.length%5)%5);return bits.match(/.{5}/g).map(b=>alphabet[parseInt(b,2)]).join('')}
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 try {
 for(const entry of JSON.parse(fs.readFileSync(path.join(root,'candidates.json'))).filter(e=>e.selected&&!e.baseline&&(!process.env.CAMPAIGN_IDS||process.env.CAMPAIGN_IDS.split(',').includes(e.battle)))) {
  const page=await browser.newPage({viewport:{width:1000,height:700}}),errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'challenge'}}):r.abort());
  await page.addInitScript(codes=>{
   for(const code of codes)localStorage.setItem(code,'win');
   window.jobs=[];window.Worker=class{postMessage(request){this.request=request;jobs.push(this)}terminate(){this.terminated=true}};
   window.words='';const draw=CanvasRenderingContext2D.prototype.drawImage,raf=requestAnimationFrame;
   window.requestAnimationFrame=f=>raf(t=>{words='';f(t)});
   CanvasRenderingContext2D.prototype.drawImage=function(source,...a){if(a.length===8&&source.width===512&&a[2]===8&&a[3]===8&&a[1]>=224&&a[1]<=240)words+=String.fromCharCode((a[1]-216)/8*32+a[0]/8);return draw.call(this,source,...a)};
  },JSON.parse(fs.readFileSync(path.join(root,'original-catalogue.json'))).catalogue.map(e=>canonical(e.code)));
  await page.goto(process.env.DRAG_URL||'http://127.0.0.1:8798/html/game.html');await page.waitForFunction(()=>jobs.length>0);
  const box=await page.locator('#game-canvas').boundingBox();const xy=(x,y)=>[box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272];
  const click=async(x,y)=>{await page.mouse.click(...xy(x,y),{delay:30});await page.waitForTimeout(350)};
  await click(248,80);await page.waitForFunction(()=>words.includes('30/30'));
  const portal=graph.catalogue.find(e=>e.id===entry.battle);assert.equal(canonical(portal.code),entry.code);
  await require('./campaign-browser-navigation.cjs')(page,xy,graph.catalogue,portal.position);
  await click(128,204);await page.waitForTimeout(400);
  const trials=JSON.parse(fs.readFileSync(path.join(root,'screen',entry.id,'matchup-00-2-1.json'))).games;
  const replay=trials.find(g=>g.outcome==='Win').replay;
  const bits=[...entry.code].map(c=>alphabet.indexOf(c).toString(2).padStart(5,'0')).join('');const b=parseInt(bits.slice(0,8),2),w=(b>>5)+1,h=((b>>2)&7)+1;
  const tile=([x,y])=>[(8-w)*16+16+x*32,(8-h)*16+16+y*32];
  const count=await page.evaluate(()=>jobs.length);
  for(const p of replay[0].turn)await click(...tile(p));
  await page.waitForFunction(n=>jobs.length>n,count);
  let request=await page.evaluate(()=>jobs.at(-1).request);let state=JSON.parse(request.snapshot);
  assert.equal(request.difficulty,'Normal');assert.deepEqual(state.turns,[replay[0].turn]);
  assert.deepEqual(['Red','Blue'].map(t=>state.level.mages.filter(m=>m.team===t).reduce((n,m)=>n+m.mana[0],0)),replay[0].mana);
  if(entry.battle==='shields-i') {
   await page.evaluate(turn=>{const job=jobs.at(-1);job.onmessage({data:{...job.request,selected:turn}})},replay[1].turn);await page.waitForTimeout(700);
   const n=await page.evaluate(()=>jobs.length);for(const p of replay[2].turn)await click(...tile(p));await page.waitForFunction(n=>jobs.length>n,n);
   state=JSON.parse(await page.evaluate(()=>jobs.at(-1).request.snapshot));assert(state.level.mages.some(m=>m.team==='Red'&&m.powerup==='Shield'),'safe detour leads to the shield');
  }
  await page.screenshot({path:path.join(root,'browser','campaign-'+entry.battle+'.png')});
  assert.deepEqual(errors,[]);console.log('PASS:',entry.battle,'legacy star and recorded opening');await page.close();
 }
 } finally {await browser.close()}
})().catch(e=>{console.error(e);process.exit(1)});
