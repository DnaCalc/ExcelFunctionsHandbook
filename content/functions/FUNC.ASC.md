---
schema: efh.function-page/v1
function_id: FUNC.ASC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — ASC function"
    locator: "https://support.microsoft.com/en-us/office/asc-function-0b6abf1c-c663-4004-a964-ebc00b723266"
    role: "documented signature and description of the full-width to half-width conversion"
  - work: "OxFunc — FUNCTION_SLICE_WIDTH_CONVERSION_HOST_PROFILE_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_WIDTH_CONVERSION_HOST_PROFILE_CONTRACT_PRELIM.md"
    role: "upstream host/profile seam contract and the observed current-host pass-through"
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
family: text_compat_locale_family
role_in_family: "The narrowing member of the width-conversion trio."
---

## What it computes

`ASC` converts full-width (double-byte) characters in a string to their half-width (single-byte)
equivalents.

The concepts come from East Asian text handling and predate Unicode. In the legacy double-byte
character sets used for Japanese, Korean and Chinese text, Latin letters, digits and katakana each
existed in two forms: a *half-width* form occupying one byte and one display column, and a
*full-width* form occupying two bytes and two display columns. Unicode preserved both, so the
distinction survives as separate code points — `A` (U+0041) versus `Ａ` (U+FF21), `ｱ` (U+FF71)
versus `ア` (U+30A2). `ASC` maps the wide forms down to the narrow ones. The mapping is not a
rounding or an approximation; it is a table.

Two details make the katakana half of the mapping harder than the Latin half:

1. **Voiced katakana are one character wide and two characters narrow.** Full-width `ガ` is a single
   code point; its half-width form is the pair `ｶ` + `ﾞ` (a base kana and a combining-style voiced
   sound mark). So `ASC` can *lengthen* a string, and a naive one-to-one mapping table is wrong.
2. **Not everything has a half-width form.** Hiragana, kanji, and most full-width punctuation
   outside the ASCII-mirror block have no narrow counterpart and pass through untouched.

Now the part that matters most for this entry, and which is easy to get wrong by reading only the
concept: **whether `ASC` converts anything at all depends on the host environment.** The function is
part of the East Asian compatibility surface, and its behaviour is tied to the language settings of
the Excel installation rather than being a fixed text transformation.

Upstream OxFunc states this as a design conclusion rather than an inconvenience. Its width-conversion
contract records a native host replay in which `ASC` applied to full-width input returned the input
**unchanged** on the host profile it was run against, and concludes that "the honest current seam is
not pure local text conversion": the host or profile decides whether width conversion is active,
and the transformation itself is only owned by the function once that mode is known. The upstream
model names four modes — pass-through, narrow, widen, and unavailable — with the host supplying
which one applies.

So the operational definition of `ASC` is two-stage: *ask the environment whether width conversion
is active for this function; if it is, apply the full-width-to-half-width table; if it is not,
return the text unchanged.* That is a genuinely unusual shape for a worksheet function, and it is
the reason this page is careful.

## Arguments

`ASC(text)`

| Argument | Required | Meaning |
|---|---|---|
| `text` | yes | The text containing characters to narrow. |

One argument, no options — no way to ask for only the Latin part, only the katakana part, or a
specific mapping table. Non-text arguments convert to text by the ordinary rules in
[coercion and lifting](../model/02-coercion-and-lifting.md).

## Result and edge cases

The return kind is `Text`.

- **Characters with no half-width form pass through**, so `ASC` applied to hiragana or kanji is the
  identity.
- **The result can be longer than the input** in UTF-16 code units, because of the voiced-katakana
  decomposition described above. Any assumption that `LEN(ASC(x)) <= LEN(x)` is false in general —
  it is false in exactly the case the function exists for.
- **On a host where width conversion is not active, `ASC` is the identity function** on all input.
  This is the case the upstream replay observed. A workbook that relies on `ASC` therefore computes
  different values on different machines, which makes it unsuitable for anything auditable without
  pinning the environment.
- **Errors propagate**, and an empty referenced cell delivers `Empty` rather than a missing
  argument; see [the value universe](../model/01-value-universe.md).
- **The generated battery on this page cannot dispatch `ASC`.** The upstream reference
  implementation reports it as requiring a host facility, so the standard probe rows have no
  outcome. That is a fact about the reference engine's seam, and it is the correct answer for a
  function whose semantics are host-supplied — not a gap to be filled with a guess.

## Errors

Microsoft's page is the authority and is linked below; it was not retrievable while this entry was
written, so this section states expectations rather than quoting it.

| Error | Status |
|---|---|
| any error value, propagated from the argument | follows from the universal error-propagation rule |
| `#NAME?` where the function is not admitted in the environment | recorded upstream as the behaviour for the sibling `JIS` on the observed host, and retained in the upstream model as a defensive projection for the unavailable mode. Whether `ASC` itself is ever unavailable in this way is not established. |

`#NAME?` for an *existing* function is jarring, and worth explaining: it is the worksheet's answer
when a name is not recognised in the current environment, which is what a compatibility function
looks like on a host that does not admit it.

## Relationships

- [`DBCS`](FUNC.DBCS.md) — the exact inverse direction: half-width to full-width. Same host
  dependency, same module upstream.
- `JIS` — the third member of the trio, and in Microsoft's catalogue the name used for the widening
  function in Japanese-language versions. Upstream's replay observed `JIS` returning `#NAME?` on the
  host it tested, which is a concrete illustration of the admission question this page describes.
- `LEN` versus `LENB` — the width distinction shows up again in the byte-counting functions, and
  the `*B` family (`LEFTB`, `MIDB`, `FINDB`, …) exists for the same East Asian reasons. Upstream
  records those as current-baseline delegates to their non-`B` counterparts rather than as
  independent byte-semantic implementations.
- [`UPPER`](FUNC.UPPER.md) and `LOWER` — case conversion, frequently confused with width conversion
  because both "change the letters". They are independent axes: `Ａ` and `ａ` differ in case, `A`
  and `Ａ` differ in width.
- Unicode's own NFKC normalisation performs a superset of `ASC`'s Latin mapping. It is not a
  substitute — NFKC also folds ligatures, superscripts and much else.

## Notes for implementers

- **Do not implement this as a local text function without a host query.** The upstream design —
  ask the host for the mode, then apply the transformation — is the shape that matches observed
  behaviour. A port that always narrows will disagree with Excel on every host where conversion is
  inactive, which upstream observed to include a real one.
- **The mapping table must handle the voiced katakana decomposition**, which is one-to-two. Build it
  as a table, not as an arithmetic offset, even though the Latin block is tantalisingly regular
  (`U+FF01`–`U+FF5E` maps to `U+0021`–`U+007E` by subtracting `0xFEE0`).
- **The full-width space `U+3000` maps to the ordinary space `U+0020`**, which interacts with
  [`TRIM`](FUNC.TRIM.md): narrowing first can change what a later `TRIM` removes.
- **Work in UTF-16 code units** and be prepared for the output to be longer.
- **Record the host profile in any test fixture.** A width-conversion test with no environment
  recorded is not reproducible, and is arguably not evidence at all.

## What has not been checked

There is no Handbook vector suite for `ASC`, and no Handbook evidence record comparing any
implementation against Excel. Nothing on this page is a measurement. Beyond that, this function
carries a stronger caveat than most in the catalogue: **its behaviour is environment-dependent, so
even a complete set of observations from one machine would not generalise.** The one upstream
observation available — that `ASC` returned full-width input unchanged on the host profile tested —
is scoped to that host and says nothing about a Japanese-language Excel installation, which is the
configuration the function exists for.

The probes that would build a real picture, and they need more than one machine:

1. **A Japanese-language Excel installation**, or at minimum an installation with East Asian
   language support enabled, running `ASC("ＡＢＣ　１２３")` and `ASC("ガギグ")`. Without this the
   function's actual behaviour is simply unobserved.
2. **The same sheet on an English installation**, to record the difference. The pair of results is
   the finding; either alone is not.
3. **The voiced-kana length change**, `LEN(ASC("ガ"))`, which tests the one-to-two mapping.
4. **The full-width space**, `LEN(TRIM(ASC(UNICHAR(12288) & "a")))`, which tests the interaction
   with `TRIM`.
5. **Characters with no half-width form** — hiragana, kanji, full-width currency symbols — to map
   the pass-through set.
6. **What exactly triggers admission**: display language, editing language, or the workbook's
   locale. This is the question a user actually needs answered and none of the sources consulted
   here answers it.

Until at least the first two exist, the correct summary of `ASC` in this Handbook is: documented
purpose known, mapping concept known, actual behaviour on any given machine unobserved.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| half-width | The single-column, historically single-byte form of a Latin, digit or katakana character |
| full-width | The double-column, historically double-byte form of the same character |
| width-conversion mode | The host-supplied setting that decides whether this function converts at all |
| pass-through | The mode in which the function returns its input unchanged |
| host facility | An engine seam supplied by the host; why the generated battery cannot dispatch this function |

## Sources

- Microsoft Support, ASC function —
  <https://support.microsoft.com/en-us/office/asc-function-0b6abf1c-c663-4004-a964-ebc00b723266>
  (the authority for signature and documented description; not retrievable while this entry was
  written, so it is cited by reference and nothing here paraphrases it).
- OxFunc `docs/function-lane/FUNCTION_SLICE_WIDTH_CONVERSION_HOST_PROFILE_CONTRACT_PRELIM.md` — the
  host/profile seam, the four width-conversion modes, the observed current-host pass-through for
  `ASC` and `DBCS`, and the observed `#NAME?` for `JIS`. Preliminary by its own statement and
  scoped to one host profile.
- OxFunc `docs/function-lane/FUNCTION_SLICE_TEXT_CORE_AND_COMPATIBILITY_FAMILY_CONTRACT_PRELIM.md`
  — the `*B` family's current-baseline delegate posture.
- Handbook `content/model/01-value-universe.md` (text as UTF-16 code units, Empty versus Missing),
  `02-coercion-and-lifting.md` (to-text coercion, error propagation).
