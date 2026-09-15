// Run against a dev web build at DRAG_URL.
const {chromium}=require('playwright-core');
const assert=require('node:assert/strict');
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 try {
 for(const loadout of ['Default','Random','Chaos']) {
  const page=await browser.newPage({viewport:{width:1000,height:700}});
  const errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'settings-test'}}):r.abort());
  await page.addInitScript(()=>{
   window.jobs=[];window.mages=[];
   window.Worker=class {postMessage(request){window.jobs.push(request)} terminate(){}};
   const draw=CanvasRenderingContext2D.prototype.drawImage,raf=requestAnimationFrame;
   window.requestAnimationFrame=f=>raf(t=>{window.mages=[];f(t)});
   CanvasRenderingContext2D.prototype.drawImage=function(source,...a){
    if(a[2]===32 && a[3]===40 && [64,104].includes(a[1]))window.mages.push([a[0],a[1]]);
    return draw.call(this,source,...a);
   };
  });
  await page.goto(process.env.DRAG_URL || 'http://127.0.0.1:8791/html/game.html');await page.waitForSelector('#game-canvas');
  const box=await page.locator('#game-canvas').boundingBox();
  const click=async(x,y)=>{await page.mouse.click(box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272);await page.waitForTimeout(120)};
  await click(248,110);await page.waitForTimeout(550);await click(128,80);await click(172,160);
  await click(48,121+22*['Default','Random','Chaos'].indexOf(loadout));
  const preview=await page.evaluate(()=>mages);
  await click(128,200);await page.waitForTimeout(950);
  await page.waitForFunction(()=>jobs.length===1);
  const first=await page.evaluate(()=>JSON.parse(jobs[0].snapshot));
  await click(-16,108);await click(128,148);await click(128,148);await page.waitForTimeout(550);
  assert.deepEqual(await page.evaluate(()=>mages),preview,'menu restores exact loadout preview');
  await click(128,200);await page.waitForTimeout(950);
  await page.waitForFunction(()=>jobs.length===2);
  assert.equal(await page.evaluate(()=>jobs.length),2,'AI mode and Blue side are restored: red AI opens again');
  assert.deepEqual(await page.evaluate(()=>JSON.parse(jobs[1].snapshot)),first,'restarting uses identical initial game settings');
  assert.deepEqual(errors,[]);
  console.log('PASS: AI '+loadout+' mode, team, exact loadout and restart preserved after leaving');
  await page.close();
 }
 } finally {await browser.close()}
})().catch(e=>{console.error(e);process.exit(1)});
