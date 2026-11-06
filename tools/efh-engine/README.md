# OneCalc evaluation bridge

The bridge calls `dna_onecalc::Session::calculate`; it adds JSON transport, bounded input and
result windows. It contains no second formula evaluator. Both the browser and the Node API
execute the same generated wasm file, including OneCalc's software x87 path.

Build from the pinned source archive **in a run folder**, leaving the source clone untouched:

```sh
mkdir -p ~/codex-runs/handbook-engine
# Read the revision from tools/efh-engine/source.json.
git -C ~/projects/DnaCalc/DnaCalc archive <revision> --output="$HOME/codex-runs/handbook-engine/source.tar"
python3 tools/efh-engine/build.py \
  --archive ~/codex-runs/handbook-engine/source.tar \
  --work ~/codex-runs/handbook-engine/build
cargo run --release --locked --manifest-path tools/efh-site/Cargo.toml -- build
```

Requires Rust with `wasm32-unknown-unknown`, wasm-bindgen CLI **0.2.117**, Python 3.12+ and Cargo's
dependency access/cache. The script checks the archive SHA-256, extracts it into the run folder,
uses the committed lockfile, tests the bridge natively, builds wasm and writes the ignored
`site/engine/`. The site generator copies it into `site/build/assets/onecalc/`. No network is
used by a reader's calculation. Evaluation is lazy: the 6 MB wasm loads only when Calculate is
pressed. Serve over HTTP; browsers restrict workers on `file:` pages.

`evaluate(request_json)` returns JSON with `engine`, `engine_revision`, and one of:
`calculated`, `worksheet-error`, `refused` or `invalid-request`. The hosts additionally report
`engine-error` and `timeout` when the wasm cannot finish. Errors and refusals are not answers.
A numeric answer includes its shortest decimal, JSON number and 16-digit binary64 `bits` string.
For a non-finite IEEE pattern carried as an Excel numeric overflow encoding, the JSON number is
null and the bits retain the pattern; it is not a blank value.
large results have a bounded 32 × 16 display window. The module's linear memory is capped at
128 MiB; each host terminates the worker on timeout.

Request: `formula` (at most 4096 UTF-8 bytes); optional `inputs` keyed by cell address (at most
256 cells; number, text, boolean, null for blank, or `{"error":"#N/A"}`); `date_system` (`1900`
or `1904`); `compatibility_version`; `now` (1900-system Excel serial for NOW/TODAY). A1 holds the formula.
The fixed locale is en-US. Missing cells remain missing. The browser allows 30 seconds to load the engine and 5 seconds for calculation. Editing the
formula invalidates its displayed result. The browser passes the local clock;
API clients must pass `now` explicitly if their formula needs it.

The variant hook accepts `udf: {language: "vba", source: "…"}` and loads a standard module
through OneCalc's own VBA loader, with a 10,000-step budget. Only the engine's portable UDF
facilities are available. A variant can link that source using its `onecalc_udf` field; a missing
hook or error bound is `null`, never an implied accuracy claim. Example API request:

```json
{"formula":"=HB_DOUBLE(3)","udf":{"language":"vba","source":"Public Function HB_DOUBLE(x As Double) As Double\nHB_DOUBLE = x * 2\nEnd Function"}}
```

Native bridge tests check transport, not Excel agreement. The browser/API tests exercise the
wasm file readers receive. Neither test set is a published Handbook function suite.

After a site build, `node tools/efh-engine/check-examples.mjs [report.json]` runs every catalogue starter
formulas on the shipped wasm, with a fixed clock supplied to NOW/TODAY. Each case expects a
calculated value unless `try_it.preview_status` explicitly documents a worksheet error or
refusal. The report counts every status. It checks that the examples are usable and limits are
explicit; it does not compare OneCalc answers to Excel or establish numerical accuracy.
