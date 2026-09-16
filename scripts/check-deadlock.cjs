// Canvas-level check of the real editor -> battle flow; no production test hooks.
const {chromium} = require('playwright-core');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const alphabet = '0123456789abcdefghjkmnpqrstvwxyz';
function code(bytes) {
 let bits=bytes.map(b=>b.toString(2).padStart(8,'0')).join('');
 bits=bits.padEnd(Math.ceil(bits.length/5)*5,'0');
 return bits.match(/.{5}/g).map(b=>alphabet[parseInt(b,2)]).join('');
}
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 try {
  const page=await browser.newPage({viewport:{width:1000,height:700}});
  const errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'deadlock-check'}}):r.abort());
  await page.addInitScript(()=>{
   window.Worker=class {postMessage(){} terminate(){}};
   window.samples=[];window.words='';window.labels=[];window.redMage=[];
   const raf=window.requestAnimationFrame,draw=CanvasRenderingContext2D.prototype.drawImage,fill=CanvasRenderingContext2D.prototype.fillRect;
   window.requestAnimationFrame=f=>raf.call(window,t=>{
    words='';labels=[];redMage=[];f(t);
    if(words.includes('Deadlock!'))samples.push({frame:Math.floor(performance.now()*.06),labels:structuredClone(labels)});
   });
   CanvasRenderingContext2D.prototype.fillRect=function(x,y,w,h){
    if(this.fillStyle==='#70783e'&&w===176&&h===24){const t=this.getTransform();labels.push({x:t.e+x,y:t.f+y,w,h,color:this.fillStyle})}
    return fill.call(this,x,y,w,h);
   };
   CanvasRenderingContext2D.prototype.drawImage=function(source,...a){
    if(a.length===8&&source.width===512&&a[0]===0&&a[1]===64&&a[2]===32&&a[3]===40){const t=this.getTransform();redMage.push({x:t.e+19*t.a,y:t.f+28})}
    if(a.length===8&&source.width===512&&a[2]===8&&a[3]===8&&a[1]>=224&&a[1]<=240)words+=String.fromCharCode((a[1]-216)/8*32+a[0]/8);
    return draw.call(this,source,...a);
   };
  });
  await page.goto(process.env.DRAG_URL||'http://127.0.0.1:8792/html/game.html');
  await page.waitForSelector('#game-canvas');
  await page.waitForFunction(()=>words.includes('Campaign'),undefined,{polling:50});
  const box=await page.locator('#game-canvas').boundingBox();
  const click=async(x,y,delay=400)=>{await page.mouse.click(box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272,{delay:30});await page.waitForTimeout(delay)};
  const enter=async(bytes)=>{
   await click(248,144);
   await page.locator('input').evaluate((input,value)=>{input.dataset.field='level_code';input.value=value;input.focus()},code(bytes));
   await page.keyboard.press('Enter');await page.waitForTimeout(400);
   await click(280,242);await page.evaluate(()=>samples=[]);
   await click(276,64,0);
  };
  // Immediate parity lock: Red Diamond and Blue Knight on matching colours.
  await enter([0x48,2,0,0,0x44,0x49,2,0x44,0]);
  await page.waitForFunction(()=>words.includes('Deadlock!'),undefined,{polling:50}).catch(async e=>{
   console.error(await page.evaluate(()=>({words,samples,labels})));
   await page.screenshot({path:'/tmp/maginet-deadlock-failure.png'});throw e;
  });
  await page.waitForFunction(()=>labels.some(b=>b.x===112),undefined,{polling:50});
  const output=process.env.DEADLOCK_OUTPUT||'assessments/campaign-overcharge';
  fs.mkdirSync(output,{recursive:true});
  await page.screenshot({path:path.join(output,'deadlock-banner.png')});
  await page.waitForTimeout(1600);
  const samples=await page.evaluate(()=>samples);
  assert.ok(samples.length>20);
  const first=samples[0].frame;
  let held=0,incoming=0,outgoing=0;
  for(const sample of samples){
   assert.equal(sample.labels.length,1);
   const {x,y}=sample.labels[0];assert.equal(y,124);
   const elapsed=sample.frame-first;
   // Actual frame samples can straddle a 60 Hz clock boundary by one tick.
   const candidates=[elapsed-1,elapsed,elapsed+1].filter(t=>t>=0&&t<90).map(t=>Math.round(112+(t<15?-296*(1-t/15):t<75?0:296*(t-75)/15)));
   assert.ok(candidates.includes(x),`unexpected banner position ${x} at frame ${elapsed}`);
   if(x<112)incoming++;else if(x===112)held++;else outgoing++;
  }
  assert.ok(incoming>0&&held>20&&outgoing>0);
  assert.equal(await page.evaluate(()=>words.includes('Deadlock!')),false);
  const centered=samples.filter(s=>s.labels[0].x===112);
  assert.ok(centered.at(-1).frame-centered[0].frame>=56,'holds center for one second');
  assert.ok(samples.at(-1).frame-first>=86&&samples.at(-1).frame-first<=91,'total duration 1500 ms');
  // A diagonal move is usable immediately after the banner.
  const beforeMove=await page.evaluate(()=>redMage);
  await click(96,96);await click(128,128);await page.waitForTimeout(450);
  const afterMove=await page.evaluate(()=>redMage);
  assert.equal(afterMove.length,1);assert.equal(beforeMove.length,1);
  assert.equal(afterMove[0].x-beforeMove[0].x,32,'mage moves diagonally in x');
  assert.ok(Math.abs(afterMove[0].y-beforeMove[0].y-32)<=2,'mage moves diagonally in y');
  assert.equal(await page.evaluate(()=>words.includes('Deadlock!')),false,'does not replay after normal moves');
  assert.deepEqual(errors,[]);
  fs.writeFileSync(path.join(output,'banner-timing.json'),JSON.stringify({incoming_frames:incoming,centered_frames:held,outgoing_frames:outgoing,samples},null,2));
  console.log('PASS: green Deadlock! banner, 250/1000/250 ms timing, left-to-right slide, one-shot display and usable diagonals');
 } finally {await browser.close()}
})().catch(e=>{console.error(e);process.exit(1)});
