import test from "node:test";
import assert from "node:assert/strict";
import { mkdtemp, rm, readFile, stat, symlink } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { once } from "node:events";
import { DatabaseSync } from "node:sqlite";
import { createHandbookServer, RateLimiter, evaluate } from "./server.mjs";

const root = resolve(process.env.EFH_SITE_ROOT || "site/build");
const engine = join(root,"assets/onecalc");
async function fixture(t, options = {}) {
  const work = await mkdtemp(join(tmpdir(),"efh-api-test-"));
  const store = join(work,"private.sqlite");
  const server = await createHandbookServer({root,store,...options});
  await new Promise(resolve => server.listen(0,"127.0.0.1",resolve));
  t.after(async () => { server.close(); server.closeAllConnections(); await once(server,"close"); await rm(work,{recursive:true,force:true}); });
  const base = `http://127.0.0.1:${server.address().port}`;
  const post = async (route,data,headers={}) => {
    const res=await fetch(base+route,{method:"POST",headers:{"Content-Type":"application/json",...headers},body:JSON.stringify(data)});
    return {status:res.status,headers:res.headers,body:await res.json()};
  };
  return {store,base,post};
}

test("API runs the same wasm module as the browser; numbers, bits, arrays and cells survive transport", async t => {
  const {post}=await fixture(t);
  for (const request of [
    {formula:"=ROUND(1.005,2)"}, {formula:"=SQRT(2)"}, {formula:"=SEQUENCE(2,3)"},
    {formula:"=SUM(B2:B3)",inputs:{B2:3,B3:4}}, {formula:"=1/0"}, {formula:"=B2"},
    {formula:"=NOTAHANDBOOKFUNCTION(1)"}, {formula:"=DATE(2026,11,6)",date_system:"1904"},
    {formula:"=LAMBDA(x,x+1)(4)"},
  ]) {
    const res=await post("/api/eval",request);assert.equal(res.status,200,JSON.stringify(res.body));
    assert.deepEqual(res.body,await evaluate(engine,request));
  }
  const sum=await post("/api/eval",{formula:"=SUM(1,2,3)"});
  assert.equal(sum.body.value.number,6);assert.equal(sum.body.value.bits,"0x4018000000000000");
  const arr=await post("/api/eval",{formula:"=SEQUENCE(2,3)"});assert.equal(arr.body.value.cells[1][2].number,6);
  assert.equal((await post("/api/eval",{formula:"=B2"})).body.status,"refused");
  assert.equal((await post("/api/eval",{formula:"=1/0"})).body.status,"worksheet-error");
});

test("OneCalc UDF hook accepts an actual VBA module",async t => {
  const {post}=await fixture(t);
  const res=await post("/api/eval",{formula:"=HANDBOOKDOUBLE(3)",udf:{language:"vba",source:"Public Function HANDBOOKDOUBLE(x As Double) As Double\nHANDBOOKDOUBLE = x * 2\nEnd Function"}});
  assert.equal(res.status,200);assert.equal(res.body.value?.number,6,JSON.stringify(res.body));
});

test("invalid requests, origins, content types and methods are rejected",async t => {
  const {post,base}=await fixture(t);
  for(const data of [{},[],{formula:5},{formula:"=1",inputs:[]},{formula:"=1",date_system:1904},{formula:"=1",inputs:{A1:2}},{formula:"="+"1".repeat(4096)}]) assert.equal((await post("/api/eval",data)).status,400);
  assert.equal((await post("/api/eval",{formula:"=1"},{Origin:"https://elsewhere.example"})).status,403);
  assert.equal((await fetch(base+"/api/eval")).status,405);
  assert.equal((await fetch(base+"/api/eval",{method:"POST",body:"{}"})).status,415);
  assert.equal((await fetch(base+"/api/eval",{method:"POST",headers:{"Content-Type":"application/json"},body:"{"})).status,400);
  const big=await post("/api/eval",{formula:"=1",padding:"x".repeat(40000)});assert.equal(big.status,413);
});

test("rate limits cannot be bypassed with X-Forwarded-For", async t => {
  const {post}=await fixture(t,{evalLimit:2});
  assert.equal((await post("/api/eval",{formula:"=1"})).status,200);
  assert.equal((await post("/api/eval",{formula:"=2"})).status,200);
  const over=await post("/api/eval",{formula:"=3"},{"X-Forwarded-For":"new-client"});
  assert.equal(over.status,429);assert.equal(over.headers.get("Retry-After"),"60");
  const limited=new RateLimiter(1,100,2);assert.equal(limited.take("a",0),true);assert.equal(limited.take("a",1),false);assert.equal(limited.take("a",100),true);
});

test("worker timeout releases its slot for later requests",async t=>{
  const {post}=await fixture(t,{timeoutMs:1});
  assert.equal((await post("/api/eval",{formula:"=1"})).status,408);
  assert.equal((await post("/api/eval",{formula:"=2"})).status,408);
});

test("contributions persist in our private SQLite store with content preserved and no public reader",async t=>{
  const {post,base,store}=await fixture(t,{contributionLimit:2});
  const details="<script>alert('x')</script>\n=ROUND(1.005,2) is 1.01; quote ' ; --";
  const res=await post("/api/contributions",{function_id:"round",kind:"observation",excel_build:"16.0 build 20527 Windows",details,contact:"example@example.invalid"});
  assert.equal(res.status,201);assert.equal(res.body.status,"pending");
  const db=new DatabaseSync(store);const row=db.prepare("SELECT * FROM contributions WHERE id=?").get(res.body.id);db.close();
  assert.equal(row.details,details);assert.equal(row.function_id,"FUNC.ROUND");assert.equal(row.contact,"example@example.invalid");
  assert.equal((await stat(store)).mode & 0o777,0o600);
  assert.equal((await fetch(base+"/api/contributions")).status,405);
  assert.equal((await fetch(base+"/private.sqlite")).status,404);
  assert.equal((await post("/api/contributions",{function_id:"missing",kind:"correction",details:"something"})).status,400);
  assert.equal((await post("/api/contributions",{function_id:"ROUND",kind:"correction",details:"third"})).status,429);
});

test("static server serves wasm and stable JSON, rejects escapes and public-store configuration",async t=>{
  const {base}=await fixture(t);
  assert.match((await fetch(base+"/assets/onecalc/engine_bg.wasm")).headers.get("Content-Type"),/application\/wasm/);
  assert.match(await (await fetch(base+"/llms.txt")).text(),/FUNC.ROUND.json/);
  const record=await (await fetch(base+"/api/functions/FUNC.ROUND.json")).json();assert.equal(record.schema,"efh.function-reference/v1");
  assert.equal((await fetch(base+"/%2e%2e%2fREADME.md")).status,404);
  assert.equal((await fetch(base+"/api/%zz")).status,400);
  await assert.rejects(()=>createHandbookServer({root,store:join(root,"private.sqlite")}),/outside/);
});


test("private store cannot be redirected into the public tree by a symlink", async t => {
  const work=await mkdtemp(join(tmpdir(),"efh-private-path-"));
  t.after(()=>rm(work,{recursive:true,force:true}));
  await symlink(root,join(work,"redirect"));
  await assert.rejects(()=>createHandbookServer({root,store:join(work,"redirect","private.sqlite")}),/outside/);
});

test("concurrent evaluations are bounded and array output is windowed", async t => {
  const {post}=await fixture(t,{maxConcurrent:1});
  const responses=await Promise.all([post("/api/eval",{formula:"=SUM(1,2,3)"}),post("/api/eval",{formula:"=SUM(4,5,6)"})]);
  assert.deepEqual(responses.map(r=>r.status).sort(),[200,503]);
  const array=await post("/api/eval",{formula:"=SEQUENCE(40,20)"});
  assert.equal(array.body.value.rows,40);assert.equal(array.body.value.columns,20);
  assert.equal(array.body.value.cells.length,32);assert.equal(array.body.value.cells[0].length,16);assert.equal(array.body.value.truncated,true);
});
