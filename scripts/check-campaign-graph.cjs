// Uses the shared catalogue export; observe actual canvas labels and connection strokes.
const {chromium}=require('playwright-core');
const assert=require('assert/strict');
const graph=JSON.parse(require('child_process').execFileSync('cargo',['run','--quiet','-p','shared','--example','campaign_catalogue'],{encoding:'utf8',stdio:['ignore','pipe','inherit']}));
const canonical=code=>{
 const alphabet='0123456789abcdefghjkmnpqrstvwxyz';
 let bits=[...code].map(c=>alphabet.indexOf(c).toString(2).padStart(5,'0')).join('');
 let bytes=Array.from({length:Math.floor(bits.length/8)},(_,i)=>parseInt(bits.slice(i*8,i*8+8),2));
 const start=3+3*bytes[1], props=[];for(let i=start;i<bytes.length;i+=2)props.push(bytes.slice(i,i+2));
 bytes=[...bytes.slice(0,start),...props.sort((a,b)=>a[0]-b[0]).flat()];
 bits=bytes.map(b=>b.toString(2).padStart(8,'0')).join('');bits+='0'.repeat((5-bits.length%5)%5);
 return bits.match(/.{5}/g).map(b=>alphabet[parseInt(b,2)]).join('');
};
for(const edge of graph.connections){const a=graph.catalogue.find(e=>e.id===edge.from).position,b=graph.catalogue.find(e=>e.id===edge.to).position;assert.equal(Math.abs(a[0]-b[0])+Math.abs(a[1]-b[1]),1,'cardinal neighbour connection');}
(async()=>{
 const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 for(const [won,target,available] of [
  ['diagonals-i','diagonals-ii',true],
  ['diagonals-iii','beams-i',true],
  ['beams-i','diagonals-iii',false],
  ['shields-i','junction-i',true],
  ['junction-i','challenge-iii',true],
  ['challenge-iii','junction-ii',true],
  ['junction-v','rite-iv',true],
  ['rite-iv','junction-v',false],
  ['junction-viii','rite-iv',true],
  ['rite-iv','junction-viii',false],
  ['shields-ii','challenge-i',false],
 ]) {
  const page=await browser.newPage({viewport:{width:1000,height:700}});const errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.route('https://tunnel.evrim.zone/**',r=>r.request().url().endsWith('/session')?r.fulfill({json:{session_id:'graph-check'}}):r.abort());
  const codes=graph.catalogue.filter(e=>['tutorial',won].includes(e.id)).map(e=>canonical(e.code));
  await page.addInitScript(codes=>{
   localStorage.clear();for(const c of codes)localStorage.setItem(c,'win');
   window.jobs=[];window.Worker=class{constructor(){jobs.push(this)}postMessage(r){this.request=r}terminate(){}};
   window.glyphs=[];window.paths=[];
   const proto=CanvasRenderingContext2D.prototype, draw=proto.drawImage, begin=proto.beginPath, move=proto.moveTo,line=proto.lineTo,stroke=proto.stroke;
   proto.drawImage=function(...a){if(a.length===9&&a[3]===8&&a[4]===8&&a[2]>=216&&a[2]<=272){glyphs.push(String.fromCharCode(((a[2]-216)/8)*32+a[1]/8));if(glyphs.length>2000)glyphs.splice(0,1000)}return draw.apply(this,a)};
   proto.beginPath=function(){this.trace=[];return begin.apply(this,arguments)};
   proto.moveTo=function(x,y){this.trace?.push(['M',x,y]);return move.apply(this,arguments)};
   proto.lineTo=function(x,y){this.trace?.push(['L',x,y]);return line.apply(this,arguments)};
   proto.stroke=function(){if(this.trace?.length===5){paths.push(this.trace);if(paths.length>100)paths.shift()}return stroke.apply(this,arguments)};
  },codes);
  await page.goto('http://127.0.0.1:8000/',{waitUntil:'domcontentloaded'});await page.waitForFunction(()=>jobs.length);
  const box=await page.locator('#game-canvas').boundingBox();
  const xy=(x,y)=>[box.x+(x+72)*box.width/400,box.y+(y+8)*box.height/272];
  const click=async(x,y)=>{await page.mouse.click(...xy(x,y),{delay:100});await page.waitForTimeout(150)};
  await click(248,80);
  const pos=graph.catalogue.find(e=>e.id===target).position;
  // Drag by one map tile per gesture, keeping the pointer on the canvas.
  for(let x=0;x<pos[0];x++){
   await page.mouse.move(...xy(196,128));await page.mouse.down();await page.mouse.move(...xy(68,128),{steps:4});await page.waitForTimeout(30);await page.mouse.up();await page.waitForTimeout(50);
  }
  for(let y=0;y<Math.abs(pos[1]);y++){
   const start=pos[1]<0?56:184,end=pos[1]<0?184:56;
   await page.mouse.move(...xy(128,start));await page.mouse.down();await page.mouse.move(...xy(128,end),{steps:4});await page.waitForTimeout(30);await page.mouse.up();await page.waitForTimeout(50);
  }
  await page.waitForTimeout(500);await page.evaluate(()=>glyphs=[]);await page.waitForTimeout(100);
  const drawn=await page.evaluate(()=>glyphs.join(''));
  assert(drawn.includes(available?'Battle':'Locked'),`${won} → ${target}: ${drawn.slice(-300)}`);
  const arrows=await page.evaluate(()=>paths);
  assert(arrows.length>0,'one-way arrowheads drawn');
  for(const path of arrows)assert(path[0][1]===path[1][1]||path[0][2]===path[1][2],'arrow shaft is cardinal');
  assert(drawn.includes('2/36'),'star count includes junction battles');
  await page.screenshot({path:`/tmp/maginet-graph-${target}-${available}.png`});
  await click(128,204);
  if(!available){await page.evaluate(()=>glyphs=[]);await page.waitForTimeout(100);assert((await page.evaluate(()=>glyphs.join(''))).includes('Locked'));}
  assert.deepEqual(errors,[]);await page.close();
 }
 await browser.close();console.log('graph arrows, loop entrances, shortcuts and blocked reverse unlocks passed');
})().catch(e=>{console.error(e);process.exit(1)});
