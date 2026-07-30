# BATTERY — the fixed input battery `EFH-B1`

`tools/efh-battery` calls the OxFunc reference engine once per (entry × battery row) and writes
`data/battery/<function_id>.json` (organ **F16**, `FOUNDATION.md` §2.9) for all **541** Handbook
entries.

## 0. The one sentence you must not misread

**These are OxFunc's answers. No Excel was involved, at any point, in producing any value in
`data/battery/`.** This is weaker than "agrees on a local corpus": there is no oracle here at all —
not a live Excel, not a captured Excel corpus, not a third-party implementation. Every row is a
single observation of one reference implementation answering one fixed input on one host. A reader
who takes a value in this organ as a statement about what Excel returns has been misled, and every
surface that renders this organ must say so in the reader's own line of sight.

Three things in the schema exist only to make that misreading hard:

- every file carries the constant top-level `label` — `"OxFunc's own answers. No Excel was
  involved."` — and the verifier fails the build if any of the 541 files differs by a byte;
- the schema has **no** field for a pass, a match, a verdict, an expectation or a comparison. There
  is nowhere to put an Excel claim even if someone wanted to;
- `outcome_bits` publishes the raw IEEE 754 pattern, so nothing about a value depends on trusting
  this tool's decimal formatting.

## 1. Why the battery is fixed

The battery's value comes from its **constancy**, not from its cleverness. Twelve inputs applied
unchanged to 541 wildly different signatures make two functions' answers directly comparable: the
reader can see that `EXP("2.5")` coerces and `HARMEAN("")` does not, that one function lifts a 2×2
array and its sibling collapses it, that one rejects `#N/A` and another propagates it — without
anybody having chosen a "good" input for either function. A per-function input set would be a
curation act, would land in `content/`, and would destroy the comparison.

Consequently the battery is **versioned and frozen**. `battery_id` is `EFH-B1`. Changing any input
value, any label, the order, or the argument-count rule mints `EFH-B2` and does not edit `EFH-B1`.

## 2. The twelve rows

The row labels and their order are pinned by `FOUNDATION.md` §2.9 and are part of the F16 schema.
Every file has exactly these twelve rows in exactly this order.

| # | `label` | Argument value supplied to **every** argument position | What it is there to expose |
|---|---|---|---|
| 1 | `zero` | number `0` | the zero boundary; division, log and reciprocal domains |
| 2 | `negative-one` | number `-1` | a negative argument; sign handling and negative-domain guards |
| 3 | `empty-string` | text `""` | text that does **not** look numeric (the empty-text case) |
| 4 | `boolean-true` | logical `TRUE` | logical → number coercion (does `TRUE` become 1?) |
| 5 | `empty-range` | an empty cell (`CoreValue::Empty`) | the empty / missing argument |
| 6 | `error-na` | error `#N/A` | error-in behaviour: propagate, collapse, or absorb |
| 7 | `max-double` | number `1.7976931348623157e308` (`f64::MAX`) | large magnitude; overflow and domain-guard behaviour |
| 8 | `min-subnormal` | number `4.9406564584124654e-324` (`f64::from_bits(1)`) | the smallest positive binary64 — a genuine domain boundary, and a non-integer |
| 9 | `inline-array` | the 2×2 array `{1,2;3,4}` | array lifting: does the answer spill, reduce, or fail? |
| 10 | `text-numeral` | text `"2.5"` | text that **does** look numeric — text→number coercion, and a non-integer |
| 11 | `too-few-args` | `arity.min - 1` copies of the number `1` | the arity floor |
| 12 | `too-many-args` | `arity.max + 1` copies of the number `1` | the arity ceiling |

**Argument count.** Rows 1–10 pass `max(arity.min, 1)` arguments, clamped to `arity.max`; a function
declared `arity.max == 0` is called with no arguments. `arity` is read from the live
`FunctionMeta`, never from `data/functions/*.json` (whose arity is still the stale 2026-04-02
snapshot until delta D-1 lands).

**Two cases have no instance, and say so instead of inventing one.**

- `arity.min == 0` → there is no "too few". The row publishes
  `no-such-case:declared-arity-min-is-zero`. **4 rows** across 541 entries.
- `arity.max >= 512` (OxFunc publishes `usize::MAX` for the genuinely unbounded surfaces; the
  largest real bound in the catalog is 255) → there is no "too many". The row publishes
  `no-such-case:declared-arity-max-is-unbounded`. **2 rows**.

### 2.1 Coverage this battery does *not* have

Stated because the gap is real, not because it is small:

- **No small positive integer at a legal arity, and no plain non-integer at a legal arity.** The
  twelve labels are pinned by `FOUNDATION.md` §2.9; the only rows carrying the value `1` are the two
  arity rows, where the call is inadmissible by construction. `min-subnormal` and `text-numeral` are
  the only non-integer inputs, and both are edge-shaped. This is the first thing `EFH-B2` should
  fix, and until it does, no page may claim this battery shows how a function behaves on ordinary
  arguments.
- **Every argument position gets the same value.** Nothing here probes mixed-shape argument lists,
  and nothing probes an argument at a position the function treats specially (a `basis` code, a
  `match_type`, a lookup vector).
- **No references, no workbook, no locale, no providers.** See §4.
- **Twelve points is twelve points.** A battery row is a single observation, not a property.

## 3. Outcome vocabulary

`outcome_kind` ∈ `number` · `text` · `boolean` · `error` · `array` · `refused-by-arity` ·
`not-dispatchable` (the F16 enumeration, unchanged).

- **`number`** — `outcome_display` is the value at **17 significant digits** (`{:.16e}`), which
  round-trips every binary64, and `outcome_bits` is the exact IEEE 754 pattern as `0x` + 16 lower-case
  hex digits. The verifier re-parses every printed decimal and asserts it reproduces the published
  bits; **1236 of 1236** number rows pass.
  - *Exception, published rather than hidden:* **23** number rows are non-finite binary64 —
    OxFunc published `inf` or `NaN` as a worksheet number (e.g. `HARMEAN(1.797…e308)` → `inf`,
    `PERCENTRANK(0, 0)` → `NaN` with bits `0xfff8000000000000`). No decimal string round-trips a NaN's
    sign and payload, so for these rows `outcome_display` is the IEEE class name (`inf`, `-inf`,
    `nan`) and **`outcome_bits` is the authoritative field**. That OxFunc publishes a non-finite as a
    number at all is an observation about OxFunc, not a defect claim, and not an Excel claim.
- **`array`** — `outcome_display` renders the spill as `{a,b;c,d}` with every numeric cell at 17
  significant digits. Per-cell bits are **not** published in F16; a reader who needs them must
  re-run the battery. 366 rows.
- **`text`**, **`boolean`**, **`error`** — the text, `TRUE`/`FALSE`, and the Excel error code
  (`#VALUE!`, `#NUM!`, `#N/A`, …). `outcome_bits` is `null`.
- **`refused-by-arity`** — used **only** on rows 11 and 12, and only when the declared `Arity` does
  not accept the argument count *and* the engine returned an error. `outcome_display` keeps the
  error code the engine actually produced. 961 rows.
- **`not-dispatchable`** — **no answer was obtained**, and `outcome_display` carries a typed reason
  token, never a value. 688 rows. This is the slot that makes fabrication unnecessary; see §4.

## 4. Typed reasons: where no answer exists

A function-level refusal writes the same typed reason into all twelve rows. Every refusal is
derived from a **declared** `FunctionMeta` axis or from catalog membership — never from prose, never
from a hand-kept list of names.

| Typed reason | Rule | Entries |
|---|---|---:|
| `cannot-call:not-in-reference-catalog` | `resolve_surface_dispatch_key(id)` is `None` | **15** |
| `cannot-call:requires-callable-argument` | `FunctionCallTarget::requires_invoker()` — the declared `callable_argument_specs` are non-empty | **8** |
| `cannot-call:nondeterministic-by-declaration:pseudo-random` | `determinism == PseudoRandom` | **3** |
| `cannot-call:nondeterministic-by-declaration:time-dependent` | `determinism == TimeDependent` | **2** |
| `cannot-call:nondeterministic-by-declaration:external-event-dependent` | `determinism == ExternalEventDependent` | **4** |
| `cannot-call:requires-host-facility:composite` | `surface_fec_dependency_profile == Composite` | **17** |
| `cannot-call:requires-host-facility:caller-context` | `… == CallerContext` | **5** |
| `cannot-call:requires-host-facility:external-provider` | `… == ExternalProvider` | **1** |
| `cannot-call:requires-host-facility:locale-profile` | `… == LocaleProfile` | **1** |
| | **total entries with no answer at all** | **56** |

The rules are applied in the order listed, so an entry is counted exactly once.

**Why refuse rather than call and publish what comes back.** Every refusal above is a case where the
value returned would be a property of *this harness* rather than of OxFunc. The battery supplies no
random source, no clock, no locale context, no host-info provider, no callable invoker, and a
resolver that owns no workbook and fails every dereference. `RAND()` with no random provider, `TEXT`
with no locale, or `MAP` with no invoker would each return something — and that something would
describe the stub we did not build. A typed `not-dispatchable` is the honest publication.

`RefOnly` and `None` surface-FEC profiles **are** called: every battery argument is a literal, so no
reference is ever constructed and the resolver is never consulted. 485 of 541 entries are called.

Two further row-level no-answers exist, both engine observations rather than policy:

- `no-answer:timed-out-in-reference-engine` — the call did not return inside a 20-second budget.
  **7 rows**, all on `max-double`: `DB`, `FACT`, `FACTDOUBLE`, `MULTINOMIAL`, `PERMUT`,
  `POISSON`, `POISSON.DIST`. (An iteration count near 1.8e308 does not finish.)
- `no-answer:panicked-in-reference-engine` — the call panicked. **3 rows**, all on `max-double`:
  `EXPAND`, `SEQUENCE`, `WRAPCOLS`. A panic is recorded, never converted into a value.

*Determinism caveat:* a timeout is wall-clock-dependent and is therefore the one field in this organ
that is not, in principle, byte-stable. In practice these seven calls miss the budget by many orders
of magnitude and two consecutive runs on this host byte-compared equal. A future run that flips one
of them is a signal to raise the budget, not a result.

## 5. `host_scoped` — the pinned x87 scope

Some OxFunc kernels deliberately execute on real **x87 80-bit hardware** (`excel_numeric::x87`),
because that is how the legacy Microsoft CRT computed the corresponding results. Values from those
kernels are legitimately **not portable** across architectures. `host_scoped` is how that becomes a
published fact instead of a hidden one.

**Definition, exactly:** `host_scoped == true` iff the function id is in the pinned list
`tools/efh-battery/x87-scope.txt` **and** this row's `outcome_kind` is `number`.

The pinned list is produced by `derive_x87_scope.py`, which is mechanical end to end:

- **A.** In `crates/oxfunc_core/src/excel_numeric/`, a function is x87-backed if its own body names
  `x87::`; the set is closed transitively over calls inside that directory. **42 functions.**
- **B.** A kernel module under `crates/oxfunc_core/src/functions/` is *seed*-tainted if it names any
  step-A function through an `excel_numeric::` path. **22 modules**: `acoth`, `atanh`,
  `bond_core_family`, `cashflow_rate_family`, `cos`, `cot`, `csc`, `cumulative_finance_family`,
  `discrete_dist_family`, `exp_fn`, `financial_time_value_family`, `ln_fn`, `log10_fn`, `log_fn`,
  `normal_dist_common`, `permut_fn`, `power_fn`, `sec`, `sin`, `special_dist_family`,
  `special_math_common`, `tan`.
- **C.** Closed transitively over `<module>::` references between kernel modules, adding **11**:
  `beta_gamma_stats_family`, `chi_f_t_family`, `confidence_test_family`, `gauss_fn`,
  `legacy_stats_alias_family`, `normal_log_family`, `odd_bond_family`,
  `operator_arithmetic_family`, `phi_fn`, `statistical_tests_family`, `test_alias_family`.
  Modules whose basename starts with `surface_dispatch` are excluded from the graph: the dispatcher
  names every module in the crate, so it is a universal edge carrying no routing information.
- **D.** Each catalog entry's owning module is read **exactly** from the `use crate::functions::{…}`
  import block of `xll_export_specs.rs`, which binds every `*_META` constant to its declaring
  module, joined with `FUNCTION_CATALOG`'s literal order and the runner's `catalog` mode
  (catalog index → function id). All 525 catalog entries map; none is guessed.

Result: **133** entries pinned (65 in a seed module, 68 transitive-only); **100** of them produced at
least one `number` row, giving **332** `host_scoped` rows.

**What this field does and does not claim.** It is a *module-granularity reachability
over-approximation*. It says the module that owns this function's `FunctionMeta` can reach the x87
backend. It does **not** prove an x87 instruction executed for this input, and step C in particular
marks whole families where only some members route through x87 — `operator_arithmetic_family`
carries `OP_NEGATE` and `OP_UNARY_PLUS` alongside `OP_POWER`, and sign flipping is ordinary IEEE
arithmetic that no x87 chain touches. (`OP_ADD` is *not* flagged, because it lives in its own
`op_add` module — an accident of file layout, not a semantic distinction, which is exactly the
resolution limit of a module-granularity rule.) The field
therefore over-flags rather than under-flags, which is the safe direction for a portability warning,
and a page rendering it must not restate it as "this value came from x87".

The `host` block (`arch`, `cpu`, `os`) is recorded on every file for the same reason. A run on a
non-`x86_64` host takes OxFunc's portable fallbacks and **is expected to differ on exactly the
`host_scoped` rows**; that cross-host comparison has not been run (see §7).

## 6. Determinism and byte-stability

- Inputs: the fixed battery, the live `FunctionMeta` at a named OxFunc commit, and the sorted id
  list from `data/functions/`. No wall clock, no random source, no locale.
- The runner **aborts and writes nothing** if the OxFunc working tree is dirty (`git status
  --porcelain` non-empty), and stamps `oxfunc_commit` + `oxfunc_tree_clean` on every file.
- UTF-8, no BOM, `\n`, trailing newline, 2-space indent, key order exactly as the F16 table.
- Arrays are emitted in the schema's semantic order (the 12 rows); ids are iterated in ordinal
  order.
- Verified: two consecutive runs on this host produced 541 byte-identical files.

## 7. How to run and how to check

```
cd tools/efh-battery
cargo build --release
./target/release/efh-battery <handbook-root> <oxfunc-root> battery      # writes data/battery/*.json
./target/release/efh-battery <handbook-root> <oxfunc-root> catalog      # catalog index -> function id
python derive_x87_scope.py <oxfunc-root> catalog.tsv > x87-scope.txt    # regenerates the pinned list
python verify_battery.py <handbook-root>                                # the T7 acceptance tests
```

`verify_battery.py` implements `FOUNDATION.md` §6 row T7 (a)–(d): 541 files; exactly 12 rows in the
fixed order on each; top-level and row key order; the constant `label`; 17-significant-digit
round-trip against `outcome_bits` on every number row; and `host_scoped` equal to the pinned-list
rule on every one of the 6492 rows.

T7 (e), byte-stability, is checked by running the battery twice and diffing the two output trees —
`cp -r data/battery /tmp/run1 && efh-battery … battery && diff -r /tmp/run1 data/battery`. It has
been run and passed on this host.

**T7 (f) — the cross-host drill — has not been run.** Only one host was available
(`x86_64` / Windows / AMD64 Family 23 Model 96). Until a second architecture runs the same battery,
the claim "differences occur only on `host_scoped` rows" is untested, and the `host_scoped` field
should be read as a warning about where differences *may* appear, not as a measured boundary.
