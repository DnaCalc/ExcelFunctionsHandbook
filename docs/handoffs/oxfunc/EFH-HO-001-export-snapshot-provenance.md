# EFH-HO-001 — Re-cut the library-context snapshot export from a clean tree

Date: 2026-07-30 · Direction: Handbook → OxFunc · Kind: outbound handoff, read-only observation
Raised under `OPERATIONS.md` section 1 clause 2. The Handbook does not write to OxFunc; this note
is the durable boundary record and OxFunc decides through its own process.

Observed at OxFunc `473efa3` (2026-07-25, working tree clean).

## 1. The ask

Re-cut `docs/function-lane/OXFUNC_LIBRARY_CONTEXT_SNAPSHOT_EXPORT_V1.csv` from a **clean working
tree**, at a named commit, and record that commit in the file's own provenance columns.

## 2. The exact fields at issue

The export carries its provenance in its first five columns, on every one of its 534 data rows.
Read from the file at `473efa3`:

| Column | Value on every row |
|---|---|
| `snapshot_id` | `oxfunc-libctx-v1` |
| `snapshot_generation` | `2026-04-02` |
| `source_commit_short` | `87ef585` |
| `source_commit_full` | `87ef5853451f61c19c4d955782094a9d7ae0771f` |
| **`source_tree_state`** | **`dirty`** |

Commit `87ef585` is reachable in OxFunc's history — `Run first W070 bead loop and archive wave 1`,
Thu 2 Apr 2026 — so the *commit* is a real anchor. What is not recoverable is the working-tree delta
that was present when the export ran.

## 3. Why it matters to us

1. **Nobody can regenerate those bytes from a commit — including us.** `source_tree_state: dirty`
   says, in the file's own words, that the state that produced these 534 rows was not committed
   anywhere. Checking out `87ef585` and re-running the exporter is not guaranteed to reproduce the
   file, and there is no way to measure how far off it would be.
2. **The Handbook's `data/` organ fuses this export with the live registry.** The row spine and the
   fields only the export owns come from the `2026-04-02` snapshot; classification and signatures
   come from `oxfunc_core` at `473efa3`. Two provenance bases, dated `2026-04-02` and `2026-07-25`,
   one of them unreproducible. We publish this as a limitation on the affected pages (`OPERATIONS.md` section 4,
   "Mixed-vintage disclosure for `data/`"), but a disclosure is a description of a defect, not a fix.
3. **It blocks independent reproducibility, which is a Handbook charter commitment.** `CHARTER.md`
   section 3 clause 4 promises that any claim can be re-verified anywhere. A reader who wants to
   check a Handbook identity or category cell against OxFunc can reach the live registry, but cannot
   reach the export's state. That is the one link in the chain we cannot hand them.
4. **It is a small fix upstream and an unfixable one downstream.** Only OxFunc can re-run the
   exporter from a clean tree. Nothing the Handbook does can recover the missing tree state.

## 4. What a re-cut would need to carry

1. `source_tree_state: clean`.
2. `source_commit_full` set to the commit that was actually checked out for the run.
3. A bumped `snapshot_generation`, so consumers can tell the two vintages apart rather than
   silently receiving different bytes under the same generation string.
4. If any cell changes relative to the `2026-04-02` snapshot, the diff — the Handbook would want to
   know which of its published cells moved, and 534 rows × 37 columns is small enough to diff.

## 5. Related, not part of this ask

Two observations the Handbook made against the same file, recorded so they are not re-discovered:

1. 114 `function_meta_curated` rows and all 14 `catalog_only` rows carry empty classification cells
   in the export, while the live registry holds a complete `FunctionMeta` for 115 of them. The
   Handbook's fix is to source `arity` and `classification` from the live registry and keep the
   export only for the fields it uniquely owns; no OxFunc change is requested.
2. Seven rows carry combined `surface_stable_id` values (`FUNC.FIND, FINDB` and siblings) that match
   neither locked id pattern. Already filed as `docs/handoffs/H2_FINDINGS_INGEST.md` item 1; not
   re-opened here.

## 6. Handbook state while this is open

`data/` continues to consume the `2026-04-02` export and publishes the mixed-vintage disclosure. No
Handbook page asserts that its export-sourced cells are reproducible. If the export is re-cut, the
disclosure is narrowed rather than removed — the historical fusion still happened, and the pages
published against it keep their provenance.
