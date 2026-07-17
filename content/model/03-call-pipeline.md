# The call pipeline

Status: draft (H1) · Sources: OxFunc 937f198

## Overview

Every worksheet function call — `ABS(A1)`, `SUM(A1:B9)`, even `3+4` — can be described by one
uniform pipeline. The pipeline separates what a function *means* (its mathematical or textual
core) from how Excel *feeds* it (how arguments are fetched, converted, and spread over arrays)
and how Excel *publishes* what it returns (how overflow, errors, and precision quirks surface
in the cell). This chapter defines that pipeline. It is grounded in OxFunc, a reference
implementation that re-creates Excel function behavior bit for bit and records each behavior as
an explicit, named declaration.

The pipeline has four stages:

```
surface arguments → argument preparation → coercion and lifting → kernel → result publication
```

- **Surface arguments** are the expressions as written in the formula: literals, cell
  references, ranges, arrays, nested calls, or nothing at all (an omitted argument).
- **Argument preparation** decides whether the function receives plain values or live
  references, and resolves references when values are wanted.
- **Coercion and lifting** converts each prepared value to the type the function needs
  (number, text, logical) and, when an argument is an array, maps the function over the
  array's elements.
- The **kernel** is the pure semantic core: for `ABS`, a function from one number to one
  number. It knows nothing about references, arrays, or the worksheet.
- **Result publication** maps the kernel's raw result to what Excel actually shows: turning
  floating-point overflow into `#NUM!`, folding error inputs by a fixed precedence order,
  or applying a publication-time precision rule.

Every function page in this handbook carries behavior chips. Each chip is a value on one of
the axes defined in this chapter or the next: the chip names are the machine enum variants of
the reference implementation, so a page's chips are checkable claims, not prose summaries.

## Stage 1: argument preparation

Before a function's own logic runs, each argument is prepared. The single question this stage
answers is: does the function see *values* or *references*?

A **reference** is a live pointer into the workbook (`A1`, `Sheet2!B3:C9`, a named range). A
**value** is the materialized content: a number, text, a logical, an error, or an array of
these. Most functions only care about content, so the engine resolves references before the
function runs. A minority genuinely inspect the reference itself — its address, its shape,
which cell it points at — and for those the reference must survive into the function.

This is a declared, two-valued axis (`ArgPreparationProfile`):

- **ValuesOnlyPreAdapter** — the majority shape (199 of the 241 functions in the reference
  catalog). References are dereferenced and arguments delivered as plain values. The function
  can never tell whether `5` arrived as a literal or via `A1`.
- **RefsVisibleInAdapter** — the deviation. The function receives the live reference and
  controls dereference timing itself. Carried by reference-aware functions such as `ROW`,
  `COLUMN`, `OFFSET`, `CELL`, `INDEX`, and the lookup functions. This split is empirically
  pinned against live Excel (16.0 build 20026) in the reference implementation.

Values-only preparation also performs one normalization worth knowing about: a reference to a
single cell materializes as a 1×1 array, and preparation collapses that to the single scalar
inside it. One caveat is preserved deliberately: when an argument position participates in
array lifting (next stage), a 1×1 *array* is kept as an array so it flows through the same
lift-and-intersect path as a larger one — matching Excel, which lifts a 1×1 array argument and
takes its top-left element rather than treating it as a bare scalar.

Preparation also keeps two "nothing" states distinct: an **omitted argument** (a gap in the
call, like the middle argument of `SUM(A1,,B1)`) and an **empty cell** (a reference that
resolved to a blank). They are different values in the pipeline, and functions may treat them
differently. Which treatment each function family applies is still an open policy question in
the sources (decision D-005 in the function-definition discussion register); the pipeline
mechanism itself — carrying the two states separately — is settled.

## Stage 2: coercion and lifting

Prepared values are then converted to the types the kernel expects, and scalar kernels are
spread over array arguments.

**Coercion** is Excel's type conversion: the text `"2"` becomes the number 2, `TRUE` becomes
1, an error input becomes a coercion failure that the function's error policy will handle. Each
function declares a coercion/lift category (`CoercionLiftProfile`) naming the shape of this
behavior — unary numeric, aggregate with a dual direct-versus-range policy, lookup/match, or
custom.

**Lifting** (also called broadcasting) is what happens when a scalar-shaped function receives
an array: `ABS({-1,2,-3})` applies the kernel elementwise and returns `{1,2,3}`. When several
arguments are arrays of different shapes, they are broadcast to the common shape (a 1-row or
1-column array is stretched to match); where a coordinate exists in the result shape but not
in some argument, that result cell is `#N/A`. A per-cell failure does not abort the whole
call: the failing cell carries its own error while the rest of the array is computed, which is
exactly how Excel's dynamic arrays behave.

How lifting is wired is itself a declared axis (`LiftBroadcastProfile`):

- **SurfaceNative** — the majority. The function's own evaluation does whatever lifting it
  needs (or none, for functions that take arrays as arrays, like `SUM`).
- **ByIndexScalarArrayLift** — the function is scalar-shaped and the dispatch layer
  broadcasts it, but only over a named list of argument positions. `ADDRESS` lifts positions
  0–4; `SWITCH` lifts positions 0, 1, and 3; the inverse-distribution functions lift 0–2. The
  position list is per-function, irreducible structure, and is empirically pinned against
  live Excel (16.0 build 20026).

For aggregate functions there is one more distinction with observable consequences: a value's
**origin**. `SUM(5, A1:A9)` treats the direct scalar `5` differently from the cells scanned
out of the range — text that would coerce as a direct argument is skipped when it comes from a
range. The preparation layer therefore tags each expanded value as direct-scalar or
array-like (and, for array-like, whether it came from an array literal, an opaque array value,
or a reference), so the aggregate's dual policy can act on provenance. The normative policy
when signals conflict is recorded as an open decision (D-004) in the sources.

## Stage 3: the kernel

The kernel is the pure core: no references, no worksheet, no host. Its shape is declared as a
`KernelSignatureClass` — a constant (`PI`), number to number (`ABS`), numbers to number
(`ATAN2`), text to text, lookup/match, or custom. Keeping the kernel pure is what makes
bit-exact verification tractable: the kernel can be tested and proven against Excel's numbers
in isolation, while the preparation and coercion layers are shared, declarative machinery
that is verified once.

## Stage 4: result publication

The kernel's raw result is not always what Excel shows. Publication policies — declared per
function, applied identically on every dispatch path — close the gap.

**Non-finite handling** (`ExcelRealPolicy`, composed of an argument-domain guard and a
non-finite rule). IEEE-754 arithmetic produces infinities and NaN; Excel cells never show
them. Each numeric function declares what happens:

- `NonFinite::Allow` — pass through; declares that the kernel cannot produce a non-finite
  result for any valid argument.
- `NonFinite::Num` — overflow or NaN publishes as `#NUM!` (`EXP`, `SINH`, `COSH`, `FACT`,
  and others).
- `NonFinite::SaturateSign` — overflow saturates to ±1 (`COTH`).

The argument-domain guard runs before the kernel: `ArgDomainGuard::CircularTrigOverflow`
encodes Excel's circular-trig limit — `SIN`, `COS`, `TAN`, `COT`, `SEC`, `CSC` return `#NUM!`
once the argument's magnitude reaches 2^27. That limit is empirically pinned against live
Excel (16.0 build 20026).

**Error folding** (`ErrorCollapseProfile`). When error values arrive as inputs, most functions
simply let the error propagate. Two families do more, and both apply Excel's canonical legacy
error-precedence order (`ErrorAlgebra::CanonicalExcelLegacy`) when several errors compete:

- `ReductionFold` — reductions and aggregations (`SUM`, `MAX`, `COUNTIF`, the database
  functions, matrix reducers) fold many inputs into one result and collapse error inputs by
  precedence.
- `SelectorBranch` — branch selectors (`IF`, `IFS`, `CHOOSE`, `IFERROR`, `IFNA`, `SWITCH`)
  choose among branches that may themselves be errors.

Both behaviors are empirically pinned against live Excel (16.0 build 20026).

**Precision publication** (`PrecisionRoundingProfile`). Almost every function publishes the
kernel's plain IEEE-754 double result (`Default`). The one currently identified separable
deviation is `IntegerExponentPublication`, carried by `POWER` and the `^` operator: when the
exponent is an exact integer, Excel computes the power by repeated multiplication (binary
exponentiation) instead of the transcendental `exp(n·ln x)` path, giving a bit-different —
and observed-in-Excel — result such as `POWER(1.05,10) = 1.6288946267774416`. Empirically
pinned against live Excel (16.0 build 20026). Note what this axis is *not*: `ROUND`, `TRUNC`,
`CEILING` and friends are not modeled here, because rounding is their defined purpose, not a
publication quirk.

**Host-side adaptation.** After the function returns, the calling engine may adapt the result
further: anchoring an array result as a dynamic-array spill, or applying a cell format hint
(the documented seed examples are `NOW` and `TODAY` entered into a General-formatted cell).
The sources treat these as engine obligations at the worksheet boundary, not function
semantics, and this handbook follows that split: function pages describe values, not cell
presentation.

## Operators are functions

Every evaluable operator is modeled as a function with its own identity in the same catalog,
sharing the whole pipeline above. The identities use an `OP_` prefix under the same namespace
as named functions: `FUNC.OP_ADD` (`+`), `FUNC.OP_CONCAT` (`&`), `FUNC.OP_POWER` (`^`),
`FUNC.OP_PERCENT`, the comparison operators, the unary operators, and the reference-algebra
operators — `FUNC.OP_RANGE_REF` (`:`), `FUNC.OP_UNION_REF`, `FUNC.OP_INTERSECTION_REF`,
`FUNC.OP_SPILL_REF` (`#`), `FUNC.OP_IMPLICIT_INTERSECTION` (`@`), and the trim-reference
family (`FUNC.OP_TRIM_REF_LEADING` / `_TRAILING` / `_BOTH`). So `3+4` is a two-argument call
to `FUNC.OP_ADD`, with the same argument preparation, broadcasting, and publication axes as
any named function — which is why operator pages in this handbook look like function pages.

Not every token becomes a function: parse-only delimiters (such as the locale-dependent
argument separator) are syntax, carry no semantics of their own, and get no function identity.

## Arity and the admission boundary

Each function declares an **arity** — a minimum and maximum argument count. Whether an
ill-formed call is rejected at formula entry or accepted and evaluated to an error is a real,
two-surface boundary in the sources: `SIN()` is rejected when the formula is entered
(admission), while `SIN("asd")` and `ASIN(2)` are accepted and evaluate to runtime errors
(`#VALUE!`, `#NUM!`). The full per-family admission policy is documented as an open decision
(D-017); the seeds above are the pinned anchors. This handbook's function pages describe the
runtime surface; admission behavior is noted where it is known.

## Functions that step outside the plain pipeline

The uniform pipeline is the default, not a straitjacket. A function steps outside it only
when observable Excel behavior requires it, and the deviation is always declared:

- **Reference-sensitive functions** (`ArgPreparationProfile::RefsVisibleInAdapter`) control
  their own dereference timing, and may return references rather than values (`OFFSET`,
  `INDEX` in reference form).
- **Lazy branch selectors** evaluate only the selected branch and return it verbatim — which
  is also why `IF(TRUE, lambda)` can pass a lambda value through unchanged.
- **Caller-aware functions** (`ROW()` with no argument, `CELL`) read the calling cell's
  position from the execution context (next chapter) instead of any argument.
- **Selective dereference** — probing a sub-range of a lookup area instead of materializing
  all of it — is deliberately *not* a general pipeline capability in the sources; it remains
  function-local, with a recorded open decision (D-018) on whether to generalize it.

## Page vocabulary

Chips a function page may display for the axes in this chapter, with the exact machine names:

| Axis / value | Plain meaning |
|---|---|
| `Arity { min, max }` | Accepted argument count range (max may be unbounded) |
| `ArgPreparationProfile::ValuesOnlyPreAdapter` | References are resolved to plain values before the function runs |
| `ArgPreparationProfile::RefsVisibleInAdapter` | The function sees live references and controls dereference itself |
| `CoercionLiftProfile::None` | No shared coercion/lift category applies |
| `CoercionLiftProfile::UnaryNumericScalarOnly` | One numeric argument, scalar only |
| `CoercionLiftProfile::UnaryNumericScalarOrArrayElementwise` | One numeric argument, mapped elementwise over arrays |
| `CoercionLiftProfile::AggregateDirectAndRangeDualPolicy` | Aggregate with different coercion for direct arguments vs range-scanned cells |
| `CoercionLiftProfile::LookupMatchProfile` | Lookup/match argument handling |
| `CoercionLiftProfile::Custom` | Function-specific coercion behavior |
| `LiftBroadcastProfile::SurfaceNative` | The function does its own array lifting (or none) |
| `LiftBroadcastProfile::ByIndexScalarArrayLift([..])` | Scalar kernel broadcast over the listed argument positions |
| `KernelSignatureClass::NullaryConst` | Kernel is a constant (no arguments) |
| `KernelSignatureClass::NumToNum` | Kernel maps one number to one number |
| `KernelSignatureClass::NumsToNum` | Kernel maps several numbers to one number |
| `KernelSignatureClass::TextToText` | Kernel maps text to text |
| `KernelSignatureClass::LookupMatch` | Kernel is a lookup/match core |
| `KernelSignatureClass::Custom` | Kernel shape is function-specific |
| `ArgDomainGuard::None` | No pre-kernel argument check |
| `ArgDomainGuard::CircularTrigOverflow` | Reject argument magnitude ≥ 2^27 with `#NUM!` (circular trig) |
| `NonFinite::Allow` | Kernel cannot produce a non-finite result; raw value passes through |
| `NonFinite::Num` | Non-finite result publishes as `#NUM!` |
| `NonFinite::SaturateSign` | Overflow saturates to ±1 |
| `ErrorCollapseProfile::None` | Error inputs propagate through ordinary value handling |
| `ErrorCollapseProfile::ReductionFold` | Aggregation that collapses error inputs by precedence |
| `ErrorCollapseProfile::SelectorBranch` | Branch selector that collapses branch errors by precedence |
| `ErrorAlgebra::CanonicalExcelLegacy` | Excel's classic error-precedence order |
| `PrecisionRoundingProfile::Default` | Publishes the plain IEEE-754 kernel result |
| `PrecisionRoundingProfile::IntegerExponentPublication` | Exact-integer exponents computed by repeated multiplication (`POWER`, `^`) |

## Sources

- `docs/function-lane/FUNCTION_ADAPTER_LAYERING_PRELIM_SPEC.md` — the layered
  preparation/coercion/kernel pipeline contract. Documented; the spec marks itself
  provisional.
- `docs/function-lane/EXCEL_FUNCTION_DEFINITION_PRELIM_SPEC.md` — class axes, pre-call
  coercion and post-call adaptation boundary, operator-as-function split, admission-vs-runtime
  boundary. Documented; preliminary by its own statement.
- `docs/function-lane/EXCEL_FUNCTION_DEFINITION_DISCUSSION.md` — open decisions D-004
  (aggregate coercion), D-005 (argument gaps), D-008 (operator inventory), D-017 (admission
  boundary), D-018 (selective dereference). Documented open questions.
- `crates/oxfunc_core/src/function.rs` — `FunctionMeta` and the axis enums; the
  `RefsVisibleInAdapter`, `ByIndexScalarArrayLift`, `ReductionFold`, `SelectorBranch`, and
  `IntegerExponentPublication` variants each state "Verified live Excel 16.0 build 20026"
  (empirically pinned).
- `crates/oxfunc_core/src/functions/excel_numeric.rs` — `ExcelRealPolicy`, `ArgDomainGuard`,
  `NonFinite`; the 2^27 circular-trig limit is stated as verified against live Excel
  (empirically pinned).
- `crates/oxfunc_core/src/functions/adapters.rs` — shared declarative runners: values-only
  preparation, unit-array preservation for lift positions, broadcasting with per-cell errors
  and `#N/A` for missing coordinates, aggregate origin tagging. Implementation source.
- `crates/oxfunc_core/src/semantic_kernel.rs` — `ErrorAlgebra` vocabulary. Implementation
  source.
- `crates/oxfunc_core/src/registry_signature_seed.rs` — the concrete `FUNC.OP_*` operator
  identities. Implementation source.
