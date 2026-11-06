# Local Handbook server

Node 24+; no npm dependencies. Build the site and [OneCalc wasm](../efh-engine/README.md) first.

```sh
node tools/efh-api/server.mjs --root site/build \
  --store .tmp/handbook/contributions.sqlite --port 8787
node --test tools/efh-api/server.test.mjs
```

Serves local static pages and server code for `handbook.dnacalc.com/api/eval` and
`/api/contributions`. It binds to **127.0.0.1** by default. This does not configure hosting or deploy.
The store must live outside the public folder. The site remains readable without the server;
browser calculations need HTTP and the wasm assets, while contributions need this server.

`POST /api/eval`, `Content-Type: application/json`:

```json
{"formula":"=SUM(B2:B3)","inputs":{"B2":3,"B3":4}}
```

The response carries OneCalc's status, typed value, binary64 bits when numeric, diagnostics and
source revision. See the [request fields and statuses](../efh-engine/README.md). It executes the
same wasm module as Try it, in an isolated worker; it does not run a native approximation.

Limits: 30 evaluations/minute per direct peer address, 120 total API requests/minute, two concurrent
evaluations, 3 seconds per evaluation, 32 KiB per request, 128 MiB wasm memory. The browser allows
30 seconds for loading and 5 seconds for calculation. Excess rate returns 429 and Retry-After; saturation returns 503;
timeout returns 408; invalid input returns 400. Worksheet errors and engine refusals return 200
with their distinct statuses. X-Forwarded-For is ignored. A future reverse proxy must enforce
per-client limits at the edge; otherwise its shared address receives one combined allowance.

`POST /api/contributions` accepts `function_id`, `kind` (`correction`, `observation`,
`implementation`, `variant`), `excel_build`, `details`, and optional `contact`. It returns a
random `id` and pending status **after** a committed SQLite insert. Maximum five submissions
per minute per peer. SQL values are parameter-bound. Contact details and submitted source remain
private. There is no public read endpoint, automatic publication, GitHub integration or email.
Cross-origin browser writes are refused.

The SQLite file uses WAL and synchronous FULL; its mode is 0600. Its directory is created with
0700. Review locally with SQLite, for example:

```sql
SELECT id, created_at, function_id, kind, excel_build, details, contact
FROM contributions WHERE status = 'pending' ORDER BY created_at;
```

Use SQLite's backup API for a live backup (copying just the main file while WAL is open can miss
submissions). Tests create and remove private temporary stores and run actual wasm evaluations.
