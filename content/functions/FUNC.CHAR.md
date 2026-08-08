---
schema: efh.function-page/v1
function_id: FUNC.CHAR
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
family: text_scalar_misc
role_in_family: "The code-point-to-character direction of the text/number pair; CODE is its inverse."
---

## What it computes

`CHAR` maps a small integer to the single character that the integer designates *in some
character set*, and returns that character as a one-character text value. The whole content of
the function is the question "which character set?", and the answer is not a mathematical
constant: Microsoft's article states that the character returned comes from the character set
used by your computer, and names two — the ANSI character set on Windows, the Macintosh
character set on Macintosh.

So the function is best read as a two-step composition, not a one-step map:

1. the argument is truncated to an integer and checked against the admissible range 1–255;
2. that integer is interpreted as an index into a host-dependent single-byte character set, and
   the resulting character is delivered as `Text` — which, in the Handbook's value model, is a
   sequence of UTF-16 code units (see [the value universe](../model/01-value-universe.md)).

Step 2 is where implementations can legitimately differ, because the two steps only agree
trivially on the ASCII subrange 32–126, where every candidate encoding coincides. Above 127 the
identity "code number 128 means the character U+0080" and the identity "code number 128 means
the Windows-1252 character €" are different functions, and the argument range 1–255 spans both.

## Arguments

`number` — required, the code number. Microsoft's article states it as a number between 1 and
255 specifying which character you want. The argument is numeric, so the ordinary to-number
coercion of [chapter 02](../model/02-coercion-and-lifting.md) applies: a logical, or text that
reads as a number, converts before the range check.

There is no second argument and no encoding selector. A formula that needs a specific encoding
cannot express it through `CHAR`; that is the reason `UNICHAR` exists.

## Result and edge cases

The return kind is `Text`, always exactly one character long when the call succeeds.

Two boundaries are specific enough to state:

- **Non-integer arguments.** The admissible range is documented as a range of numbers, not of
  integers, so some truncation or rounding rule must exist. Which one Excel applies —
  truncation toward zero, floor, or round-half-away — is not stated in Microsoft's article and
  has not been checked here.
- **Excel for the web.** Microsoft's article carries an explicit platform restriction: on Excel
  for the web only `CHAR(9)`, `CHAR(10)`, `CHAR(13)` and `CHAR(32)` and above are supported.
  This is a documented per-surface difference, not a quirk of one build, and any implementation
  claiming to match "Excel" must say which Excel.

Array arguments, empty cells and omitted arguments follow the shared call model; nothing about
`CHAR` overrides it.

## Errors

Microsoft's article states the admissible argument range but does not, in its own text, name
the error value produced outside that range. The Handbook therefore records the range as
documented and the error value as unverified: the reference engine's outcome for out-of-range
and non-numeric arguments is what the battery panel on this page renders, and it is the
reference engine's outcome, not a statement about Excel.

Error inputs propagate rather than convert, per the universal rule in
[chapter 02](../model/02-coercion-and-lifting.md).

## Relationships

- **`CODE`** is the inverse direction — character to number — and shares the same
  host-character-set dependency and therefore the same open question.
- **`UNICHAR`** and **`UNICODE`** are the Unicode-explicit pair. They are the functions to
  reach for when the code point matters and the host must not: `UNICHAR` takes a Unicode code
  point rather than a byte-sized index. `CHAR` is not deprecated in favour of them; both pairs
  are live.
- Readers routinely reach for `CHAR(10)` to embed a line break in a formula-built string, and
  for `CHAR(34)` to embed a double quote. Neither use is a special case in the function; they
  are just the two control characters that formula text cannot easily carry literally.

## Notes for implementers

The interesting decision is the 128–159 window. In Windows-1252 (the encoding usually meant by
"ANSI" on a Western Windows install) that window holds printable characters — the euro sign,
curly quotes, the en and em dashes — mapped to Unicode code points scattered far outside
128–159. In a "code unit equals code number" implementation the same window holds the C1
control characters. A program that treats `CHAR` as `chr()` over UTF-16 code units and one that
treats it as a Windows-1252 decode agree everywhere except here, and disagree visibly on the
most commonly typed non-ASCII characters in English text.

The OxFunc reference engine at the commit this page was curated against takes the code-unit
reading: it truncates toward zero, requires 1–255, and emits the single UTF-16 code unit equal
to the argument. That is an implementation fact about OxFunc, not a claim about Excel.

A second decision follows from the first: "the ANSI character set" is not one encoding. The
system ANSI code page differs by Windows locale (1252, 1251, 1250, 932, …), so a faithful
host-dependent implementation is locale-sensitive in a way the function's axis chips do not
currently record.

## What has not been checked

No Handbook vector suite exists for `CHAR`, and no Excel-comparison evidence record names it.
In particular nobody has checked, for this Handbook:

- what Excel returns for `CHAR(128)` through `CHAR(159)` on a Windows host, which is the single
  probe that decides between the two readings above;
- whether that answer changes with the Windows system ANSI code page, which would settle
  whether `CHAR` deserves a locale-dependency axis;
- what Excel returns for a non-integer argument such as `CHAR(65.9)`, which fixes the rounding
  rule;
- which error value Excel produces for `CHAR(0)`, `CHAR(256)` and `CHAR("x")`;
- whether the Excel-for-the-web restriction in the documentation is a hard error or a
  substitution.

A suite of 255 rows plus a dozen boundary rows, captured on two Windows locales and once on
Excel for the web, would settle every question on this page.

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| ANSI character set | Microsoft's name for the Windows system single-byte code page; not one fixed encoding |
| code unit | One UTF-16 unit of a `Text` value, as defined in the value-universe chapter |
| host character set | The character set the running Excel host resolves `CHAR` against |

## Sources

- Microsoft, "CHAR function" —
  <https://support.microsoft.com/en-us/office/char-function-bbd249c8-b36e-4a91-8017-1c133f9b837a>
  (argument range 1–255; the ANSI/Macintosh character-set statement; the Excel-for-the-web
  restriction). Retrieved for this page.
- Handbook `content/model/01-value-universe.md` (what `Text` is) and
  `content/model/02-coercion-and-lifting.md` (to-number coercion, error propagation).
- OxFunc `crates/oxfunc_core/src/functions/text_scalar_misc.rs` at commit `473efa3` — read for
  the reference engine's own reading of the mapping. Implementation fact only.
