---
schema: efh.function-page/v1
function_id: FUNC.REPLACEB
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
  - Notes
  - Page vocabulary
  - Sources
family: text_b_compat_family
role_in_family: The byte-addressed positional writer; REPLACE's DBCS twin.
---

# REPLACEB

## What it computes

`REPLACEB(old_text, start_num, num_bytes, new_text)` returns `old_text` with the window of
`num_bytes` **bytes** beginning at byte position `start_num` removed and `new_text` put in its
place. Byte positions are counted from 1.

It differs from [REPLACE](FUNC.REPLACE.md) in exactly one respect — the unit in which the
window is addressed — and that difference only exists under one condition. Microsoft's rule for
the whole B family:

> The B functions count each double-byte character as 2 **only when a DBCS language has been
> enabled for editing and set as the default language**. Otherwise they count each character
> as 1.

On a machine whose default editing language is not one of the DBCS languages, `REPLACEB` is
`REPLACE`. On a machine whose default is Japanese, Chinese or Korean, the same formula
addresses a different window of the same string. The active language is an unlisted input to
this function.

The bytes are those of the **DBCS code page for the active language** — Shift-JIS, GBK, Big5,
EUC-KR — not UTF-8 and not UTF-16.

## Arguments

| Argument | Meaning |
|---|---|
| `old_text` | The string to modify. Required. |
| `start_num` | 1-based **byte** position of the first byte to replace. Required. |
| `num_bytes` | How many bytes to remove. Required. |
| `new_text` | What to put in their place. Required. |

All four are required. The signature is `REPLACE`'s with `num_chars` renamed; the arity and
positions are identical.

`new_text` is inserted as text, not as bytes. There is no requirement that its byte length
match `num_bytes`, and no documented rule connecting the two.

## Result and edge cases

Returns `Text`.

Microsoft's `REPLACE, REPLACEB` page states the signature and argument meanings and does not
enumerate boundary behaviours. Everything below the signature is therefore unverified here.

The distinctively `REPLACEB` question is the one the documentation does not touch: **what
happens when `start_num` or `start_num + num_bytes` falls inside a double-byte character
rather than on its boundary.** Overwriting half of a character has no obviously correct
answer — drop the partial character, keep its orphaned byte, round the boundary outward,
error — and Microsoft names none of them. Under DBCS this is not exotic: any arithmetic on
byte offsets over mixed-width text reaches it on the first mixed string.

Empty, missing and error arguments follow the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md). The implementing module named in
the presence projection carries an open upstream array-support defect stream
(`BUG-FUNC-016`), so the array shape of this function is also unsettled.

## Errors

Microsoft's page carries no error table for `REPLACEB`. The reachable errors are the shared
ones: an error value in any argument propagates, and non-numeric text in `start_num` or
`num_bytes` surfaces `#VALUE!` under the shared coercion rules. Whether a partial-character
window is an error condition is exactly the open question above.

## Relationships

- **[REPLACE](FUNC.REPLACE.md)** — the character-addressed twin, documented by Microsoft on the
  same page. The Handbook publishes them as two entries because they address in two different
  units.
- **[SUBSTITUTE](FUNC.SUBSTITUTE.md)** — the by-content alternative, and the one to reach for
  under DBCS, because matching a substring sidesteps byte-offset arithmetic entirely. There is
  no `SUBSTITUTEB`; the by-content route simply does not need one.
- **`LENB`, `FINDB`, `SEARCHB`, `LEFTB`, `RIGHTB`, [MIDB](FUNC.MIDB.md)** — the rest of the B
  family, sharing one implementing module in the reference engine and this page's central
  caveat.
- `LENB` and `FINDB` are the functions that make `REPLACEB` usable at all, because they are the
  only ones that produce coordinates in the same byte space.

## Notes for implementers

`REPLACEB` inherits everything hard about `REPLACE` — three-piece reconstruction, growth past
the text cap, edges of the window past the end of the string — and adds the byte question on
top.

The practical consequence for verification: a `REPLACEB` test vector that does not record the
default editing language is not a test vector. The same inputs legitimately produce two
different correct answers.

The mixed-unit boundary deserves an explicit, recorded decision. `old_text` arrives as UTF-16
code units; `start_num` and `num_bytes` are code-page bytes; `new_text` arrives as UTF-16
again. An implementation is converting between two coordinate systems on every call, and a
partial-character window is where the conversion stops being a bijection.

## What has not been checked

No Handbook vector suite exists for `REPLACEB`, and no Excel-comparison evidence record names
it. Nobody has checked this function against Excel within the Handbook's record — and the
configuration under which its distinguishing behaviour appears at all, a DBCS default editing
language, has not been exercised by the Handbook in any form.

Inputs worth probing first:

1. **The two-machine baseline**: `REPLACEB("日本語", 3, 2, "X")` on an English-default
   installation and on a Japanese-default installation. Until those two answers differ as
   documented, nothing else on this page is worth measuring.
2. **A window starting mid-character** under DBCS: `REPLACEB("日本語", 2, 2, "X")`.
3. **A window ending mid-character** under DBCS: `REPLACEB("日本語", 1, 3, "X")`.
4. **Mixed-width text**: `REPLACEB("A日B", 2, 2, "X")`, to confirm byte accounting is per
   character rather than a whole-string mode.
5. **`start_num` of 0 and a `start_num` past the end**, which `REPLACE`'s page also leaves
   undefined and which should be probed on both functions together so the answers can be
   compared.
6. **`LENB` cross-checks** on the same strings, establishing that `REPLACEB` and `LENB` agree
   on the byte coordinate system.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `REPLACEB` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-LENB-001: Under a single-byte Windows locale such as en-ZA, the B-suffixed text functions count characters rather than UTF-8 or UTF-16 bytes

*Quirk; measured; recorded 2026-11-22.* Applies to FINDB, LEFTB, LENB, MIDB, REPLACEB, RIGHTB, SEARCHB.

In Excel 16.0.20527.20088 with user culture en-ZA (a single-byte Windows code page), LENB,
LEFTB, RIGHTB, MIDB, FINDB, SEARCHB, and REPLACEB count characters rather than UTF-8 or UTF-16
bytes: LENB(UNICHAR(20013)) returns 1, LEFTB(UNICHAR(20013)&"abc",2) returns "中a", and
FINDB("b",UNICHAR(20013)&"b") returns 2 (double-byte counting applies only under DBCS languages
such as ja-JP, ko-KR, or zh-CN).

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=LENB(UNICHAR(20013))` | as typed | `1.0` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=LEFTB(UNICHAR(20013)&"abc",2)` | as typed | `中a` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=FINDB("b",UNICHAR(20013)&"b")` | as typed | `2.0` (`0x4000000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=REPLACEB(UNICHAR(20013)&"abc",2,1,"x")` | as typed | `中xbc` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured across rows 18-27 of dczsn3_bytes in Excel 16.0.20527.20088 (en-ZA)
and matched across full-bank rows (LENB 2,083/2,083, LEFTB 2,478/2,478, RIGHTB 2,481/2,481,
MIDB 2,913/2,913, FINDB 2,861/2,861, SEARCHB 2,861/2,861, REPLACEB 2,951/2,951).

- Open: DBCS locales (ja-JP, ko-KR, zh-CN) remain uncaptured and refused in calc-func (dc-zsn.3).

Evidence:

- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dczsn3_bytes.capture.json`, rows 18 to 27 at `4fb9b728df` — Excel 16.0.20527.20088 single-byte locale B-function measurements
- DnaCalc `crates/calc-func/src/catalogue/text_functions.rs`, single_byte at `4fb9b728df` — Single-byte vs DBCS locale guard
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dczsn3_bytes.capture.json`, sheet dczsn3_bytes row 18 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dczsn3_bytes.capture.json`, sheet dczsn3_bytes row 21 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dczsn3_bytes.capture.json`, sheet dczsn3_bytes row 24 column 4 at `4fb9b728df` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dczsn3_bytes.capture.json`, sheet dczsn3_bytes row 26 column 4 at `4fb9b728df` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| DBCS | Double-byte character set; the encodings in which one character can cost two bytes |
| default editing language | The host setting that decides whether B functions count bytes or characters |
| partial-character window | A byte window whose edge falls inside a double-byte character |

## Sources

- Microsoft, "REPLACE, REPLACEB functions" —
  <https://support.microsoft.com/en-us/office/replace-function-8d799074-2425-4a8a-84bc-82472868878a>
  (signature, argument meanings, and the DBCS default-language condition for the B family).
- Handbook, [The value universe](../model/01-value-universe.md) — worksheet text as UTF-16
  code units and the 32,767-unit cap, which the byte view must be reconciled against.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — numeric-argument
  coercion and error propagation.
- Handbook projection `data/presence/FUNC.REPLACEB.json` — the shared `text_b_compat_family`
  implementing module, its sibling set, and the `BUG-FUNC-016` defect stream.
