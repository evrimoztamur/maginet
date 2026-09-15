// Rendered browser checks; run against a dev web build at DRAG_URL.
const {chromium}=require('playwright-core');
const assert=require('node:assert/strict');
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 try {
 for(const team of ['Red','Blue']) for(const mode of ['Local','AI']) {
  const page=await browser.newPage({viewport:{width:1000,height:700}}),errors=[];
  page.on('pageerror',e=>errors.push(e.message));
  await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'side-test'}}):r.abort());
  await page.addInitScript(()=>{
   window.jobs=[];window.sprites=[];window.spriteSamples=[];
   window.Worker=class {postMessage(request){this.request=request;window.jobs.push(this)} terminate(){this.terminated=true}};
   const draw=CanvasRenderingContext2D.prototype.drawImage,raf=window.requestAnimationFrame;
   window.requestAnimationFrame=f=>raf.call(window,t=>{window.sprites=[];f(t);window.spriteSamples.push(structuredClone(window.sprites));if(window.spriteSamples.length>120)window.spriteSamples.shift()});
   CanvasRenderingContext2D.prototype.drawImage=function(source,...a){
    const t=this.getTransform();
    if(a[2]===32 && a[3]===40 && [64,104].includes(a[1])) window.sprites.push({team:a[1]===64?'Red':'Blue',x:t.e,y:t.f,upright:t.d>0,sort:a[0]});
    return draw.call(this,source,...a);
   };
  });
  await page.goto(process.env.DRAG_URL || 'http://127.0.0.1:8789/html/game.html');await page.waitForSelector('#game-canvas');
  const box=await page.locator('#game-canvas').boundingBox();
  const click=async(x,y)=>{await page.mouse.click(box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272,{delay:30});await page.waitForTimeout(100)};
  await click(248,110);
  const checkJumping=async(selected)=>{
   await page.evaluate(()=>spriteSamples=[]);
   await page.waitForTimeout(500);
   const samples=await page.evaluate(()=>spriteSamples);
   for(const side of ['Red','Blue']) {
    const heights=new Set(samples.flatMap(frame=>frame.filter(s=>s.team===side && s.sort===0).map(s=>s.y)));
    assert.equal(heights.size>1,side===selected,`${selected} selected: only that row jumps`);
   }
  };
  await checkJumping('Red');
  // Select by tile row; mage artwork above the tiles must not switch teams.
  await click(172,160);await checkJumping('Blue');
  await click(172,108);await checkJumping('Blue');
  await click(172,128);await checkJumping('Red');
  await click(172,team==='Red'?128:160);
  await checkJumping(team);
  if(process.env.SIDE_SCREENSHOTS) await page.screenshot({path:`${process.env.SIDE_SCREENSHOTS}/${team}-${mode}-menu.png`});
  const preview=await page.evaluate(()=>sprites.map(s=>[s.team,s.sort]));
  await click(128,80);await click(48,80); // Mode changes retain the preview and selection.
  assert.deepEqual(await page.evaluate(()=>sprites.map(s=>[s.team,s.sort])),preview);
  await click(48,121); // Changing loadout retains the selected team.
  if(mode==='AI') await click(128,80);
  await checkJumping(team);
  await page.evaluate(()=>jobs=[]);
  await click(128,200);
  await page.waitForTimeout(950);
  const initial=await page.evaluate(()=>sprites);
  assert.equal(initial.length,8);
  assert(initial.every(s=>s.upright));
  const redY=initial.find(s=>s.team==='Red').y,blueY=initial.find(s=>s.team==='Blue').y;
  assert.equal(redY>blueY,team==='Red');
  const point=(x,y)=>[48+32*x,48+32*(team==='Blue'?5-y:y)];
  if(mode==='AI' && team==='Blue') {
   assert.equal(await page.evaluate(()=>jobs.length),1,'red AI opens for blue human');
   assert.equal(await page.evaluate(()=>JSON.parse(jobs[0].request.snapshot).turns.length),0);
   await page.evaluate(()=>{const j=jobs[0],g=JSON.parse(j.request.snapshot);j.onmessage({data:{...j.request,selected:g.available_turns[0]}})});
   await page.waitForTimeout(800);
   assert.equal(await page.evaluate(()=>jobs.length),1,'AI stops for blue human');
  } else {
   assert.equal(await page.evaluate(()=>jobs.length),0);
   // Default red mage at (1,5), move toward the center.
   await click(...point(1,5));await click(...point(1,4));
   await page.waitForTimeout(1100);
   if(mode==='AI') {
    assert.equal(await page.evaluate(()=>jobs.length),1,'blue AI replies to red human');
    assert.equal(await page.evaluate(()=>JSON.parse(jobs[0].request.snapshot).turns.length),1);
   }
  }
  if(mode==='Local' || team==='Blue') {
   const beforeBlue=await page.evaluate(()=>sprites.find(s=>s.team==='Blue' && s.sort===0));
   await click(...point(1,0));await click(...point(1,1));await page.waitForTimeout(1100);
   const movedBlue=await page.evaluate(()=>sprites.find(s=>s.team==='Blue' && s.sort===0));
   assert(Math.abs(movedBlue.y-beforeBlue.y)>25,'human can move blue on its turn');
   if(mode==='AI') {
    assert.equal(await page.evaluate(()=>jobs.length),2);
    assert.equal(await page.evaluate(()=>JSON.parse(jobs[1].request.snapshot).turns.length),2);
   }
  }
  const after=await page.evaluate(()=>sprites);
  assert(after.every(s=>s.upright));
  assert.equal(after.find(s=>s.team==='Red').y>after.find(s=>s.team==='Blue').y,team==='Red');
  await click(-16,140);await page.waitForTimeout(500);
  assert(Math.abs(await page.evaluate(()=>sprites.find(s=>s.team==='Red').y)-redY)<=2,'undo keeps perspective');
  await click(-16,108);await click(128,116);await page.waitForTimeout(100);
  assert(Math.abs(await page.evaluate(()=>sprites.find(s=>s.team==='Red').y)-redY)<=2,'rematch keeps perspective');
  if(process.env.SIDE_SCREENSHOTS) await page.screenshot({path:`${process.env.SIDE_SCREENSHOTS}/${team}-${mode}.png`});
  assert.deepEqual(errors,[]);
  console.log(`PASS: ${team} ${mode}, selection, upright sprites, fixed view, turn assignment, undo and rematch`);
  await page.close();
 }
 } finally {await browser.close()}
})().catch(e=>{console.error(e);process.exit(1)});
