const {chromium}=require('playwright-core');
const assert=require('node:assert/strict');
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 try {
 const page=await browser.newPage({viewport:{width:1000,height:700}});
 const errors=[];page.on('pageerror',e=>errors.push(e.message));
 await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'navigation'}}):r.abort());
 await page.addInitScript(()=>{
  window.slides=[];window.cursors=[];window.cursorCopies=0;window.selfCopies=0;
  const draw=CanvasRenderingContext2D.prototype.drawImage;
  const cursorLayers=new WeakSet();
  CanvasRenderingContext2D.prototype.drawImage=function(source,...a){
   if(source===this.canvas) selfCopies++;
   if(a.length===8 && a[0]===64 && a[1]===8 && a[2]===16 && a[3]===16) {
    cursorLayers.add(this.canvas);
    const p=this.getTransform().transformPoint(new DOMPoint(a[4],a[5]));
    cursors.push([p.x,p.y]);
   }
   if(a.length===2 && cursorLayers.has(source))cursorCopies++;
   if(source.width===400 && source.height===272 && a.length===2 && a[0]!==0)
    slides.push({x:a[0],at:performance.now()});
   return draw.call(this,source,...a);
  };
 });
 await page.goto(process.env.DRAG_URL || 'http://127.0.0.1:8791/html/game.html');await page.waitForSelector('#game-canvas');
 const box=await page.locator('#game-canvas').boundingBox();
 const click=async(x,y)=>page.mouse.click(box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272);
 for(const [x,y,direction] of [[248,110,1],[172,228,-1],[248,210,1],[128,248,-1]]) {
  await page.mouse.move(box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272);
  await page.waitForTimeout(40);
  await page.evaluate(()=>{slides=[];cursors=[];cursorCopies=0});
  await click(x,y);await page.waitForTimeout(650);
  const samples=(await page.evaluate(()=>slides)).filter(s=>Math.abs(s.x)<400);
  assert(samples.length>8,'transition renders both screens over multiple frames');
  assert(samples.every(s=>Number.isInteger(s.x)),'all offsets stay on pixel grid');
  assert.equal(Math.sign(samples[0].x),-direction,'outgoing screen exits in correct direction');
  assert.equal(Math.sign(samples[1].x),direction,'incoming screen enters from correct edge');
  assert.equal(Math.abs(samples[0].x-samples[1].x),400,'screens remain adjacent');
  assert(samples.at(-1).at-samples[0].at>=180,'slide lasts roughly 250 ms including its rounded final pixels');
 }
 const cursorState=await page.evaluate(()=>({cursors,cursorCopies}));
 assert.equal(cursorState.cursorCopies,0,'cursor is never copied into a snapshot or sliding screen');
 assert.equal(new Set(cursorState.cursors.map(JSON.stringify)).size,1,'stationary mouse stays at one rendered position throughout slide');
 assert.equal(await page.evaluate(()=>selfCopies),0,'menu board construction avoids canvas self-copies');
 assert.deepEqual(errors,[]);
 console.log('PASS: forward/back slides, simultaneous snapshots, 250 ms timing, stationary cursor, integer pixels');
 } finally {await browser.close()}
})().catch(e=>{console.error(e);process.exit(1)});
