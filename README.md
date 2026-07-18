# The Handbook of Excel Functions

A comprehensive, honest, evidence-backed reference for every Excel worksheet function and
operator — the Abramowitz & Stegun of Excel functions.

For each of the 534 published functions and operators, the Handbook grows toward: uniform call
semantics, full metadata and localized names, links to official documentation (and where reality
diverges from it), implementations in four flavours across multiple languages, versioned and
independently checkable test suites, curated references, residual-portrait plates (the ULP
Atlas), and the research record of how bit-exact behavior was found ("The Last Bit").

Built on the DNA Calc program's OxFunc lane (function semantics, bit-exact Excel parity work)
and on Gneiss (accountable-knowledge ledger: claims, contexts, receipts). The Handbook is its
own knowledge system: OxFunc is its strongest evidence source, not its definition.

## Read in this order

1. [CHARTER.md](CHARTER.md) — mission, doctrine, scope, claim-language rules, licenses.
2. [OPERATIONS.md](OPERATIONS.md) — organs and ownership, sync doctrine, suite versioning,
   ratchets, evidence intake, phase register.
3. [ledger/VOCABULARY.md](ledger/VOCABULARY.md) — the `efh.*` claim vocabulary (draft).
4. [GNEISS-PROFILE.md](GNEISS-PROFILE.md) — the declared boundary and experimental realization
   profile for the Handbook as a Gneiss Knowledge System.

## Layout

- `data/` — machine projections from OxFunc (the only mechanically synced organ).
- `content/` — curated prose: call-model chapters, function pages, "The Last Bit" series,
  homepage exhibits, and the versioned working-term explanation registry.
- `implementations/` — per function × flavour × language; only suite-verified entries are
  published.
- `vectors/` — versioned test suites (JSONL, bits-hex, hash-manifested).
- `ledger/` — the Gneiss claims ledger and its canonical export.
- `site/` — generated static site (plus `site/mockups/` — approved design mockups).
- `tools/` — `efh-ingest` (Rust) and `efh` (C#).
- `tests/` — vector runners, ratchets, determinism drills.
- `docs/handoffs/` — outbound notes to sibling repos.

## Status

Phase H0 (bootstrap). Nothing is published yet; the phase register in
[OPERATIONS.md](OPERATIONS.md) is the living status surface.

## Licenses

Code: MIT ([LICENSE.md](LICENSE.md)). Content: CC BY 4.0
([LICENSE-CONTENT.md](LICENSE-CONTENT.md)).
