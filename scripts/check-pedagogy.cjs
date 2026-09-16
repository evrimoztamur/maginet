const {chromium}=require('playwright-core');
const assert=require('assert/strict');
const fs=require('fs');
const path=require('path');
const root=path.join(__dirname,'../assessments/campaign-pedagogy');
const graph=JSON.parse(require('child_process').execFileSync('cargo',['run','--quiet','-p','shared','--example','campaign_catalogue'],{encoding:'utf8',stdio:['ignore','pipe','inherit']}));
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 for(const candidate of JSON.parse(fs.readFileSync(path.join(root,'candidates.json'))).filter(e=>e.selected)) {
  console.log('checking',candidate.adopt_as);
  const metadata=JSON.parse(fs.readFileSync(path.join(root,'followup',candidate.id,'metadata.json')));
  const first=JSON.parse(fs.readFileSync(path.join(root,'followup',candidate.id,'matchup-00-1-1.json'))).games[0].replay[0];
  const page=await browser.newPage({viewport:{width:1000,height:700}});const errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'combat-check'}}):r.abort());
  await page.addInitScript(code=>{
   localStorage.clear();localStorage.setItem('hg18a09m4g0m81g00c4068035g14r0v008','win');localStorage.setItem(code,'win');localStorage.setItem('difficulty','Normal');
   window.jobs=[];window.Worker=class{constructor(){jobs.push(this)}postMessage(r){this.request=r}terminate(){}};
  },JSON.parse(fs.readFileSync(path.join(root,'followup',JSON.parse(fs.readFileSync(path.join(root,'candidates.json'))).find(e=>e.baseline&&e.battle===candidate.battle).id,'metadata.json'))).catalogue[0].code);
  await page.goto(process.env.GAME_URL||'http://127.0.0.1:8000/',{waitUntil:'domcontentloaded'});await page.waitForFunction(()=>jobs.length);
  const box=await page.locator('#game-canvas').boundingBox();
  const xy=(x,y)=>[box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272];
  const click=async(x,y)=>{await page.mouse.click(...xy(x,y),{delay:100});await page.waitForTimeout(150)};
  await click(248,80);
  const pos=graph.catalogue.find(e=>e.id===candidate.adopt_as).position;
  await require('./campaign-browser-navigation.cjs')(page,xy,graph.catalogue.filter(e=>!e.hidden),pos);
  await page.waitForTimeout(500);await page.screenshot({path:'/tmp/maginet-pedagogy-map.png'});await click(128,204);
  const before=await page.evaluate(()=>jobs.length);
  const bytes=[...metadata.catalogue[0].code].map(c=>'0123456789abcdefghjkmnpqrstvwxyz'.indexOf(c).toString(2).padStart(5,'0')).join('');
  const byte=parseInt(bytes.slice(0,8),2),width=(byte>>5)+1,height=((byte>>2)&7)+1;
  for(const [x,y] of first.turn)await click((8-width)*16+16+32*x,(8-height)*16+16+32*y);
  await page.waitForFunction(n=>jobs.length>n,before);
  const request=await page.evaluate(()=>jobs.at(-1).request);const snapshot=JSON.parse(request.snapshot);
  assert.equal(request.difficulty,'Normal');assert.equal(snapshot.can_stalemate,true);
  assert.equal(snapshot.level.board.width,width);assert.equal(snapshot.level.board.height,height);
  assert.equal(snapshot.level.mages.length,parseInt(bytes.slice(8,16),2));
  assert.deepEqual(snapshot.turns,[first.turn]);
  assert.deepEqual(['Red','Blue'].map(team=>snapshot.level.mages.filter(m=>m.team===team).reduce((sum,m)=>sum+m.mana[0],0)),first.mana);
  assert.deepEqual(errors,[]);
  fs.mkdirSync(path.join(root,'browser'),{recursive:true});await page.screenshot({path:path.join(root,'browser',candidate.adopt_as+'.png')});await page.close();
 }
 await browser.close();console.log('all eight revised battles unlock from legacy saves, load, and reproduce their first recorded move in the browser');
})().catch(e=>{console.error(e);process.exit(1)});
