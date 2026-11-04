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
both output checks before replacing an earlier build. Keep output paths outside
source directories. Serve the folder with any ordinary static file server, at
the root or under a URL prefix. Internal links include filenames and are relative.

The 541-entry spine is read from `data/index.json`. Each entry renders its
projection, curated `content/functions/FUNC.*.md` when present, and applicable
notes from `content/notes/FN-*.json`. Generated note sections in Markdown are
removed **in memory only**: the source files are never changed. The resulting
notes API is built from those same JSON inputs; `/coverage` discloses any drift
from the checked-in notes API. Older prose is identified as older when notes
provide a later observation record. Projected localization is explicitly
unreviewed, and Microsoft text retains its licence exclusion.

The approved mockup supplies the palette, typography, reading lenses, featured
exhibit and editorial layout. Its fictional plot, counters, warrants and receipt
are replaced with recorded note values and explicit missingness. Seven source
doors expose the current pre-ledger boundary. No ledger receipt or implementation
warrant is synthesized. New suites or implementations cause a build error until
admission is integrated; they cannot silently inherit the current absence text.

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

Output ordering is stable and there are no wall-clock timestamps. The source
revision in `api/build.json` and source links is a build input, as are the tool
version, committed projections and curated files. Repeat builds of an unchanged
checkout are byte-identical. The integration test compares the whole output.

Publishing remains a separate owner decision. This command never deploys or
changes hosting configuration.
