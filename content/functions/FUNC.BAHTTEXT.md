---
schema: efh.function-page/v1
function_id: FUNC.BAHTTEXT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — BAHTTEXT function"
    locator: "https://support.microsoft.com/en-us/office/bahttext-function-5ba4d0b4-abd3-4325-8d22-7a92d59aab9c"
    role: "documented signature and description"
  - work: "OxFunc — FUNCTION_SLICE_MISC_ORDINARY_CONVERSION_TRIAD_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_MISC_ORDINARY_CONVERSION_TRIAD_CONTRACT_PRELIM.md"
    role: "upstream current-baseline contract: satang rounding, Thai-script output, #NUM! domain"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Notes for implementers"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: misc_conversion_family
role_in_family: "The number-to-words member of the miscellaneous conversion group; the only
  natural-language spell-out function in the worksheet surface."
---

## What it computes

`BAHTTEXT` spells a number out in Thai words as an amount of currency, appending the Thai word for
baht — and, when there is a fractional part, the satang.

It is worth being clear about how unusual this is. `BAHTTEXT` is the only function in the ordinary
worksheet surface that performs **number-to-words conversion in a natural language**. There is no
`ENGLISHTEXT`; the equivalent in every other language is a user-defined function or a long nested
formula. Its presence in the catalogue is a historical artefact of Excel's Thai localisation, and
it is available regardless of the installation's language — the output is always Thai script.

The computation has three stages:

1. **Round to satang.** A baht is divided into 100 satang, so the number is rounded to two decimal
   places. Upstream OxFunc's contract records this as rounding to satang under the kernel's
   rounding policy; which rounding rule that is — half-up, half-even, or truncation — is precisely
   the kind of detail this Handbook will not assert without evidence.
2. **Spell the integer part** using Thai numeral words, then append the word for baht. Thai numeral
   spelling has its own irregularities — notably the special forms used for "one" in the units
   place and for "twenty" — which is what makes this a table-and-rules problem rather than a digit
   substitution.
3. **Spell the fractional part** in satang and append the satang word; when the fraction is zero,
   append instead the Thai word meaning "exactly" or "whole" (ถ้วน), which is the conventional way
   to close a written currency amount and to prevent a figure being altered after the fact.

That last convention is why the zero case and the non-zero case produce structurally different
strings rather than one with an empty tail.

## Arguments

`BAHTTEXT(number)`

| Argument | Required | Meaning |
|---|---|---|
| `number` | yes | The number to convert to Thai baht text. |

One argument. It is a *number*, so text that reads as a number converts and `TRUE` converts to `1`
under the general to-number rule in [coercion and lifting](../model/02-coercion-and-lifting.md).

There is no argument controlling the script, the currency word, or whether the trailing "exactly"
appears. There is no way to get the words without the currency.

## Result and edge cases

The return kind is `Text`, in Thai script.

- **Negative numbers.** Upstream's contract records the current baseline as rejecting negative
  magnitudes with `#NUM!`. That is a domain restriction, not a formatting choice: there is no
  spelled negative form.
- **Very large magnitudes** are likewise recorded upstream as rejected with `#NUM!`. Where the
  boundary sits — and whether it is a magnitude limit, a digit-count limit, or the point at which a
  double stops representing integers exactly — is not established here.
- **Rounding is observable at the boundary.** A value ending in exactly half a satang
  (`1.005`, `2.675`) will expose the rounding rule, and will also expose the fact that such values
  are not exactly representable as doubles in the first place. This is the one place where
  `BAHTTEXT` has a genuine floating-point subtlety, and it is inherited from the argument, not
  created by the function.
- **Zero** produces the zero-baht form with the "exactly" tail.
- **Errors propagate**; an empty referenced cell delivers `Empty`, distinct from an omitted
  argument (see [the value universe](../model/01-value-universe.md)).
- **Array arguments.** Whether `BAHTTEXT` lifts elementwise is not asserted here; the general rules
  are in [coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

Microsoft's page is the authority and is linked below; it was not retrievable while this entry was
written, so this section states expectations and upstream readings rather than quoting it.

| Error | Status |
|---|---|
| `#NUM!` for negative `number` | recorded in upstream OxFunc's current-baseline contract; not a Handbook claim |
| `#NUM!` for a magnitude beyond the supported range | recorded in the same upstream contract; the exact boundary is not established |
| `#VALUE!` for a non-numeric argument | expected from the general to-number coercion rule |
| any error value, propagated | follows from the universal error-propagation rule |

## Relationships

- `TEXT` — the general number-formatting function, and the usual alternative when what you actually
  wanted was a formatted numeral rather than words. `TEXT` cannot spell numbers out in any
  language.
- `DOLLAR` and `FIXED` — the other currency-adjacent text functions. Both format digits; neither
  spells. `DOLLAR` is locale-sensitive in its symbol, which is a different kind of localisation
  from `BAHTTEXT`'s fixed Thai output.
- `EUROCONVERT` and `CONVERT` — the other members of the miscellaneous conversion group upstream.
  They convert *quantities*, not representations, and share nothing with `BAHTTEXT` semantically.
  (Upstream's replay observed `EUROCONVERT` returning `#NAME?` on the host tested, a reminder that
  admission of the compatibility functions varies.)
- Readers sometimes expect `BAHTTEXT` to respect the workbook's currency locale. It does not; the
  output is Thai regardless of where the workbook is opened.
- Nothing supersedes `BAHTTEXT` and it supersedes nothing.

## Notes for implementers

- **The spelling rules are the specification.** Thai numeral spelling is not digit-by-digit: the
  unit-place "one" has a distinct form, "twenty" has a distinct form for the tens digit, and the
  grouping above a million repeats a scale word rather than introducing new ones. Get these from a
  Thai orthography reference, not from a transliteration table.
- **Fix the rounding rule and record it.** Rounding to two decimals is the only arithmetic in the
  function and it is where any divergence will live. Do the rounding on the decimal value the user
  sees, and be explicit about half-way cases.
- **The trailing "exactly" is conditional on the satang being zero after rounding**, not before.
  `BAHTTEXT(1.001)` and `BAHTTEXT(1)` should agree if rounding takes the first to one baht exactly.
- **Emit the Thai script directly as Unicode**, not through a legacy Thai code page. The result is
  worksheet text, which is UTF-16.
- **Validate the domain before spelling**, so that a negative or oversized input produces `#NUM!`
  rather than a partially built string.
- **Do not localise the output.** It is Thai on every installation; making it follow the UI language
  would be a divergence, not an improvement.

## What has not been checked

There is no Handbook vector suite for `BAHTTEXT`, and no Handbook evidence record comparing any
implementation against Excel. Nothing on this page is a measurement. Upstream OxFunc records that a
native Excel replay matched its seeded `BAHTTEXT` rows on the host baseline it used, and the
Handbook does not restate that as a result of its own; it is an upstream finding, scoped to that
packet and that host.

This function is also an unusually good vector-suite candidate, because its output is exact text
with no floating-point residual to argue about. The probes that would build one:

1. **The rounding rule.** `BAHTTEXT(1.005)`, `BAHTTEXT(1.015)`, `BAHTTEXT(2.675)`,
   `BAHTTEXT(0.005)` — enough to separate half-up from half-even, and to see how the
   binary-representation error at these values is handled.
2. **The magnitude boundary.** Sweep powers of ten upward until `#NUM!` appears, then bisect. Record
   whether the boundary is at a round decimal figure or at a floating-point landmark such as 2^53.
3. **The Thai scale words above a million**, by probing values at 10^6, 10^7, 10^12 and just below
   each, which is where number-to-words implementations most often diverge.
4. **The "one" and "twenty" irregular forms**, by probing 1, 11, 21, 20, 101, 111.
5. **Zero and near-zero.** `BAHTTEXT(0)`, `BAHTTEXT(0.004)`, `BAHTTEXT(0.005)` — whether a value
   that rounds to zero produces the zero form with the "exactly" tail.
6. **Negative zero and tiny negatives**, `BAHTTEXT(-0)` and `BAHTTEXT(-0.001)`, which test whether
   the domain check runs before or after rounding.
7. **Array lifting**, in one cell.
8. **Whether the output varies by installation language**, which the Handbook expects to be "no" and
   has not confirmed.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| satang | One hundredth of a baht; the fractional unit the number is rounded to |
| spell-out | Rendering a number as words rather than digits |
| "exactly" tail | The Thai closing word (ถ้วน) appended when the satang part is zero |
| domain check | The magnitude and sign test that produces `#NUM!` before any spelling happens |

## Sources

- Microsoft Support, BAHTTEXT function —
  <https://support.microsoft.com/en-us/office/bahttext-function-5ba4d0b4-abd3-4325-8d22-7a92d59aab9c>
  (the authority for signature and documented description; not retrievable while this entry was
  written, so it is cited by reference and nothing here paraphrases it).
- OxFunc `docs/function-lane/FUNCTION_SLICE_MISC_ORDINARY_CONVERSION_TRIAD_CONTRACT_PRELIM.md` —
  the current-baseline contract: scalar numeric input, rounding to satang, Thai-script baht/satang
  output, and rejection of negative or excessively large magnitudes with `#NUM!`; also the observed
  `#NAME?` for `EUROCONVERT` on the same host. Provisional by its own statement.
- Handbook `content/model/01-value-universe.md` (the `Number` and `Text` kinds, Empty versus
  Missing), `02-coercion-and-lifting.md` (to-number coercion, error propagation).
