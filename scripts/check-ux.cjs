// Run against a web build. Uses canvas output and isolated browser storage.
const { chromium } = require('playwright-core');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const url = process.env.DRAG_URL || 'http://127.0.0.1:8790/html/game.html';
const path = require('node:path');
const replayRoot = process.env.TUTORIAL_REPLAY || fs.mkdtempSync(path.join(require('node:os').tmpdir(), 'maginet-ux-replay-'));
if (!process.env.TUTORIAL_REPLAY) require('node:child_process').execFileSync('cargo', [
 'run', '--quiet', '-p', 'generate', '--', 'analyse', '--code', 'hg18a09m4g0m81000c4068039g1g',
 '--games', '1', '--red-profile', 'hard', '--blue-profile', 'easy', '--replays', '--output', replayRoot,
], {cwd:path.join(__dirname,'..'),stdio:'pipe'});
(async () => {
 const browser = await chromium.launch({executablePath: process.env.CHROME_PATH || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome', headless:true});
 try {
  for (const touch of [false, true]) {
   const page = await browser.newPage({viewport:{width:1000,height:700},hasTouch:touch});
   const errors=[]; page.on('pageerror',e=>errors.push(e.message));
   await page.route('https://tunnel.evrim.zone/**', r=>r.request().url().endsWith('/session') ? r.fulfill({json:{session_id:'ux-check'}}) : r.abort());
   await page.addInitScript(() => {
    window.jobs=[];
    window.Worker=class {constructor(){jobs.push(this)} postMessage(request){this.request=request} terminate(){this.terminated=true}};
    const draw=CanvasRenderingContext2D.prototype.drawImage;
    const fill=CanvasRenderingContext2D.prototype.fillRect;
    const raf=window.requestAnimationFrame;
    window.sprites=[];window.labels=[];window.words='';
    window.requestAnimationFrame=f=>raf.call(window,t=>{sprites=[];labels=[];words='';f(t)});
    CanvasRenderingContext2D.prototype.fillRect=function(x,y,w,h){
     const t=this.getTransform(); labels.push({x:t.e+x,y:t.f+y,w,h,color:this.fillStyle});return fill.call(this,x,y,w,h);
    };
    CanvasRenderingContext2D.prototype.drawImage=function(source,...a){
     if(a.length===8 && source.width===512) {
      const t=this.getTransform();sprites.push({sx:a[0],sy:a[1],w:a[2],h:a[3],x:t.e,y:t.f});
      if(a[2]===8 && a[3]===8 && a[1]>=224 && a[1]<=240) words+=String.fromCharCode((a[1]-216)/8*32+a[0]/8);
     }
     return draw.call(this,source,...a);
    };
   });
   let box;
   const fresh=async()=>{await page.goto(url);await page.waitForSelector('#game-canvas');await page.waitForFunction(()=>words.includes('Campaign'), undefined, {polling:50});box=await page.locator('#game-canvas').boundingBox()};
   const point=(x,y)=>({x:box.x+(x+72)*box.width/400,y:box.y+(y+8)*box.height/272});
   const click=async(x,y)=>{const p=point(x,y);if(touch)await page.touchscreen.tap(p.x,p.y);else await page.mouse.click(p.x,p.y,{delay:30});await page.waitForTimeout(350)};
   const text=()=>page.evaluate(()=>words);
   const shot=async(name)=>page.screenshot({path:`/tmp/maginet-ux-${name}-${touch?'touch':'mouse'}.png`});
   const highlights=()=>page.evaluate(()=>sprites.filter(s=>s.sy===256 && s.w===32 && [32,64].includes(s.sx)));
   await fresh();
   await click(248,210);
   await click(14,90);assert.equal(await page.evaluate(()=>localStorage.getItem('music_volume')),'9');
   await click(130,90);assert.equal(await page.evaluate(()=>localStorage.getItem('music_volume')),'10');
   await click(14,134);assert.equal(await page.evaluate(()=>localStorage.getItem('clip_volume')),'7');
   for(const [x,label] of [[22,'Easy'],[72,'Normal'],[120,'Hard']]) {
    await click(x,178);assert.equal(await page.evaluate(()=>localStorage.getItem('difficulty')),label);
   }
   await click(108,222);assert.equal(await page.evaluate(()=>localStorage.getItem('onscreen_controls')),'off');
   await click(32,222);assert.equal(await page.evaluate(()=>localStorage.getItem('onscreen_controls')),'on');
   assert.ok(await page.evaluate(()=>labels.filter(b=>b.w>=28&&b.h>=28).length>=9),'larger audio and grouped controls');
   await click(296,36);assert.ok((await text()).includes('Press three more timesto reset campaign!'));
   await shot('settings');
   await click(72,250);await click(248,144);
   // Dimension controls remain outside the board even at maximum size.
   for(let i=0;i<2;i++){await click(-54,118);await click(-54,214)}
   await click(96,112);await click(276,110);await click(96,112);
   await shot('editor');
   await fresh();await click(248,174);
   assert.ok((await text()).includes(touch?'Tap an adjacent square':'Click an adjacent square'));
   // Select the enemy. No direction controls appear and no turn is sent.
   await click(192,112);
   let idle=await highlights();assert.equal(idle.length,8);assert.ok(idle.every(s=>s.sx===64));
   assert.ok(await page.evaluate(()=>!sprites.some(s=>s.sy===144&&s.sx>=160&&s.w===16)),'enemy has no movement arrows');
   assert.equal(await page.evaluate(()=>sprites.some(s=>s.sx===64&&s.sy===8&&s.w===16)),!touch,'touch devices have no cursor');
   await shot('enemy');
   const jobs=await page.evaluate(()=>window.jobs.length);
   await click(192,80);await click(192,80);
   assert.equal(await page.evaluate(()=>window.jobs.length),jobs,'enemy cannot move');
   await click(96,112);
   idle=await highlights();assert.equal(idle.length,8);assert.ok(idle.every(s=>s.sx===64));
   await shot('idle-pattern');
   await click(128,112);if(touch)await click(128,112);
   await page.waitForFunction(()=>words.includes('Mages have different attack patterns.'), undefined, {polling:50});
   await shot('attacking');
   await click(276,touch?92:188);
   await page.waitForFunction(()=>words.includes('Use the back arrow to undo a move.'), undefined, {polling:50});
   await shot('undo');
   await click(276,touch?92:188);
   await page.waitForFunction(()=>words.includes('Deal the final blow!'), undefined, {polling:50});
   // The undo control remains available after leaving the lesson.
   await click(-24,144);await click(-24,144);await page.waitForTimeout(700);
   assert.ok((await text()).includes(touch?'Tap the Red Mage.':'Click the Red Mage.'));
   // Replay a deterministic native tutorial win with controlled opponent replies.
   const file=fs.readdirSync(replayRoot).find(f=>f.startsWith('matchup-')&&f.endsWith('.json'));
   const replay=JSON.parse(fs.readFileSync(`${replayRoot}/${file}`)).games[0].replay;
   for(const step of replay) {
    if(step.team==='Red') {
     for(const [i,[x,y]] of step.turn.entries()) {
      await click(64+x*32,80+y*32);
      if(touch && i===1) await click(64+x*32,80+y*32);
     }
    } else {
     await page.waitForFunction(()=>jobs.some(j=>j.request&&!j.terminated), undefined, {polling:50});
     await page.evaluate(turn=>{const j=jobs.findLast(j=>j.request&&!j.terminated);j.onmessage({data:{id:j.request.id,revision:j.request.revision,selected:turn}})},step.turn);
     await page.waitForTimeout(800);
    }
   }
   await page.waitForTimeout(1000);
   await shot('replay-end');
   await page.waitForFunction(()=>words.includes('Continue'), undefined, {polling:50});
   await page.waitForTimeout(2400);await click(128,160);
   await page.waitForFunction(()=>words.includes('Diagonal rune'), undefined, {polling:50});await shot('diagonal-slide');
   await click(194,232);assert.ok((await text()).includes('even on their turn.'));await shot('shield-slide');
   await click(194,232);assert.ok((await text()).includes('including allies.'));await shot('beam-slide');
   await click(60,232);assert.ok((await text()).includes('Shield'));
   await click(194,232);await click(194,232);assert.ok((await text()).includes('Campaign'));
   // Campaign panning never loses every displayed portal, including long drags.
   await click(248,80);
   const drag=async(from,to)=>{
    const a=point(...from),b=point(...to);
    if(touch) {
     const send=async(type,p)=>page.evaluate(({type,p})=>{const target=document.querySelector('#game-canvas');const t=new Touch({identifier:1,target,clientX:p.x,clientY:p.y});target.dispatchEvent(new TouchEvent(type,{bubbles:true,cancelable:true,touches:type==='touchend'?[]:[t],changedTouches:[t]}))},{type,p});
     await send('touchstart',a);await page.waitForTimeout(50);await send('touchmove',b);await page.waitForTimeout(50);await send('touchend',b);
    } else {await page.mouse.move(a.x,a.y);await page.mouse.down();await page.mouse.move(b.x,b.y,{steps:4});await page.waitForTimeout(60);await page.mouse.up()}
    await page.waitForTimeout(500);
   };
   for(let i=0;i<10;i++) {
    await drag([128,48],[128,180]);
    assert.ok(await page.evaluate(()=>sprites.some(s=>s.w===64&&s.sx>=256&&Math.abs(s.x+32-200)<=65&&Math.abs(s.y+32-136)<=65)),'north pan retains a portal');
   }
   for(let i=0;i<10;i++)await drag([128,180],[128,48]);
   await shot('campaign-bounds');
   assert.deepEqual(errors,[]);await page.close();
  }
  console.log('PASS: mouse/touch settings, editor, enemy inspection, neutral patterns, cursor, tutorial/undo/item slides, and campaign bounds');
 } finally {await browser.close();if(!process.env.TUTORIAL_REPLAY)fs.rmSync(replayRoot,{recursive:true,force:true})}
})().catch(e=>{console.error(e);process.exit(1)});
