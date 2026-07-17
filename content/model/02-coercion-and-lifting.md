# Coercion and lifting

Status: draft (H1) · Sources: OxFunc 937f198

## Why this chapter exists

A worksheet function declares what it wants — a number here, text there — and the engine
converts what it is given. Those conversions, called coercions, are where two spreadsheets
computing "the same formula" quietly disagree. This chapter states the scalar conversion
rules, the error-propagation discipline, what happens to blank and omitted arguments, how
scalar functions lift over arrays, and the single most misunderstood asymmetry in the
worksheet model: why `SUM("3")` is 3 while `SUM(A1)` is 0 when `A1` contains `"3"`.

A recurring theme, worth stating up front: **some rules are universal, and some are
per-function-family policy.** The model refuses to invent a global rule where Excel does not
have one. Every rule below is labeled accordingly. Value kinds and boundary names used here
are defined in the value-universe chapter.

## The conversion primitives

The model defines a small set of typed primitives that function contracts are written
against:

| Primitive | What it does |
|---|---|
| to-number | Convert a scalar value to a number, or fail with a typed reason |
| to-text | Convert a scalar value to text |
| to-logical | Convert a scalar value to TRUE/FALSE |
| propagate-error | Pass an incoming error value through as the result |
| reference-resolve | Turn a reference into the value(s) it designates (an explicit, separate step) |
| array-lift | Apply a scalar function elementwise over an array argument |

Every coercion outcome is explicit: a conversion either produces the target kind or produces
a *named* failure (non-numeric text, empty cell, missing argument, worksheet error,
unsupported kind, unresolved reference). Nothing is silently absorbed. What a function does
with a named failure — surface `#VALUE!`, substitute a default, skip the value — is the
per-family part.

## Converting to number

The to-number rules for each scalar kind, as pinned in the source model:

| Given | Result |
|---|---|
| Number | Itself, unchanged |
| Logical | TRUE → 1, FALSE → 0 |
| Text | Parsed as a number if it reads as one (leading/trailing whitespace ignored); otherwise a non-numeric-text failure, which functions typically surface as `#VALUE!` |
| Error | The error propagates; no conversion is attempted |
| Empty | A distinct empty-cell outcome; in most numeric contexts the function maps it to 0 (per-family policy) |
| Missing | A distinct missing-argument outcome; the function applies its documented default (per-family policy) |
| Array | Not directly convertible; handled by lifting or by aggregate scanning (below) |
| Reference | Resolved first, then the resolved value converts by these same rules |

Two honesty notes on the text row:

1. **The parse must produce a finite number.** Text that parses to an overflow (or to a
   non-numeric special value) is treated as non-numeric, not as infinity.
2. **The full recognizer is broader than the source currently pins.** Excel's text-to-number
   recognition accepts locale-dependent forms — thousands separators, currency symbols,
   percent signs, date and time text — whose exact grammar the source model has probed only
   at the edges (plain decimals and scientific notation, including values near the
   floating-point underflow boundary). The pinned baseline covers the plain-number core;
   the locale-dependent surface is an open area, and per-function pages will say when a
   behavior depends on it.

## Converting to text and to logical

The model requires to-text and to-logical as primitives, but — unlike to-number — the source
pins them only as requirements, with the detailed behavior distributed across individual
function implementations rather than centralized. The Handbook therefore states the outline
here and defers precise per-context behavior to per-function pages:

- **To text:** numbers render using general formatting rules; TRUE and FALSE render as the
  text `TRUE` and `FALSE`; errors propagate rather than rendering.
- **To logical:** zero converts to FALSE and any nonzero number to TRUE; text that reads as
  a logical name (`"TRUE"`, `"FALSE"`) is accepted in some contexts. Which contexts is
  per-family policy, not a universal rule.

## Error propagation

The universal rule, stated by the source as a hard discipline: **coercion never silently
discards a worksheet error.** An error value arriving at a conversion propagates out as the
result, unless the function's family explicitly declares masking or branching behavior.

The declared exceptions are familiar: `IFERROR` and `IFNA` exist to branch on errors;
`ISERROR` and its siblings inspect them; some aggregates (`AGGREGATE` with the appropriate
option) skip them. Each is a per-family declaration, visible on the function's page — never
an implicit engine behavior.

Note the direction of this rule: it is about coercion, not about arrival. An error in a
scanned range does surface from `SUM` — skipping errors is not part of the ignore-text scan
policy. `SUM(A1:A3)` with an error in `A2` is that error.

## Blank and missing at the call boundary

The value-universe chapter defines the Missing/Empty distinction; here is how it plays out
at a call:

1. **Omitting a required argument is an admission failure, not a runtime value.** Entering
   `=SIN()` is refused at formula entry — observed as an entry-time rejection, distinct from
   every runtime outcome in this chapter. There is no error *value* for a missing required
   argument, because the formula never runs.
2. **Omitting an optional argument delivers the Missing marker.** The function applies its
   documented default. An empty slot between commas (`SUM(1,,2)` — result 3) is Missing,
   not Empty; here `SUM` treats the omitted slot as contributing nothing.
3. **An empty referenced cell delivers Empty, and the function decides.** In a direct scalar
   numeric context the common mapping is zero; in an aggregate's range scan the cell is
   skipped. The two outcomes are kept distinct precisely because functions treat them
   distinctly — the model forbids conflating them upstream of the function.

## References at the call boundary

Function kernels consume already-resolved values. Reference resolution is an explicit,
separate step with its own failure modes — it is not folded into coercion:

- A reference argument is resolved, then the resolved value converts by the ordinary rules.
- An unresolvable defined name surfaces as `#NAME?`.
- Reference-producing functions (`OFFSET`, `INDIRECT`, spill-anchor expressions) yield
  reference values that are normalized and resolved before a consuming function converts
  them; each such path is declared per-function, including whether resolution happens at
  evaluation time (as with `INDIRECT`'s text-built references).
- External-workbook references add an open-versus-closed state axis, recorded per-function.

Functions declared reference-aware receive the reference itself and fall outside this
section's scalar rules.

## Array lifting

When a scalar function receives an array where a scalar was declared, the engine *lifts* the
function elementwise: `ABS({-1,2,-3})` applies `ABS` to each element and yields
`{1,2,3}`.

The pinned baseline for scalar-lift families (the `SIN` / `ASIN` / `ABS` style of function):

1. **Lifting is elementwise, per argument.** Each declared scalar position that receives an
   array contributes its elements pointwise; the page for each function states which
   argument positions lift.
2. **Element failures stay element-local.** A mixed array like `{1,"asd",3}` under a lifted
   numeric function produces an array whose middle element is an error — the failure does
   not collapse the whole result. (Domain errors behave the same way: `ASIN({0,2})` has one
   good element and one `#NUM!`.)
3. **This policy is provisional.** The source marks it with an explicit revision trigger:
   any observed contradiction in a declared scalar-lift family forces a policy revision
   rather than a quiet exception.

Aggregate families (`SUM` and peers) are explicitly *not* lift kernels — an array argument
to an aggregate is consumed by scanning, not by elementwise lifting. That distinction is the
subject of the next section.

## Direct arguments versus range scans

The flagship asymmetry. With `"3"` typed into `A1` as text:

- `SUM("3")` → **3**. The text is a *direct argument*, and direct scalar arguments undergo
  ordinary to-number coercion.
- `SUM(A1)` → **0**. The cell arrives via a *range scan*, and Excel's documented scan rule
  for `SUM` ignores text in scanned ranges — including text that would coerce perfectly
  well. Logical values in scanned ranges are likewise ignored, while a direct
  `SUM(TRUE)` is 1.

Same value, same function, different boundary, different rule. Both behaviors are correct;
they are two different coercion policies keyed on how the value reached the function.

The model's treatment, and this is a deliberate design position in the source: **there is no
global precedence rule between direct-argument coercion and range-scan coercion.** Each
aggregate family carries an explicit policy row stating both behaviors; nothing falls back
to an implicit engine-wide default. `COUNT`, `AVERAGE`, `SUMPRODUCT`, and the database
functions each get their own row because Excel's own choices differ across them.

The source's executable baseline expresses scan behavior as named policies — a strict
all-numeric scan (any unconvertible value fails the whole aggregate) and an
ignore-text-and-empty scan (skip what does not convert) — with the per-family matrix
choosing among them. The matrix rows are marked provisional against probe evidence, with
contradiction triggers: an observed counterexample forces the row to be revised, never
papered over.

## What is universal and what is policy

A closing summary of the boundary between engine law and family policy, as the source draws
it:

| Universal (engine law) | Per-family policy |
|---|---|
| Missing and Empty are distinct outcomes at the call boundary | What a function *does* with Missing (default) or Empty (zero, skip, …) |
| Errors propagate through coercion | Masking or branching on errors (`IFERROR`, inspection functions, opt-in skipping) |
| Text→number parsing yields a value or a named failure | Whether a family surfaces the failure, skips the value, or never attempts the parse (range scans) |
| Reference resolution is explicit and precedes conversion | Which functions are reference-aware; when resolution happens |
| Admission at formula entry precedes all runtime coercion | Which arguments are optional |
| Lifted element failures stay element-local (provisional) | Which argument positions lift at all |
| — | Direct-argument versus range-scan precedence (deliberately per-family) |

## Page vocabulary

Machine names that per-function pages may display, mapped to plain language:

| Machine name | Meaning |
|---|---|
| `MissingArg` | Coercion outcome: the argument slot was omitted at the call site |
| `EmptyCell` | Coercion outcome: the value came from a cell with no content |
| `NonNumericText` | Coercion outcome: text that does not read as a number |
| `WorksheetError` | Coercion outcome: an error value arrived and propagates |
| `UnsupportedValueKind` | Coercion outcome: the value kind has no conversion in this context |
| `RefResolution` | Coercion outcome: a reference could not be resolved |
| `StrictAllNumeric` | Scan policy: any unconvertible scanned value fails the aggregate |
| `IgnoreTextAndEmpty` | Scan policy: skip scanned values that do not convert |
| `direct_arg_coercion_rule_set` | The family's rules for values passed directly as arguments |
| `range_scan_coercion_rule_set` | The family's rules for values reached by scanning a range |
| `scalar_kernel_lift` | The function is a scalar kernel applied elementwise over arrays |
| `not_lift_kernel` | The function consumes arrays whole (aggregates); no elementwise lift |
| `elementwise_error_bearing` | Lifted results carry element-level errors without collapsing |
| `required_arg_admission_check` | Omission of this argument is rejected at formula entry |
| `coerce_post_resolve` | References are resolved first; coercion applies to the result |
| `eval_time_deref` | The reference is built and resolved during evaluation (e.g. `INDIRECT`) |
| `empty_cell_distinct_from_missing` | The family observes the Empty/Missing distinction explicitly |
| `interop_truncate_then_coerce` | Over-cap text is truncated at the interop boundary before coercion |
| `provisional` | Policy recorded with a contradiction trigger; revised on counterexample |

## Sources

- `docs/function-lane/COERCION_AND_CONVERSION_PRELIM_SPEC.md` — the primitive set, boundary
  rules, array-lift baseline, and the deliberate refusal of a global direct-versus-scan
  precedence. Provisional policy spec with named revision triggers.
- `docs/function-lane/COERCION_DECISION_TABLE.csv` — the per-family policy matrix (twelve
  rows at this snapshot), each tied to probe-scenario evidence and marked provisional.
- `docs/function-lane/COERCION_SCENARIO_MANIFEST_SEED.csv` — the probe scenarios behind the
  matrix (direct-versus-scan contrast, mixed-array lift, entry-time rejection of omitted
  required arguments, empty-versus-missing); empirical basis.
- `crates/oxfunc_core/src/coercion.rs` — the executable to-number baseline: scalar
  conversion, typed failure outcomes, reference resolution seam, and the named aggregate
  scan policies, with tests. To-text and to-logical are required by the spec but not yet
  centralized here; their descriptions above are correspondingly outline-level.
- `docs/function-lane/VALUE_UNIVERSE_PRELIM_SPEC.md` — boundary definitions consumed by this
  chapter (call-argument boundary, raw-versus-published normalization).
