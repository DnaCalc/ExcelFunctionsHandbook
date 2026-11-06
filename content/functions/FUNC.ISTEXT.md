---
schema: efh.function-page/v1
function_id: FUNC.ISTEXT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - "What it does"
  - "Try it"
  - "Quirks"
  - "How to implement"
  - "Version differences"
  - "Alternatives"
  - "Sources"
family: is_predicates_family
role_in_family: The Text-kind test; one half of the family's only exact complementary pair.
---

# ISTEXT

## What it does

`ISTEXT(value)` returns TRUE when the value is text and FALSE otherwise. It inspects the value's
kind; it does not try to interpret its contents as a number. Thus `ISTEXT("19")` is TRUE while
`ISTEXT(19)` is FALSE.

The empty string `""` is text, including when a formula returns it. A genuinely empty cell is
different. Formatting a numeric cell to look like a label does not turn the stored number into text.

## Try it

Compare `=ISTEXT("19")`, `=ISTEXT(19)`, `=ISTEXT("")` and `=ISTEXT(NA())`. To test a blank cell
here, supply `{"B2":null}` in the input-cell editor and use `=ISTEXT(B2)`.

## Quirks

An error value is a value to classify: ISTEXT returns FALSE for it rather than propagating it.
That differs from a numeric function which must read the argument as a number before calculating.

An empty string can occupy a nonempty cell and still satisfy ISTEXT. Checking whether text has
length zero is a separate operation. Likewise, a string that looks like a date remains text
until a consumer parses it or Excel's cell-entry machinery converts it.

## How to implement

Read the argument as a value and inspect its type tag. Return a logical value indicating whether
that tag denotes text. Preserve the distinction between text, numeric, logical, error and blank
values. Do not call a general text-to-number converter or format a value before classifying it.

When a reference is supplied, classify the referenced value under the formula's reference and
array rules. An array-valued call needs a logical result at each evaluated position. Keep a
missing input in this preview separate from an explicitly supplied blank: a missing input cannot
be classified until the host supplies it.

## Version differences

The recorded build below returns TRUE for `ISTEXT("")`. Microsoft documents the classifier
family for Microsoft 365 and Excel 2016 through 2024. No difference between particular builds
is established by that one observation.

## Alternatives

[ISNUMBER](FUNC.ISNUMBER.md) and [ISLOGICAL](FUNC.ISLOGICAL.md) inspect other value kinds.
[ISNONTEXT](FUNC.ISNONTEXT.md) is the complementary text test. [ISBLANK](FUNC.ISBLANK.md) asks
about an empty cell; [LEN](FUNC.LEN.md) measures text length. Neither is a replacement for a
strict test of whether the value is text.

## Sources

- [Microsoft: IS functions](https://support.microsoft.com/en-us/excel/functions/is-functions).
