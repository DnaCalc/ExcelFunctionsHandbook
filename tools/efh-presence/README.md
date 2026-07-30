# efh-presence (T2)

Generates **F2** — `data/presence/<function_id>.json` for all 541 Handbook entries, plus
`data/presence/index.json`.

```
cargo run --release -- [--oxfunc DIR] [--functions DIR] [--out DIR] [--commit SHA40]
```

Defaults: `--oxfunc ../../../OxFunc`, `--functions ../../data/functions`, `--out ../../data/presence`.

Standalone crate: **not** a workspace member, **no** dependencies, **no** `oxfunc_core` path
dependency. A cold `cargo build --release` takes under ten seconds and a full scan of
~1 100 OxFunc files takes about six. It compiles nothing from OxFunc and runs nothing from it.

## What this organ is, and what it is not

`data/presence` is a **presence map**. It records where a function id or surface name occurs in
another repository's files, how many `#[test]` attributes those files contain, and nothing else.
The schema has no field for a pass, a match, or a verdict, and
`t2g_no_entry_carries_a_verdict_shaped_field` fails the build if one appears. No OxFunc code was
compiled, no test was run, no oracle was consulted. Nine standing limits (`L1`..`L9`) travel on
every record as **ids only**; their prose lives in `content/model/scan-limits.json` (F14), which
this tool must never write. Build guard G-12 reconciles the two id sets.

## Publication gate

Before any file is created, opened for writing or removed:

```
git -C <oxfunc> rev-parse HEAD          -> must be 40 hex; recorded as oxfunc_commit
git -C <oxfunc> status --porcelain      -> must be EMPTY
```

A non-empty porcelain aborts with exit code **4**, prints the offending paths, and writes
**nothing**. `--commit` mismatch aborts with exit **3**. A git failure aborts with exit **2**.
Consequently every emitted record carries `oxfunc_tree_clean: true` — the `false` state is
unreachable by construction, which is the point of the gate.

## Matching discipline

Re-derived from, not copied out of, `harvest_impl_map.py` sections (1)–(3). Nothing that script
emitted is read.

A **run** is a maximal span of `[A-Za-z0-9_.]`. The characters either side of a run are outside
that class, so the leading guard `(?<![A-Za-z0-9_])(?<![A-Za-z0-9]\.)` holds automatically at
every run start and can never hold inside one. Therefore every match starts at a run start, and
one pass per file is enough. The trailing guard `(?![A-Za-z0-9_])(?!\.[A-Za-z0-9])` restricts a
match to a prefix of the run that either ends it or is followed inside it by `..` or `._`.

| token | rule | used for |
|---|---|---|
| `ID_QUOTED` | the literal `"FUNC.NAME"` with both double quotes | `impl_modules` grep half, `registered_in`, `dispatch_in`. Exact; no false positives |
| `ID_BARE` | a run equal to `FUNC.NAME` | id occurrence counts, `lean_modules`, doc mentions. A superset of `ID_QUOTED`, so occurrences are never double-counted |
| `NAME_WORD` | surface names of length ≥ 4 at run boundaries | doc / corpus / fixture mentions |
| `NAME_GUARDED` | surface names of length ≤ 3, run boundaries **plus** one of: followed by optional whitespace then `(`; preceded by `=` and optional whitespace; double-quoted; backtick-quoted | the 53 short names |
| `FORMULA_TOKEN` | any surface name followed by optional whitespace then `(` | the fixture half of `fixture_hits` |

Matching is case-sensitive. `name_match_confidence` is `low` **iff** the surface name is ≤ 3
characters (53 of 541) and describes name-based fields only; id-based fields are exact.
`NORM.INV` does not match inside `LOGNORM.INV`; `SUM` does not match inside `SUMIF`;
`FUNC.T` does not match inside `FUNC.T.TEST`; a bare `T` in English prose does not match at all.

## Scan scope

Excluded from every walk: `.claude/worktrees` (a full duplicate agent worktree that would double
every count), `target/`, `.git/`.

| set | path | files at `473efa3` |
|---|---|---:|
| rust function modules | `crates/oxfunc_core/src/functions/**/*.rs` | 254 |
| surface-dispatch tables | basename starts `surface_dispatch` | 3 |
| registry seeds | `src/registry_signature_seed.rs`, `src/registry_context_seed.rs` | 2 |
| tests / fixtures | `crates/oxfunc_core/tests/**` | 17 |
| lean sources | `formal/**/*.lean` | 247 |
| fuzzer corpus | `smart-fuzzer/corpus/**` | 4 |
| doc catalogs | the three named `docs/*.md` | 3 |
| bug streams | `docs/bugs/streams/**` | 42 |
| function lane | `docs/function-lane/**` | 563 |
| declared modules outside `src/functions` | resolved from `artifacts.rust_module` | 1 |

The Lake build tree under `formal/` (`.olean`/`.ilean`/`.c`/`.trace`/`.hash` and generated
`.json`) is not scanned: it is derived from the `.lean` sources and scanning it would double-count.
`lean_sources_scanned` is therefore exactly the number of files actually opened.

The last row matters. `oxfunc_core::locale_format` is declared by four entries' `artifacts` block
and resolves to `crates/oxfunc_core/src/locale_format.rs`, one directory **above** the 254-module
grep set. Its `#[test]` count and line count are read from disk. Assuming zero for it is what makes
FOUNDATION §3.3 say `locale_format.rs 0 / 4`; the file actually holds 6.

## `EXCL-SURFACE-DISPATCH-1`

`mention_modules` is the literal set of modules naming the quoted id (plus declared artifacts).
`impl_modules` is that set minus any module whose **basename starts with `surface_dispatch`** —
the whole rule, no threshold and no curated list. Counted at `473efa3`, `surface_dispatch.rs` names
**526** of the 541 ids as quoted ids and carries **110** `#[test]`s of its own;
`surface_dispatch_unary_numeric_spec_generator.rs` names 46 more and
`surface_dispatch_by_index_generated.rs` names none. (`harvest_impl_map.py`'s docstring says 527
for `surface_dispatch.rs`; the re-derived figure at this commit is 526.) Leaving those tables in
would give almost every entry 2+ modules, 110+ tests and ~525 siblings. Every per-function figure
(`tests_in_impl_modules`,
`module_shared_by`, `tests_per_module`, `module_tests_minus_sibling_count`,
`source_lines_per_module`) is on the `impl_modules` view. Prose for the rule id lives in F14.

## `module_tests_minus_sibling_count`

For each of the entry's `impl_modules`: `#[test]` count in that module **minus** the number of
Handbook ids mapped to it. A negative value means at least that many of the ids sharing the module
cannot have a test of their own. It counts test *functions*, not assertions, not inputs, and
records nothing about whether anything passes.

## Rule DMC-1 — `doc_mention_classification`

Applied per mentioning file, in order; the first clause that fires decides.

1. basename, uppercased, contains a **behavioural** filename token → `behavioural-finding`
   (`BUG-`, `DEFECT`, `DEVIATION`, `DISCREPANCY`, `DIVERGENCE`, `FAILURE`, `FINDING`, `MISMATCH`,
   `NOTES`, `REGRESSION`, `REPRO`, `ROOT_CAUSE`)
2. basename, uppercased, contains a **bulk** filename token → `bulk-inventory`
   (`CATALOG`, `EXPORT`, `INDEX`, `INVENTORY`, `LEDGER`, `MANIFEST`, `REGISTER`, `REGISTRY`,
   `ROLLUP`, `SEED`, `SNAPSHOT`, `TABLE`, `TRANCHE`)
3. extension is not `.md` → `bulk-inventory` (a delimited or structured export enumerates surfaces
   in rows and states no finding in prose)
4. some occurrence is **not** on a Markdown table row **and** its nearest preceding ATX header
   (`^#{1,6}\s`), uppercased, contains a behavioural header token → `behavioural-finding`
   (`BEHAVIOR`, `BEHAVIOUR`, `DEFECT`, `DEVIATION`, `DISCREPANCY`, `DIVERGENCE`, `EXPECTED`,
   `FINDING`, `MISMATCH`, `OBSERVED`, `REPRO`, `ROOT CAUSE`, `SYMPTOM`)
5. otherwise → `bulk-inventory`

The three token lists and the clause order are the whole rule. They must be reproduced verbatim as
`doc_mention_classification_rule` in F14; if F14 states something else, F14 and this file disagree
and the build should fail rather than pick a winner.

Worked example, the one FOUNDATION names: `HARMEAN` is mentioned in exactly 10 files, all under
`docs/function-lane/`, and all 10 are decided by clause 1 or 2 as `bulk-inventory` — seven `.csv`,
two `.json` and `FUNCTION_LANE_EVIDENCE_ID_REGISTRY.md`. That is what lets a page say "every file
in OxFunc that names HARMEAN is an inventory, not a finding" without a human writing the sentence.

## Determinism

Every array ordinal-sorted; every data-keyed object emitted in ordinal key order (including
`sibling_histogram`, whose keys sort `"0","1","11","12","14","18","2",…`); every path
forward-slashed and relative to the OxFunc root; no wall clock; UTF-8 without BOM; `\n`; 2-space
indent; trailing newline. Stale `*.json` in the output directory that this run did not produce are
removed, so a shrinking id set still byte-compares. `cargo test` asserts two consecutive runs are
byte-identical across all 542 files.

## Acceptance tests

`cargo test --release` runs the §6 T2 suite (`tests/acceptance.rs`) plus the matcher unit tests.
See the header of `tests/acceptance.rs` for the mapping to T2 (a)–(f).
