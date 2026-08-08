---
schema: efh.function-page/v1
function_id: FUNC.UNICHAR
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — UNICHAR function"
    locator: "https://support.microsoft.com/en-us/office/unichar-function-ffeb64f5-f131-44c6-b332-5cd72f0659b8"
    role: "documented signature, description and error conditions"
  - work: "OxFunc — STRING_BEHAVIOR_RESEARCH_NOTES.md"
    locator: "docs/function-lane/STRING_BEHAVIOR_RESEARCH_NOTES.md"
    role: "upstream local-baseline observations of the UNICHAR/UNICODE round trip and surrogate tails"
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
family: text_unicode_fn
role_in_family: "The code-point-to-text direction of the Unicode pair."
---

## What it computes

`UNICHAR` maps a number to the text consisting of the single Unicode character with that code
point.

Precisely: given a number `n`, truncate it to an integer, interpret that integer as a Unicode code
point, and return a one-character string containing it. "One character" here means one Unicode
scalar value — which, in the UTF-16 code-unit model this Handbook uses for worksheet text (see
[the value universe](../model/01-value-universe.md)), is *two* code units when the code point is
outside the Basic Multilingual Plane. So `UNICHAR(128512)` is one character but occupies two code
units, and functions that count code units will say so.

The interesting part of `UNICHAR` is not the mapping, which is trivial, but the domain. Three
regions of the integer line behave differently and the distinction is the whole function:

1. **Valid scalar values** — `1` through `0xD7FF` and `0xE000` through `0x10FFFF`. These are the
   code points that can be encoded in UTF-16 and stored in a worksheet string.
2. **Surrogate code points** — `0xD800` through `0xDFFF`. These are not characters. They exist
   only as the two halves of a UTF-16 surrogate pair, and asking for one on its own is a request
   for half a character. Whether Excel refuses this or produces an unpaired surrogate is exactly
   the kind of question this Handbook does not answer from memory; see below.
3. **Out of range** — `0`, negatives, and anything above `0x10FFFF`. Microsoft's page documents
   these as errors.

This is why `UNICHAR` is the right tool for constructing test data. It is the only ordinary way to
write an arbitrary code point into a worksheet from a formula, which makes it the entry point for
probing every other text function's Unicode behaviour.

## Arguments

`UNICHAR(number)`

| Argument | Required | Meaning |
|---|---|---|
| `number` | yes | The Unicode code point, as a number. |

There is only one argument, and only one subtlety in it: it is a *number*, so text that reads as a
number converts (see [coercion and lifting](../model/02-coercion-and-lifting.md)) and `TRUE`
converts to `1`. Fractional values are truncated toward zero rather than rejected — the Handbook
states that as the expected behaviour for this family and lists it below as unverified.

The argument is a code point, not a byte and not a UTF-16 code unit. That distinction is invisible
below `0x10000` and decisive above it.

## Result and edge cases

The return kind is `Text`: a string of one Unicode scalar, which is one or two UTF-16 code units.

`UNICHAR` is a scalar kernel and lifts elementwise over array arguments; the rules, including
element-local errors, are in [coercion and lifting](../model/02-coercion-and-lifting.md). This is
what makes `UNICHAR(SEQUENCE(n) + k)` the standard idiom for generating a character sweep, and it
is worth knowing that a single bad element in that sweep yields an error in that cell only, not a
failed call.

Specific edges:

- **`UNICHAR(10)` and `UNICHAR(13)`** produce line-break characters, which is the usual reason to
  reach for this function outside Unicode work — though `CHAR` is the more common idiom there.
- **`UNICHAR(160)`** produces the non-breaking space, the character [`TRIM`](FUNC.TRIM.md) does
  not remove. This pairing is the standard repair for web-pasted text.
- **The result of a supplementary-plane call is one character of length two** under code-unit
  counting. An upstream OxFunc note records a local-baseline observation that `LEN` counts Unicode
  scalars on a current Excel compatibility version while `LENB` retains code-unit counting, so the
  two disagree for exactly these characters. That fork is upstream's, recorded under its own
  compatibility-version scope, and this Handbook does not extend it.
- **Errors propagate** from the argument ahead of any domain check.
- **An empty referenced cell** delivers `Empty`, which is distinct from an omitted argument; see
  [the value universe](../model/01-value-universe.md).

## Errors

Microsoft's page is the authority on the documented error conditions and is linked below; the
Handbook could not retrieve it while this entry was written, so this section states expectations to
be checked against it rather than quoting it.

| Error | Status |
|---|---|
| `#VALUE!` for `number = 0`, negative, or above the largest valid code point | expected; confirm against Microsoft's page and by probe |
| `#VALUE!` for text that does not read as a number | expected from the shared to-number coercion rule, not from a function-specific statement |
| any error value, propagated from the argument | follows from the universal error-propagation rule in the coercion chapter |

The status of the surrogate range `0xD800`–`0xDFFF` is not something this page asserts at all. It
is listed as an open probe below.

## Relationships

- [`UNICODE`](FUNC.UNICODE.md) — the inverse direction, and the sibling in the same implementation
  module upstream. `UNICODE(UNICHAR(n)) = n` is the round trip; an OxFunc research note records
  observing it hold for a supplementary-plane code point on its local baseline.
- `CHAR` — the older, narrower counterpart. `CHAR` takes a number in a single-byte range and is
  interpreted through the system character set; `UNICHAR` takes a Unicode code point directly.
  `CHAR` is not deprecated and is not superseded by `UNICHAR`; the two answer different questions,
  and they agree only over the ASCII range.
- `CODE` — `CHAR`'s inverse, and the older counterpart to `UNICODE`.
- `TEXT`, `VALUE` — unrelated despite the naming; those convert between numbers and *formatted*
  representations, not between numbers and code points.
- Readers confuse `UNICHAR(65)` with `"65"`. It is `"A"`.

## Notes for implementers

- **Truncate before validating, and say which you do first.** `UNICHAR(65.9)` and `UNICHAR(-0.5)`
  land differently depending on the order, and the second is the interesting one: truncation toward
  zero turns `-0.5` into `0`, which is documented as an error anyway, but truncation toward
  negative infinity would give `-1`. Order matters for the error *reason*, and may matter for the
  error at all.
- **Decide the surrogate policy explicitly.** Rust's `char`, .NET's `Rune`, and Python's `chr`
  disagree about lone surrogates: two of them refuse, one produces them. Whichever your host
  language does by default will silently become your `UNICHAR` behaviour if you do not intervene.
- **Do not route through UTF-8 and back.** A round trip through a UTF-8 string type will reject or
  mangle exactly the inputs whose behaviour is most in question. Work in UTF-16 code units, which
  is the model the worksheet uses.
- **The result must be able to hold an unpaired surrogate** if you decide to admit one, which means
  your text type cannot be a validated-UTF-8 or validated-UTF-16 string. The value-universe chapter
  records that ill-formed UTF-16 is a reachable state in Excel by another route (interop
  truncation splitting a pair), so this is not a hypothetical requirement.
- **Lift over arrays.** The sweep idiom depends on it.

## What has not been checked

There is no Handbook vector suite for `UNICHAR`, and no Handbook evidence record comparing any
implementation against Excel. Nothing on this page is a measurement. The exact upper bound, the
truncation rule, and the surrogate policy are all stated above as expectations, not findings.

The probes, in the order that would pin the most:

1. **The surrogate range.** `UNICHAR(55296)` (`0xD800`) and `UNICHAR(57343)` (`0xDFFF`). Three
   outcomes are plausible — `#VALUE!`, a lone surrogate that `LEN` reports as `1`, or a
   replacement character — and they are distinguishable by following with `LEN` and `UNICODE`.
2. **The upper bound.** `UNICHAR(1114111)` (`0x10FFFF`) against `UNICHAR(1114112)`.
3. **The lower bound.** `UNICHAR(0)` is documented as an error; confirm, and check `UNICHAR(1)`.
4. **Fractional and near-integer inputs.** `UNICHAR(65.9)`, `UNICHAR(65.0000001)`, and
   `UNICHAR(2^53 + 1)` test truncation and the point at which the double loses integer precision.
5. **Unassigned and noncharacter code points**, such as `UNICHAR(65534)` (`0xFFFE`) and a
   private-use code point, to see whether Excel validates assignment or only encodability.
6. **The round trip at scale.** `UNICODE(UNICHAR(n))` over a sweep of `n` is a cheap consistency
   check that would also serve as the seed of a vector suite for both functions.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| code point | A Unicode scalar value; what `number` denotes |
| code unit | A UTF-16 16-bit unit; worksheet text is a sequence of these |
| supplementary plane | Code points above `0xFFFF`; one character, two code units |
| surrogate | A code point in `0xD800`–`0xDFFF`; half of a UTF-16 pair, not a character |
| round trip | `UNICODE(UNICHAR(n)) = n` |

## Sources

- Microsoft Support, UNICHAR function —
  <https://support.microsoft.com/en-us/office/unichar-function-ffeb64f5-f131-44c6-b332-5cd72f0659b8>
  (the documented signature, description, and error conditions; the page was not retrievable while
  this entry was written, so it is cited by reference and its exact wording is not reproduced).
- OxFunc `docs/function-lane/STRING_BEHAVIOR_RESEARCH_NOTES.md` — local-baseline observations
  including the `UNICODE(UNICHAR(128512))` round trip and the dangling-surrogate tail. Scoped to
  that baseline.
- OxFunc `docs/function-lane/FUNCTION_SLICE_TEXT_CORE_AND_COMPATIBILITY_FAMILY_CONTRACT_PRELIM.md`
  — the recorded `LEN`/`LENB` compatibility-version fork for supplementary-plane characters.
- Handbook `content/model/01-value-universe.md` (text as UTF-16 code units; ill-formed UTF-16 as a
  reachable state), `02-coercion-and-lifting.md` (to-number coercion and array lifting).
