---
schema: efh.function-page/v1
function_id: FUNC.EXP
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0001
  - EV-MATH-0008
open_problems: []
references:
  - work: "Microsoft Support — EXP function"
    locator: "https://support.microsoft.com/en-us/office/exp-function-c578f034-2c45-4c37-bc8c-329660a63abe"
    role: "documented signature, the value of e, the inverse relationship to LN, and the pointer to ^ for other bases"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions"
    locator: "chapter 4.2 (Exponential Function), 4.2.1-4.2.45"
    role: "definition, series, identities, and the classical polynomial approximations"
  - work: "Tang, Table-driven implementation of the exponential function in IEEE floating-point arithmetic (ACM TOMS, 1989)"
    locator: null
    role: "the table-driven reduce-approximate-reconstruct algorithm that most modern libms use"
  - work: "Cody & Waite, Software Manual for the Elementary Functions"
    locator: "the EXP chapter"
    role: "the classical two-part reduction x = k*ln2 + r and its error analysis"
  - work: "fdlibm (Sun Microsystems freely distributable libm)"
    locator: "e_exp.c and s_expm1.c"
    role: "the reference branch structure, and the expm1 companion that cancellation demands"
episodes: []
body_sections:
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Excel versions, platforms and CPUs
  - Sources
family: exp_fn
role_in_family: "The public natural exponential, implemented by an extended instruction chain with one final binary64 store and environment-scoped instruction leaves."
---

# EXP

## What it does

`EXP(number)` raises `e` to the supplied exponent. Microsoft documents EXP and LN as inverses; use `base^exponent` for a different base. EXP is useful when a growth model is written as an exponent, or when returning log-scale values to their original scale.

## Try it

`=EXP(0.05)` is about `1.051271096376024`. Applied to an initial balance of `200`, `=200*EXP(0.05)` is about `210.2542192752048`. The exponent is the total continuously compounded rate over the interval; it is not a count of discrete interest payments.

`=EXP(0)` returns `1`. A negative exponent gives a positive reciprocal-scale result, such as `=EXP(-1)` at about `0.3678794411714423`.

## Quirks

A finite mathematical exponential can exceed the largest representable binary64 number. The recorded `EXP(1000)` worksheet reading is `#NUM!`, which the current finite-result policy reproduces. At the opposite end, rounding can produce a subnormal or zero in the kernel; the worksheet's publication policy and input route must be considered separately.

For tiny `x`, `EXP(x)` rounds close to one. Subtracting one can lose the quantity you wanted, and may give zero. PMT's small-rate route has a separate compensated exponential-minus-one step.

Do not replace an entire composition with one exponential: the stored LN result and multiply in BINOM.DIST's all-success route, and the normalized product in GEOMEAN, are separately observed stages. The LN/EXP environment comparison also records machine-dependent answers on some inputs.

## How to implement

The current source uses an extended x87 chain. Its working precision has a 64-bit significand and extended exponent range; the final result is stored to binary64 once.

1. Load `x` exactly and multiply it, in extended precision with ties-to-even, by the resident `log2(e)` constant. The source constant is `0xB8AA3B295C17F0BC * 2^-63`. Call the extended result `t`; do not store it as a binary64 intermediate.
2. Work on `u=abs(t)`. Round `u` to the nearest integer, ties-to-even, giving `k`. Compute the extended fraction `f=u-k`, which lies between `-0.5` and `0.5`.
3. Evaluate the modeled `F2XM1(f)` instruction, add extended `1`, and scale the result by `2^k` with the modeled `FSCALE` operation. This gives an approximation to `2^u` without an intermediate binary64 assignment.
4. If `t` was negative, take the extended reciprocal of that positive result. Store the answer once to binary64, ties-to-even. Map a nonfinite published result to the worksheet error through the surface policy.

At `x=0`, `t=k=f=0`, so the fractional instruction gives zero and the reconstructed result is exactly one. For `x=1`, `t` is about `1.442695`, `k=1` and `f` about `0.442695`; doubling the fractional result reconstructs about `2.718281828459045`. Negative inputs take the reciprocal before storage, rather than storing the positive answer and dividing afterward.

`F2XM1` is an approximate instruction leaf. A portable implementation must model it or declare a different numerical target; replacing it with an ideal power-of-two leaf is contradicted by recorded broader windows. The local fingerprint labels do not identify every input region or the CPU's private implementation. The source offers native and software x87 backends, with residual and environment limits described in the cited records.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 36 |
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 734 |
| 18429 | Windows, 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz | 54 |
| 20131 | Windows, 64-bit, AMD Ryzen 7 4800H with Radeon Graphics | 54 |
| 20228 | Windows, 64-bit, CPU not recorded | 90 |
| 20430 | Windows, 64-bit, CPU not recorded | 1,612 |

- XC-EXP-001: build 19929 versus build 20430; 3 same-input pairs, 0 differing answers. Compared axes: excel-build. Numeric answers were compared by their recorded binary64 bits; other answers by typed value.
- XC-EXP-002: build 20131 versus build 18429; 54 same-input pairs, 6 differing answers. Compared axes: cpu, excel-build, os. Numeric answers were compared by their recorded binary64 bits; other answers by typed value.

The differing machines changed Excel build and other recorded environment properties together. These observations do not isolate a build-only or CPU-only cause. The linked record retains each input, answer, and recheck status.

A recorded example, XD-EXP-001: the input `1.816072400038452` has bits `0x3ffd0ea1eed55587`. On 2026-07-03, build 20131 on AMD Ryzen 7 4800H with Radeon Graphics returned `6.1476654002836915` (`0x401897359943e077`). On the other machine, 2026-07-05, build 18429 on Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz returned `6.147665400283691` (`0x401897359943e076`). These answers differ by one unit in the last place. Each machine ran this probe once; no repeat is recorded. The input was placed in a cell through `Range.Value2` and its round trip was checked, rather than entered as a rounded decimal formula literal.

Agreement on these compared inputs does not establish agreement on unmeasured inputs.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.EXP.json).

## Alternatives

Use [LN](FUNC.LN.md) for the inverse and [POWER](FUNC.POWER.md) or `^` for another base. See [GEOMEAN](FUNC.GEOMEAN.md) and [BINOM.DIST](FUNC.BINOM.DIST.md) for measured compositions that reuse EXP, and [PMT](FUNC.PMT.md) for a compensated exponential-minus-one route.

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.EXP.json; edits here are overwritten -->

Readings of EXP with a recorded Excel build: 2,580 input(s) over 6 recorded environment(s),
Excel 16.0 build(s) 18429, 19929, 20131, 20228, 20430, all on Windows; recorded CPUs: AMD Zen
2, Intel Coffee Lake. 734 input(s) were also read in captures that record no build (some may
repeat inputs counted above); they are not counted. 2 same-input comparison(s) between
environments over 57 input-pair(s): 6 differ. A CPU dependence is observed (see below). The
Last Bit probes read EXP on an Intel Core i7-9750H (Excel build 18429) and an AMD Ryzen 7 4800H
(Excel build 20131): 6 input(s) give results one unit apart in the last place; CPU, Excel build
and OS all differ between the two machines. No reading from a Mac, Excel for the web or 32-bit
Excel is recorded.

Readings of `EXP` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 36 | date not recorded |
| 16.0 build 20131 | Windows, bitness not recorded | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 734 | 2026-07-21 to 2026-07-23 |
| 16.0 build 18429 | Windows, 64-bit | Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | 54 | 2026-07-05 |
| 16.0 build 20131 | Windows, 64-bit | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 54 | 2026-07-03 |
| 16.0 build 20228 | Windows, 64-bit | CPU not recorded | 90 | 2026-08-09 |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 1,612 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-EXP-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 3 | 0 |
| XC-EXP-002 | cpu, excel-build, os | 16.0 build 20131 vs 16.0 build 18429 | 54 | 6 |

Differences observed:

| Difference | Inputs | Excel | Result | Rechecked |
|---|---|---|---|---|
| XD-EXP-001 | `1.816072400038452` (`0x3ffd0ea1eed55587`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `6.1476654002836915` (`0x401897359943e077`) | not-rechecked |
| XD-EXP-001 | `1.816072400038452` (`0x3ffd0ea1eed55587`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `6.147665400283691` (`0x401897359943e076`) | not-rechecked |
| XD-EXP-002 | `0.24258475769541302` (`0x3fcf0d047067aa41`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `1.2745392715968609` (`0x3ff464834a8f9e6e`) | not-rechecked |
| XD-EXP-002 | `0.24258475769541302` (`0x3fcf0d047067aa41`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `1.2745392715968606` (`0x3ff464834a8f9e6d`) | not-rechecked |
| XD-EXP-003 | `-0.8724294708581383` (`0xbfebeaf135acdf8d`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `0.4179349540619822` (`0x3fdabf723fe34b62`) | not-rechecked |
| XD-EXP-003 | `-0.8724294708581383` (`0xbfebeaf135acdf8d`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `0.41793495406198217` (`0x3fdabf723fe34b61`) | not-rechecked |
| XD-EXP-004 | `-123.79980266985712` (`0xc05ef32ff789928c`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `1.7156505734780084e-54` (`0x34c508a816adb8c2`) | not-rechecked |
| XD-EXP-004 | `-123.79980266985712` (`0xc05ef32ff789928c`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `1.7156505734780087e-54` (`0x34c508a816adb8c3`) | not-rechecked |
| XD-EXP-005 | `-0.36640908811018186` (`0xbfd7733f1a98fc89`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `0.6932191554814062` (`0x3fe62ed9f0381bae`) | not-rechecked |
| XD-EXP-005 | `-0.36640908811018186` (`0xbfd7733f1a98fc89`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `0.6932191554814063` (`0x3fe62ed9f0381baf`) | not-rechecked |
| XD-EXP-006 | `6.036367018871443` (`0x4018253d6552d2ce`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `418.3703390523158` (`0x407a25ece8a4620c`) | not-rechecked |
| XD-EXP-006 | `6.036367018871443` (`0x4018253d6552d2ce`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `418.3703390523157` (`0x407a25ece8a4620b`) | not-rechecked |

- XD-EXP-001: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x401897359943e076). Intel's result equals the idealised x87 model (0x401897359943e076). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-EXP-002: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x3ff464834a8f9e6d). Intel's result equals the idealised x87 model (0x3ff464834a8f9e6d). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-EXP-003: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x3fdabf723fe34b61). Intel's result equals the idealised x87 model (0x3fdabf723fe34b61). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-EXP-004: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x34c508a816adb8c3). Intel's result equals the idealised x87 model (0x34c508a816adb8c3). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-EXP-005: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x3fe62ed9f0381baf). Intel's result equals the idealised x87 model (0x3fe62ed9f0381baf). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-EXP-006: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x407a25ece8a4620b). Intel's result equals the idealised x87 model (0x407a25ece8a4620b). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).

Observed (XS-EXP-001, cpu): Handbook evidence record EV-MATH-0001 marks 6 of its counts for
this function as CPU-scoped: the counted result depends on how the host CPU executes x87
instructions. Every reading recorded here was taken on an AMD Zen 2 processor or on a host
whose CPU was not recorded; the one Intel reading (The Last Bit probes) differs from AMD on
some inputs; see the differences. OxFunc found Excel's last bit here follows the x87
F2XM1/FYL2X microcode on the hardest inputs (about 1 in 30). A dump of those x87 primitives on
an AMD Zen 2 and an AMD Zen 1 host (Excel only on the first) differed on none of 3,157 rows
(2026-08-23). The Handbook's model chapter 05 (version axes) states the same mechanism, that on
about one in two thousand of the hardest inputs the last bit tracks the microcode of the CPU
Excel runs on; The Last Bit (July 2026) measured it.

Evidence for every row: [`content/evidence/environments/FUNC.EXP.json`](../../content/evidence/environments/FUNC.EXP.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Sources

- [Microsoft: EXP function](https://support.microsoft.com/en-us/excel/functions/exp-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/x87_chains.rs); implementation steps and the limits identified in the text.
- [Overflow witness and finite-result policy](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/exp_fn.rs).
- [Extended constants and software instruction model](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/soft87.rs).
- [Fingerprint scope and broader-window counterexamples](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/numeric-fingerprints.md).
