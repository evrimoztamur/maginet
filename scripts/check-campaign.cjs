const {chromium} = require('playwright-core');
const assert = require('assert/strict');
(async () => {
  const browser = await chromium.launch({executablePath:process.env.CHROME_PATH || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
  for (const completed of [false,true]) {
    const page = await browser.newPage({viewport:{width:1000,height:700}});
    const errors=[];page.on('pageerror',e=>errors.push(e.message));
    await page.route('https://tunnel.evrim.zone/**',route=>route.request().url().endsWith('/session')?route.fulfill({json:{session_id:'campaign-check'}}):route.abort());
    await page.addInitScript(completed=>{
      localStorage.clear();localStorage.setItem('difficulty','Hard');
      if(completed)localStorage.setItem('hg18a09m4g0m81g00c4068035g14r0v008','win');
      window.jobs=[];
      window.Worker=class {constructor(){jobs.push(this)}postMessage(request){this.request=request}terminate(){this.terminated=true}};
    },completed);
    await page.goto(process.env.DRAG_URL || 'http://127.0.0.1:8000/',{waitUntil:'domcontentloaded'});
    await page.waitForSelector('#game-canvas');
    await page.waitForFunction(()=>jobs.length>0);
    const box=await page.locator('#game-canvas').boundingBox();
    const click=async(x,y)=>{await page.mouse.click(box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272,{delay:100});await page.waitForTimeout(150)};
    await click(248,80); // Campaign; selected portal depends on the tutorial save.
    await page.screenshot({path:`/tmp/maginet-campaign-${completed?'unlocked':'fresh'}.png`});
    const before=await page.evaluate(()=>jobs.length);
    await click(128,204); // Battle enters Tutorial or Basics I.
    if(completed) {
      // Basics I: Red begins at (1,2), rendered on a 5x4 board.
      await click(96,144);await click(128,144);
    } else {await click(96,112);await click(128,112);await page.waitForTimeout(500);await click(276,188);await click(276,188);}
    await page.waitForFunction(n=>jobs.length>n,before);
    const request=await page.evaluate(()=>jobs.at(-1).request);
    assert.equal(request.difficulty,completed?'Hard':'Easy');
    const snapshot=JSON.parse(request.snapshot);
    assert.equal(snapshot.can_stalemate,completed);
    if(!completed) assert.ok(snapshot.level.powerups.every(([,powerup])=>powerup!=='Diagonal'),'mandatory tutorial has no diagonal pickups');
    assert.deepEqual(errors,[]);
    await page.close();
  }
  await browser.close();console.log('campaign fresh tutorial and saved-progress battle smoke checks passed');
})().catch(e=>{console.error(e);process.exit(1)});
