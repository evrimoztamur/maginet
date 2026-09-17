// Inspect actual canvas geometry and capture desktop/touch presentation without a debug API.
const {chromium}=require('playwright-core');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const native=!!process.env.MOBILE_PKG;
const out=path.resolve('assessments/campaign-challenge/browser'+(native?'/mobile':''));fs.mkdirSync(out,{recursive:true});
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 try {
 for(const touch of native?[true]:[false,true]) for(const edge of [false,true]) {
  const page=await browser.newPage({viewport:native?{width:844,height:390}:touch?{width:600,height:420}:{width:1000,height:700},hasTouch:touch});
  const errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'effects'}}):r.abort());
  if(native) {
   await page.route('**/static/js/pkg/**',r=>{const file=new URL(r.request().url()).pathname.split('/static/js/pkg/')[1];return r.fulfill({path:path.join(process.env.MOBILE_PKG,file),contentType:file.endsWith('.wasm')?'application/wasm':'text/javascript'})});
   await page.route('**/effects-check.html',r=>r.fulfill({path:path.resolve('html/ios.html'),contentType:'text/html'}));
   await page.route('**/api/session',r=>r.fulfill({json:{session_id:'effects-mobile'}}));
  }
  await page.addInitScript(({edge})=>{
   window.maginetNative={postMessage(body){if(JSON.parse(body).action==='state')window.dispatchEvent(new CustomEvent('maginet-access',{detail:true}))}};
   const mage=(index,team,position,powerup=null)=>({index,team,position,powerup,sort:'Plus',mana:[4,4],spell:{pattern:[[-2,0],[-1,0],[1,0],[2,0],[0,-2],[0,-1],[0,1],[0,2]]}});
   const level={board:{width:4,height:4,style:'Grass'},mages:[mage(0,'Red',edge?[1,0]:[0,0]),mage(1,'Blue',[3,3],'Shield'),mage(2,'Red',[0,3],'Shield')],mage_index:3,powerups:[[edge?[0,0]:[1,0],'Beam']],starting_team:'Red'};
   localStorage.setItem('levels',JSON.stringify({0:level}));
   window.Worker=class{postMessage(){}terminate(){}};
   window.beams=[];window.shields=[];
   window.captureBeam=false;window.beamFrozen=false;window.frameBeams=[];
   const raf=window.requestAnimationFrame.bind(window);
   window.requestAnimationFrame=callback=>{const run=t=>{
    if(beamFrozen){raf(run);return}
    frameBeams=[];callback(t);
    if(captureBeam&&frameBeams.length){
     const xs=frameBeams.map(b=>b.x),ys=frameBeams.map(b=>b.y);
     if(Math.max(...xs)-Math.min(...xs)>110&&Math.max(...ys)-Math.min(...ys)>110&&frameBeams.some(b=>b.alpha<0.5))beamFrozen=true;
    }
   };return raf(run)};
   const draw=CanvasRenderingContext2D.prototype.drawImage;
   window.beamParticles=0;
   CanvasRenderingContext2D.prototype.drawImage=function(source,...args){
    const t=this.getTransform();
    if(args[1]===16&&args[2]===16&&args[3]===16&&t.a===1&&[32,48].includes(args[0]))shields.push({sprite:args[0],scaleY:t.d});
    if(args[1]===56&&args[0]>=120&&args[0]<=136&&args[2]===8){beamParticles++;const b={x:t.e,y:t.f,alpha:this.globalAlpha};beams.push(b);frameBeams.push(b)}
    return draw.call(this,source,...args);
   };
  },{edge});
  await page.goto(native?'http://127.0.0.1:8798/effects-check.html':process.env.DRAG_URL||'http://127.0.0.1:8798/html/game.html');await page.waitForSelector('#game-canvas');await page.waitForTimeout(500);
  const box=await page.locator('#game-canvas').boundingBox();
  const click=async(x,y)=>{const p=native?[(x+167)*390/272,(y+8)*390/272]:[box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272];if(touch)await page.touchscreen.tap(...p);else await page.mouse.click(...p,{delay:30});await page.waitForTimeout(400)};
  await click(248,144);await click(276,240);await click(276,64);await page.waitForTimeout(400);
  await page.waitForFunction(()=>shields.some(s=>s.sprite===32&&s.scaleY===1)&&shields.some(s=>s.sprite===48&&s.scaleY===1));
  await page.screenshot({path:path.join(out,`shields-${touch?'touch':'desktop'}.png`)});
  await page.evaluate(()=>{beams=[];beamParticles=0;captureBeam=true});
  await click(edge?112:80,80);await click(edge?80:112,80);if(touch)await click(edge?80:112,80);
  // Wait for the actual expanded, fading effect, not a count that also includes idle rune sparks.
  await page.waitForFunction(()=>beamFrozen,null,{polling:50});
  await page.screenshot({path:path.join(out,`beam-${edge?'edge':'inner'}-${touch?'touch':'desktop'}.png`)});
  assert(await page.evaluate(()=>beamParticles)>100,'existing beam particles fill the beam body');
  const samples=await page.evaluate(()=>beams);
  assert(Math.max(...samples.map(b=>b.x))-Math.min(...samples.map(b=>b.x))>110,'horizontal particles reach both edges');
  assert(Math.max(...samples.map(b=>b.y))-Math.min(...samples.map(b=>b.y))>110,'vertical particles reach both edges');
  assert(samples.some(b=>b.alpha<0.5)&&samples.some(b=>b.alpha===1),'particles fade independently');
  await page.evaluate(()=>{captureBeam=false;beamFrozen=false});
  await page.waitForTimeout(900);await page.evaluate(()=>beams=[]);
  await click(-16,140);await page.waitForTimeout(900);
  assert.equal(await page.evaluate(()=>beams.length),0,'undo never fires a beam');
  assert.deepEqual(errors,[]);console.log('PASS:',native?'mobile':touch?'touch':'desktop',edge?'edge':'inner','particles and shields');await page.close();
 }
 console.log('PASS: beam particles span both axes, independently fade, native 16x16 team shields, touch, desktop, undo');
 } finally {await browser.close()}
})().catch(e=>{console.error(e);process.exit(1)});
