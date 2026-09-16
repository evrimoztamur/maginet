// Requires a --features mobile build in MOBILE_PKG; no native code or purchase is bypassed in the app.
// This test supplies the same bridge events as a reviewer-enabled native shell.
const {chromium}=require('playwright-core');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const pkg=process.env.MOBILE_PKG||'/tmp/maginet-ux-mobile-pkg';
const root=path.join(__dirname,'..');
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true,args:['--disable-gpu','--disable-accelerated-2d-canvas']});
 try {
 const page=await browser.newPage({viewport:{width:1000,height:500},hasTouch:true});
 const errors=[];page.on('pageerror',e=>errors.push(e.message));
 await page.route('**/static/js/pkg/**',r=>{
  const file=new URL(r.request().url()).pathname.split('/static/js/pkg/')[1];
  r.fulfill({path:path.join(pkg,file),contentType:file.endsWith('.wasm')?'application/wasm':'text/javascript'});
 });
 await page.route('**/reviewer-check.html',r=>r.fulfill({body:fs.readFileSync(path.join(root,'html/ios.html'),'utf8'),contentType:'text/html'}));
 await page.route('**/api/session',r=>r.fulfill({json:{session_id:'reviewer-check'}}));
 await page.addInitScript(()=>{
  window.maginetNative={postMessage(body){if(JSON.parse(body).action==='state') {
   window.dispatchEvent(new CustomEvent('maginet-review',{detail:false}));
   window.dispatchEvent(new CustomEvent('maginet-access',{detail:false}));
  }}};
  window.Worker=class{postMessage(){}terminate(){}};
  const draw=CanvasRenderingContext2D.prototype.drawImage,raf=window.requestAnimationFrame;
  window.words='';window.buttons=[];
  const fill=CanvasRenderingContext2D.prototype.fillRect;
  CanvasRenderingContext2D.prototype.fillRect=function(x,y,w,h){const t=this.getTransform();buttons.push({x:t.e+x,y:t.f+y,w,h});return fill.call(this,x,y,w,h)};
  window.requestAnimationFrame=f=>raf.call(window,t=>{words='';buttons=[];f(t)});
  CanvasRenderingContext2D.prototype.drawImage=function(source,...a){
   if(a.length===8&&source.width===512&&a[2]===8&&a[3]===8&&a[1]>=224&&a[1]<=240)words+=String.fromCharCode((a[1]-216)/8*32+a[0]/8);
   return draw.call(this,source,...a);
  };
 });
 await page.goto('http://127.0.0.1:8790/reviewer-check.html');
  await page.waitForFunction(()=>words.includes('Campaign'), undefined, {polling:50});
 const box=await page.locator('#game-canvas').boundingBox();
 const tap=async(x,y)=>{await page.touchscreen.tap(box.x+(x+144)*box.width/544,box.y+(y+8)*box.height/272);await page.waitForTimeout(400)};
 const text=()=>page.evaluate(()=>words);
 await tap(248,210);await page.screenshot({path:'/tmp/maginet-ux-native-settings.png'});
 assert.ok((await text()).includes('Reviewer Access'));
 const controls=await page.evaluate(()=>buttons.filter(b=>b.y>=60&&b.w>=20&&b.h>=20));
 assert.equal(controls.length,13,'all settings controls and native account actions are visible');
 for(let i=0;i<controls.length;i++)for(let j=i+1;j<controls.length;j++) {
  const a=controls[i],b=controls[j];
  assert.ok(Math.max(b.x-a.x-a.w,a.x-b.x-b.w,b.y-a.y-a.h,a.y-b.y-b.h)>=8,'native settings buttons retain an eight-pixel gutter');
 }

 await tap(64,246);await tap(248,80);
 assert.ok(!(await text()).includes('Unlock all levels'));
 await page.evaluate(()=>{
  window.dispatchEvent(new CustomEvent('maginet-review',{detail:true}));
  window.dispatchEvent(new CustomEvent('maginet-access',{detail:true}));
 });
 await page.waitForFunction(()=>words.includes('Unlock all levels'), undefined, {polling:50});
 const saves=await page.evaluate(()=>JSON.stringify({...localStorage}));
 await tap(-4,238);
 assert.ok((await text()).includes('Crossfire')&&(await text()).includes('Ascension III'),'reveals every secret portal');
 assert.equal(await page.evaluate(()=>JSON.stringify({...localStorage})),saves,'unlock never forges earned stars');
 assert.ok(!(await text()).includes('Unlock all levels'));
 await page.screenshot({path:'/tmp/maginet-ux-review-unlocked.png'});
 await tap(128,238);await tap(248,80);assert.ok((await text()).includes('Crossfire'),'shortcut survives navigation');
 await page.evaluate(()=>window.dispatchEvent(new CustomEvent('maginet-review',{detail:false})));
 await page.waitForFunction(()=>words.includes('Back')&&!words.includes('Crossfire'), undefined, {polling:50});
 assert.ok(!(await text()).includes('Unlock all levels'),'paid ownership alone has no reviewer shortcut');
 await page.evaluate(()=>window.dispatchEvent(new CustomEvent('maginet-review',{detail:true})));
 await page.waitForFunction(()=>words.includes('Unlock all levels'), undefined, {polling:50});
 assert.ok(!(await text()).includes('Crossfire'),'review ended clears the temporary unlock');
 assert.deepEqual(errors,[]);
 console.log('PASS: mobile Settings layout and reviewer unlock/revoke, hidden levels, navigation persistence, untouched saves, and ownership separation');
 } finally {await browser.close()}
})().catch(e=>{console.error(e);process.exit(1)});
