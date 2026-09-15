// Isolated browser storage only; no real campaign progress or service calls.
const {chromium}=require('playwright-core');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const source=fs.readFileSync(require('node:path').join(__dirname,'../shared/src/campaign.rs'),'utf8');
const codes=[...new Set([...source.matchAll(/"([0-9a-hjkmnp-tv-z]{20,})"/g)].map(m=>m[1]))];
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 try {
  for(const touch of [false,true]) {
   const page=await browser.newPage({viewport:{width:1000,height:700},hasTouch:touch});
   const errors=[];page.on('pageerror',e=>errors.push(e.message));
   await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'reset-star-check'}}):r.abort());
   await page.addInitScript(()=>{
    const draw=CanvasRenderingContext2D.prototype.drawImage,raf=window.requestAnimationFrame;
    window.starFrames=[];window.starPose=null;window.sparkles=0;window.tooltips=[];
    const fill=CanvasRenderingContext2D.prototype.fillRect;
    CanvasRenderingContext2D.prototype.fillRect=function(...a) {
     if(this.fillStyle==='#001515' && a[3]===16) {const t=this.getTransform();tooltips.push([t.e,t.f,a[2],a[3]])}
     return fill.apply(this,a);
    };
    window.requestAnimationFrame=f=>raf.call(window,t=>{
     window.starPose=null;window.sparkles=0;window.tooltips=[];f(t);
     if(window.starPose) window.starFrames.push(window.starPose);
    });
    CanvasRenderingContext2D.prototype.drawImage=function(source,...a){
     if(a[0]===32 && a[1]===320 && a[2]===32 && a[3]===32) {
      const t=this.getTransform();window.starPose=[t.e,t.f,t.a,t.b,t.c,t.d];
     }
     if(a[0]>=96 && a[0]<=112 && a[1]===56 && a[2]===8 && a[3]===8) window.sparkles++;
     return draw.call(this,source,...a);
    };
   });
   await page.goto(process.env.DRAG_URL||'http://127.0.0.1:8789/html/game.html');
   await page.waitForSelector('#game-canvas');
   await page.evaluate(codes=>{
    codes.forEach(code=>localStorage.setItem(code,'win'));
    localStorage.setItem('onscreen_controls','off');
    localStorage.setItem('music_volume','3');
    localStorage.setItem('levels','{"my-level":"saved"}');
    localStorage.setItem('unrelated-result','win');
   },codes);
   const box=await page.locator('#game-canvas').boundingBox();
   const click=async(x,y)=>{
    const p={x:box.x+(x+72)*box.width/400,y:box.y+(y+8)*box.height/272};
    if(touch) await page.touchscreen.tap(p.x,p.y);
    else await page.mouse.click(p.x,p.y,{delay:25});
    await page.waitForTimeout(70);
   };
   await click(248,210);await page.waitForTimeout(400);
   const gentle=await page.evaluate(()=>starFrames);
   assert.ok(gentle.length>5 && Math.max(...gentle.map(p=>p[1]))-Math.min(...gentle.map(p=>p[1]))<=4,'gentle idle spring');
   const idleY=gentle.at(-1)[1];
   await page.evaluate(()=>starFrames=[]);
   assert.equal(await page.evaluate(()=>tooltips.length),0,'hint starts hidden');
   await click(296,36);
   assert.equal(await page.evaluate(()=>tooltips.length),1,'first click reveals exactly one tooltip line');
   assert.equal(await page.evaluate(()=>tooltips[0][1]),66,'tooltip sits beneath the star');
   await page.screenshot({path:`/tmp/maginet-star-countdown-${touch?'touch':'mouse'}.png`});
   await click(296,36);
   assert.equal(await page.evaluate(code=>localStorage.getItem(code),codes[0]),'win','two clicks preserve progress');
   assert.ok(await page.evaluate(y=>starFrames.some(p=>Math.abs(p[1]-y)>3),idleY),'clicks kick the positional spring');
   assert.equal(await page.evaluate(()=>starFrames.every(p=>Number.isInteger(p[0]) && Number.isInteger(p[1]) && p[2]===1 && p[3]===0 && p[4]===0 && p[5]===1)),true,'star stays unscaled, unrotated, and on the pixel grid');
   await page.waitForTimeout(3100); // An expired sequence starts over.
   await click(296,36);
   assert.equal(await page.evaluate(code=>localStorage.getItem(code),codes[0]),'win','expired click sequence cannot reset');
   await click(296,36);await click(296,36);
   assert.equal(await page.evaluate(code=>localStorage.getItem(code),codes[0]),'win','three clicks preserve progress');
   assert.equal(await page.evaluate(()=>tooltips.length),1,'countdown remains a single line');
   await click(296,36);
   assert.equal(await page.evaluate(codes=>codes.every(code=>localStorage.getItem(code)===null),codes),true,'fourth click resets every catalogue entry');
   assert.equal(await page.evaluate(()=>starPose),null,'star explodes out of view');
   assert.ok(await page.evaluate(()=>sparkles)>=80,'gold star-particle shower');
   const saves=await page.evaluate(()=>['onscreen_controls','music_volume','levels','unrelated-result'].map(k=>localStorage.getItem(k)));
   assert.deepEqual(saves,['off','3','{"my-level":"saved"}','win'],'noncampaign saves survive');
   await page.screenshot({path:`/tmp/maginet-reset-star-${touch?'touch':'mouse'}.png`});
   await page.waitForTimeout(900);
   assert.ok(await page.evaluate(()=>starPose!==null),'star springs back after the burst');
   assert.deepEqual(errors,[]);
   await page.close();
  }
  console.log('PASS: mouse/touch four-click countdown, spring jiggle, gold particle burst, campaign-only reset, respawn');
 } finally {await browser.close();}
})().catch(e=>{console.error(e);process.exit(1)});
