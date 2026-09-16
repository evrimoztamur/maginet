// Follow visible neighbouring portals: the map snaps away from empty regions.
module.exports=async function navigate(page,xy,entries,target) {
 const queue=[[[0,0]]],seen=new Set(['0,0']);let route;
 while(queue.length){
  const current=queue.shift(),last=current.at(-1);
  if(last[0]===target[0]&&last[1]===target[1]){route=current;break}
  for(const entry of entries){
   const p=entry.position,key=p.join(',');
   if(seen.has(key)||Math.abs(p[0]-last[0])+Math.abs(p[1]-last[1])!==1)continue;
   seen.add(key);queue.push([...current,p]);
  }
 }
 if(!route)throw new Error(`No visible portal route to ${target}`);
 for(let i=1;i<route.length;i++){
  const dx=route[i][0]-route[i-1][0],dy=route[i][1]-route[i-1][1];
  const start=dx?[128+dx*64,128]:[128,dy>0?184:56];
  const end=[start[0]-dx*128,start[1]-dy*128];
  await page.mouse.move(...xy(...start));await page.mouse.down();await page.waitForTimeout(50);
  await page.mouse.move(...xy(...end),{steps:8});await page.waitForTimeout(50);
  await page.mouse.up();await page.waitForTimeout(350);
 }
};
