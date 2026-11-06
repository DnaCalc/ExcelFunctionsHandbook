---
schema: efh.function-page/v1
function_id: FUNC.LN
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-MATH-0001
open_problems: []
references: []
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
family: ln_fn
role_in_family: >-
  Sole member of its module; the natural logarithm, and the kernel the rest of the log family
  is defined through — LOG divides by it and LOG10 is its scaled companion.
---

# LN

## What it does

`LN(number)` returns the natural logarithm of a positive number. It answers the question “what exponent of `e` gives this number?” Microsoft documents it as the inverse of EXP. Use it for multiplicative growth, log returns and transformations of positive data.

## Try it

`=LN(8)` is about `2.079441541679836`. Dividing by `LN(2)` converts that exponent to base two: `=LN(8)/LN(2)` is about `3`. `=LN(1)` returns zero.

For a growth factor of `1.25`, `=LN(1.25)` gives a continuously compounded rate of about `0.2231435513142098`. These are rounded mathematical examples; they do not promise a particular last bit on every machine.

## Quirks

The documented domain is positive real numbers. The current calculation code returns `#NUM!` for zero or a negative value; Microsoft's opened function page does not specify that error code. Numeric conversion and elementwise array handling happen outside the logarithm kernel.

If you form `LN(1+x)` for a tiny `x`, the addition may already have rounded to `1`, so LN receives exactly one and returns zero. A more accurate logarithm cannot recover a perturbation lost before the call. PMT uses a separate small-rate series for this reason.

Recorded answers differ between two machines for some identical inputs; their Excel builds and operating systems also differ. A host-native logarithm is therefore an implementation choice with an environment scope. The old battery also admits a subnormal directly to the kernel while a separate Excel input route flushed that value to zero: those are different input paths, not an established contradiction in LN itself.

## How to implement

The current compatibility implementation evaluates an x87 instruction chain, rather than calling the language's `log`:

1. Convert the accepted binary64 input exactly to an extended value with a 64-bit significand. Reject `x<=0` at the worksheet surface; return zero for `x=1`.
2. Load the resident extended constant `ln(2)`. In the source model it is the exact binary value `0xB17217F7D1CF79AC * 2^-64`; do not replace it with a binary64 constant.
3. Evaluate the modeled `FYL2X` instruction with that constant and `x`, meaning the instruction's approximation to `ln(2)*log2(x)`. Keep the instruction result extended until the final assignment.
4. Store once to binary64 with round-to-nearest, ties-to-even. Apply the worksheet error and input policies outside this kernel.

For `x=1`, step 1 returns `0` without a transcendental call. For `x=8`, step 3 uses `log2(8)=3`; the result is near `3*ln(2)`, giving about `2.079441541679836` after storage.

This specifies the chain but leaves an instruction model to implement. The product has native x87 and software backends; the CPU's approximation and the software model are not justified by substituting an ideally rounded mathematical logarithm. The fingerprint work identifies constants within declared instruction models on measured discriminants. Private instruction internals, unmeasured input regions and agreement across CPUs remain outside that result.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 19929 | Windows, bitness not recorded, CPU not recorded | 45 |
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 349 |
| 18429 | Windows, 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz | 52 |
| 20131 | Windows, 64-bit, AMD Ryzen 7 4800H with Radeon Graphics | 52 |
| 20228 | Windows, 64-bit, CPU not recorded | 90 |
| 20430 | Windows, 64-bit, CPU not recorded | 1,227 |

- XC-LN-001: build 19929 versus build 20430; 2 same-input pairs, 0 differing answers. Compared axes: excel-build. Numeric answers were compared by their recorded binary64 bits; other answers by typed value.
- XC-LN-002: build 20131 versus build 18429; 52 same-input pairs, 17 differing answers. Compared axes: cpu, excel-build, os. Numeric answers were compared by their recorded binary64 bits; other answers by typed value.

The differing machines changed Excel build and other recorded environment properties together. These observations do not isolate a build-only or CPU-only cause. The linked record retains each input, answer, and recheck status.

A recorded example, XD-LN-001: the input `3.677168983357276e-222` has bits `0x11f544676cf1bf2f`. On 2026-07-03, build 20131 on AMD Ryzen 7 4800H with Radeon Graphics returned `-509.8717474865596` (`0xc07fddf2ad7e124d`). On the other machine, 2026-07-05, build 18429 on Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz returned `-509.87174748655957` (`0xc07fddf2ad7e124c`). These answers differ by one unit in the last place. Each machine ran this probe once; no repeat is recorded. The input was placed in a cell through `Range.Value2` and its round trip was checked, rather than entered as a rounded decimal formula literal.

Agreement on these compared inputs does not establish agreement on unmeasured inputs.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.LN.json).

## Alternatives

Use [LOG](FUNC.LOG.md) for an explicit base, [LOG10](FUNC.LOG10.md) for base ten, and [IMLN](FUNC.IMLN.md) for a principal complex logarithm. [EXP](FUNC.EXP.md) converts a natural-log result back to a positive scale. For tiny financial rates, see the separate series in [PMT](FUNC.PMT.md) before constructing `1+rate`.

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.LN.json; edits here are overwritten -->

Readings of LN with a recorded Excel build: 1,815 input(s) over 6 recorded environment(s),
Excel 16.0 build(s) 18429, 19929, 20131, 20228, 20430, all on Windows; recorded CPUs: AMD Zen
2, Intel Coffee Lake. 349 input(s) were also read in captures that record no build (some may
repeat inputs counted above); they are not counted. 2 same-input comparison(s) between
environments over 54 input-pair(s): 17 differ. A CPU dependence is observed (see below). The
Last Bit probes read LN on an Intel Core i7-9750H (Excel build 18429) and an AMD Ryzen 7 4800H
(Excel build 20131): 17 input(s) give results one unit apart in the last place; CPU, Excel
build and OS all differ between the two machines. No reading from a Mac, Excel for the web or
32-bit Excel is recorded.

Readings of `LN` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 45 | date not recorded |
| 16.0 build 20131 | Windows, bitness not recorded | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 349 | 2026-07-21 to 2026-07-23 |
| 16.0 build 18429 | Windows, 64-bit | Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | 52 | 2026-07-05 |
| 16.0 build 20131 | Windows, 64-bit | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 52 | 2026-07-03 |
| 16.0 build 20228 | Windows, 64-bit | CPU not recorded | 90 | 2026-08-09 |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 1,227 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-LN-001 | excel-build | 16.0 build 19929 vs 16.0 build 20430 | 2 | 0 |
| XC-LN-002 | cpu, excel-build, os | 16.0 build 20131 vs 16.0 build 18429 | 52 | 17 |

Differences observed:

| Difference | Inputs | Excel | Result | Rechecked |
|---|---|---|---|---|
| XD-LN-001 | `3.677168983357276e-222` (`0x11f544676cf1bf2f`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-509.8717474865596` (`0xc07fddf2ad7e124d`) | not-rechecked |
| XD-LN-001 | `3.677168983357276e-222` (`0x11f544676cf1bf2f`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-509.87174748655957` (`0xc07fddf2ad7e124c`) | not-rechecked |
| XD-LN-002 | `8.703064031041396e-161` (`0x1eb393c13e56f40a`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-368.5525248209083` (`0xc07708d724444078`) | not-rechecked |
| XD-LN-002 | `8.703064031041396e-161` (`0x1eb393c13e56f40a`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-368.5525248209082` (`0xc07708d724444077`) | not-rechecked |
| XD-LN-003 | `2.5665223849584096e-91` (`0x2d20badbb5556728`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-208.59269163749926` (`0xc06a12f75473f581`) | not-rechecked |
| XD-LN-003 | `2.5665223849584096e-91` (`0x2d20badbb5556728`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-208.59269163749923` (`0xc06a12f75473f580`) | not-rechecked |
| XD-LN-004 | `6.926860213817972e-91` (`0x2d3693905c941f14`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-207.59983682352345` (`0xc069f331dcfe7f08`) | not-rechecked |
| XD-LN-004 | `6.926860213817972e-91` (`0x2d3693905c941f14`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-207.59983682352342` (`0xc069f331dcfe7f07`) | not-rechecked |
| XD-LN-005 | `3.884270428873988e-85` (`0x2e68258fb8991289`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-194.362797730105` (`0xc0684b9c09fc3ba6`) | not-rechecked |
| XD-LN-005 | `3.884270428873988e-85` (`0x2e68258fb8991289`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-194.36279773010497` (`0xc0684b9c09fc3ba5`) | not-rechecked |
| XD-LN-006 | `14971650289.99094` (`0x420be30a078fed72`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `23.429424269114342` (`0x40376deebfb7f46e`) | not-rechecked |
| XD-LN-006 | `14971650289.99094` (`0x420be30a078fed72`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `23.42942426911434` (`0x40376deebfb7f46d`) | not-rechecked |
| XD-LN-007 | `3.945299342281314e+26` (`0x4574658f6970c8b6`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `61.23973724816965` (`0x404e9eafb5cc42c6`) | not-rechecked |
| XD-LN-007 | `3.945299342281314e+26` (`0x4574658f6970c8b6`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `61.23973724816964` (`0x404e9eafb5cc42c5`) | not-rechecked |
| XD-LN-008 | `5.825300971601942e+29` (`0x461d6906690fdadb`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `68.53717836388884` (`0x40512261215c415e`) | not-rechecked |
| XD-LN-008 | `5.825300971601942e+29` (`0x461d6906690fdadb`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `68.53717836388883` (`0x40512261215c415d`) | not-rechecked |
| XD-LN-009 | `1.92097381483287e+65` (`0x4d7d2f67e7585ef8`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `150.3208632972981` (`0x4062ca44831b0c3a`) | not-rechecked |
| XD-LN-009 | `1.92097381483287e+65` (`0x4d7d2f67e7585ef8`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `150.32086329729808` (`0x4062ca44831b0c39`) | not-rechecked |
| XD-LN-010 | `9.251441304921035e+75` (`0x4fb4742155e8cc52`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `174.91866133068783` (`0x4065dd65ac726cee`) | not-rechecked |
| XD-LN-010 | `9.251441304921035e+75` (`0x4fb4742155e8cc52`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `174.9186613306878` (`0x4065dd65ac726ced`) | not-rechecked |
| XD-LN-011 | `8.014279760257424e+77` (`0x501baf60a3904e63`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `179.3802770810877` (`0x40666c2b3ad75615`) | not-rechecked |
| XD-LN-011 | `8.014279760257424e+77` (`0x501baf60a3904e63`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `179.38027708108768` (`0x40666c2b3ad75614`) | not-rechecked |
| XD-LN-012 | `2.6144394572616708e+104` (`0x559d2eada7837182`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `240.42989938923228` (`0x406e0dc1bc5d2a56`) | not-rechecked |
| XD-LN-012 | `2.6144394572616708e+104` (`0x559d2eada7837182`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `240.42989938923225` (`0x406e0dc1bc5d2a55`) | not-rechecked |
| XD-LN-013 | `3.076124606476124e+231` (`0x6fffb3b90594a629`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `533.020827041705` (`0x4080a82aa75e37f7`) | not-rechecked |
| XD-LN-013 | `3.076124606476124e+231` (`0x6fffb3b90594a629`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `533.0208270417049` (`0x4080a82aa75e37f6`) | not-rechecked |
| XD-LN-014 | `1.7339261820228413e-07` (`0x3e8745b9995cbfbe`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-15.567707344427221` (`0xc02f22aa897c0721`) | not-rechecked |
| XD-LN-014 | `1.7339261820228413e-07` (`0x3e8745b9995cbfbe`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-15.56770734442722` (`0xc02f22aa897c0720`) | not-rechecked |
| XD-LN-015 | `2615289.4664142975` (`0x4143f3fcbbb376b5`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `14.776885343968463` (`0x402d8dc3ea722c2a`) | not-rechecked |
| XD-LN-015 | `2615289.4664142975` (`0x4143f3fcbbb376b5`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `14.77688534396846` (`0x402d8dc3ea722c29`) | not-rechecked |
| XD-LN-016 | `5.98342654462436e-107` (`0x29e190bf0a3bd3c8`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-244.58761154572932` (`0xc06e92cdb6ba751a`) | not-rechecked |
| XD-LN-016 | `5.98342654462436e-107` (`0x29e190bf0a3bd3c8`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-244.5876115457293` (`0xc06e92cdb6ba7519`) | not-rechecked |
| XD-LN-017 | `4.5244421323604415e-95` (`0x2c58290e4b16f55a`) | Excel 16.0 build 20131, Windows 64-bit, AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | `-217.23608955054897` (`0xc06b278e0bac5120`) | not-rechecked |
| XD-LN-017 | `4.5244421323604415e-95` (`0x2c58290e4b16f55a`) | Excel 16.0 build 18429, Windows 64-bit, Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz (Coffee Lake) | `-217.23608955054894` (`0xc06b278e0bac511f`) | not-rechecked |

- XD-LN-001: The results differ by one unit in the last place; the AMD result is the correctly rounded one (0xc07fddf2ad7e124d). Intel's result equals the idealised x87 model (0xc07fddf2ad7e124c). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-002: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0xc07708d724444077). Intel's result equals the idealised x87 model (0xc07708d724444077). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-003: The results differ by one unit in the last place; the AMD result is the correctly rounded one (0xc06a12f75473f581). Intel's result equals the idealised x87 model (0xc06a12f75473f580). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-004: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0xc069f331dcfe7f07). Intel's result equals the idealised x87 model (0xc069f331dcfe7f07). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-005: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0xc0684b9c09fc3ba5). Intel's result equals the idealised x87 model (0xc0684b9c09fc3ba5). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-006: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x40376deebfb7f46d). Intel's result equals the idealised x87 model (0x40376deebfb7f46d). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-007: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x404e9eafb5cc42c5). Intel's result equals the idealised x87 model (0x404e9eafb5cc42c5). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-008: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x40512261215c415d). Intel's result equals the idealised x87 model (0x40512261215c415d). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-009: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x4062ca44831b0c39). Intel's result equals the idealised x87 model (0x4062ca44831b0c39). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-010: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x4065dd65ac726ced). Intel's result equals the idealised x87 model (0x4065dd65ac726ced). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-011: The results differ by one unit in the last place; the AMD result is the correctly rounded one (0x40666c2b3ad75615). Intel's result equals the idealised x87 model (0x40666c2b3ad75614). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-012: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x406e0dc1bc5d2a55). Intel's result equals the idealised x87 model (0x406e0dc1bc5d2a55). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-013: The results differ by one unit in the last place; the AMD result is the correctly rounded one (0x4080a82aa75e37f7). Intel's result equals the idealised x87 model (0x4080a82aa75e37f6). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-014: The results differ by one unit in the last place; the AMD result is the correctly rounded one (0xc02f22aa897c0721). Intel's result equals the idealised x87 model (0xc02f22aa897c0720). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-015: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0x402d8dc3ea722c29). Intel's result equals the idealised x87 model (0x402d8dc3ea722c29). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-016: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0xc06e92cdb6ba7519). Intel's result equals the idealised x87 model (0xc06e92cdb6ba7519). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).
- XD-LN-017: The results differ by one unit in the last place; the Intel result is the correctly rounded one (0xc06b278e0bac511f). Intel's result equals the idealised x87 model (0xc06b278e0bac511f). Probe category: microcode. The two machines also ran different Excel builds (20131, 18429).

Observed (XS-LN-001, cpu): Handbook evidence record EV-MATH-0001 marks 6 of its counts for this
function as CPU-scoped: the counted result depends on how the host CPU executes x87
instructions. Every reading recorded here was taken on an AMD Zen 2 processor or on a host
whose CPU was not recorded; the one Intel reading (The Last Bit probes) differs from AMD on
some inputs; see the differences. OxFunc found Excel's last bit here follows the x87
F2XM1/FYL2X microcode on the hardest inputs (about 1 in 30). A dump of those x87 primitives on
an AMD Zen 2 and an AMD Zen 1 host (Excel only on the first) differed on none of 3,157 rows
(2026-08-23). The Handbook's model chapter 05 (version axes) states the same mechanism, that on
about one in two thousand of the hardest inputs the last bit tracks the microcode of the CPU
Excel runs on; The Last Bit (July 2026) measured it.

Evidence for every row: [`content/evidence/environments/FUNC.LN.json`](../../content/evidence/environments/FUNC.LN.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Sources

- [Microsoft: LN function](https://support.microsoft.com/en-us/excel/functions/ln-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/x87_chains.rs); implementation steps and the limits identified in the text.
- [Worksheet domain check](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/ln_fn.rs).
- [Extended constants and software instruction model](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/soft87.rs).
- [Fingerprint scope and instruction-model limits](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/docs/generated/numeric-fingerprints.md).
