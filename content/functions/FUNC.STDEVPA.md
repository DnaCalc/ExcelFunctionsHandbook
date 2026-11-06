---
schema: efh.function-page/v1
function_id: FUNC.STDEVPA
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — STDEVPA function"
    locator: "https://support.microsoft.com/en-us/office/stdevpa-function-5578d4d6-455a-4308-9991-d405afe2c28c"
    role: "documented signature, the n divisor, the text-as-zero and TRUE-as-one inclusion rules, and the #DIV/0! condition"
  - work: "Chan, T. F., Golub, G. H. and LeVeque, R. J., \"Algorithms for Computing the Sample Variance: Analysis and Recommendations\""
    locator: "The American Statistician 37(3), 1983, pp. 242-247"
    role: "the error analysis of the textbook one-pass formula against the two-pass and updating forms"
  - work: "OxFunc — aggregate_common.rs and variance_common.rs"
    locator: "crates/oxfunc_core/src/functions/aggregate_common.rs, crates/oxfunc_core/src/functions/variance_common.rs"
    role: "reference-engine kernel: the AVERAGEA-like inclusion policy and the shared two-pass population variance"
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
family: stdevpa_fn
role_in_family: >-
  STDEV.P with the A-variant inclusion rule: the same n-divisor population estimator over a data
  set that additionally admits text as zero and logicals as one and zero.
---

# STDEVPA

## What it does

`STDEVPA(value1, [value2], ...)` measures population standard deviation. It reports spread in the original units by taking the square root of variance. The divisor is `n`: use this when the observations constitute the whole population. This A-family function also admits logical values and text under its inclusion rules.

## Try it

Put these four entries in `B2:B5`:

| Cell | Value |
|---|---|
| B2 | `4` |
| B3 | `8` |
| B4 | logical `TRUE` |
| B5 | text `missing` |

`=STDEVPA(B2:B5)` returns `3.112474899497183`. The admitted values are `4,8,1,0`. Their squared deviations total `38.75`; divide by `4`, then take the staged square root.

Compare `=STDEVPA({4,8,TRUE,"missing"})`: the current array rule skips the logical and text elements, leaving only `4,8`, and returns `2`. The range and array forms therefore use different counts.

## Quirks

Keep the argument's origin until collection is complete:

| Origin | Logical value | Text | Empty cell |
|---|---|---|---|
| Direct scalar | `TRUE` is 1, `FALSE` is 0 | Numeric text converts; other text gives `#VALUE!` | Ordinary scalar coercion |
| Reference cell | 1 or 0 | Counted as 0, including numeric-looking text | Skipped |
| Array element | Skipped | Skipped | Skipped |

These are the current implementation rules. Retained Excel build 20527 controls distinguish the array route; the source records, for example, `VARPA({1,TRUE,3})=1` and `STDEVPA({1,"a",3})=1`. They do not establish every collection edge on every build. Microsoft’s remarks describe broad inclusion, then also say array/reference text is ignored; that wording does not settle the observed range/array distinction.

No admitted values gives `#DIV/0!`. One admitted value has zero population variance. Worksheet errors propagate. Changing the collection rule changes the count as well as the total. A centered mathematical formula alone does not reproduce the observed numerical path.

## How to implement

Apply the collection rules above, then use the [stored variance recipe](FUNC.VAR.S.md#how-to-implement). This function uses the **population** branch, with final divisor `n` and the staged square root.

For the reference example, `n=4`, `S=13`, `Q=81` and `T=R(S*S)=169`. The population numerator is `R(n*Q)-T=324-169=155`; the denominator is `R(n*n)=16`, giving variance `9.6875`. Its staged square root is `3.112474899497183`. Neither centered-pass guard fires: the cancellation ratio is `42.25/81`, below `0.99`, and the variance is too large for the mean-size test. These integer and quarter-valued intermediate steps are exact; the division by three and any square root still use the prescribed stores.

The same two guards can redirect this call to the centered pass. For `1000,1001,1002`, that pass sums `1+0+1=2` and divides by `3`. The result then takes the staged square root. The exact scalar guard edges are measured; their transfer to references and arrays remains open. Nonfinite raw-moment fallback is inferred. No Handbook vector suite is published for this function.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20430 | Windows, 64-bit, CPU not recorded | 1,212 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.STDEVPA.json).

## Alternatives

Use [STDEVA](FUNC.STDEVA.md) for the sample version with the same broad collection policy. [STDEV.P](FUNC.STDEV.P.md) skips referenced logical values and text. [VARPA](FUNC.VARPA.md) returns the corresponding population variance.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `STDEVPA` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-VAR.S-001: The variance family uses raw moments with stored steps, and two tests switch it to a centered sum

*Excel differs from the mathematical answer; measured; recorded 2026-11-04.* Applies to STDEV, STDEV.P, STDEV.S, STDEVA, STDEVP, STDEVPA, VAR, VAR.P, VAR.S, VARA, VARP, VARPA.

With RN(v) meaning v rounded to 64 bits on the x87 unit and then stored to binary64, S the
running RN sum of the n values and Q the running RN sum of RN(x*x), the sample variance is
RN(RN(Q - RN(RN(S*S)/n))/(n-1)) and the population variance RN(RN(RN(n*Q) - RN(S*S))/RN(n*n)).
Excel then uses a centered sum instead, adding RN(d*d) with d = RN(x - mean) and dividing by
n-1 or n, when either of two stored tests holds: RN(RN(RN(S*S)/n)/Q) > 0.99, or
RN(variance*1000) <= |RN(S/n)|. The STDEV functions are the root of this, rounded to 64 bits
and then stored. So the answer depends on which path a call takes: the first two VAR.S calls
below differ by a few input steps in two arguments, the first passes the 0.99 test and is
correctly rounded, and the second takes the raw moments and lands 76 units high. The next pair
shows the size test: scaling the inputs by 4 leaves the first path.

| Call | Inputs (binary64) | Excel | Correct value | Recorded in |
|---|---|---|---|---|
| `VAR.S(0.8769085090206681, 1, 1.123091490979332)` | number1 `0.8769085090206681` (`0x3fec0fa26efa7e14`); number2 `1` (`0x3ff0000000000000`); number3 `1.123091490979332` (`0x3ff1f82ec882c0f6`) (cells) | `0.01515151515151495` (`0x3f8f07c1f07c1e94`) | — | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `VAR.S(0.8769085090206679, 1, 1.1230914909793321)` | number1 `0.8769085090206679` (`0x3fec0fa26efa7e12`); number2 `1` (`0x3ff0000000000000`); number3 `1.1230914909793321` (`0x3ff1f82ec882c0f7`) (cells) | `0.015151515151515138` (`0x3f8f07c1f07c1f00`) | `0.015151515151515006` (`0x3f8f07c1f07c1eb4`) | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `VAR.S(3.244042371786199e-10, 0.00034050419142641947)` | number1 `3.244042371786199e-10` (`0x3df64afacd3a935a`); number2 `0.00034050419142641947` (`0x3f3650b65dc7a1d6`) (cells) | `5.797144172853e-08` (`0x3e6f1f88c6bbc51a`) | — | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `VAR.S(1.2976169487144796e-09, 0.0013620167657056779)` | number1 `1.2976169487144796e-09` (`0x3e164afacd3a935a`); number2 `0.0013620167657056779` (`0x3f5650b65dc7a1d6`) (cells) | `9.275430676564799e-07` (`0x3eaf1f88c6bbc519`) | `9.2754306765648e-07` (`0x3eaf1f88c6bbc51a`) | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |
| `=VAR.S({157,376.82,189.207,12.38,12.473,483,306.24,324.1852,482.235,78})` | as typed | `31907.480453381795` (`0x40df28debfbf8a84`) | `31907.480453381777` (`0x40df28debfbf8a7f`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |
| `STDEV.S(0, 1, 1.031052478719249)` | number1 `0` (`0x0000000000000000`); number2 `1` (`0x3ff0000000000000`); number3 `1.031052478719249` (`0x3ff07f30e248f14c`) (cells) | `0.5865198874587454` (`0x3fe2c4c55ae2d85e`) | — | Excel 16.0 build 20527, Windows; bitness and CPU not recorded in this reading |

Correct value: Exact rational variance of the binary64 inputs, correctly rounded.

Basis (measured): All 14,679 recorded rows of the twelve functions match: VAR.S, VAR.P, VAR,
VARP, VARA, VARPA, STDEV.S, STDEV.P, STDEV, STDEVP, STDEVA and STDEVPA. Six of those rows
separate the stored steps from plain binary64. 910 fresh scalar variance readings (Excel build
20527) replay exactly in rational arithmetic. Four cancellation edges were bisected to adjacent
inputs; sixteen triples chosen to separate the stored ratio from 0.99*Q and from a division by
0.99 all agree with the ratio, where 0.99*Q misses all 16 and the division 14. Seven of eight
magnitude brackets reach adjacent inputs, and sixteen chosen roots all need the 64-bit root
followed by a store.

Inferred, not measured: These tests were located with scalar arguments in separate cells; that
the same edges hold for ranges and arrays is inferred, and their readings have not been taken.
RN(variance*1000) <= |mean| is not separated from the rearrangement variance <= |mean|/1000.
That a raw moment which overflows also takes the centered sum is inferred from recorded answers
that stay finite.

- Open: Whether the two tests keep the same edges for ranges and arrays.

Evidence:

- DnaCalc `crates/calc-func/src/ported/variance_common.rs`, variance_from_values at `9c23b24e8d` — the rules in the product; the commit message carries the row counts
- DnaCalc `crates/calc-func/tests/f1_stats.rs`, variance_cancellation_guard_stores_the_ratio at `9c23b24e8d` — recorded answers pinned
- DnaCalc `crates/calc-func/witness-status.tsv` at `af74247e1e` — rows matched per function
- DnaCalc-Research `evidence/round25/variance-published-canonical-scope-receipt.json` at `a547d79ed6` — the 14,679 rows before and after
- DnaCalc `crates/calc-func/tests/f1_stats.rs`, variance_cancellation_guard_keeps_the_observed_edge at `9c23b24e8d` (readings)
- DnaCalc `crates/calc-func/tests/f1_stats.rs`, variance_centered_guard_uses_the_absolute_mean at `9c23b24e8d` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/dcj9k_statistics.capture.json`, sheet dcj9k_statistics, row 3, column 4 at `c9f9d35416` (readings)
- DnaCalc `crates/calc-func/tests/f1_stats.rs`, stdev_stores_the_extended_square_root at `9c23b24e8d` (readings)

<!-- efh:notes end -->

## Sources

- [Microsoft: STDEVPA function](https://support.microsoft.com/en-us/office/stdevpa-function-5578d4d6-455a-4308-9991-d405afe2c28c), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/variance_common.rs); implementation steps and the limits identified in the text.
- [DnaCalc collection rules](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/catalogue/aggregate.rs), `Held::Values` and `number_of`; [retained array controls](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/tests/f1_stats.rs), `variances`.
- [FN-VAR.S-001](../notes/FN-VAR.S-001.json): guard bisections, stored arithmetic and unresolved scope.
