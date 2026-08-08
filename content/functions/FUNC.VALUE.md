---
schema: efh.function-page/v1
function_id: FUNC.VALUE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — VALUE function"
    locator: "https://support.microsoft.com/en-us/office/value-function-257d0108-07dc-437d-ae1c-bc2d3953d8c2"
    role: "documented signature, description and error conditions"
  - work: "OxFunc — FUNCTION_SLICE_VALUE_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_VALUE_CONTRACT_PRELIM.md"
    role: "upstream locale-parse seam contract and the empirically pinned accept/reject lanes"
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
family: value_fn
role_in_family: "The text-to-number parser; the locale-facing member of the number/format group."
---

## What it computes

`VALUE` parses text into a number.

It is tempting to describe this as "the inverse of `TEXT`", and that is the right intuition, but
the honest description is narrower and stranger: **`VALUE` accepts the text forms that Excel itself
would recognise as a number if you typed them into a cell, under the current locale, and returns
the number they denote.** That is not a fixed grammar. It is a grammar parameterised by the
machine's regional settings.

The forms in play include, at minimum: plain decimals, scientific notation, digit-group separators,
a currency symbol, a percent sign, and date and time text. Each of those brings a locale
dependency:

- the **decimal separator** is `.` in some locales and `,` in others, which means `"1,5"` is one
  and a half in one place and fifteen — or a rejection — in another;
- the **group separator** is the mirror image of that problem, and a space in several locales;
- the **currency symbol** is per-locale, and `VALUE` accepting `"$1.00"` says nothing about whether
  it accepts `"€1,00"`;
- **date text** is the worst case, because `"1/2/2024"` denotes two different days in two locales
  and returns a *serial number*, not the digits you can see.

The Handbook's shared coercion chapter already draws this boundary: it pins the plain-number core
and states that the locale-dependent surface — separators, currency, percent, date and time text —
is an open area. See [coercion and lifting](../model/02-coercion-and-lifting.md). `VALUE` is the
function where that open area is the entire subject matter.

Upstream OxFunc treats `VALUE` the same way, and usefully so: its slice contract routes text
through an explicit locale-profile parser seam rather than a fixed grammar, and records which lanes
were empirically pinned on one host profile — plain decimal text, space-grouped numeric text,
currency text with the local symbol, percent text, and ISO `yyyy-mm-dd` date text were accepted;
slash-style date text and blank or whitespace-only text were rejected — under Excel 16.0 build
19725 with a named channel and host profile. That is an upstream observation on one host, recorded
with its scope, and it is exactly the kind of result that does not generalise: change the locale
and the accept/reject split is expected to move.

Two structural facts complete the definition. A number arriving as the argument passes through
unchanged. And a successful date-text parse returns the date's **serial number** — a plain
`Number`, formatting-free — which is why `VALUE("2024-01-02")` shows as a large integer unless the
cell is formatted as a date.

## Arguments

`VALUE(text)`

| Argument | Required | Meaning |
|---|---|---|
| `text` | yes | The text to convert to a number. |

One argument. The subtleties are all about what may occupy it:

- **Already-numeric input** passes through. `VALUE(5)` is `5`.
- **Logicals** are the surprising rejection: upstream's contract records logical, blank, missing,
  and unsupported aggregate or reference payloads as `#VALUE!` in its admitted slice — so `VALUE`
  is *stricter* than the general to-number coercion, which maps `TRUE` to `1`. That is a real
  difference between `VALUE(TRUE)` and `SUM(TRUE)`, and it is upstream's recorded behaviour rather
  than a Handbook finding.
- **Leading and trailing whitespace** is ignored by the general to-number rule; whether `VALUE`
  matches that exactly, and whether a whitespace-only string is rejected as upstream records, is
  worth confirming rather than assuming.
- **An empty referenced cell** delivers `Empty`, which is a distinct outcome from an omitted
  argument; see [the value universe](../model/01-value-universe.md).

## Result and edge cases

The return kind is `Number`.

Edges that matter in practice:

- **Percent text divides.** If `"50%"` is accepted, the result is `0.5`, not `50`. A pipeline that
  parses percentages with `VALUE` and then divides by 100 is wrong by a factor of 100 exactly when
  the parse succeeds.
- **Date and time text returns serials.** Time text returns a fraction of a day. `VALUE("12:00")`
  is `0.5` if accepted.
- **Overflow is not infinity.** The shared coercion rule states that text parsing must produce a
  finite number; text denoting an overflow is treated as non-numeric rather than as infinity.
- **Precision is the parser's, not the text's.** `VALUE` of a 30-digit numeral returns the nearest
  double, silently. Round-tripping through `TEXT` and back is lossy in both directions and the
  losses do not cancel.
- **Arrays.** Whether `VALUE` lifts over an array argument is not something this page asserts; the
  general rules are in [coercion and lifting](../model/02-coercion-and-lifting.md), and the
  specific answer for `VALUE` is listed as an open probe below.
- **Errors propagate.**

## Errors

Microsoft's page is the authority on the documented error conditions and is linked below; it was
not retrievable while this entry was written, so this section states expectations rather than
quoting it.

| Error | Status |
|---|---|
| `#VALUE!` when the text is not in a recognised numeric or date/time format | expected; this is the function's characteristic failure |
| `#VALUE!` for logical, blank, and unsupported argument kinds | recorded upstream for its admitted slice; not a Handbook claim, and notably stricter than general to-number coercion |
| any error value, propagated from the argument | follows from the universal error-propagation rule |

Note what *is not* here: there is no error for precision loss, and none for a value that parses but
means something other than what the author intended. `VALUE` fails loudly or succeeds silently.

## Relationships

- `TEXT` — the opposite direction, and the function `VALUE` is usually paired with. `TEXT` is
  equally locale-dependent, in the format-string argument as well as the output, which means a
  `VALUE(TEXT(x, fmt))` round trip is doubly exposed to locale.
- `NUMBERVALUE` — the function you probably want. It takes explicit decimal and group separator
  arguments, which removes the ambient-locale dependency that makes `VALUE` non-portable.
  `NUMBERVALUE` does not supersede `VALUE` in Microsoft's catalogue and both remain current, but
  for any workbook that travels between machines, `NUMBERVALUE` is the safer instrument. Upstream
  notes that `NUMBERVALUE`'s own locale-default lanes are separately blocked and unresolved, so it
  is not a free lunch either.
- `--` (double unary minus) and `+0` are the idiomatic shorthands for text-to-number coercion. They
  go through the *general* to-number rule described in
  [coercion and lifting](../model/02-coercion-and-lifting.md), not through `VALUE`'s parser, so
  they are not interchangeable with `VALUE` — most visibly for logicals.
- `DATEVALUE` and `TIMEVALUE` — the specialised parsers for the date and time lanes that `VALUE`
  also happens to cover. When you know you have a date, they say so in the formula and are easier
  to reason about.
- `VALUETOTEXT` is *not* the inverse of `VALUE` despite the name symmetry; see
  [`VALUETOTEXT`](FUNC.VALUETOTEXT.md), which renders any value as text for display or for
  formula-bar round-tripping.
- `ISNUMBER(VALUE(x))` is the common "does this parse" test, and it inherits every locale question
  on this page.

## Notes for implementers

- **Make the locale an explicit input, not ambient state.** Upstream's design — a locale-profile
  seam queried from the host, with the parser written against the profile — is the right shape.
  A parser that reads the process locale at call time is untestable and irreproducible.
- **Do not use the host language's number parser.** `strtod`, `double.Parse`, and `f64::from_str`
  each accept things Excel does not (hex floats, `inf`, `nan`, underscores in some languages) and
  reject things Excel does (currency symbols, percent, grouped digits). Every one of those
  mismatches is a silent divergence.
- **Reject non-finite parses.** The shared rule is explicit: text that parses to an overflow is
  non-numeric, not infinite.
- **Keep the date lane separate from the number lane**, and be prepared for the two to disagree
  about the same string — `"1.5"` as a number and as a date are both plausible readings, and the
  precedence between them is a real decision that must be recorded.
- **Percent must divide at the end**, after the numeric parse, and must compose with grouping and
  currency rather than being a separate grammar.
- **Round once.** Parse to the exact decimal value and round to the nearest double a single time;
  parsing digit-by-digit with accumulating multiplication introduces error that no test of small
  numerals will catch.

## What has not been checked

There is no Handbook vector suite for `VALUE`, and no Handbook evidence record comparing any
implementation against Excel. Nothing on this page is a measurement. The accept/reject lanes quoted
from upstream are scoped to one Excel build, one channel, and one host profile, and the whole point
of this function is that they are expected to move when any of those change.

`VALUE` is also one of the functions the Handbook's generated battery cannot exercise: the upstream
reference implementation reports it as requiring a host facility, so no dispatched outcome exists
for the standard probe rows. That is a fact about the reference engine's seam, not about Excel.

The probes that would settle the most, in order:

1. **The locale matrix.** Run the same short list of strings — `"1.5"`, `"1,5"`, `"1 234,5"`,
   `"1,234.5"`, `"$1.00"`, `"€1,00"`, `"50%"`, `"1E3"`, `"2024-01-02"`, `"1/2/2024"`, `"12:00"`,
   `" 1 "`, `""` — on at least an `en-US`, a `de-DE`, and a `fr-FR` machine. That single sheet is
   the entire open question, and it is the highest-value probe in this part of the catalogue.
2. **Logicals and blanks.** `VALUE(TRUE)`, `VALUE(A1)` with `A1` empty, and `VALUE(" ")`, to
   confirm or refute upstream's recorded strictness against Excel directly.
3. **Array lifting.** `VALUE({"1","2"})` — one cell, and it determines whether `VALUE` participates
   in dynamic-array pipelines at all.
4. **The date/number precedence.** Strings that are legal under both readings, such as `"1.5"` in a
   locale where `.` is also a date separator.
5. **Precision.** `VALUE` of a 17-significant-digit numeral and of a value near the subnormal
   boundary, compared against a correctly rounded reference. This is where a vector suite would
   have real teeth, because it is checkable without Excel once the vectors exist.
6. **Whitespace and sign forms.** Leading `+`, parenthesised negatives (`"(1.5)"`, an accounting
   convention some locales accept), and non-breaking spaces as group separators.

Until the locale matrix exists, no statement about which strings `VALUE` accepts should be read as
general.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| locale profile | The named set of separators, symbols and date orders the parser is written against |
| lane | One recognised text form (plain decimal, percent, currency, ISO date, …) |
| serial number | Excel's numeric representation of a date or time; what a date-text parse returns |
| ambient locale | Regional settings read from the machine rather than passed as an argument |
| host facility | An engine seam that must be supplied by the host; why the generated battery cannot dispatch this function |

## Sources

- Microsoft Support, VALUE function —
  <https://support.microsoft.com/en-us/office/value-function-257d0108-07dc-437d-ae1c-bc2d3953d8c2>
  (the authority for signature, description and documented errors; not retrievable while this entry
  was written, so it is cited by reference and nothing here paraphrases it).
- OxFunc `docs/function-lane/FUNCTION_SLICE_VALUE_CONTRACT_PRELIM.md` — the locale-profile parser
  seam, the pinned accept and reject lanes with their Excel build, channel and host-profile scope,
  and the recorded `#VALUE!` for logical, blank, missing and unsupported argument kinds.
  Preliminary by its own statement.
- Handbook `content/model/02-coercion-and-lifting.md` — the general to-number rule, the
  finite-parse requirement, and the explicit statement that the locale-dependent recognition
  surface is an open area.
- Handbook `content/model/01-value-universe.md` — Empty versus Missing, and the `Number` kind.
