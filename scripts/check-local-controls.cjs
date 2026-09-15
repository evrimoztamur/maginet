// Run against a dev web build at DRAG_URL.
const {chromium}=require('playwright-core');
const assert=require('node:assert/strict');
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 try {
 for(const team of ['Red','Blue']) {
  const page=await browser.newPage({viewport:{width:1000,height:700},hasTouch:true});
  const errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'local-controls'}}):r.abort());
  await page.addInitScript(()=>{
   const draw=CanvasRenderingContext2D.prototype.drawImage,fill=CanvasRenderingContext2D.prototype.fillRect,raf=requestAnimationFrame;
   const ui=new WeakSet();window.controls=[];window.board=[];
   CanvasRenderingContext2D.prototype.fillRect=function(...a){if(['#001515','#001f1f'].includes(this.fillStyle))ui.add(this);return fill.apply(this,a)};
   window.requestAnimationFrame=f=>raf(t=>{window.controls=[];window.board=[];f(t)});
   CanvasRenderingContext2D.prototype.drawImage=function(source,...a){
    const t=this.getTransform();
    if(a[2]===32 && a[3]===40 && [64,104].includes(a[1])) {
     (ui.has(this)?controls:board).push({team:a[1]===64?'Red':'Blue',x:t.e,y:t.f,d:t.d});
    }
    return draw.call(this,source,...a);
   };
  });
  await page.goto(process.env.DRAG_URL || 'http://127.0.0.1:8791/html/game.html');await page.waitForSelector('#game-canvas');
  const box=await page.locator('#game-canvas').boundingBox();
  const point=(x,y)=>({x:box.x+(x+72)*box.width/400,y:box.y+(y+8)*box.height/272});
  const click=async(x,y)=>{const p=point(x,y);await page.mouse.click(p.x,p.y);await page.waitForTimeout(100)};
  const touch=async(events)=>page.evaluate(events=>{
   const target=document.querySelector('#game-canvas');
   for(const [type,x,y] of events){
    const t=new Touch({identifier:1,target,clientX:x,clientY:y});
    target.dispatchEvent(new TouchEvent(type,{bubbles:true,cancelable:true,changedTouches:[t],touches:type==='touchend'?[]:[t]}));
   }
  },events.map(([type,x,y])=>{const p=point(x,y);return [type,p.x,p.y]}));
  const tap=async(x,y)=>{await touch([['touchstart',x,y],['touchend',x,y]]);await page.waitForTimeout(100)};
  await click(248,110);await click(172,team==='Red'?128:160);await click(128,200);await page.waitForTimeout(950);
  const check=async(turn)=>{
   const sprites=await page.evaluate(()=>controls);
   assert(sprites.length>0);
   assert(sprites.every(s=>s.team===turn && (s.d<0)===(turn!==team)), 'roster faces the active local player');
   assert(sprites.every(s=>turn===team?s.y>140:s.y<100),'roster switches screen edges');
  };
  await check('Red');
  const tile=(x,y)=>[48+32*x,48+32*(team==='Blue'?5-y:y)];
  await tap(...tile(1,5));await tap(...tile(1,4));
  // Confirm the board preview using the matching physical pad arrow.
  const redCenter=team==='Red'?[349-72,221-8]:[400-1-349-72,272-1-221-8];
  await tap(redCenter[0],redCenter[1]+(team==='Red'?-30:30));
  await page.waitForTimeout(1100);
  await check('Blue');
  // Select on the rotated roster, then drag the pad toward the physical destination.
  const flipped=team==='Red';
  const roster=flipped?[400-1-26-72,272-1-198-8]:[26-72,198-8];
  await tap(...roster);
  // Choose a known board mage to make the expected displacement unambiguous.
  await tap(...tile(1,0));
  const center=flipped?[400-1-349-72,272-1-221-8]:[349-72,221-8];
  const dy=team==='Red'?32:-32;
  const before=await page.evaluate(()=>board.filter(s=>s.team==='Blue').map(s=>s.y));
  await touch([['touchstart',...center],['touchmove',center[0],center[1]+dy]]);
  await page.waitForTimeout(80);
  await touch([['touchend',center[0],center[1]+dy]]);
  await page.waitForTimeout(1100);
  await check('Red');
  const after=await page.evaluate(()=>board.filter(s=>s.team==='Blue').map(s=>s.y));
  assert(after.some((y,i)=>Math.abs(y-before[i])>25),'top pad drag moves the board mage');
  assert.deepEqual(errors,[]);
  console.log('PASS: '+team+' view, local roster rotation, touch confirmation and opposite-side pad dragging');
  await page.close();
 }
 } finally {await browser.close()}
})().catch(e=>{console.error(e);process.exit(1)});
