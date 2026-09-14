const {chromium} = require('playwright-core');
const {execFileSync} = require('child_process');
const path = require('path');
const assert = require('assert/strict');
(async () => {
 const browser = await chromium.launch({executablePath:process.env.CHROME_PATH || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
 const page = await browser.newPage({viewport:{width:1000,height:700}});
 const errors=[]; page.on('pageerror', e=>{errors.push(e.message); console.log('ERROR',e.message)}); page.on('console', m=>{if(m.type()==='error') console.log(m.text())});
 await page.route('https://tunnel.evrim.zone/**', route => route.request().url().endsWith('/session') ? route.fulfill({json:{session_id:'solver-check'}}) : route.abort());
 await page.goto('http://127.0.0.1:8000/', {waitUntil:'domcontentloaded'});
 await page.waitForSelector('#game-canvas');
 const snapshots = execFileSync('cargo',['run','--quiet','-p','shared','--example','search_bench','--','--snapshot'],{cwd:path.join(__dirname,'..'),encoding:'utf8',stdio:['ignore','pipe','ignore']}).trim().split('\n');
 const result = await page.evaluate(async snapshot => {
   const api = await import('/static/js/ai-client.js');
   let frames=0; const timer=setInterval(()=>frames++,10);
   const job=api.startSearch(snapshot,'Hard','42',7,3);
   const started=performance.now();
   let raw;
   while (!(raw=api.pollSearch(job))) { if(performance.now()-started>10000) throw Error('worker timeout'); await new Promise(r=>setTimeout(r,10)); }
   api.cancelSearch(job); clearInterval(timer);
   return { reply:JSON.parse(raw), frames, elapsed:performance.now()-started };
 }, snapshots[1]);
 assert.equal(result.reply.failed,undefined); assert.ok(result.reply.selected); assert.ok(result.reply.stats.visited_nodes>0); assert.ok(result.frames>5);
 console.log('real worker',JSON.stringify(result));
 const lifecycle = await page.evaluate(async snapshot => {
   const api=await import('/static/js/ai-client.js'); const Original=window.Worker;
   const instances=[];
   window.Worker=class { constructor(){instances.push(this)} postMessage(value){this.request=value} terminate(){this.terminated=true} };
   const job=api.startSearch(snapshot,'Normal','42',10,4); const worker=instances[0];
   worker.onmessage({data:{id:9,revision:4,selected:[[0,0],[1,0]]}});
   if(api.pollSearch(job)!==null) throw Error('stale id accepted');
   worker.onmessage({data:{id:10,revision:3,selected:[[0,0],[1,0]]}});
   if(api.pollSearch(job)!==null) throw Error('stale revision accepted');
   api.cancelSearch(job);
   if(!worker.terminated || worker.onmessage!==null || worker.onerror!==null) throw Error('owner did not release worker');
   const failed=api.startSearch(snapshot,'Normal','42',11,5); instances[1].onerror();
   if(!JSON.parse(api.pollSearch(failed)).failed) throw Error('failure not surfaced');
   api.cancelSearch(failed);
   window.Worker=Original; return true;
 }, snapshots[0]);
 assert.ok(lifecycle); assert.deepEqual(errors,[]);
 console.log('stale identifiers, revisions, callback cleanup, and worker failure: passed');
 await browser.close();
})().catch(e=>{console.error(e);process.exit(1)});
