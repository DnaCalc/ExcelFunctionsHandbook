# Rounding family: ROUND, ROUNDUP, ROUNDDOWN, TRUNC, MROUND, MOD

Bead: dc-j9k.18. Downstream behaviour of Excel's NaN bit patterns: dc-j9k.19.

## Full-bank result (CalcFunc-Evidence d224c5fb, all 3,910,050 rows)

| Function | Before | After | Gains | Losses | Still failing |
|---|---:|---:|---:|---:|---:|
| MOD | 147,458 | **152,432 / 152,432** | +4,974 | 0 | 0 |
| MROUND | 45,078 | **50,782 / 50,782** | +5,704 | 0 | 0 |
| ROUND | 31,480 | 36,821 / 36,945 | +5,341 | 0 | 124 |
| ROUNDUP | 41,406 | 52,022 / 52,841 | +10,616 | 0 | 819 |
| ROUNDDOWN | 53,821 | 59,182 / 59,995 | +5,361 | 0 | 813 |
| TRUNC | 44,913 | 50,504 / 51,317 | +5,591 | 0 | 813 |
| Bank total | 3,267,249 | 3,304,836 | +37,587 | 0 | |

The comparator reports every input, selection, answer, source and environment field unchanged.

Commands (in a scratch clone, with CalcFunc-Evidence beside it):
```
harness-before witness judge --bank full --trace full-before.jsonl   # 3,267,249 (= published table)
harness-tiny   witness judge --bank full --trace full-tiny.jsonl     # 3,304,836
harness-tiny   witness compare --before full-before.jsonl --after full-tiny.jsonl
cargo test --release -p calc-func                                    # all pass
cargo test --release -p calc-func --features soft-x87 --test f1_math --test witnesses   # pass
```
Corpus, offline from committed captures (`cargo xtask corpus --oracle --no-capture`): pass 21,480 of
22,830 before and after; mismatched 1,053 -> 1,050; refused 205 -> 208. The three cells moved are
TRUNC(A8), ROUND(A7,0) and ROUND(A7,-308), which the engine now refuses (see NaN bits below).

## Rules established

1. **ROUND** counts digits as int32 and keeps k = D + e + 1 significant digits, wrapping in 32 bits.
   D = trunc(num_digits), clamped to ±2147483647. k ≥ 15 keeps the reading, k < 0 gives 0.
2. **ROUNDUP, ROUNDDOWN, TRUNC** take the low 16 bits of trunc(|d|) with d's sign. |d| ≥ 2^32 or low
   bits ≥ 32768 truncates at 10^∓100; ROUNDUP then adds no unit (d > 0) or gives #NUM! (d < 0).
3. **ROUNDUP** is truncation + unit, both as binary64, added on the x87 unit.
   - A unit below the normal range is not added (either sign).
   - For positive x, no unit when fl(reading) − fl(truncation) < 2^-1026. In the 1,148 rows nearest
     this boundary, this quantity has 1 violation, and that one is a classification artefact; exact
     value, reading and x-minus-truncation give 54 to 547. Negative x has no such distance.
4. **Fifteen-digit reading** for |n| ≤ 27 (n = 14 − e): RN64(|x|·10^n), half up for directed, down
   for ROUND. Beyond that the old exact reading stays (open).
5. **MROUND**: q = x87(n/m); count = floor(q), +1 when 0.5 − frac(q) ≤ 5E-15; x87(count·m). The data
   pins the cut-off between adjacent binary64 values at 0.5. All 50,782 rows match.
6. **MOD** with a subnormal remainder of the other sign:
   - #NUM! when the divisor's Veltkamp low part (split at 2^27 + 1) is subnormal, or |q| ≥ 2^26;
   - remainder below 2^-1026: |m| + |r|; otherwise the ordinary step.
   - All 152,432 bank rows match, and so do 1,128 new Excel readings. The new readings overturned
     the earlier power-of-two test: m = 1E-296 behaves like a power of two, m = 1E-300 does not, and
     a table over exponent and trailing zeros showed the boundary between e = −1000 and −990 (which
     is e − 26 reaching the normal range) and at about 27 trailing zeros.
   - The Veltkamp reading is my interpretation of that pattern. What is measured is the rule.
7. **Past DBL_MAX**: a decimal result ≥ 2^1024 keeps its 53-bit significand and its exponent goes
   into the field unchecked, giving NaN bits. ROUND, ROUNDDOWN, TRUNC return them (627/627 rows,
   plus all new near-DBL_MAX readings); ROUNDUP gives #NUM!.

## NaN bits downstream (Excel 16.0.20527, `excel/nan-downstream.json`, `excel/nan-downstream-sheet1.xml`)

- Display 2E+308; A4 (negative) also shows 2E+308.
- ISNUMBER TRUE, ISERROR FALSE, TYPE 1, ERROR.TYPE #N/A.
- =A1, N, MAX, ABS, IFERROR and =-A1 keep the bits unchanged.
- =A1=A1 TRUE, =A1>0 TRUE.
- =A1+0, =A1*1, =A1-A1, ROUND(A1,0), SUM: #NUM!. TEXT(A1,"0"): #VALUE!.
- Saved as t="e" `<v>#NUM!</v>`; reopened as #NUM! until a full recalculation restores the bits.

calc-func returns the bits and CalcValue carries them. `NotModeled::publishable` in calc-engine
refuses non-finite numbers, so through the engine the cell is a typed refusal. Finishing this needs
engine, formatting and file-layer work, recorded in dc-j9k.19.

## Still failing

| Function | Cause | Rows |
|---|---|---:|
| ROUND / ROUNDUP / ROUNDDOWN / TRUNC | 15-digit near-ties at scale 10^\|n\|, \|n\| > 27 | 123 / 548 / 556 / 555 |
| ROUNDUP / ROUNDDOWN / TRUNC | Reading just below DBL_MIN: Excel gives 0x000ffffffffffffa (3 ulp below nearest); ROUND gives 0 | 256 each |
| ROUNDUP | Tiny-magnitude rows that are the same near-tie class | 14 |
| all four | Other (last-bit, about 1e-12 relative) | 1 / 1 / 1 / 2 |

Powers above 10^27, from 3,414 new near-tie answers (`excel/q1rows.json`, offsets within 4·2^-64):
a threshold on the exact offset holds at 550 of 569 scales. No candidate fits. Best results, rows
wrong out of 3,414: a 66-bit power rounded up, 217; split at multiples of 16, 203; exact power, 233.
Also tried: repeated squaring, tables of 10^(2^k), multiply versus divide, 63–72 bit products,
truncating products, digit loops, two-step scaling, 16–20 digit integer routes. One control at
n = −10 also fails the single RN64 model, at an offset just over half a unit, so that model is not
complete inside |n| ≤ 27 either. It stays because it gains rows with no loss, but it is an
approximation of the real step.

## Files in this folder

`excel/questions.json`, `excel/answers.json` (4,857 rows), `excel/q5.json`, `excel/q5-answers.json`
(432 rows), `excel/q1-report.txt`, `excel/q3-report.txt`, the model scripts, `src/` (the edits).
Excel used: private instance started and quit by the runner, new blank workbook, Value2 doubles, all
inputs read back equal; build 20527.
