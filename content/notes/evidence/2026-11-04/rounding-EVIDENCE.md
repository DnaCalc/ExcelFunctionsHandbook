# Rounding family: ROUND, ROUNDUP, ROUNDDOWN, TRUNC, MROUND (MOD left open)

Patch: `rounding-family.patch` (next to this file). Base: DnaCalc 6073fff1. It also applies cleanly
(`git apply --check`) to the orchestrator clone at 1ec418b2. It touches calc-func only:
`src/catalogue/rounding.rs`, `src/decimal.rs`, a new test in `tests/f1_math.rs`, and the
re-pinned calculated predictions in `tests/witnesses/{ROUND,ROUNDUP,ROUNDDOWN,TRUNC,MROUND}.json`
(67 rows. They were all gains, re-pinned with `witness refresh`, and no Excel answer changed).

## Full-bank result (OxFunc-Evidence 4cb48d15, all 3,910,050 rows)

| Function | Before | After | Gains | Losses | Still failing |
|---|---:|---:|---:|---:|---:|
| ROUND | 31,480 / 36,945 | 36,391 | +4,911 | 0 | 554 |
| ROUNDUP | 41,406 / 52,841 | 49,013 | +7,607 | 0 | 3,828 |
| ROUNDDOWN | 53,821 / 59,995 | 59,080 | +5,259 | 0 | 915 |
| TRUNC | 44,913 / 51,317 | 50,409 | +5,496 | 0 | 908 |
| MROUND | 45,078 / 50,782 | **50,782** | +5,704 | 0 | **0** |
| MOD | 147,458 / 152,432 | unchanged | 0 | 0 | 4,974 |
| Bank total | 3,267,249 | 3,296,226 | +28,977 | **0** | |

There were no losses in any function. The comparator reports that every input, selection, answer,
source and environment field is unchanged.

Commands (firehorse, in my own clone `~/claude-work/rounding/DnaCalc`, with `OxFunc-Evidence`
symlinked beside it):
```
cargo build --release --locked -p xtask-harness --bin xtask-harness   # once at 6073fff1, once patched
harness-before witness judge --bank full --trace full-before.jsonl   # 3,267,249 / 3,910,050 (= published table)
harness-final  witness judge --bank full --trace full-final.jsonl    # 3,296,226 / 3,910,050
harness-final  witness compare --before full-before.jsonl --after full-final.jsonl
cargo test --release -p calc-func                 # all pass
cargo test --release -p calc-func --features soft-x87 --test f1_math --test witnesses   # pass
cargo build --release -p calc-func --target wasm32-unknown-unknown                     # builds
```
Failing first: before the change, `rounding_family_matches_recorded_bank_rows` failed at its first row
(`got 0e0, expected -5.48022208827565e-221`). After the change it passes.
Traces, logs and scripts are in `~/claude-work/rounding/{traces,*.py}` on firehorse.

## What was established

All of these are read off the data as structure. No constant was tuned except the MROUND tolerance,
and the data pins that one to a window one binary64 step wide.

1. **ROUND counts digits as int32 and keeps k = D + e + 1 significant digits, with the sum wrapping in 32 bits.**
   - D = trunc(num_digits), clamped symmetrically to ±2147483647.
   - e = the exact decimal exponent of |x|.
   - k ≥ 15 keeps the 15-digit reading, k < 0 gives 0, and anything else rounds normally.
   - The boundaries move by exactly −e on both the INT_MAX and INT_MIN sides, for e from −308 to 308.
   - This covers the old special case `ROUND(1.5,2147483647)=0`.
   - The out-of-range case also behaves as the symmetric clamp predicts: d ≥ 2^31 acts as +INT_MAX (the boundary falls between e=−1 and e=0), and d ≤ −2^31 acts as −INT_MAX (between e=−3 and e=−2).
2. **ROUNDUP, ROUNDDOWN and TRUNC count digits from the low 16 bits of trunc(|d|), with the sign of d.**
   - If |d| ≥ 2^32, or the low 16 bits are ≥ 32768, the count is out of range and the truncation happens at 10^∓100.
   - Examples: 32768..65535 → 100, 65536 → 0, 65537 → 1, −2147483748 → −100 (low 16 bits = 100, in range).
   - In the out-of-range case ROUNDUP adds no unit when d > 0, and gives #NUM! when d < 0 and a digit is dropped.
   - All 13,914 informative ROUNDDOWN/TRUNC rows with large d are consistent with this rule.
3. **ROUNDUP is the ROUNDDOWN result plus one unit 10^-p, both converted to binary64 and then added.** This explains the last-bit pattern, for example ROUNDUP(-1.125,2) = -1.1300000000000001 and ROUNDUP(3.535,2) = 3.5399999999999996.
4. **Fifteen-digit reading on the x87 unit, for |n| ≤ 27, where n = 14 − e and 10^|n| is exact in 64 bits.**
   - y = RN64(|x|·10^n), or RN64(|x|/10^-n) when n < 0.
   - A half is rounded up for the directed functions and down for ROUND.
   - This is why 16-digit ties such as 1594628480062905 go down in ROUND (…900) but up in ROUNDDOWN.
   - For |n| > 27 the product keeps the old exact reading, because the larger-power scaling is not identified (see open items).
5. **MROUND: q = x87(n/m). The count is floor(q), plus 1 when 0.5 − frac(q) ≤ 5E-15. The result is x87(count·m).**
   - The cut-off is pinned between 4.996E-15 (rounded up) and 5.0515E-15 (rounded down). These are adjacent binary64 values at 0.5, measured at fl=0 over 4,264 rows.
   - 5E-15 is the decimal value in that window. With it, all 50,782 MROUND rows match.
   - It keeps MROUND(6.05,0.1)=6, since 60.49999999999999 lies 7.1E-15 below the half.

## Still failing, by cause (each is a named next target)

| Function | Cause | Rows |
|---|---|---:|
| ROUND / ROUNDDOWN / TRUNC | Excel returns NaN bit patterns near DBL_MAX (for example ROUND(1.79e308,-307) → 0x7ff005419221015d, i.e. 1.8e308 with the exponent field overflowed). Its decimal→binary conversion does not saturate. | 430 / 102 / 95 |
| ROUND / ROUNDUP / ROUNDDOWN / TRUNC | 15-digit near-ties when the scale is 10^\|n\| with \|n\| > 27. A single correctly rounded 64-bit power gave +184 / −44 on ROUNDDOWN (and the same on ROUNDUP), and the tested chained-multiplication schemes do not fit, so the power method is open. | 123 / 548 / 556 / 555 |
| ROUNDDOWN / ROUNDUP / TRUNC | Inputs at DBL_MIN give Excel's subnormal 0x000ffffffffffffa (the reading 2.22507385850720E-308 is converted 3 ulp below nearest), even for ROUNDUP(DBL_MIN,0). | 256 each |
| ROUNDUP | Positive tiny x: a dropped part below about 2^-1026 (DBL_MIN/16) is ignored. The threshold is absolute, not 15-digit. | 2,521 |
| ROUNDUP | Negative tiny x (differences 1e-313…1e-322 near the 15-digit edge) | 502 |
| ROUND / ROUNDUP / ROUNDDOWN / TRUNC | other (1 row is a shared last-bit case at 1e-12 relative) | 1 / 1 / 1 / 2 |
| MOD | Every failure has both operands below 1e-290. Excel gives #NUM! at quotients near 1–7e7 there, and sometimes returns −n itself (for example MOD(-1.2232472900445720E-296, 1.223247290044539E-296) = +1.223247290044572E-296). The intermediate subnormal handling is not identified. | 4,974 |

MOD is **not changed**. Its mismatches lie entirely in the tiny-operand domain, and I found no rule that
explains them.

## Minimal Excel questions that would settle the open items (one fresh workbook, Value2 doubles)

1. Power scaling for \|n\| > 27: ROUNDDOWN(x,0) for x = the double nearest k.5·10^(e−14) for one chosen
   15-digit k, across e = 42..60 and e = −13..−40 (about 40 cells). The pattern of hi/lo answers per e
   identifies whether 10^n comes from a table, from repeated squaring, or from FSCALE/F2XM1.
2. ROUNDUP positive-tiny threshold: ROUNDUP(1E-300 + t, 307) for t = 2^-1026·(1 ± 2^-20) and
   t = 2^-1027, 2^-1025. Also the same at 1E-298 and 1E-305 (9 cells). This shows whether the rule is
   "x − trunc ≥ 2^-1026" or something else.
3. MOD tiny: MOD(-(1+j·2^-52)·m, m) for m = 1E-296 and j = 1..8, and MOD(n,m) with n/m = 2^25..2^27
   at m = 1E-305 (about 20 cells).
4. NaN-bit conversion: ROUND(1.7976931348623157E308, -307 … -300). This confirms that the
   exponent-overflow encoding comes from one conversion routine (8 cells).

The owner's new ruling allows these to be run through the spot-check runner pattern. I did not drive
Excel in this round.
