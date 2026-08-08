---
schema: efh.function-page/v1
function_id: FUNC.IFNA
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Page vocabulary
  - Sources
family: ifna_fn
role_in_family: "The selective error mask: catches #N/A only and lets every other error through."
---

# IFNA

## What it computes

`IFNA(value, value_if_na)` returns `value` unless `value` is the specific error `#N/A`, in which
case it returns `value_if_na`.

The test is on **one error code**, not on error-ness. `#DIV/0!`, `#VALUE!`, `#REF!`, `#NAME?`,
`#NUM!`, `#NULL!` and the extended-family codes all pass straight through unmasked. That single
sentence is the whole reason the function exists.

Why it matters: `#N/A` is the conventional "no match" result of the lookup family
([chapter 01](../model/01-value-universe.md)), and it is the only error in ordinary lookup use that
means *the data said no* rather than *the formula is broken*. `IFNA` is therefore the mask that
suppresses the expected answer while leaving genuine breakage visible. `IFERROR` around the same
lookup suppresses both, which is how a deleted column or a misspelled range name turns into a
plausible-looking blank instead of an error.

Like `IF` and `IFERROR`, `IFNA` is a lazy selector: the fallback is reached only when the primary
is `#N/A`, and the selected value is returned verbatim without coercion.

## Arguments

`IFNA(value, value_if_na)`

- **`value`** — required. The expression to test; in practice, the lookup you actually wanted.
- **`value_if_na`** — required. The registry records arity exactly 2; neither argument is
  optional.

The misunderstood position is `value_if_na` for the same reason as in `IFERROR`: `IFNA(x, "")`
produces an empty *string*, a `Text` value, not a blank cell. A formula cannot publish emptiness
([chapter 01](../model/01-value-universe.md), admission matrix).

## Result and edge cases

Returns whatever kind the selected argument carries.

- **Primary is `#N/A`.** The fallback is returned.
- **Primary is any other error.** Returned unchanged — this is the defining behaviour, and the one
  readers migrating from `IFERROR` are surprised by.
- **Primary is a normal value.** Returned verbatim.
- **Fallback is itself `#N/A`.** Returned as-is; the mask does not iterate.
- **Empty cell as the primary.** The reference engine maps `Empty` to numeric `0`; blank is not
  `#N/A`.
- **Omitted argument slot.** The reference engine maps `Missing` to `#VALUE!` — which, being an
  error other than `#N/A`, is *not* masked and surfaces. This is the opposite of `IFERROR`'s
  behaviour on the same input, and it is a good illustration of how the two functions differ.
- **Arrays.** Whether the mask applies elementwise to an array primary or to the array as a whole
  is not settled by any source consulted here; see below.

## Errors

| Error | Condition |
|---|---|
| any error other than `#N/A` | Passed through from `value` unchanged — by design, not by failure. |
| `#N/A` | Only if `value_if_na` is itself `#N/A`. |
| `#VALUE!` | From argument preparation, including an omitted argument slot in the reference engine. |

Microsoft's `IFNA` page is the documented source for the syntax and the argument contract; it was
not re-fetched at this revision.

## Relationships

- **`IFERROR`** is the total mask. The pair is the clearest total-versus-selective contrast in the
  library, and choosing between them is a real design decision in every lookup formula.
- **`ISNA`** is the inspection counterpart; `IF(ISNA(x), y, x)` is the older idiom, which evaluates
  `x` twice.
- **`ISERR`** is `ISERROR` minus `#N/A` — the complement of what `IFNA` catches, from the
  inspection side.
- **`NA()`** produces the value `IFNA` looks for.
- **`XLOOKUP`** takes an `if_not_found` argument, folding this behaviour into the lookup itself;
  where `XLOOKUP` is available it usually removes the need for `IFNA` entirely, and the two should
  not be stacked.
- `IFNA` arrived in Excel 2013 and has no Compatibility-category predecessor; the pre-2013 form is
  the `IF(ISNA(...))` idiom.

## Notes for implementers

1. **Match on the code, not the family.** The test is equality with the single `#N/A` code. An
   implementation that tests "is it a legacy-family error" or "is it a lookup error" is a different
   function.
2. **Do not route `IFNA` through a shared `IFERROR` helper parameterised only by fallback.** The
   two differ in exactly one predicate, which makes the shared-helper refactor tempting and makes
   an accidental widening of the predicate invisible in review.
3. **The `Missing` mapping is observable.** Mapping an omitted slot to `#VALUE!` means `IFNA` and
   `IFERROR` give different answers for the same malformed call. Whichever mapping is chosen, it
   should be chosen deliberately.
4. **Preserve rich payloads on the pass-through path**, as with `IFERROR`.

## What has not been checked

There is no Handbook vector suite for `IFNA`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `IFNA`. Nobody has checked its behaviour against Excel for
this Handbook.

Probes worth running, in priority order:

1. `=IFNA(1/0, "x")` — confirms that a non-`#N/A` error passes through. This is the function's
   entire premise and it is unverified here.
2. `=IFNA({1, NA(), 1/0}, "x")` — elementwise or whole-array masking, and whether the two error
   codes are treated differently within one array.
3. `=IFNA(, "x")` versus `=IFERROR(, "x")` — the omitted-slot divergence the reference engine
   predicts.
4. `=IFNA(A1, "x")` with `A1` empty — confirms blank yields `0` rather than the fallback.
5. `=IFNA(VLOOKUP(...), "x")` where the lookup range is a `#REF!` — the realistic scenario the
   function exists for, confirming the break stays visible.

## Page vocabulary

| Term | Meaning |
|---|---|
| selective mask | Branches on one specific error code rather than on error-ness |
| total mask | Branches on any error value; `IFERROR`'s behaviour, contrasted here |
| lazy selector | The fallback is only evaluated when the primary matches |

## Sources

- Microsoft, IFNA function —
  <https://support.microsoft.com/en-us/office/ifna-function-6626c961-a569-42fc-a49d-79b4951fd461>
  (documented source for syntax and argument contract; not re-fetched at this revision).
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md) (the error registry; `#N/A` as the
  lookup convention) and [02 Coercion and lifting](../model/02-coercion-and-lifting.md) (declared
  error-branching families).
- `data/functions/FUNC.IFNA.json`, `data/presence/FUNC.IFNA.json`.
- OxFunc `crates/oxfunc_core/src/functions/ifna_fn.rs` at commit 473efa3 — the single-code test and
  the `Missing`→`#VALUE!`, `Empty`→`0` argument mapping, read as implementation facts about the
  reference engine.
