const {chromium}=require('playwright-core');
const assert=require('assert/strict');
const fs=require('fs');
const path=require('path');
const root=path.join(__dirname,'../assessments/campaign-combat-redesign');
const graph=JSON.parse(require('child_process').execFileSync('cargo',['run','--quiet','-p','shared','--example','campaign_catalogue'],{encoding:'utf8',stdio:['ignore','pipe','inherit']}));
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 for(const candidate of JSON.parse(fs.readFileSync(path.join(root,'candidates.json'))).filter(e=>e.selected)) {
  const metadata=JSON.parse(fs.readFileSync(path.join(root,'final',candidate.id,'metadata.json')));
  const first=JSON.parse(fs.readFileSync(path.join(root,'final',candidate.id,'matchup-00-1-1.json'))).games[0].replay[0];
  const page=await browser.newPage({viewport:{width:1000,height:700}});const errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'combat-check'}}):r.abort());
  await page.addInitScript(code=>{
   localStorage.clear();localStorage.setItem('hg18a09m4g0m81g00c4068035g14r0v008','win');localStorage.setItem(code,'win');localStorage.setItem('difficulty','Normal');
   window.jobs=[];window.Worker=class{constructor(){jobs.push(this)}postMessage(r){this.request=r}terminate(){}};
  },metadata.catalogue[0].code);
  await page.goto('http://127.0.0.1:8000/',{waitUntil:'domcontentloaded'});await page.waitForFunction(()=>jobs.length);
  const box=await page.locator('#game-canvas').boundingBox();
  const xy=(x,y)=>[box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272];
  const click=async(x,y)=>{await page.mouse.click(...xy(x,y),{delay:100});await page.waitForTimeout(150)};
  await click(248,80);
  const pos=graph.catalogue.find(e=>e.id===candidate.battle).position;
  for(let x=0;x<pos[0];x++){
   await page.mouse.move(...xy(196,128));await page.mouse.down();await page.mouse.move(...xy(68,128),{steps:4});await page.waitForTimeout(30);await page.mouse.up();await page.waitForTimeout(50);
  }
  for(let y=0;y<Math.abs(pos[1]);y++){
   await page.mouse.move(...xy(128,pos[1]<0?56:184));await page.mouse.down();await page.mouse.move(...xy(128,pos[1]<0?184:56),{steps:4});await page.waitForTimeout(30);await page.mouse.up();await page.waitForTimeout(50);
  }
  await page.waitForTimeout(500);await click(128,204);
  const before=await page.evaluate(()=>jobs.length);
  for(const [x,y] of first.turn)await click(80+32*x,80+32*y);
  await page.waitForFunction(n=>jobs.length>n,before);
  const request=await page.evaluate(()=>jobs.at(-1).request);const snapshot=JSON.parse(request.snapshot);
  assert.equal(request.difficulty,'Normal');assert.equal(snapshot.can_stalemate,true);
  assert.equal(snapshot.level.board.width,4);assert.equal(snapshot.level.board.height,4);
  assert.equal(snapshot.level.mages.length,candidate.battle==='rite-ii'?4:6);
  assert.deepEqual(snapshot.turns,[first.turn]);
  assert.deepEqual(['Red','Blue'].map(team=>snapshot.level.mages.filter(m=>m.team===team).reduce((sum,m)=>sum+m.mana[0],0)),first.mana);
  assert.deepEqual(errors,[]);
  await page.screenshot({path:`/tmp/maginet-redesign-${candidate.battle}.png`});await page.close();
 }
 await browser.close();console.log('all three redesigned battles load and reproduce their first recorded move in the browser');
})().catch(e=>{console.error(e);process.exit(1)});
