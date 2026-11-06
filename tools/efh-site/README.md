# Static site v1

From the repository root:

```sh
cargo run --release --locked --manifest-path tools/efh-site/Cargo.toml -- build
```

Writes the static folder `site/build/` (ignored by git). No C#, OxFunc checkout,
server runtime, account, tracker, remote font, or publication step is involved.
Rust dependencies are JSON parsing, Markdown parsing and regular expressions;
Cargo needs its usual dependency download on a first build. Subsequent builds
work offline with those crates cached.

```sh
cargo test --locked --manifest-path tools/efh-site/Cargo.toml
cargo run --locked --manifest-path tools/efh-site/Cargo.toml -- check
cargo run --locked --manifest-path tools/efh-site/Cargo.toml -- check-links
cargo run --locked --manifest-path tools/efh-site/Cargo.toml -- check-claims
```

All commands accept `--repo PATH` and `--out PATH`. An explicit relative output
path is relative to the caller's working directory. The default is relative to
the repository. The generator refuses to replace a nonempty directory without
its own output marker. It generates into a sibling staging directory and runs
both output checks before replacing an earlier build, with rollback if installing
the checked folder fails. It rejects symlink outputs and rechecks directory
ownership at replacement time. Keep output paths outside
source directories. Serve the folder with any ordinary static file server, at
the root or under a URL prefix. Internal links include filenames and are relative.

The 541-entry spine is read from `data/index.json`. Each page uses the same order:
What it does, Try it, Quirks, How to implement, Version differences, Alternatives.
Sources and evidence appear in a compact footer. Existing essays are divided by subject;
editorial metadata stays in the source files and JSON. `content/reference/FUNC.*.json` owns
examples, section overrides, Excel-build records, implementations by language and named variants.
The stable `api/functions/FUNC.*.json` combines these with explanations and attached observations.
`llms.txt` documents predictable page and JSON URLs.

Series Markdown is labelled Draft unless its frontmatter explicitly says
`status: ready` or `status: published`. `MATERIAL_*` files and the unreviewed PDF
are not copied, linked, or rendered.

Claim-language checks examine HTML text (including text split across inline tags
or encoded as character entities) and reader-visible attributes. They reject the
equality phrases restricted by Charter §7.8, common unscoped completion and
verification wording, and count statements missing the catalogue reconciliation.
This conservative lexical check is **not a semantic proof of Charter compliance**.
Prose blocks that fail it are visibly withheld with a source link. Their source
paths, reasons and excerpts are recorded in `api/build.json`; source content is
unchanged. Note JSON is not rewritten to pass a gate: offending note wording
fails the build for editorial correction. Coverage lists outstanding references.
Link checks cover local targets, fragments, duplicate ids and label/control ids;
they do not contact external servers.

The source-schema validator (`tools/efh-validate`) is an independent research-data diagnostic.
Its existing findings do not fail site builds. Site builds validate the reference records,
catalogue identity, local links and rendered equality wording.

Output ordering is stable and there are no wall-clock timestamps. The source
revision and working-tree state in `api/build.json` are build inputs, as are the tool
version, committed projections and curated files. Repeat builds of an unchanged
checkout are byte-identical. The integration test compares the whole output.
Builds from a dirty checkout say so; a revision alone cannot reproduce local
changes. CRLF Markdown from Windows is normalized only in memory.

## Headless Chrome checks

With Node 22+ and an installed Chrome (no npm packages):

```sh
node tools/efh-site/browser-check.mjs
```

The harness serves output locally with no cache under `/preview/handbook/`,
launches headless Chrome and writes screenshots plus a JSON report to the
ignored `.tmp/site-browser/`. Use `--out PATH`, `--screenshots PATH` or
`--chrome PATH`; `CHROME` also selects the browser executable.

Its matrix covers eleven routes at 390 and 1440 px in light and dark. It checks keyboard
navigation, search/category/empty states, six function sections, compact sources, no-JS reading,
file URLs, and 320/375 px overflow. It also calculates actual formulas in browser wasm, compares
numeric bits with the local API, and submits the contribution form into a temporary private
SQLite store. Build the engine before running it. Screenshots and the JSON report support visual
review; they do not establish results on iOS or other browser engines.

## Calculation and contributions

Build the pinned [OneCalc bridge](../efh-engine/README.md) into ignored `site/engine/` before the
site build; the generator includes those assets. A reference-only build without the engine is
also allowed, and Calculate reports that the calculator is unavailable. Run the
[local server](../efh-api/README.md) for the evaluation API and private contribution collection.
`node --test tools/efh-api/server.test.mjs` exercises real wasm and a temporary SQLite store.

This command never deploys or changes hosting configuration.
