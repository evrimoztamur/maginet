// Shared web/mobile preference parser, persisted selections, and actual tutorial worker requests.
const {chromium}=require('playwright-core');
const assert=require('node:assert/strict');
const path=require('node:path');
const native=!!process.env.MOBILE_PKG;
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 try {
 for(const preference of [null,'','invalid','Easy','Hard']) {
  const page=await browser.newPage({viewport:native?{width:844,height:390}:{width:1000,height:700},hasTouch:native});
  const errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'defaults'}}):r.abort());
  if(native) {
   await page.route('**/static/js/pkg/**',r=>{const file=new URL(r.request().url()).pathname.split('/static/js/pkg/')[1];return r.fulfill({path:path.join(process.env.MOBILE_PKG,file),contentType:file.endsWith('.wasm')?'application/wasm':'text/javascript'})});
   await page.route('**/defaults-check.html',r=>r.fulfill({path:path.resolve('html/ios.html'),contentType:'text/html'}));
   await page.route('**/api/session',r=>r.fulfill({json:{session_id:'defaults-mobile'}}));
  }
  await page.addInitScript(preference=>{
   localStorage.clear();if(preference!==null)localStorage.setItem('difficulty',preference);
   window.maginetNative={postMessage(body){if(JSON.parse(body).action==='state')window.dispatchEvent(new CustomEvent('maginet-access',{detail:true}))}};
   window.jobs=[];window.Worker=class{postMessage(r){this.request=r;jobs.push(this)}terminate(){this.terminated=true}};
  },preference);
  await page.goto(native?'http://127.0.0.1:8798/defaults-check.html':process.env.DRAG_URL||'http://127.0.0.1:8798/html/game.html');
  await page.waitForSelector('#game-canvas');await page.waitForTimeout(500);
  const box=await page.locator('#game-canvas').boundingBox();
  const click=async(x,y)=>{const p=native?[(x+167)*390/272,(y+8)*390/272]:[box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272];if(native)await page.touchscreen.tap(...p);else await page.mouse.click(...p,{delay:30});await page.waitForTimeout(350)};
  await click(248,110);await click(128,80);await click(172,160);await page.evaluate(()=>jobs=[]);await click(128,200);
  await page.waitForFunction(()=>jobs.length>0);
  assert.equal(await page.evaluate(()=>jobs.at(-1).request.difficulty),['Easy','Hard'].includes(preference)?preference:'Normal');
  assert.equal(await page.evaluate(()=>localStorage.getItem('difficulty')),preference,'reading a default never overwrites a preference');
  await page.reload();await page.waitForTimeout(500);await click(248,174);await page.evaluate(()=>jobs=[]);
  await click(96,112);await click(128,112);if(native)await click(128,112);
  await page.waitForFunction(()=>jobs.length>0);
  assert.equal(await page.evaluate(()=>jobs.at(-1).request.difficulty),'Easy');
  assert.equal(await page.evaluate(()=>localStorage.getItem('difficulty')),preference,'tutorial does not persist Easy');
  assert.deepEqual(errors,[]);await page.close();
 }
 console.log('PASS: fresh, empty, invalid, saved Easy/Hard preferences; tutorial override without modifying storage ('+(native?'mobile':'web')+')');
 } finally {await browser.close()}
})().catch(e=>{console.error(e);process.exit(1)});
