# The Handbook of Excel Functions

A comprehensive, honest, evidence-backed reference for every Excel worksheet function and
operator — the Abramowitz & Stegun of Excel functions.

For each of the 550 canonical entries — 550 entries = 527 functions + 23 operators = 534 published
Excel rows + 7 split byte-variant rows + 9 added functions — the Handbook grows toward: uniform call
semantics, full metadata and localized names, links to official documentation (and where reality
diverges from it), implementations in four flavours across multiple languages, versioned and
independently checkable test suites, curated references, residual-portrait plates (the ULP
Atlas), and the research record of how last-bit agreement with Excel was investigated ("The Last
Bit").

The canonical reference is the function page and its stable JSON. Each entry explains what the
function does, offers a runnable formula, describes quirks and implementation choices, compares
Excel versions, and gives alternatives. Sources and measured evidence sit below the explanation.

Read [CHARTER.md](CHARTER.md), [OPERATIONS.md](OPERATIONS.md) and the
[site tool documentation](tools/efh-site/README.md) for scope, ownership and local builds.

## Layout

- `data/` — machine projections (the only mechanically synced organ); `data/calcfunc/` holds what
  `tools/efh-calcfunc` regenerates from a pinned DnaCalc commit.
- `content/` — curated prose: call-model chapters, function pages, "The Last Bit" series,
  homepage exhibits, and the versioned working-term explanation registry.
  `content/notes/` holds the function notes (quirks, answers that differ from the mathematical
  value, details for matching Excel's bits, documentation differences); see
  [content/notes/NOTES.md](content/notes/NOTES.md) and [the HOWTO](content/notes/HOWTO.md).
- `implementations/` — per function × flavour × language; only suite-verified entries are
  published.
- `vectors/` — versioned test suites (JSONL, bits-hex, hash-manifested).
- `content/reference/` — handbook-owned examples, versions, implementations and variants.
- `site/` — generated static site (plus `site/mockups/` — approved design mockups).
- `tools/` — `efh-calcfunc` (Python and Rust), `efh-site` (Rust) and `efh` (C#).
- `tests/` — vector runners, ratchets, determinism drills.
- `docs/decisions/` — steward decision records (`EFH-DR-nnn`).
- `docs/handoffs/` — outbound notes to sibling repos.

## Build the static preview

```sh
cargo run --release --locked --manifest-path tools/efh-site/Cargo.toml -- build
```

Writes the ignored folder `site/build/`, with all 550 entry pages, function notes, a category
index and browser-local search, The Last Bit drafts, coverage and licences. The standalone Rust
generator does not require C# or an OxFunc checkout and does not publish anything. See
[its commands and checks](tools/efh-site/README.md). Function JSON lives at
`api/functions/FUNC.<NAME>.json`; `llms.txt` documents agent access. All catalogue entries
use the same six-section layout. Build and platform information travels with measured answers.

`vectors/` has no published suites. The restrictions on equality wording in Charter §7 remain.
Local building never publishes the site.

## Licenses

Code: MIT ([LICENSE.md](LICENSE.md)). Content: CC BY 4.0
([LICENSE-CONTENT.md](LICENSE-CONTENT.md)).
