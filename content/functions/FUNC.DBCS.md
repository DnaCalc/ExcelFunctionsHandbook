---
schema: efh.function-page/v1
function_id: FUNC.DBCS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — DBCS function"
    locator: "https://support.microsoft.com/en-us/office/dbcs-function-a4025e73-63d2-4958-9423-21a24794c9e5"
    role: "documented signature and description of the half-width to full-width conversion"
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
role_in_family: "The widening member of the width-conversion trio."
---

## What it computes

`DBCS` converts half-width (single-byte) characters in a string to their full-width (double-byte)
equivalents. It is the inverse direction of [`ASC`](FUNC.ASC.md), and the background there applies
here unchanged: the half-width/full-width distinction comes from the legacy East Asian double-byte
character sets, where Latin letters, digits and katakana each had a one-column narrow form and a
two-column wide form, and Unicode preserved both as separate code points. `DBCS` maps narrow to
wide — `A` (U+0041) to `Ａ` (U+FF21), `ｱ` (U+FF71) to `ア` (U+30A2).

The widening direction has its own asymmetries, and they are not the mirror image of the narrowing
ones:

1. **Widening can *shorten* the string.** The half-width sequence `ｶ` + `ﾞ` (base kana plus voiced
   sound mark) widens to the single character `ガ`. So `DBCS` performs a composition, which means it
   must look ahead: a character-by-character map is wrong precisely where the function is most
   used.
2. **Almost everything ASCII has a full-width form.** Unlike narrowing, where much of the
   full-width repertoire has no counterpart, the `U+0021`–`U+007E` range maps cleanly into
   `U+FF01`–`U+FF5E`. That regularity makes the Latin half of `DBCS` deceptively easy and tempts
   implementers into an arithmetic offset that then mishandles the kana.
3. **The space is a special case.** The ordinary space `U+0020` widens to the ideographic space
   `U+3000`, which is not in the arithmetic range.

And, as with `ASC`, the decisive fact is that **whether `DBCS` converts anything at all depends on
the host environment.** Upstream OxFunc's width-conversion contract records a native host replay in
which `DBCS` applied to half-width input returned the input **unchanged** on the profile tested,
and models the function as a two-stage operation: the host or profile supplies a width-conversion
mode — pass-through, narrow, widen, or unavailable — and the transformation is applied only when
the mode says so.

So the operational definition is: *ask the environment whether width conversion is active; if it
is, apply the half-width-to-full-width mapping with kana composition; if it is not, return the text
unchanged.*

## Arguments

`DBCS(text)`

| Argument | Required | Meaning |
|---|---|---|
| `text` | yes | The text containing characters to widen. |

One argument and no options. Non-text arguments convert to text by the ordinary rules in
[coercion and lifting](../model/02-coercion-and-lifting.md) — which, note, means `DBCS(123)` asks
about the string `"123"` and would widen the digits if conversion were active.

## Result and edge cases

The return kind is `Text`.

- **The result can be shorter than the input** in UTF-16 code units, because of kana composition.
  The dual of the `ASC` warning: `LEN(DBCS(x)) >= LEN(x)` is not a safe assumption either.
- **On a host where width conversion is inactive, `DBCS` is the identity.** This is what the
  upstream replay observed. A workbook that depends on `DBCS` therefore computes different values
  on different machines.
- **Characters with no full-width form pass through.**
- **Errors propagate**; an empty referenced cell delivers `Empty`, distinct from an omitted
  argument (see [the value universe](../model/01-value-universe.md)).
- **The generated battery on this page cannot dispatch `DBCS`** — the upstream reference
  implementation reports it as requiring a host facility, so the standard probe rows have no
  outcome. That is the honest result for a function whose semantics are host-supplied.

## Errors

Microsoft's page is the authority and is linked below; it was not retrievable while this entry was
written, so this section states expectations rather than quoting it.

| Error | Status |
|---|---|
| any error value, propagated from the argument | follows from the universal error-propagation rule |
| `#NAME?` where the function is not admitted in the environment | recorded upstream for the sibling `JIS` on the observed host, and retained in the upstream model as a defensive projection for the unavailable mode. Whether `DBCS` itself is ever unavailable this way is not established. |

## Relationships

- [`ASC`](FUNC.ASC.md) — the inverse direction, same host dependency, same module upstream. They
  are inverses in intent but not exactly: `DBCS(ASC(x))` need not be `x`, because narrowing loses
  the distinction between characters that share a half-width form and widening composes sequences
  that were not composed before.
- `JIS` — Microsoft's name for the widening function in Japanese-language versions; upstream's
  replay observed it returning `#NAME?` on the host it tested, which is the clearest available
  illustration of the admission question. Readers should not assume `DBCS` and `JIS` are the same
  entry point on every installation.
- `LENB`, `LEFTB`, `MIDB`, `FINDB`, `SEARCHB`, `REPLACEB` — the byte-oriented compatibility family
  that exists for the same East Asian reasons. Upstream records them as current-baseline delegates
  to their non-`B` counterparts, with `LENB` explicitly retaining UTF-16 code-unit counting after
  `LEN` moved to Unicode-scalar counting.
- [`UPPER`](FUNC.UPPER.md) and `LOWER` — a different axis entirely. Width and case are independent.
- Unicode NFKC normalisation goes the *other* way (it narrows), so it is not a substitute for
  `DBCS` in any direction.

## Notes for implementers

- **Query the host before transforming.** A port that always widens will disagree with Excel on
  every host where conversion is inactive — a configuration upstream actually observed.
- **Compose the kana, do not map them.** The `ｶ` + `ﾞ` → `ガ` case requires a two-character lookahead
  and a composition table. An offset-based implementation over the ASCII range will pass every
  Latin test and fail every kana one.
- **Handle the space explicitly.** `U+0020` → `U+3000` is outside the arithmetic range and is the
  most common non-Latin case in real data.
- **Be prepared for the output to be shorter**, and do not size the output buffer from the input
  length without allowing for it.
- **Record the host profile in every fixture.** Without it, a width-conversion result is not
  reproducible evidence.

## What has not been checked

There is no Handbook vector suite for `DBCS`, and no Handbook evidence record comparing any
implementation against Excel. Nothing on this page is a measurement. As with [`ASC`](FUNC.ASC.md),
there is a stronger caveat: **behaviour is environment-dependent, so observations from one machine
do not generalise.** The single upstream observation available — half-width input returned
unchanged on the host tested — is scoped to that host and says nothing about a Japanese-language
installation, which is the configuration the function was designed for.

The probes worth running, and they need more than one machine:

1. **A Japanese-language (or East-Asian-enabled) Excel installation** running `DBCS("ABC ｶﾞ")` and
   `DBCS("abc 123")`. Without this, the function's real behaviour is unobserved.
2. **The same sheet on an English installation**, so the pair can be compared. The difference is
   the finding.
3. **Kana composition**, `LEN(DBCS("ｶﾞ"))` — `1` confirms composition, `2` confirms a naive map.
4. **The space**, `UNICODE(DBCS(" "))` — `12288` confirms the ideographic-space mapping.
5. **Round-tripping**, `EXACT(DBCS(ASC(x)), x)` over a mixed string, which measures how lossy the
   pair is in practice.
6. **What triggers admission** — display language, editing language, or workbook locale — and
   whether `DBCS` and `JIS` are admitted independently. This is the question users actually have,
   and none of the sources consulted here answers it.

Until the first two exist, the honest summary is: documented purpose known, mapping concept known,
actual behaviour on any given machine unobserved.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| half-width | The single-column, historically single-byte character form |
| full-width | The double-column, historically double-byte character form |
| composition | Combining a base kana and a voiced sound mark into one full-width character |
| width-conversion mode | The host-supplied setting deciding whether this function converts at all |
| host facility | An engine seam supplied by the host; why the generated battery cannot dispatch this function |

## Sources

- Microsoft Support, DBCS function —
  <https://support.microsoft.com/en-us/office/dbcs-function-a4025e73-63d2-4958-9423-21a24794c9e5>
  (the authority for signature and documented description; not retrievable while this entry was
  written, so it is cited by reference and nothing here paraphrases it).
- OxFunc `docs/function-lane/FUNCTION_SLICE_WIDTH_CONVERSION_HOST_PROFILE_CONTRACT_PRELIM.md` — the
  host/profile seam, the four modes, the observed pass-through for `ASC` and `DBCS`, and the
  observed `#NAME?` for `JIS`. Preliminary by its own statement and scoped to one host profile.
- OxFunc `docs/function-lane/FUNCTION_SLICE_TEXT_CORE_AND_COMPATIBILITY_FAMILY_CONTRACT_PRELIM.md`
  — the `*B` family delegate posture and the `LEN`/`LENB` divergence.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md`.
