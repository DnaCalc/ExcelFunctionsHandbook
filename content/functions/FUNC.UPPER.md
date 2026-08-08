---
schema: efh.function-page/v1
function_id: FUNC.UPPER
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — UPPER function"
    locator: "https://support.microsoft.com/en-us/office/upper-function-c11f29b3-d1a3-4537-8df6-04d0049963d6"
    role: "documented signature and description"
  - work: "OxFunc — FUNCTION_SLICE_TEXT_CORE_AND_COMPATIBILITY_FAMILY_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_TEXT_CORE_AND_COMPATIBILITY_FAMILY_CONTRACT_PRELIM.md"
    role: "upstream admitted-slice rules: logical textification and single-array lift"
  - work: "OxFunc — BUG-FUNC-008 text scalar and delimiter array-support gap"
    locator: "docs/bugs/streams/BUG-FUNC-008_text_scalar_and_delimiter_array_support_gap.md"
    role: "upstream live-Excel observation that UPPER spills over an array-valued argument"
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
family: text_scalar_misc
role_in_family: "The upward member of the case-mapping pair in the scalar text group."
---

## What it computes

`UPPER` returns the argument text with every character replaced by its uppercase form.

That sentence is short, and every hard question about the function hides inside "its uppercase
form". Case mapping is not a property of a character in isolation; it is a function of a character,
a mapping table, and — in the general case — a locale and the surrounding context. The four
mappings a text function might implement are genuinely different:

1. **ASCII only.** `a`–`z` map to `A`–`Z`; everything else is left alone. Simple, locale-free, and
   wrong for most of the world.
2. **Simple Unicode uppercase.** One code point maps to one code point, using Unicode's simple
   uppercase mapping. `é` becomes `É`.
3. **Full Unicode uppercase.** One code point may map to several. The canonical example is `ß`,
   whose full uppercase mapping is `SS` — a mapping that *changes the length of the string*.
4. **Locale-tailored uppercase.** Turkish and Azerbaijani map dotless `ı` to `I` and dotted `i` to
   `İ`, which differs from every other locale's mapping of `i`.

These four disagree on real, ordinary text, and the difference is observable in a spreadsheet.
`UPPER` is therefore one of the functions where "it just uppercases" is not a specification.
**Which of the four Excel implements, this Handbook has not established.** That is the honest state
and it is why the "What has not been checked" section below is unusually specific.

What can be said with a basis: upstream OxFunc's text-core contract records the case-mapping family
as characterised only against an ASCII-seeded baseline, with Unicode case-fold parity explicitly
outside the admitted slice. And separately, its `SEARCH` rule and the `match_mode` rules of the
newer text functions are recorded as ASCII-only case-insensitivity on that baseline — a consistent
picture of "the ASCII core is pinned and the rest is not", which is the same picture this page
paints.

## Arguments

`UPPER(text)`

| Argument | Required | Meaning |
|---|---|---|
| `text` | yes | The text to convert. |

One argument, no options — in particular no locale argument, which is the reason locale-tailored
mapping is such a live question: if Excel does tailor, it must be doing so from ambient state, and
ambient state is exactly what makes a formula's result depend on the machine it runs on.

Non-text arguments convert to text first. Upstream records the specific rule that `LOWER` and
`UPPER` textify logicals on the ordinary values-only seam, so `UPPER(TRUE)` is the string `TRUE`
rather than an error or a number. Numbers arrive as their general-format rendering; since digits
and the exponent marker have no distinct cases, `UPPER` of a number is generally that rendering
unchanged, though the marker's case in scientific notation is worth checking rather than assuming.

## Result and edge cases

The return kind is `Text`. `UPPER("")` is `""`.

`UPPER` is a scalar kernel and lifts elementwise over array arguments — the rules are in
[coercion and lifting](../model/02-coercion-and-lifting.md). Specific to this function: an upstream
OxFunc defect record (`BUG-FUNC-008`) reports a live-Excel replay in which `UPPER` spilled over a
single array-valued argument, so `UPPER({"a","b"})` is a two-element array. Upstream observation,
one Excel build, not a Handbook claim.

Boundaries worth naming:

- **Length can change.** If Excel implements full Unicode uppercase, `UPPER` can return a longer
  string than it was given — the `ß` → `SS` case. A caller that assumes `LEN(UPPER(x)) = LEN(x)`
  is making an assumption about which of the four mappings Excel uses.
- **The 32,767-code-unit cap** therefore becomes reachable from below; see
  [the value universe](../model/01-value-universe.md) for how the cap behaves on the formula path.
- **`UPPER` is not a normaliser.** It does not compose or decompose combining marks. `UPPER` of a
  decomposed `e` + combining acute is not guaranteed to equal `UPPER` of the precomposed `é`, and
  `EXACT` will tell you they differ.
- **`UPPER` is not injective**, so it is not safely reversible: `LOWER(UPPER(x))` is not `x` in
  general.
- **An empty referenced cell** delivers `Empty`, distinct from an omitted argument.
- **Errors propagate.**

## Errors

Microsoft's page is the authority and is linked below; it was not retrievable while this entry was
written, so nothing here paraphrases it. `UPPER` is a total function over text and there is no
documented domain restriction to state. The reachable errors are the ordinary ones:

| Error | Condition |
|---|---|
| any error value | Propagated from the argument; coercion does not discard worksheet errors. |
| `#VALUE!` | An argument kind with no text conversion in this context, or a result that exceeds the text cap on the formula path. |

The second row's cap behaviour is described in [the value universe](../model/01-value-universe.md)
as an observed distinction between the formula and interop paths; whether `UPPER` in particular
reaches it has not been probed.

## Relationships

- `LOWER` — the mirror function, same implementation module upstream, and subject to every question
  on this page in the opposite direction. Note that lowercase has its own asymmetries: Greek final
  sigma is context-dependent in a way no uppercase mapping is.
- `PROPER` — the third member of the case family. Upstream records its word-boundary behaviour as
  restarting capitalisation at non-letter separators including apostrophes and digits, which is why
  `PROPER("o'brien")` is not what a reader expects. `PROPER` is not `UPPER` of the first letter.
- `EXACT` — the case-sensitive comparison. `EXACT(UPPER(a), UPPER(b))` is the usual hand-rolled
  case-insensitive comparison, and it inherits every mapping question above.
- `=` (the comparison operator) is already case-insensitive for text, which makes `UPPER` unnecessary
  in many comparisons where people reach for it out of habit from other languages.
- `SEARCH` versus `FIND`, and the `match_mode` arguments of [`TEXTAFTER`](FUNC.TEXTAFTER.md),
  [`TEXTBEFORE`](FUNC.TEXTBEFORE.md) and [`TEXTSPLIT`](FUNC.TEXTSPLIT.md), express
  case-insensitivity directly and are usually better than uppercasing both sides.
- [`ASC`](FUNC.ASC.md) and [`DBCS`](FUNC.DBCS.md) are the *width* conversions, sometimes mistaken
  for case conversions because they also "change letters". They do not touch case.

## Notes for implementers

- **Name your mapping.** Write down which of the four mappings you implement and treat it as part
  of the signature. A port that silently uses the host language's default (`to_uppercase` in Rust
  is full Unicode; `ToUpper` in .NET is culture-sensitive; `str.upper` in Python is full Unicode)
  will inherit that language's choice as Excel semantics.
- **Culture-sensitive APIs are a trap in both directions.** `ToUpper()` on a Turkish thread gives a
  different answer than on an invariant one. If you want reproducibility, use the invariant form
  explicitly — and record that you did, because it may be a divergence from Excel rather than a fix.
- **Allow the length to change** in your buffers and in your result-cap check. In-place uppercase
  over a fixed-size UTF-16 buffer is only correct for simple mapping.
- **Operate on scalars, not code units.** Case mapping of a surrogate pair requires decoding it
  first; a code-unit loop leaves supplementary-plane letters (Deseret, Adlam, Warang Citi) unmapped.
- **Lift a single array-valued argument.**

## What has not been checked

There is no Handbook vector suite for `UPPER`, and no Handbook evidence record comparing any
implementation against Excel. Nothing on this page is a measurement. In particular, **which case
mapping Excel implements is not established here** — the four candidates above are all live, and
the upstream contract this page draws on characterises only an ASCII-seeded baseline and explicitly
places Unicode case-fold parity outside its admitted slice.

A short probe sheet settles almost all of it, and it is worth running before writing any port:

1. **`UPPER("ß")`** — `"ß"` means ASCII-only or simple mapping; `"SS"` means full mapping. One cell
   distinguishes the two largest families. Follow with `LEN` to confirm the length change.
2. **`UPPER("é")` and `UPPER("ﬁ")`** (the fi ligature) — separates ASCII-only from simple, and
   simple from full.
3. **`UPPER("i")` on a Turkish system locale, and again on an English one** — the only probe that
   can reveal locale tailoring, and the only one that requires two machines or a locale switch. If
   the answers differ, `UPPER` is machine-dependent and every workbook that uses it for keys is at
   risk.
4. **`UPPER` of a supplementary-plane cased letter**, e.g. Deseret small `𐐨` (`UNICHAR(66600)`),
   which tells you whether the implementation decodes pairs.
5. **`UPPER` of a decomposed sequence**, `UPPER("e" & UNICHAR(769))`, checked with `EXACT` against
   `UPPER(UNICHAR(233))`, which tells you whether any normalisation is happening.
6. **`UPPER` of Greek final sigma and of a Greek word with accents**, which is where full mapping
   and simple mapping most visibly part company in a real language.
7. **The scientific-notation case.** `UPPER(1E+30)` — whether the exponent marker's case in the
   general-format rendering is affected at all.

Every one of those is a single cell. Until they are recorded, treat the definition at the top of
this page as "returns the uppercase form under an unidentified mapping".

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| simple mapping | Unicode's one-code-point-to-one-code-point uppercase mapping |
| full mapping | Unicode's uppercase mapping that may lengthen the string (`ß` → `SS`) |
| locale tailoring | Locale-specific overrides of the mapping, notably Turkish dotted/dotless `i` |
| normalisation | Composition or decomposition of combining sequences; `UPPER` is not one |
| lift | Elementwise application of this scalar kernel over an array argument |

## Sources

- Microsoft Support, UPPER function —
  <https://support.microsoft.com/en-us/office/upper-function-c11f29b3-d1a3-4537-8df6-04d0049963d6>
  (the authority for signature and description; not retrievable while this entry was written, so it
  is cited by reference and nothing here paraphrases it).
- OxFunc `docs/function-lane/FUNCTION_SLICE_TEXT_CORE_AND_COMPATIBILITY_FAMILY_CONTRACT_PRELIM.md`
  — the ASCII-seeded baseline for the case family, the rule that `LOWER`/`UPPER` textify logicals,
  the `PROPER` word-boundary rule, and the recorded single-array lift. Provisional by its own
  statement.
- OxFunc `docs/bugs/streams/BUG-FUNC-008_text_scalar_and_delimiter_array_support_gap.md` — the
  live-Excel replay that pinned the array lift.
- Handbook `content/model/01-value-universe.md` (UTF-16 code units, the 32,767 cap and its two
  enforcement paths), `02-coercion-and-lifting.md` (to-text coercion, lifting, error propagation).
