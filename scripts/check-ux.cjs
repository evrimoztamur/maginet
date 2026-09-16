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
      const t=this.getTransform();sprites.push({sx:a[0],sy:a[1],w:a[2],h:a[3],x:t.e,y:t.f,scale:t.a});
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
   await click(-6,78);assert.equal(await page.evaluate(()=>localStorage.getItem('music_volume')),'9');
   await click(134,78);assert.equal(await page.evaluate(()=>localStorage.getItem('music_volume')),'10');
   await click(-6,124);assert.equal(await page.evaluate(()=>localStorage.getItem('clip_volume')),'7');
   for(const [x,label] of [[6,'Easy'],[64,'Normal'],[122,'Hard']]) {
    await click(x,171);assert.equal(await page.evaluate(()=>localStorage.getItem('difficulty')),label);
   }
   await click(108,222);assert.equal(await page.evaluate(()=>localStorage.getItem('onscreen_controls')),'off');
   await click(32,222);assert.equal(await page.evaluate(()=>localStorage.getItem('onscreen_controls')),'on');
   const controls=await page.evaluate(()=>labels.filter(b=>['#006080','#007faa','#008080'].includes(b.color)&&b.w>=20&&b.h>=20));
   assert.equal(controls.length,10,'audio, difficulty, controls and Back are visible');
   for(let i=0;i<controls.length;i++)for(let j=i+1;j<controls.length;j++) {
    const a=controls[i],b=controls[j];
    assert.ok(Math.max(b.x-a.x-a.w,a.x-b.x-b.w,b.y-a.y-a.h,a.y-b.y-b.h)>=8,'settings buttons retain an eight-pixel gutter');
   }
   await click(296,36);assert.ok((await text()).includes('Press three more timesto reset campaign!'));
   await shot('settings');
   await click(64,246);await click(248,144);
   await click(96,112);await click(276,116);
   await shot('editor');
   // Growing the board preserves selection and keeps its edge controls clear.
   for(let i=0;i<2;i++){await click(110,246);await click(212,142)}
   const board=await page.evaluate(()=>sprites.find(s=>s.sx===256&&s.sy===0&&s.w===256&&s.scale===0.75));
   assert.ok(board,'large editor boards fit inside the dimension controls');
   const dimensions=await page.evaluate(()=>labels.filter(b=>b.w===20&&b.h===20&&((b.x===144||b.x===172)&&b.y===244||b.x===274)));
   assert.equal(dimensions.length,4);
   for(const b of dimensions)assert.ok(b.x>=board.x+192+8||b.y>=board.y+192+8,'edge buttons do not cover tiles');
   // Opposite corners must still map to the correct tiles after the board fits.
   await click(180,212);await click(276,116);
   await click(12,44);await click(276,148);
   await shot('editor-large');
   await click(276,207);
   const saved=await page.evaluate(()=>JSON.parse(localStorage.getItem('levels'))['0']);
   assert.equal(saved.board.width,8);assert.equal(saved.board.height,8);
   assert.deepEqual(saved.mages.map(m=>m.position),[[3,2],[7,7]]);
   assert.deepEqual(saved.powerups,[[[0,0],'Diagonal']]);
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
   await page.waitForFunction(()=>words.includes('Mages attack when they move.'), undefined, {polling:50});
   assert.ok((await text()).includes('Back arrow: twice to undo.'));
   assert.ok(!(await text()).includes('Next'),'battle lessons never require Next');
   assert.ok(!(await text()).includes('Deal the final blow!'),'nonlethal opening stays in Attacking');
   const title=await page.evaluate(()=>labels.find(b=>b.color==='#557f55'&&b.w===96));
   const hint=await page.evaluate(()=>labels.find(b=>b.color==='#002a2a'));
   assert.equal(72-title.y-title.h,16,'title has a sixteen-pixel margin above the board');
   assert.equal(hint.y-200,16,'instructions have a sixteen-pixel margin below the board');
   await shot('attacking');
   // Undo remains available while the opponent is thinking.
   await click(-24,144);await click(-24,144);await page.waitForTimeout(700);
   assert.ok((await text()).includes(touch?'Tap the Red Mage.':'Click the Red Mage.'));
   // Replay a deterministic native tutorial win with controlled opponent replies.
   const file=fs.readdirSync(replayRoot).find(f=>f.startsWith('matchup-')&&f.endsWith('.json'));
   const replay=JSON.parse(fs.readFileSync(`${replayRoot}/${file}`)).games[0].replay;
   for(const [index,step] of replay.entries()) {
    if(index===6) {
     await click(128,112);
     if(touch)await click(128,144);
     else {const p=point(128,144);await page.mouse.move(p.x,p.y);await page.waitForTimeout(100)}
     const frame=await page.evaluate(()=>({sprites,labels}));
     const diamond=frame.sprites.findIndex(s=>s.sx===32&&s.sy===256&&s.w===32);
     const mage=frame.sprites.findIndex(s=>s.sy===104&&s.w===32&&s.h===40);
     const pips=frame.sprites.findIndex(s=>s.sx>=64&&s.sx<=72&&s.sy>=32&&s.sy<=40&&s.w===8);
     assert.ok(diamond>=0&&diamond<mage&&mage<pips,'red diamond stays below the enemy; targeting pips stay above');
     assert.ok(frame.sprites.some(s=>s.sx===160&&s.sy===144&&s.w===16),'attack arrows are white');
     assert.ok(frame.sprites.some(s=>s.sx===0&&s.sy===32&&s.w===16),'ordinary movement arrows are cyan');
     if(touch) {
      const pad=frame.labels.filter(b=>b.w===28&&b.h===28);
      assert.ok(pad.some(b=>b.color==='#007faa'),'selected attack has the brighter background');
      assert.ok(pad.some(b=>b.color==='#003e60'),'ordinary movement has the darker background');
     }
     await shot('attack-preview');
    }
    if(index===replay.length-1)assert.ok((await text()).includes('Deal the final blow!'),'winning move receives the final-blow hint');
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
