// Campaign introductions remain playable while their first-turn explanation is visible.
const {chromium}=require('playwright-core');
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path');
const root=path.join(__dirname,'..');
const url=process.env.DRAG_URL||'http://127.0.0.1:8790/html/game.html';
const native=!!process.env.MOBILE_PKG;
const graph=JSON.parse(require('node:child_process').execFileSync('cargo',['run','--quiet','-p','shared','--example','campaign_catalogue'],{cwd:root,encoding:'utf8',stdio:['ignore','pipe','inherit']}));
const out=path.join(root,'target/ux-learning');fs.mkdirSync(out,{recursive:true});
const cases=[
 {id:'diagonals-i',title:'Diagon Rune',text:'This rune allows you to movediagonally, too.',size:[4,4],turn:[[0,1],[1,1]],item:'Diagonal'},
 {id:'beams-i',title:'Beam Crystal',text:'This crystal discharges astrong cardinal beam, hurtingall in its way.',size:[5,4],turn:[[3,3],[3,2]],item:'Beam'},
 {id:'shields-i',title:'Shield Rune',text:'This rune reflects attacksto the enemy.',size:[4,3],turn:[[1,2],[0,2]],item:'Shield',detour:true},
];
function routeTo(target) {
 const queue=[[[0,0]]],seen=new Set(['0,0']);
 while(queue.length) {
  const route=queue.shift(),last=route.at(-1);
  if(last.join()===target.join())return route.slice(1).map((p,i)=>[p[0]-route[i][0],p[1]-route[i][1]]);
  for(const [dx,dy] of [[1,0],[0,1],[-1,0],[0,-1]]) {
   const next=[last[0]+dx,last[1]+dy];
   if(!seen.has(next.join())&&graph.catalogue.some(e=>e.position.join()===next.join())) {seen.add(next.join());queue.push([...route,next])}
  }
 }
 throw new Error('No route to campaign introduction');
}
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true,args:['--disable-gpu','--disable-accelerated-2d-canvas']});
 try {
  for(const touch of native?[true]:[false,true])for(const lesson of cases) {
   const page=await browser.newPage({viewport:native?{width:844,height:390}:{width:1000,height:700},deviceScaleFactor:native?2:1,hasTouch:touch});
   const errors=[];page.on('pageerror',e=>errors.push(e.message));
   await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'learning-check'}}):r.abort());
   if(native) {
    await page.route('**/static/js/pkg/**',r=>{const file=new URL(r.request().url()).pathname.split('/static/js/pkg/')[1];return r.fulfill({path:path.join(process.env.MOBILE_PKG,file),contentType:file.endsWith('.wasm')?'application/wasm':'text/javascript'})});
    await page.route('**/learning-check.html',r=>r.fulfill({path:path.join(root,'html/ios.html'),contentType:'text/html'}));
    await page.route('**/api/session',r=>r.fulfill({json:{session_id:'learning-check'}}));
   }
   await page.addInitScript(codes=>{
    for(const code of codes)localStorage.setItem(code,'win');
    window.maginetSafeLeft=47;window.maginetSafeRight=47;window.maginetSafeBottom=21;
    window.maginetNative={postMessage(body){if(JSON.parse(body).action==='state')window.dispatchEvent(new CustomEvent('maginet-access',{detail:true}))}};
    window.jobs=[];window.Worker=class{constructor(){jobs.push(this)}postMessage(request){this.request=request}terminate(){this.terminated=true}};
    const draw=CanvasRenderingContext2D.prototype.drawImage,fill=CanvasRenderingContext2D.prototype.fillRect,raf=window.requestAnimationFrame;
    window.words='';window.labels=[];window.buttonFlashed=false;
    window.requestAnimationFrame=f=>raf.call(window,t=>{words='';labels=[];f(t)});
    CanvasRenderingContext2D.prototype.fillRect=function(x,y,w,h){const t=this.getTransform();labels.push({x:t.e+x,y:t.f+y,w,h,color:this.fillStyle});if(w===24&&h===24&&this.globalCompositeOperation==='lighter')buttonFlashed=true;return fill.call(this,x,y,w,h)};
    CanvasRenderingContext2D.prototype.drawImage=function(source,...a){
     if(a.length===8&&source.width===512&&a[2]===8&&a[3]===8&&a[1]>=224&&a[1]<=240)words+=String.fromCharCode((a[1]-216)/8*32+a[0]/8);
     return draw.call(this,source,...a);
    };
   },graph.catalogue.map(e=>e.code));
   await page.goto(native?new URL('/learning-check.html',url).href:url);
   const wait=word=>page.waitForFunction(word=>words.includes(word),word,{polling:50});
   await wait('Campaign');
   const box=await page.locator('#game-canvas').boundingBox();
   const point=(x,y)=>native?{x:(x+167)*390/272,y:(y+1)*390/272}:{x:box.x+(x+72)*box.width/400,y:box.y+(y+8)*box.height/272};
   const click=async(x,y)=>{const p=point(x,y);if(touch)await page.touchscreen.tap(p.x,p.y);else await page.mouse.click(p.x,p.y,{delay:30});await page.waitForTimeout(350)};
   await click(248,80);
   const entry=graph.catalogue.find(e=>e.id===lesson.id);
   for(const [dx,dy] of routeTo(entry.position)) {
    const a=point(128+dx*64,128+dy*64),b=point(128-dx*64,128-dy*64);
    if(touch) {
     for(const [type,p] of [['touchstart',a],['touchmove',b],['touchend',b]]) {
      await page.evaluate(({type,p})=>{const target=document.querySelector('#game-canvas');const t=new Touch({identifier:1,target,clientX:p.x,clientY:p.y});target.dispatchEvent(new TouchEvent(type,{bubbles:true,cancelable:true,touches:type==='touchend'?[]:[t],changedTouches:[t]}))},{type,p});await page.waitForTimeout(60);
     }
    } else {await page.mouse.move(a.x,a.y);await page.mouse.down();await page.mouse.move(b.x,b.y,{steps:4});await page.waitForTimeout(60);await page.mouse.up()}
    await page.waitForTimeout(350);
   }
   await click(128,204);await wait(lesson.title);
   await page.evaluate(()=>buttonFlashed=false);await page.waitForTimeout(1100);
   assert.equal(await page.evaluate(()=>buttonFlashed),false,'ordinary campaign battles never blink the undo button');
   assert.ok(await page.evaluate(text=>words.includes(text),lesson.text),'short explanation accompanies the board');
   assert.ok(await page.evaluate(()=>!words.includes('Next')),'opening lesson needs no modal or Next');
   const oy=(8-lesson.size[1])*16,ox=(8-lesson.size[0])*16;
   const labels=await page.evaluate(()=>window.labels);
   const title=labels.find(b=>b.color==='#557f55'&&b.w===96),body=labels.find(b=>b.color==='#002a2a');
   assert.equal(title.y,oy-40+(native?1:8));assert.equal(body.y,oy+lesson.size[1]*32+16+(native?1:8));
   const tile=([x,y])=>[ox+16+x*32,oy+16+y*32];
   await click(...tile(lesson.turn[0]));await wait(lesson.title);
   await page.screenshot({path:path.join(out,`${lesson.id}-${native?'mobile':touch?'touch':'mouse'}.png`)});
   await click(...tile(lesson.turn[1]));if(touch)await click(...tile(lesson.turn[1]));
   await page.waitForFunction(()=>jobs.some(j=>j.request&&!j.terminated),undefined,{polling:50});
   const snapshot=await page.evaluate(()=>JSON.parse(jobs.findLast(j=>j.request&&!j.terminated).request.snapshot));
   assert.deepEqual(snapshot.turns,[lesson.turn],'pickup can be played directly while the hint is visible');
   if(lesson.detour)assert.ok(snapshot.level.powerups.some(([p,item])=>p.join()==='0,1'&&item==='Shield'),'shield remains available after the safe opening detour');
   else assert.ok(!snapshot.level.powerups.some(([p])=>p.join()===lesson.turn[1].join()),'item was collected');
   assert.ok(await page.evaluate(title=>!words.includes(title),lesson.title),'hint ends after the first move');
   await click(-24,108);await click(128,116);await click(128,116);await wait(lesson.title);
   assert.deepEqual(errors,[]);await page.close();
  }
  console.log('PASS: campaign powerup headings/copy, board spacing, playable first-turn openings, dismissal, and rematches');
 } finally {await browser.close()}
})().catch(e=>{console.error(e);process.exit(1)});
