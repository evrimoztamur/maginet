// Run against a web build served at DRAG_URL (default: localhost:8789/html/game.html).
// Uses rendered sprite transforms so the test needs no production debug API.
const {chromium} = require('playwright-core');
const assert = require('node:assert/strict');
(async () => {
 const browser = await chromium.launch({executablePath:process.env.CHROME_PATH || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 try {
 const page = await browser.newPage({viewport:{width:1000,height:700},hasTouch:true});
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
   window.sprites={shadows:[],red:[]}; window.samples=[]; window.arrowSteps=[];
   window.requestAnimationFrame=f=>raf.call(window,t=>{
     window.sprites={shadows:[],red:[],rosterRed:[],rosterShadows:[],markers:[],rosterMarkers:[]}; f(t);
     if(window.sprites.red.length) window.samples.push(structuredClone(window.sprites));
   });
   CanvasRenderingContext2D.prototype.drawImage=function(source,...a) {
     const t=this.getTransform();
     if(source.width===512 && source.getContext) window.controlAtlas=source;
     if(interfaces.has(this)) {
       if(a[1]===208 && a[2]===32 && a[0]===0) window.sprites.rosterShadows.push([t.e+16,t.f+4]);
       if(a[1]===64 && a[2]===32 && a[3]===40) window.sprites.rosterRed.push([t.e+19*t.a,t.f+28]);
       if(a[0]===72 && a[1]===0 && a[2]===8 && a[3]===5) window.sprites.rosterMarkers.push(t.f);
       if(a[0]===160 && a[1]===144 && a[2]===16) window.arrowSteps.push([t.e,t.f]);
       return draw.call(this,source,...a);
     } // Ignore interface roster portraits.
     if(a[0]===72 && a[1]===0 && a[2]===8 && a[3]===5) window.sprites.markers.push(t.f);
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
 const touch=async(events)=>page.evaluate(({events})=>{
   const target=document.querySelector('#game-canvas');
   for(const [type,id,x,y] of events) {
     const t=new Touch({identifier:id,target,clientX:x,clientY:y});
     target.dispatchEvent(new TouchEvent(type,{bubbles:true,cancelable:true,changedTouches:[t],touches:type==='touchend'?[]:[t],targetTouches:type==='touchend'?[]:[t]}));
   }
 },{events:events.map(([type,id,x,y])=>{const p=point(x,y);return [type,id,p.x,p.y]})});

 const tap=async(x,y)=>{await touch([['touchstart',1,x,y],['touchend',1,x,y]]);await page.waitForTimeout(80)};
 // Settings persists the controller preference and removes hidden hit areas.
 await fresh();
 await page.goto(url);await page.waitForSelector('#game-canvas');
 await tap(248,210);await tap(100,204);
 assert.equal(await page.evaluate(()=>localStorage.getItem('onscreen_controls')),'off');
 await page.screenshot({path:'/tmp/maginet-controls-settings.png'});
 await tap(128,248);await tap(248,174);
 assert.equal((await sample()).rosterRed.length,0,'Off hides the roster');
 await tap(-51,243); // Hidden roster must not select a mage.
 assert.equal((await sample()).markers.length,0,'hidden roster has no hit target');
 await tap(96,112);await tap(128,112);
 const disabledOrigin=(await sample()).shadows[0];
 await tap(128,112);await page.waitForTimeout(450);
 assert.deepEqual((await sample()).shadows[0],[disabledOrigin[0]+32,disabledOrigin[1]],'board taps still work with controls off');
 await fresh();
 assert.equal((await sample()).rosterRed.length,0,'Off persists across reloads');
 await page.goto(url);await page.waitForSelector('#game-canvas');
 await tap(248,210);await tap(32,204);
 assert.equal(await page.evaluate(()=>localStorage.getItem('onscreen_controls')),'on');
 const origin=await fresh();
 assert.equal(await page.evaluate(()=>{
   const c=controlAtlas.getContext('2d');
   const blue=c.getImageData(0,32,32,16).data;
   const white=c.getImageData(160,144,32,16).data;
   for(let i=0;i<blue.length;i+=4) {
     if(blue[i+3]!==white[i+3]) return false;
     if(blue[i+3] && (white[i]!==255 || white[i+1]!==255 || white[i+2]!==255)) return false;
   }
   return true;
 }),true,'white arrows preserve every original alpha pixel');
 await tap(-51,243); // First roster portrait.
 await tap(307,213); // East pad arrow previews.
 await page.waitForTimeout(2100);
 assert.ok(await page.evaluate(()=>new Set(arrowSteps.map(p=>p.join(','))).size>2),'ordinary pad arrows animate');
 assert.deepEqual((await sample()).shadows[0],origin,'preview does not time out or move');
 await tap(96,144); // Different board destination replaces east.
 await tap(307,213); // East replaces south rather than committing.
 assert.deepEqual((await sample()).shadows[0],origin,'switching surfaces replaces preview');
 await page.screenshot({path:'/tmp/mobile-before.png'});
 await tap(128,112); // Same east destination confirms from board.
 await page.waitForTimeout(450);
 assert.deepEqual((await sample()).shadows[0],[origin[0]+32,origin[1]],'cross-surface confirmation');
 await tap(-16,140);await page.waitForTimeout(500); // Undo.
 assert.deepEqual((await sample()).shadows[0],origin,'touch undo works');
 await tap(-51,243);await tap(307,213);
 await touch([['touchcancel',1,307,213]]); // A fresh active gesture cancellation clears preview.
 await touch([['touchstart',2,277,213],['touchcancel',2,277,213]]);
 await tap(307,213);
 assert.deepEqual((await sample()).shadows[0],origin,'cancellation clears selected destination');
 await touch([['touchstart',3,277,213],['touchmove',3,298,216]]);
 await page.waitForTimeout(80);
 const held=await sample();
 assert.equal(held.rosterMarkers.length,1,'selected roster mage has the board marker');
 assert.equal(held.red[0][1]-held.shadows.at(-1)[1],held.rosterRed[0][1]-held.rosterShadows[0][1],'roster matches board drag height');
 assert.equal(held.markers[0]-held.shadows.at(-1)[1],held.rosterMarkers[0]-held.rosterShadows[0][1],'selection markers share their animation and drag offset');
 await touch([['touchend',3,309,213]]);
 await page.waitForTimeout(450);
 assert.deepEqual((await sample()).shadows[0],[origin[0]+32,origin[1]],'center drag moves relative to mage, exactly one tile');
 await tap(-16,140);await page.waitForTimeout(500);
 await tap(-51,243);
 await touch([['touchstart',4,277,213],['touchend',4,180,213]]);
 await page.waitForTimeout(100);
 assert.deepEqual((await sample()).shadows[0],origin,'invalid pad drop cancels');
 await tap(307,213);await tap(-16,116); // Preview then menu.
 await tap(-16,116);await tap(307,213);
 assert.deepEqual((await sample()).shadows[0],origin,'menu opening clears preview');
 await page.screenshot({path:'/tmp/maginet-mobile-landscape.png'});
 for (const viewport of [{width:390,height:844},{width:768,height:1024}]) {
   await page.setViewportSize(viewport);
   await page.goto(url);await page.waitForSelector('#game-canvas');
   const box=await page.locator('#game-canvas').boundingBox();
   point=(x,y)=>({x:box.x+(263-y)*box.width/272,y:box.y+(x+72)*box.height/400});
   await click(248,174);await page.waitForTimeout(100);
   await page.evaluate(()=>{
     const fill=CanvasRenderingContext2D.prototype.fillRect;
     const raf=window.requestAnimationFrame;
     window.highlights=[];
     window.requestAnimationFrame=f=>raf.call(window,t=>{window.highlights=[];f(t)});
     CanvasRenderingContext2D.prototype.fillRect=function(...a){
       if((this.fillStyle==='#001f1f' || this.fillStyle==='#007faa') && (a[2]===40 || a[2]===28)) window.highlights.push([this.getTransform().e,this.getTransform().f]);
       return fill.apply(this,a);
     };
   });
   await tap(307,242); // Portrait bottom-left roster.
   await tap(307,42); // East in board coordinates points down on the rotated canvas.
   assert.equal(await page.evaluate(()=>highlights.length),2,'portrait roster and destination are selected');
   await page.screenshot({path:`/tmp/maginet-mobile-portrait-${viewport.width}.png`});
   await tap(128,112);await page.waitForTimeout(450);
   assert.equal(await page.evaluate(()=>highlights.length),0,'portrait cross-surface confirmation submits');
 }
 assert.deepEqual(errors,[]);
 console.log('PASS: roster, persistent preview, switching, cross-surface confirmation, touch undo/menu, cancellation, center drag and invalid drop');
 } finally { await browser.close(); }
})().catch(e=>{console.error(e);process.exit(1)});
