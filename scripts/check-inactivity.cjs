// Real editor -> local battle check of the eight full-turn inactivity pips.
const {chromium}=require('playwright-core');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const alphabet='0123456789abcdefghjkmnpqrstvwxyz';
function code(bytes){let bits=bytes.map(b=>b.toString(2).padStart(8,'0')).join('');bits=bits.padEnd(Math.ceil(bits.length/5)*5,'0');return bits.match(/.{5}/g).map(b=>alphabet[parseInt(b,2)]).join('')}
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 try{
  const page=await browser.newPage({viewport:{width:1000,height:700}});
  const errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'inactivity-check'}}):r.abort());
  await page.addInitScript(()=>{
   window.Worker=class{postMessage(){} terminate(){}};
   window.words='';window.pips=[];
   const raf=requestAnimationFrame,draw=CanvasRenderingContext2D.prototype.drawImage;
   window.requestAnimationFrame=f=>raf.call(window,t=>{words='';pips=[];f(t)});
   CanvasRenderingContext2D.prototype.drawImage=function(source,...a){
    if(a.length===8&&source.width===512){
     const t=this.getTransform();
     if(a[2]===8&&a[3]===8&&a[1]>=224&&a[1]<=240)words+=String.fromCharCode((a[1]-216)/8*32+a[0]/8);
     if(t.e===334&&a[2]===8&&((a[0]===128&&[0,8].includes(a[1])&&a[3]===8)||(a[0]===136&&a[1]===0&&a[3]===16)))pips.push({x:t.e,y:t.f,filled:a[0]===136||a[1]===8});
    }
    return draw.call(this,source,...a);
   };
  });
  await page.goto(process.env.DRAG_URL||'http://127.0.0.1:8792/html/game.html');
  await page.waitForFunction(()=>words.includes('Campaign'),undefined,{polling:50});
  const box=await page.locator('#game-canvas').boundingBox();
  const click=async(x,y,delay=80)=>{await page.mouse.click(box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272,{delay:30});await page.waitForTimeout(delay)};
  await click(248,144,400);
  const level=code([0xfc,2,0,4,0x44,0xfd,4,0x44,1,0x24,2]);
  await page.locator('input').evaluate((input,value)=>{input.dataset.field='level_code';input.value=value;input.focus()},level);
  await page.keyboard.press('Enter');await page.waitForTimeout(400);
  await click(280,242,400);await click(276,64,500);
  const output=process.env.INACTIVITY_OUTPUT||'assessments/campaign-draws/browser';
  fs.mkdirSync(output,{recursive:true});
  const samples=[];
  const check=async(ply,expected)=>{
   const sample=await page.evaluate(()=>({pips,words}));
   assert.equal(sample.pips.length,8,JSON.stringify(sample));
   const filled=sample.pips.filter(p=>p.filled).length;
   assert.equal(filled,expected,`ply ${ply}: expected ${expected} filled pips`);
   samples.push({ply,filled,stalemate:sample.words.includes('Stalemate')});
  };
  const positions=[[0,0],[7,7]];
  let ply=0;
  const move=async(to)=>{
   const side=ply%2,from=positions[side];
   await click(16+32*from[0],16+32*from[1]);
   await click(16+32*to[0],16+32*to[1],450);
   positions[side]=to;ply++;
  };
  const quiet=async()=>{
   const side=ply%2,[x,y]=positions[side],left=side===0?(y===0?0:1):6;
   await move([x===left?left+1:left,y]);
  };
  await check(0,0);
  for(let i=1;i<=22;i++){await quiet();await check(i,Math.floor(Math.max(0,i-7)/2))}
  await page.screenshot({path:path.join(output,'seven-turn-pips.png')});
  await move([1,1]);await check(23,0);
  await page.screenshot({path:path.join(output,'pickup-resets-pips.png')});
  for(let i=1;i<=16;i++){await quiet();await check(23+i,Math.floor(i/2))}
  await page.waitForFunction(()=>words.includes('Stalemate'),undefined,{polling:50});
  assert(samples.slice(0,-1).every(s=>!s.stalemate),'no early draw');
  assert.deepEqual(errors,[]);
  await page.screenshot({path:path.join(output,'eight-turn-stalemate.png')});
  fs.writeFileSync(path.join(output,'inactivity-pips.json'),JSON.stringify({level,samples},null,2)+'\n');
  console.log('PASS: eight pips advance once per two quiet plies, pickup resets all pips, draw occurs after exactly sixteen subsequent quiet plies');
 }finally{await browser.close()}
})().catch(e=>{console.error(e);process.exit(1)});
