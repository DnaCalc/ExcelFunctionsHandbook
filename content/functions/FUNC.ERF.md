---
schema: efh.function-page/v1
function_id: FUNC.ERF
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0018
  - EV-DIST-0024
open_problems: []
references:
  - work: "Microsoft Support — ERF function"
    locator: "https://support.microsoft.com/en-us/office/erf-function-c53c7e7b-5482-4b6c-883e-56df3c9af349"
    role: "syntax, argument meanings, the two #VALUE! remarks and the worked examples"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapter 7"
    locator: "7.1 — the error function"
    role: "definition, series, and the relation to erfc"
  - work: "NIST Digital Library of Mathematical Functions, chapter 7"
    locator: "https://dlmf.nist.gov/7.2"
    role: "definition (7.2), symmetry (7.4) and the relation to the incomplete gamma function (7.11)"
episodes: []
body_sections:
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Notes
  - Sources
family: special_dist_family
role_in_family: >-
  The original, two-limit error function: the only member of the family that integrates between
  two points, and the surface for which the shared body is inferred, not scored.
---

# ERF

## What it does

`ERF(lower_limit, [upper_limit])` returns the normalised integral of `exp(-t*t)` between the limits. With one argument it integrates from zero to that argument; with two it subtracts the lower primitive from the upper.

## Try it

`=ERF(0.25)` is about `0.276326390168237`. `=ERF(0.1,0.25)` computes the integral over just that interval; reversing the two bounds negates it.

## Quirks

Nonnumeric arguments give `#VALUE!`. The PRECISE name does not imply correctly rounded results on every input. The measured below-half stage and the unresolved upper stage need separate treatment. Subnormal inputs transferred to worksheet cells can be read back as zero, so the input route is part of a useful reproduction.


The recorded two-argument ERF wrapper reads left to right and rejects logicals: `ERF(TRUE,#REF!)` returns `#VALUE!`, while `ERF(#REF!,TRUE)` returns `#REF!`. DELTA and GESTEP have a different recorded read order.

## How to implement

For normal `abs(x)<0.5`, use `R(op)=RN53(RN64(op))`: round each operation to a 64-bit significand, then store to binary64. Decode the following hexadecimal strings as **binary64 bit patterns**, not integers converted to floating-point values.

| Power | Numerator coefficient | Denominator coefficient |
|---:|---|---|
| 0 | `3ff1df5e70c7956b` | `3fefada15778ece6` |
| 1 | `3fc201d8627a7d2c` | `3fdd191aca8d2499` |
| 2 | `3fa6f289d165b303` | `3fb79f85ba68803e` |
| 3 | `3f5d93a5813e372c` | `3f856de5da86af12` |
| 4 | `3f2932767df264b2` | `3f4609751b5b6488` |
| 5 | — | `3ef4f20a473a5ab0` |

1. Store `t=R(x*x)`.
2. Evaluate each polynomial by Horner, beginning with its highest-power coefficient. Every step is `v=R(R(v*t)+coefficient)`. Keep the multiplication store separate from the addition store; do not use FMA.
3. Store `p=R(x*N)` and return `R(p/D)`.

Worked `x=0.25`: `t=0.0625`; the staged coefficient evaluation gives `N=1.126001525484713`, `D=1.018724202200851`, `p=0.2815003813711783`, and result `0.276326390168237` (`0x3fd1af54e232d60a`). These are outputs of the stated reconstruction, with all input coefficients retained above.

The degrees and stores are supported by discriminating readings. The final bits of several higher-order coefficients remain open: the fixed c14 model matched 59,982 of a fresh 60,000-row holdout. Do not extrapolate this ratio above `abs(x)=0.5`. For larger magnitudes the current code uses the signed stored complement `copysign(R(1-upper_ERFC(abs(x))),x)`. At exactly plus or minus 0.5 it instead retains its runtime ERF fallback. The fixed ratio and the mathematical value agree in bits at the positive half point, so that single recorded answer does not identify which private branch owns it. Zero and nonnormal inputs also retain the fallback and their separate transfer/publication rules. The upper ERFC exponential stage is measured, but its remaining rational factor is a stand-in. No published Handbook suite establishes a whole-domain equality claim.

For the two-limit form the current kernel subtracts the two separately evaluated primitives in binary64. Nearly equal endpoints can lose relative accuracy; that wrapper is a separate obligation from the rational body.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 42 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.ERF.json).

Retained controls from the indexed captures:

| Build and recorded CPU | Captured call or arguments | Excel result |
|---|---|---|
| 19929, CPU not recorded | [`=ERF(0)`](https://github.com/DnaCalc/CalcFunc-Evidence/blob/4287ba192d2e3a7fd54671ea1cee1c34cb2e84e2/runs/W097-R-B-validation-cellref-module/comparisons/excel_sample_comparisons.jsonl.gz) | `0.0` (`0x0000000000000000`) |

These are earlier recorded answers, not new Excel runs or an additional cross-build comparison. The function JSON retains each source locator, exact argument bits where available, and input-route status. Call notation does not imply that captured binary64 values were entered as decimal formula literals.

## Alternatives

[ERFC](FUNC.ERFC.md) computes the complementary tail directly. [NORM.S.DIST](FUNC.NORM.S.DIST.md) provides a standard-normal CDF with a different argument transformation.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `ERF` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-DELTA-001: DELTA and GESTEP read their second argument first

*Quirk; measured; recorded 2026-11-04.* Applies to DELTA, ERF, GESTEP.

When both arguments of DELTA or GESTEP are bad, the error comes from the second one:
DELTA(TRUE,#REF!) is #REF! and DELTA(#REF!,TRUE) is #VALUE!. With two errors, DELTA gives the
second. ERF reads left to right: ERF(TRUE,#REF!) is #VALUE! and ERF(#REF!,TRUE) is #REF!. A
logical is rejected in either position of these functions, whether typed or held in a cell:
DELTA(TRUE,0) and DELTA(2,TRUE) are #VALUE!, and DELTA(B1,0) with TRUE in B1 is #VALUE!, while
an empty B1 reads as 0 and gives 1.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `=DELTA(TRUE,#REF!)` | as typed | `#REF!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=DELTA(#REF!,TRUE)` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=GESTEP(TRUE,#REF!)` | as typed | `#REF!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=GESTEP(#REF!,TRUE)` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ERF(TRUE,#REF!)` | as typed | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=ERF(#REF!,TRUE)` | as typed | `#REF!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `DELTA(A1, A2)` | number1 `#N/A`; number2 `#REF!` (cells) | `#REF!` | Excel 16.0.20527.20088, Windows, bitness and CPU not recorded in this reading file |
| `=DELTA(B1,0)` | B1 `TRUE` (cells) | `#VALUE!` | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `=DELTA(B1,0)` | B1 `(empty cell)` (cells) | `1` (`0x3ff0000000000000`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): 16 recorded DELTA and GESTEP formula readings, 8 readings each with errors
and text held in cells, and the ERF companions on the same sheets. GESTEP gave the same pattern
as DELTA in every pair.

Evidence:

- DnaCalc `apps/dna-onecalc/tests/acceptance.rs`, delivered_fn4_scalar_logical_positions_and_error_order at `287c5b9a3b` — recorded answers pinned
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheets qa001 to qa008, qa013 to qa020, qa109 to qa116 at `2d0eddcac8` — recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/DELTA.reading.json` at `2d0eddcac8` — 8 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/GESTEP.reading.json` at `2d0eddcac8` — 8 recorded readings
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa007, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa008, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa019, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa020, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa115, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa116, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/DELTA.reading.json`, fn7-DELTA-argument-order-7 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa003, row 20, column 10 at `2d0eddcac8` (readings)
- DnaCalc `crates/calc-func/tests/captures/fn4_questions/functions-fn4-arguments.capture.json`, sheet qa004, row 20, column 10 at `2d0eddcac8` (readings)

### FN-ERF.PRECISE-001: Below 0.5, ERF is a stored (4, 5) rational from a table not yet identified

*Detail for matching Excel's bits; measured; recorded 2026-11-04.* Applies to ERF, ERF.PRECISE, ERFC, ERFC.PRECISE, GAUSS.

With RN(v) meaning v rounded to 64 bits on the x87 unit and then stored to binary64,
ERF.PRECISE for |x| < 0.5 is x*N(t)/D(t) with t = RN(x*x), p = RN(x*N), y = RN(p/D), and N and
D Horner sums in t in which every product and every sum is RN. N has degree 4 and D degree 5,
and the coefficients read off Excel's answers, order 0 upward, are N: 0x3ff1df5e70c7956b,
0x3fc201d8627a7d2c, 0x3fa6f289d165b303, 0x3f5d93a5813e372c, 0x3f2932767df264b2; D:
0x3fefada15778ece6, 0x3fdd191aca8d2499, 0x3fb79f85ba68803e, 0x3f856de5da86af12,
0x3f4609751b5b6488, 0x3ef4f20a473a5ab0. The rational stops at 0.5 itself. It equals erf to a
flat 9E-17 up to 0.5, and its error climbs at once beyond, so 0.5 is its design interval and
not Cody's 0.46875. No published table has been matched: Cody's 1969 rational is refuted by its
linear terms, and Cephes has the same shape with different ratios. Evidence record
EV-DIST-0026, which predates this note, attributes ERF.PRECISE and ERFC.PRECISE to the a < 1
branches of the NSWC GRATIO routine and says no coefficient table exists; below 0.5 this note
replaces that attribution. Because every step rounds, answers need not be correctly rounded: of
the 654 recorded rows of build 20131 in [0.25, 0.5), 400 differ from the correctly rounded erf,
by up to 3 units, and small arguments can be 1 unit high, as below. ERFC and ERFC.PRECISE below
0.5 are 1 less this value, and GAUSS at tiny arguments takes half of it at its stored
z/sqrt(2).

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `ERF.PRECISE(0.4524739583333333)` | x `0.4524739583333333` (`0x3fdcf55555555555`) (cells) | `0.4777590107351978` (`0x3fde939a879e86f6`) | `0.4777590107351976` (`0x3fde939a879e86f3`) | Excel 16.0 build 20131, platform and bitness not recorded, AMD Ryzen 7 4800H |
| `ERF.PRECISE(9.094947017729282e-13)` | x `9.094947017729282e-13` (`0x3d70000000000000`) (cells) | `1.0262548740643186e-12` (`0x3d720dd750429b6e`) | `1.0262548740643184e-12` (`0x3d720dd750429b6d`) | Excel 16.0 build 20131, platform and bitness not recorded, AMD Ryzen 7 4800H |
| `ERF.PRECISE(9.999999999999998e-11)` | x `9.999999999999998e-11` (`0x3ddb7cdfd9d7bdb9`) (cells) | `1.1283791670955125e-10` (`0x3ddf044332d68160`) | `1.1283791670955123e-10` (`0x3ddf044332d6815f`) | Excel 16.0 build 20131, platform and bitness not recorded, AMD Ryzen 7 4800H |

Correct value: mpmath erf at 800 bits from the binary64 input, correctly rounded.

Basis (measured): All 618,225 recorded ERF.PRECISE rows with |x| < 0.5 match; they were only
scored, never used to read the coefficients, and ERFC and ERFC.PRECISE match every recorded row
in that range. Two fresh sets of 60,000 random doubles below 0.5 (Excel build 20527) match
59,996 and 59,982; the second was asked after the constants were fixed, and 17 of its 18 misses
are in [0.25, 0.5). With every step stored, 223 of 38,000 rows at 19 sites stay unexplained;
the other store placements leave 568 to 1,244. 2,000 consecutive doubles below 0.5 match 1,988,
and 2,000 from 0.5 up match 858, chance level, which places the cut.

Inferred, not measured: That the legacy ERF(x) shares this body is inferred: ERF.PRECISE, ERFC
and ERFC.PRECISE rows were scored in full, and for ERF itself the record holds only nine
non-zero single-argument inputs below 0.5 (build 19929), all equal to the rational.

- Open: The last bits of the coefficients of order 3 and above. Orders 0 and 1 are exact: one unit off in a linear term breaks 500 to 1,100 of 160,000 rows. Order 2 sits at the minimum of the same scan. Higher orders are good to the level that shows, about 0.15% of rows above 0.25.
- Open: The source table. Hart's ERF tables remain unchecked.
- Open: Which branch owns x = 0.5: there the rational and a correctly rounded erf give the same bits.
- Open: Subnormal arguments: Excel reads a subnormal written to a cell back as 0, so they cannot be asked that way.

Evidence:

- [`content/notes/evidence/2026-11-04/erf-EVIDENCE-round3.md`](../../content/notes/evidence/2026-11-04/erf-EVIDENCE-round3.md) — research write-up: how the rational was read, the checks, the final constants
- [`content/notes/evidence/2026-11-04/erf-EVIDENCE-round2.md`](../../content/notes/evidence/2026-11-04/erf-EVIDENCE-round2.md) — earlier round: the structure, the refutation of Cody, the edge bisection
- [`content/notes/evidence/2026-11-04/erf-EVIDENCE.md`](../../content/notes/evidence/2026-11-04/erf-EVIDENCE.md) — first write-up; its Cody attribution is withdrawn
- DnaCalc `crates/calc-func/src/ported/normal_dist_common.rs`, erf_approx, ERF_N, ERF_D at `36a7f3d7aa` — the rational in the product; the commit message carries the row counts
- CalcFunc-Evidence `cache/oracle/build-20131/ERF.PRECISE.jsonl.gz`, line 1219, key ERF.PRECISE|0x3fdcf55555555555 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `cache/oracle/build-20131/ERF.PRECISE.jsonl.gz`, line 2115, key ERF.PRECISE|0x3d70000000000000 at `4d5320dfdc` (readings)
- CalcFunc-Evidence `cache/oracle/build-20131/ERF.PRECISE.jsonl.gz`, line 209, key ERF.PRECISE|0x3ddb7cdfd9d7bdb9 at `4d5320dfdc` (readings)

<!-- efh:notes end -->

## Sources

- [Microsoft: ERF function](https://support.microsoft.com/en-us/excel/functions/erf-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/normal_dist_common.rs); implementation steps and the limits identified in the text.
- [ERF research and holdout results](../notes/evidence/2026-11-04/erf-EVIDENCE-round3.md).
