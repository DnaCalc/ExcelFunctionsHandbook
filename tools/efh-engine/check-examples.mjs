#!/usr/bin/env node
// Smoke the actual starter formulas. This is not an Excel comparison or a Handbook vector suite.
import { readFile, writeFile } from "node:fs/promises";
import { resolve, join } from "node:path";
import { evaluate } from "../efh-api/server.mjs";
const root=resolve("site/build");
const index=JSON.parse(await readFile(join(root,"api/index.json"),"utf8"));
const records=[];let next=0;
await Promise.all(Array.from({length:2},async()=>{
  while(next<index.rows.length){
    const row=index.rows[next++];
    const record=JSON.parse(await readFile(join(root,`api/functions/${row.function_id}.json`),"utf8"));
    const request={...record.try_it,now:46000};
    let result;
    try {result=await evaluate(join(root,"assets/onecalc"),request);}
    catch(error){result={status:"host-error",message:error.message};}
    records.push({function_id:row.function_id,formula:request.formula,expected:record.try_it.preview_status||"calculated",status:result.status,diagnostics:result.diagnostics||[],message:result.message});
  }
}));
records.sort((a,b)=>a.function_id.localeCompare(b.function_id));
const failures=records.filter(row=>row.status!==row.expected);
const counts=Object.fromEntries(Object.entries(Object.groupBy(records,row=>row.status)).map(([status,rows])=>[status,rows.length]));
const report={entries:records.length,counts,failures,records};
if(process.argv[2]) await writeFile(process.argv[2],JSON.stringify(report,null,2)+"\n");
console.log(JSON.stringify({entries:report.entries,counts,failures},null,2));
if(records.length!==541||failures.length)process.exitCode=1;
