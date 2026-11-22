---
schema: efh.function-page/v1
function_id: FUNC.PRICE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
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
  - Notes
  - Excel versions, platforms and CPUs
  - Sources
family: null
role_in_family: null
---

# PRICE

## What it does

`PRICE(settlement,maturity,rate,yld,redemption,frequency,[basis])` returns a coupon security’s clean price per 100 face value from an annual yield. Rate is the annual coupon rate; frequency is one, two or four payments per year.

## Try it

`=PRICE(DATE(2027,1,1),DATE(2028,1,1),0.06,0.06,100,2,0)` is about `100`. Settlement is on a coupon date, the annual coupon and yield are both six percent, and redemption is par.

`=PRICE(DATE(2027,4,1),DATE(2027,7,1),0.06,0.08,100,2,0)` is about `99.4803921569`. Only one coupon remains. The clean price subtracts 1.5 of accrued coupon interest from a dirty price about 100.9803921569.

## Quirks

Basis defaults to zero: 0 US 30/360, 1 actual/actual, 2 actual/360, 3 actual/365, 4 European 30/360. Dates and selectors are truncated under their documented rules. Use DATE values rather than locale-dependent date text. Settlement must precede maturity; negative coupon rate, invalid frequency/basis or nonpositive redemption gives `#NUM!`. The documented PRICE domain rejects negative yield. Clean price excludes accrued coupon interest.

## How to implement

The current code keeps coupon scheduling, the settlement fraction, discount powers and accumulation as separate stages. Unless R is named, the arithmetic below is ordinary binary64 with a store after each operation.

1. Truncate and validate dates, frequency and basis. Build coupon dates backward from maturity in steps of `12/frequency` months, retaining the source’s month-end rules. Stop with previous coupon <= settlement < next coupon and count N remaining coupons.
2. Set `C=(100*rate)/frequency`. Obtain coupon denominator E and accrued span A from the selected day-count helpers. The current denominator is 360/frequency for bases 0,2,4, 365/frequency for basis 3 and the actual coupon-period length for basis 1.
3. Set the **internal** remaining span `DSC=E-A`, then `off=DSC/E`. Do not replace it with a fresh day count from settlement to next coupon. In particular, public COUPDAYSNC at bases 2 and 3 can differ from this derived complement; the retained coupon note records that discrepancy.
4. If N=1, compute `den=1+(yld/frequency)*(DSC/E)`, dirty price `(redemption+C)/den`, and clean price `dirty-(C*A)/E`. Validate the positive denominator.
5. If N>1, form `base=1+yld/frequency`. In forward coupon order, accumulate `C/discount(base,off+k)` for k=0 through N-1. Add redemption as a **separate** final term divided by `discount(base,off+N-1)`. Finally subtract accrued `(C*A)/E`.

The current discount helper uses square-and-multiply for a nonnegative integer exponent below 1024, with binary64 multiplication after each selected product and square. For other exponents it obtains stored public LN(base), forms `R(exponent*LN(base))`, then calls public EXP on that stored product. This is distinct from blindly calling a host pow or substituting the public POWER wrapper’s root shortcuts.

For the short-bond example, C=3, E=180, A=90 and DSC=90. The denominator is `1+(0.08/2)*0.5=1.02`. Dirty price is `103/1.02`, about 100.9803921569; accrued is `3*90/180=1.5`, giving clean price about 99.4803921569.

The calendar helpers include a credited, changed translation of ExcelFinancialFunctions; the project’s pricing stages are separately constrained by recorded answers. The source retains 25 integer-discount controls over five bonds and two 315-row fractional-discount holdouts on their stated scopes. Calendar seams, unmeasured route boundaries, LN/EXP internals and remaining numerical mismatches stay open. [YIELD](FUNC.YIELD.md) currently retains a different forward-power path for its solver.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20131 | Windows, bitness not recorded, AMD Ryzen 7 4800H with Radeon Graphics | 8,417 |
| 20228 | Windows, 64-bit, CPU not recorded | 208 |

- XC-PRICE-001: build 20131 versus build 20228; 2 same-input pairs, 0 differing answers. Compared axes: excel-build. Numeric answers were compared by their recorded binary64 bits; other answers by typed value.

Agreement on these compared inputs does not establish agreement on unmeasured inputs.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.PRICE.json).

## Alternatives

[YIELD](FUNC.YIELD.md) performs the inverse calculation; [ODDFPRICE](FUNC.ODDFPRICE.md) and [ODDLPRICE](FUNC.ODDLPRICE.md) handle irregular coupons.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `PRICE` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-PRICE-001: PRICE counts coupons n periods at once from maturity, uses derived dsc = E - A, and splits integer vs fractional powers

*Detail for matching Excel's bits; measured; recorded 2026-11-22.*

In Excel 16.0.20131 through 16.0.20527, PRICE(settlement, maturity, rate, yld, redemption,
frequency, [basis]) steps n coupon periods backward from maturity in one call (keeping the
29th/30th day instead of clamping February 28 into month-ends), sets dsc = E - A rather than
the direct day count (diverging from COUPDAYSNC on bases 2 and 3), uses x87-stored binary
exponentiation for integer exponents and exp(RN53(RN64(exp * ln(base)))) for fractional
exponents, and divides redemption by its discount in x87. Of 9,153 full-bank rows, 9,103 match
and 50 differ by 1 ULP.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `PRICE(43861, 47118, 0.04, 0.0001, 99.5, 1, 2)` | settlement `43861.0` (`0x40e56aa000000000`); maturity `47118.0` (`0x40e701c000000000`); rate `0.04` (`0x3fa47ae147ae147b`); yld `0.0001` (`0x3f1a36e2eb1c432d`); redemption `99.5` (`0x4058e00000000000`); frequency `1.0` (`0x3ff0000000000000`); basis `2.0` (`0x4000000000000000`) (cells) | `135.04922273876548` (`0x4060e1933b90a6f4`) | Excel 16.0.20131, Windows (bitness not recorded), AMD Ryzen 7 4800H with Radeon Graphics |
| `PRICE(43890, 45658, 0.04125, 0.049, 101.5, 2, 1)` | settlement `43890.0` (`0x40e56e4000000000`); maturity `45658.0` (`0x40e64b4000000000`); rate `0.04125` (`0x3fa51eb851eb851f`); yld `0.049` (`0x3fa916872b020c4a`); redemption `101.5` (`0x4059600000000000`); frequency `2.0` (`0x4000000000000000`); basis `1.0` (`0x3ff0000000000000`) (cells) | `97.87894757525414` (`0x40587840ad54a75d`) | Excel 16.0 build 20228, Windows 64-bit; CPU not recorded |
| `=PRICE(39493,43054,0.0575,0.065,100,2,0)` | as typed | `94.63436162132213` (`0x4057a899617c5aa1`) | Excel 16.0.20527.20088 Current Channel, Windows 64-bit (NT 10.0.26200), x86-64, AMD Ryzen 7 4800H, en-ZA |

Basis (measured): Measured across Excel 16.0.20131 and 16.0.20228 (9,103/9,153 full-bank and
120/120 pinned PRICE rows) and 16.0.20527.20088 in prefix_metadata (row 381).

- Open: 50 PRICE full-bank 1-ULP last-bit rows remain open (dc-j9k.62).

Evidence:

- DnaCalc `crates/calc-func/src/ported/bond_core_family.rs`, lines 267-363 and 490-502 at `f8f4e0aa00` — n-period coupon walk, dsc=E-A, and excel_bond_pow staging
- DnaCalc `crates/calc-func/witness-reference/PRICE.tsv` at `f8f4e0aa00` — 50 remaining full-bank 1-ULP PRICE rows
- DnaCalc `crates/calc-func/witness-status.tsv`, PRICE at `f8f4e0aa00` — 9,103/9,153 full-bank and 120/120 pinned PRICE rows matched
- DnaCalc `crates/calc-func/tests/witnesses/PRICE.json`, oc7905 at `f8f4e0aa00` (readings)
- DnaCalc `crates/calc-func/tests/witnesses/PRICE.json`, price-yield-companion-d-leap-b1-f2-002 at `f8f4e0aa00` (readings)
- DnaCalc `crates/calc-func/tests/captures/catalogue/sections/prefix_metadata.capture.json`, sheet prefix_metadata row 381 column 1 at `f8f4e0aa00` (readings)

<!-- efh:notes end -->

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.PRICE.json; edits here are overwritten -->

Readings of PRICE with a recorded Excel build: 8,625 input(s) over 2 recorded environment(s),
Excel 16.0 build(s) 20131, 20228, all on Windows; recorded CPUs: AMD Zen 2. 8,417 input(s) were
also read in captures that record no build (some may repeat inputs counted above); they are not
counted. 1 same-input comparison(s) between environments over 2 input-pair(s): no result
differs. A CPU dependence is expected and untested (see below). No reading from a Mac, Excel
for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `PRICE` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 20131 | Windows, bitness not recorded | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 8,417 | 2026-07-19 |
| 16.0 build 20228 | Windows, 64-bit | CPU not recorded | 208 | 2026-08-09 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-PRICE-001 | excel-build | 16.0 build 20131 vs 16.0 build 20228 | 2 | 0 |

Differences observed: none.

Expected, not yet tested (XS-PRICE-001, cpu): Handbook evidence record EV-FIN-0022 marks 2 of
its counts for this function as CPU-scoped: the counted result depends on how the host CPU
executes x87 instructions. Every reading recorded here was taken on an AMD Zen 2 processor or
on a host whose CPU was not recorded; no reading on an Intel CPU, a Mac or Excel for the web
exists, so a CPU difference is neither observed nor ruled out.

Evidence for every row: [`content/evidence/environments/FUNC.PRICE.json`](../../content/evidence/environments/FUNC.PRICE.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Sources

- [Microsoft: PRICE function](https://support.microsoft.com/en-us/excel/functions/price-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/bond_core_family.rs); implementation steps and the limits identified in the text.
- [Stored LN/product/EXP discount chain](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/mod.rs); `excel_pow_chain`, separate from runtime pow.
- [COUPDAYSNC’s retained internal/public difference](FUNC.COUPDAYSNC.md); the published coupon-day quantity is not an unconditional replacement for PRICE’s internal complement.
