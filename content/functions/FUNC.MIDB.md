---
schema: efh.function-page/v1
function_id: FUNC.MIDB
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
role_in_family: The byte-counting interior slice; MID's DBCS twin, addressing by byte offset.
---

# MIDB

## What it computes

`MIDB(text, start_num, num_bytes)` returns the run of `text` that begins at **byte** position
`start_num` and is `num_bytes` bytes long, where byte positions are counted from 1.

The whole content of this function is the word "byte", and Microsoft states precisely when it
means anything:

> The B functions count each double-byte character as 2 **only when a DBCS language has been
> enabled for editing and set as the default language**. Otherwise they count each character
> as 1.

So `MIDB` has two behaviours and which one you get is a property of the machine, not of the
formula. On a default-English installation `MIDB(text, s, k)` and `MID(text, s, k)` are the
same function. On an installation whose default language is Japanese, Chinese (Simplified or
Traditional), or Korean, every character in a double-byte range costs two positions.

The byte count is against the **DBCS code page for the active language**, not UTF-8 and not
UTF-16. That distinction matters for anyone implementing this: a UTF-8 byte count would make a
Latin-1 accented character cost two bytes and a CJK character cost three, and neither matches
the DBCS rule.

## Arguments

| Argument | Meaning |
|---|---|
| `text` | The string to slice. Required. |
| `start_num` | 1-based **byte** position of the first byte to take. Required. |
| `num_bytes` | How many bytes to take. Required, and must not be negative. |

All three are required. The argument name is the only signature difference from `MID`; the
arity and positions are identical.

## Result and edge cases

Returns `Text`.

The boundary rules Microsoft documents for `MID` are stated on the same page for `MIDB` with
"characters" replaced by "bytes": a `start_num` past the end of `text` yields the empty string,
and a window running past the end yields the bytes to the end of `text`.

The genuinely `MIDB`-specific question — what happens when `start_num` or `start_num +
num_bytes` lands **inside** a double-byte character rather than on its boundary — is not
answered by Microsoft's page. There are several defensible behaviours (return the half byte as
a replacement character, drop the partial character, round the boundary outward, round it
inward) and the Handbook does not know which one Excel implements. This is the single most
important open question about this function and it is not a corner case: any offset arithmetic
over mixed-width text hits it immediately.

Empty, missing and error arguments follow the shared call model; see
[Coercion and lifting](../model/02-coercion-and-lifting.md).

## Errors

As documented on Microsoft's `MID, MIDB` page:

| Error | Condition |
|---|---|
| `#VALUE!` | `start_num` is less than 1. |
| `#VALUE!` | `num_bytes` is negative. |

Non-numeric text in a numeric argument slot and incoming error values behave as the shared
coercion rules describe. The Handbook has not verified any of this against Excel.

## Relationships

- [MID](FUNC.MID.md) is the character-counting twin. Microsoft documents the two on a single
  page, `MID, MIDB`; the Handbook publishes them as two entries because they are two functions
  with two different addressing units.
- `LEFTB`, `RIGHTB`, `LENB`, `FINDB`, `REPLACEB`, `SEARCHB` are the rest of the B family. They
  share one implementing module in the reference engine and they share this page's central
  caveat: outside a DBCS default language they are aliases of their non-B counterparts.
- `LENB` is the function you need to reason about `MIDB` offsets at all, because `MIDB`
  positions are `LENB` positions, not `LEN` positions.

## Notes for implementers

Three things make this function harder than it looks.

1. **It is host-state dependent.** The active default editing language is an input to the
   function that does not appear in its argument list. Any implementation that wants to match
   Excel needs that state threaded in, and any test vector for `MIDB` is meaningless without
   recording it.
2. **The code page is per-language.** Shift-JIS, GBK, Big5 and EUC-KR assign different byte
   widths to different characters. "Two bytes for CJK" is a summary, not a specification.
3. **Partial-character windows are reachable and undefined here.** Pick a behaviour, document
   it, and mark it as unverified — do not present a guess as the rule.

## What has not been checked

No Handbook vector suite exists for `MIDB`, and no Excel-comparison evidence record names it.
Nobody has checked this function against Excel within the Handbook's record — and for `MIDB`
that gap is wider than usual, because the interesting behaviour only appears on a DBCS-default
installation, which is a configuration the Handbook has not exercised at all.

What would settle it, in order:

1. **The same formula on two machines.** `MIDB("日本語ABC", 2, 3)` on an English-default
   installation and on a Japanese-default installation. Two results from one formula is the
   whole documented claim, and confirming it is the prerequisite for every other probe.
2. **A window starting mid-character**: `MIDB("日本語", 2, 2)` under DBCS, which begins at the
   second byte of the first character. This is the undocumented case above.
3. **A window ending mid-character**: `MIDB("日本語", 1, 3)`.
4. **Mixed-width text**, e.g. `MIDB("A日B", 2, 2)`, to check that byte accounting is per
   character and not a whole-string mode.
5. **`LENB` cross-checks** on the same strings, so that the byte coordinate system used by
   `MIDB` and by `LENB` can be confirmed to be the same one.

Until at least probe 1 exists, everything this page says about DBCS behaviour is Microsoft's
documented claim and not a Handbook observation.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `MIDB` that an implementer matching Excel, or a reader explaining an
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
| default editing language | The host-level setting that decides whether B functions count bytes or characters |
| byte position | The 1-based coordinate `MIDB` addresses, measured the way `LENB` measures |

## Sources

- Microsoft, "MID, MIDB functions" —
  <https://support.microsoft.com/en-us/office/mid-function-d5f9e25c-d7d6-472e-b568-4ecb12433028>
  (the shared signature, the boundary rules, and the DBCS default-language condition).
- Handbook, [The value universe](../model/01-value-universe.md) — worksheet text as UTF-16
  code units, which is what the byte view has to be reconciled against.
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — numeric-argument
  coercion and error propagation.
- Handbook projection `data/presence/FUNC.MIDB.json` — the shared `text_b_compat_family`
  implementing module and its sibling set.
