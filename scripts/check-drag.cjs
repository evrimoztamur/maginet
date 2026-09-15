// Run against a web build served at DRAG_URL (default: localhost:8789/html/game.html).
// Uses rendered sprite transforms so the test needs no production debug API.
const {chromium} = require('playwright-core');
const assert = require('node:assert/strict');
(async () => {
 const browser = await chromium.launch({executablePath:process.env.CHROME_PATH || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 try {
 const page = await browser.newPage({viewport:{width:1000,height:700}});
 const errors=[]; page.on('pageerror', e=>errors.push(e.message));
 await page.route('https://tunnel.evrim.zone/**', r=>r.request().url().endsWith('/session') ? r.fulfill({json:{session_id:'drag-check'}}) : r.abort());
 await page.addInitScript(() => {
   // Keep the tutorial's AI from moving while we inspect the local action.
   window.Worker=class {postMessage() {} terminate() {}};
   const draw=CanvasRenderingContext2D.prototype.drawImage;
   const fill=CanvasRenderingContext2D.prototype.fillRect;
   const interfaces=new WeakSet();
   CanvasRenderingContext2D.prototype.fillRect=function(...a) {
     if(this.fillStyle==='#001515' || this.fillStyle==='#001f1f') interfaces.add(this);
     return fill.apply(this,a);
   };
   const raf=window.requestAnimationFrame;
   window.sprites={shadows:[],red:[]}; window.samples=[];
   window.requestAnimationFrame=f=>raf.call(window,t=>{
     window.sprites={shadows:[],red:[]}; f(t);
     if(window.sprites.red.length) window.samples.push(structuredClone(window.sprites));
   });
   CanvasRenderingContext2D.prototype.drawImage=function(source,...a) {
     const t=this.getTransform();
     if(interfaces.has(this)) return draw.call(this,source,...a); // Ignore interface roster portraits.
     if(a[1]===208 && a[2]===32 && a[0]===0) window.sprites.shadows.push([t.e+16,t.f+4]);
     if(a[1]===64 && a[2]===32 && a[3]===40) window.sprites.red.push([t.e+19*t.a,t.f+28]);
     return draw.call(this,source,...a);
   };
 });
 const url=process.env.DRAG_URL || 'http://127.0.0.1:8789/html/game.html';
 let point;
 const click=async(x,y)=>{const p=point(x,y);await page.mouse.click(p.x,p.y,{delay:30});await page.waitForTimeout(60)};
 const move=async(x,y)=>{const p=point(x,y);await page.mouse.move(p.x,p.y)};
 const sample=()=>page.evaluate(()=>sprites);
 const fresh=async()=>{
   await page.goto(url); await page.waitForSelector('#game-canvas');
   const box=await page.locator('#game-canvas').boundingBox();
   point=(x,y)=>({x:box.x+(x+72)*box.width/400,y:box.y+(y+8)*box.height/272});
   await click(248,174); await page.waitForTimeout(60);
   return (await sample()).shadows[0];
 };
 const origin=await fresh();
 await move(96,112);await page.mouse.down();await move(117,115);await page.waitForTimeout(60);
 let s=await sample();
 assert.equal(s.red.length,1,'drag renders exactly one red mage');
 assert.deepEqual(s.shadows.at(-1),[origin[0]+21,origin[1]+3],'shadow tracks ground and drag renders last');
 assert.equal(s.red[0][0],origin[0]+21,'preserves grab offset');
 assert.ok(s.red[0][1]-s.shadows.at(-1)[1]>=-13 && s.red[0][1]-s.shadows.at(-1)[1]<=-11,'jump-height bob');
 await move(32,112);await page.mouse.up();await page.waitForTimeout(60);
 assert.deepEqual((await sample()).shadows[0],origin,'outside drop snaps back');
 // Releasing over the menu must not open it: a following valid drag must work.
 await move(96,112);await page.mouse.down();await move(-16,116);await page.mouse.up();await page.waitForTimeout(60);
 await move(96,112);await page.mouse.down();await move(117,115);await page.waitForTimeout(40);
 await page.evaluate(()=>samples=[]);await page.mouse.up();await page.waitForTimeout(400);
 assert.deepEqual((await sample()).shadows[0],[origin[0]+32,origin[1]],'valid drop lands on destination');
 const landing=await page.evaluate(()=>samples);
 assert.ok(landing.every(s=>s.red.length===1 && s.red[0][0]>=origin[0]+21),'landing never returns to origin or duplicates sprite');
 // Undo restores normal animation; ordinary taps still submit a move afterward.
 await click(-16,140);await page.waitForTimeout(500);
 await click(96,112);await click(128,112);await page.waitForTimeout(400);
 assert.deepEqual((await sample()).shadows[0],[origin[0]+32,origin[1]]);
 // Real changedTouches semantics, including an unrelated finger ending first.
 await fresh();
 const touch=async(events)=>page.evaluate(({events})=>{
   const target=document.querySelector('#game-canvas');
   for(const [type,id,x,y] of events) {
     const t=new Touch({identifier:id,target,clientX:x,clientY:y});
     target.dispatchEvent(new TouchEvent(type,{bubbles:true,cancelable:true,changedTouches:[t],touches:type==='touchend'?[]:[t],targetTouches:type==='touchend'?[]:[t]}));
   }
 },{events:events.map(([type,id,x,y])=>{const p=point(x,y);return [type,id,p.x,p.y]})});
 await touch([['touchstart',7,96,112],['touchstart',8,200,200],['touchend',8,200,200],['touchmove',7,117,115]]);
 await page.waitForTimeout(60);
 assert.equal((await sample()).red[0][0],origin[0]+21,'additional finger does not steal drag');
 await touch([['touchcancel',8,200,200]]);await page.waitForTimeout(40);
 assert.equal((await sample()).red[0][0],origin[0]+21,'unrelated cancellation is ignored');
 await touch([['touchcancel',7,117,115]]);await page.waitForTimeout(40);
 assert.deepEqual((await sample()).shadows[0],origin,'matching cancellation returns mage');
 // All three events occur before an animation frame; release coordinates alone cross threshold.
 await touch([['touchstart',9,96,112],['touchend',9,128,112]]);
 await page.waitForTimeout(400);
 assert.deepEqual((await sample()).shadows[0],[origin[0]+32,origin[1]],'short touch uses changed release coordinates');
 await click(-16,140);
 assert.deepEqual((await sample()).shadows[0],[origin[0]+32,origin[1]],'touch-generated mouse events cannot undo the drop');
 for(const event of ['blur','resize','maginet-background']) {
   await fresh();
   await move(96,112);await page.mouse.down();await move(117,115);await page.waitForTimeout(40);
   await page.evaluate(event=>window.dispatchEvent(new Event(event)),event);
   await page.mouse.up();await page.waitForTimeout(60);
   assert.deepEqual((await sample()).shadows[0],origin,`${event} cancels without submission`);
 }
 // Local battle: selecting a different mage after an invalid touch drag still works.
 await page.goto(url);await page.waitForSelector('#game-canvas');
 await click(248,110);await click(48,80);await click(128,200);await page.waitForTimeout(80);
 const reds=(await sample()).red;
 assert.ok(reds.length>1);
 const tilePoint=r=>[r[0]-72,Math.round((r[1]-8-16)/32)*32+16];
 const first=tilePoint(reds[0]),second=tilePoint(reds[1]);
 await touch([['touchstart',10,...first],['touchend',10,0,0]]);await page.waitForTimeout(40);
 await touch([['touchstart',11,...second],['touchend',11,...second]]);await page.waitForTimeout(40);
 const destination=[second[0],second[1]+(second[1]<128?32:-32)];
 await touch([['touchstart',12,...destination],['touchend',12,...destination]]);await page.waitForTimeout(60);
 assert.ok((await sample()).shadows.some(p=>p[0]===second[0]+72 && p[1]===second[1]+8),'first touch previews destination');
 await touch([['touchstart',13,...destination],['touchend',13,...destination]]);await page.waitForTimeout(450);
 assert.ok((await sample()).shadows.some(p=>p[0]===destination[0]+72 && p[1]===destination[1]+8),'second touch confirms selected different mage');
 assert.deepEqual(errors,[]);
 console.log('PASS: mouse/touch drag, offset, float, single sprite, invalid/UI drops, landing continuity, undo/taps, multi-touch, cancellation, short release');
 } finally { await browser.close(); }
})().catch(e=>{console.error(e);process.exit(1)});
