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

Its declared matrix covers seven routes at 390 and 1440 px in light and dark,
with homepage, ROUND and a projection-only page's screenshots, plus ROUND note
details. It checks runtime/request errors, no external resource requests,
keyboard skip/focus, lens behaviour, source doors, search/category/empty states,
JavaScript-disabled reading, file URLs, and 320/375 px overflow with text-width
stress. It selects IMLOG10 while that entry lacks prose, then another thin entry
from the generated catalogue. The report identifies the selection. Inspect the screenshots
as well as the machine report. These are headless Chrome checks, without a
claim about real iOS devices or other browser engines.

This command never deploys or changes hosting configuration.
