---
schema: efh.function-page/v1
function_id: FUNC.YIELD
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

# YIELD

## What it does

`YIELD(settlement,maturity,rate,pr,redemption,frequency,[basis])` solves an annual yield from a coupon security’s clean price per 100 face value. Rate is the annual coupon rate; frequency is one, two or four payments per year.

## Try it

`=YIELD(DATE(2027,1,1),DATE(2028,1,1),0.06,100,100,2,0)` is about `0.06`. This on-coupon par bond takes the current exact par shortcut, rather than needing a generic numerical inversion.

`=YIELD(DATE(2027,4,1),DATE(2027,7,1),0.06,100,100,2,0)` is about `0.0591133005`. The final coupon is three and accrued interest is 1.5, so the clean price of 100 corresponds to an invested dirty amount of 101.5.

## Quirks

Basis defaults to zero: 0 US 30/360, 1 actual/actual, 2 actual/360, 3 actual/365, 4 European 30/360. Dates and selectors are truncated under their documented rules. Use DATE values rather than locale-dependent date text. Settlement must precede maturity; negative coupon rate, invalid frequency/basis or nonpositive redemption gives `#NUM!`. Price must be positive. The one-coupon and multiple-coupon branches use different formulas.

## How to implement

After validating dates, positive price/redemption, nonnegative coupon rate and the frequency/basis, build the coupon schedule and remaining count N. The current code uses three distinct routes, all with ordinary binary64 arithmetic.

1. For an on-coupon settlement with N>1 and both price and redemption exactly 100, return the coupon rate when its coupon amount is finite. This par shortcut has its own retained controls; it is not evidence of the general solver’s accuracy.
2. For N=1, calculate coupon `C=100*rate/frequency`, coupon denominator E, accrued span A and the **direct** settlement-to-next-coupon span S using the current day-count helpers. Return `(((redemption+C)/(price+C*A/E))-1)*frequency*E/S`, preserving the source’s left-to-right multiplication and division. This one-coupon YIELD span differs from PRICE’s use of derived `E-A` where the calendar rules distinguish them.
3. For N>1, solve the internal clean-price callback for the supplied target price. Its schedule quantities use the derived settlement complement, but its current discount leaf is the runtime `crt_pow`, whereas the public PRICE route uses its newer integer and x87-chain helper. Calling public PRICE unconditionally would also reject the negative trial yields that this solver needs to probe.

The current general solver starts at `lo=-frequency+1e-10`; an unevaluable lower endpoint is treated as infinite price. Start hi at one and double it, up to eighty times, until its internal price is no greater than target. Reject an unreachable target or propagate an unhandled evaluation error.

Bisect with ordinary arithmetic midpoint `(lo+hi)/2` for up to 2,200 iterations. Return an exact-price midpoint immediately. Otherwise move lo upward when its price is above target and hi downward when below. Stop when the midpoint can no longer lie strictly between the stored bounds, then choose the endpoint with smaller absolute price residual; a tie chooses hi. This is an arithmetic-midpoint search, distinct from the encoding-midpoint distribution inverses.

For the one-coupon example, C=3, A=90, E=180 and S=90. The first ratio is `103/101.5`; subtracting one and multiplying by `2*180/90=4` gives about `0.0591133005`.

Microsoft documents a one-coupon formula and a hundred-iteration Newton method for longer bonds. The current candidate instead uses the bisection route above. Its internal price callback, root selection, return rounding and remaining report differences are not fully identified. The pinned source’s explicit split between PRICE and YIELD is a limit to retain, rather than an algebraic identity to remove.

## Version differences

The recorded environments below distinguish a reading from a same-input comparison. Counts describe captured inputs in each environment; overlapping captures and comparisons are not totals of unique worksheet calls.

| Excel 16.0 build | Platform and CPU | Captured inputs |
|---|---|---:|
| 20228 | Windows, 64-bit, CPU not recorded | 504 |

The record contains no same-input comparison between these environments. Reading different inputs on different builds does not establish either a version change or cross-build agreement.

[Recorded environments and comparison details](https://github.com/DnaCalc/ExcelFunctionsHandbook/blob/5dc75391aa5125ccc5abdb095da85950980d6fd8/content/evidence/environments/FUNC.YIELD.json).

## Alternatives

[PRICE](FUNC.PRICE.md) performs the inverse calculation; [ODDFPRICE](FUNC.ODDFPRICE.md) and [ODDLPRICE](FUNC.ODDLPRICE.md) handle irregular coupons.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `YIELD` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

### FN-YIELD-001: YIELD near par on a coupon date returns a closed-form seed with stored partial sums, not the solved root

*Detail for matching Excel's bits; measured; recorded 2026-11-11.*

On a coupon date, with redemption 100 and more than one coupon period left, YIELD at a price
close to 100 does not return the root of the price equation. It returns the closed-form seed y
= f * ((n*cp - d) / (d*(n+1)/2 + n*R)), with n the periods left, cp = 100*rate/f the coupon per
period, R the redemption and d = price - R, whenever the Newton step (P(y) - price) / P'(y)
taken at the seed is below 1e-10; otherwise the solver runs. On the recorded 20228 and 20527
rows Excel's switch lies between steps of 8.8e-11 and 1.17e-10. The association of the seed
matters: DNA Calc forms the total coupon as S(S(S(n*rate)*100)/f) and the denominator as
S(S(n*R + S(S(d*n)/2)) + S(d/2)), each partial result stored to binary64 before the next, and
with that association 31 more full-bank rows matched (YIELD 66 to 97 of 533, none lost). The
same seed is taken at a price of exactly 100: Excel does not return the coupon rate directly
there, so DNA Calc's former shortcut for price 100 and redemption 100 was removed. Before the
seed was introduced (commit d44aaf0a) YIELD matched 16 of 533 bank rows with a worst miss of
4.4e15 units in the last place; with the plain binary64 seed it matched 66 with a worst miss of
2,844 units. Offline, that plain binary64 order reproduced Excel's bits on 64 of the 97
small-step rows with redemption 100, against at most 58 for the x87-extended orders tried.

| Call | Inputs (binary64) | Excel | Recorded in |
|---|---|---|---|
| `YIELD(44013, 44562, 0.05, 100.00001525878906, 100, 2, 0)` | settlement `44013` (`0x40e57da000000000`); maturity `44562` (`0x40e5c24000000000`); rate `0.05` (`0x3fa999999999999a`); pr `100.00001525878906` (`0x4059000040000000`); redemption `100` (`0x4059000000000000`); frequency `2` (`0x4000000000000000`); basis `0` (`0x0000000000000000`) (cells) | `0.04999989318848743` (`0x3fa9999604189993`) | Excel 16.0 build 20228, Windows 64-bit; CPU not recorded |
| `YIELD(44013, 46753, 0.05, 99.99999713897711, 100, 2, 0)` | settlement `44013` (`0x40e57da000000000`); maturity `46753` (`0x40e6d42000000000`); rate `0.05` (`0x3fa999999999999a`); pr `99.99999713897711` (`0x4058fffff4000004`); redemption `100` (`0x4059000000000000`); frequency `2` (`0x4000000000000000`); basis `0` (`0x0000000000000000`) (cells) | `0.050000004577636696` (`0x3fa99999c0ebedf7`) | Excel 16.0 build 20228, Windows 64-bit; CPU not recorded |
| `YIELD(44013, 46753, 0.05, 100.000002861023, 100, 2, 0)` | settlement `44013` (`0x40e57da000000000`); maturity `46753` (`0x40e6d42000000000`); rate `0.05` (`0x3fa999999999999a`); pr `100.000002861023` (`0x405900000c000004`); redemption `100` (`0x4059000000000000`); frequency `2` (`0x4000000000000000`); basis `0` (`0x0000000000000000`) (cells) | `0.04999999542236326` (`0x3fa9999972474536`) | Excel 16.0 build 20228, Windows 64-bit; CPU not recorded |

Basis (measured): Three recorded rows are cited (Excel build 20228, near-seed discovery set);
the unit test in DnaCalc holds five rows (three from 20228 and two price 100 + 1 ulp rows at a
zero coupon from build 20527) and DNA Calc returns Excel's recorded bits on all five. The
matched counts are quoted from the commit messages (full bank, per-row traces).

Inferred, not measured: That Excel's code forms this seed, with this association and this 1e-10
switch, is inferred; the seed form is read off the bank, and the switch only brackets between
8.8e-11 and 1.17e-10.

- Open: Rows 024 and 026 of the bank (basis-dependent) stay unmatched.
- Open: Designs with redemption other than 100 (the oncoupon-b4-f1 rows: step below 1e-10 at the seed but Excel about 450 units from it) carry a seed that is not identified.
- Open: 26 rows of the oncoupon-short design are one unit below Excel: the evaluation order of the seed is open there.
- Open: 436 YIELD bank rows in all were still unmatched when the bead comment was written.

Evidence:

- DnaCalc `crates/calc-func/src/ported/bond_core_family.rs`, near_par_seed; yield_near_par_returns_the_closed_form_seed at `d44aaf0a03` — the seed as first introduced (plain binary64 order)
- DnaCalc `crates/calc-func/src/ported/bond_core_family.rs`, near_par_seed at `530c7e4bc2` — the Y2 association and the removed price-100 shortcut
- CalcFunc-Evidence `work/w109/G6-solvers/answers-yield-near-seed-discovery-20260809.json.gz` at `b4818a6a23` — 20228 near-seed discovery answers
- DnaCalc `.beads/issues.jsonl`, dc-6332.2 at `b8738ae6a6` — item 2: YIELD 66 to 97 of 533, 31 won and 0 lost
- CalcFunc-Evidence `work/w109/G6-solvers/answers-yield-near-seed-discovery-20260809.json.gz`, yield-near-seed-discovery-d-oncoupon-short-b0-f2-030 at `b4818a6a23` (readings)
- CalcFunc-Evidence `work/w109/G6-solvers/answers-yield-near-seed-discovery-20260809.json.gz`, yield-near-seed-discovery-d-oncoupon-long-b0-f2-039 at `b4818a6a23` (readings)
- CalcFunc-Evidence `work/w109/G6-solvers/answers-yield-near-seed-discovery-20260809.json.gz`, yield-near-seed-discovery-d-oncoupon-long-b0-f2-040 at `b4818a6a23` (readings)

<!-- efh:notes end -->

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.YIELD.json; edits here are overwritten -->

Readings of YIELD with a recorded Excel build: 504 input(s) over 1 recorded environment(s),
Excel 16.0 build(s) 20228, all on Windows; none of these captures records the CPU. No input has
been read in two environments, so no difference is known and none is ruled out. A CPU
dependence is expected and untested (see below). No reading from a Mac, Excel for the web,
32-bit Excel or an Intel CPU is recorded.

Readings of `YIELD` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 20228 | Windows, 64-bit | CPU not recorded | 504 | 2026-08-09 |

No check has read the same inputs in two environments, so the absence of a listed difference says nothing about other builds, platforms or CPUs.

Differences observed: none.

Expected, not yet tested (XS-YIELD-001, cpu): Handbook evidence record EV-FIN-0022 marks 2 of
its counts for this function as CPU-scoped: the counted result depends on how the host CPU
executes x87 instructions. Every reading recorded here was taken on an AMD Zen 2 processor or
on a host whose CPU was not recorded; no reading on an Intel CPU, a Mac or Excel for the web
exists, so a CPU difference is neither observed nor ruled out.

Evidence for every row: [`content/evidence/environments/FUNC.YIELD.json`](../../content/evidence/environments/FUNC.YIELD.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Sources

- [Microsoft: YIELD function](https://support.microsoft.com/en-us/excel/functions/yield-function), opened 6 November 2026; syntax, documented argument rules and errors.
- [DnaCalc implementation](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/bond_core_family.rs); implementation steps and the limits identified in the text.
- [Stored LN/product/EXP discount chain](https://github.com/DnaCalc/DnaCalc/blob/da3ecc4fb19a3a5529708c311d0e8f05688ac165/crates/calc-func/src/ported/excel_numeric/mod.rs); `excel_pow_chain`, separate from runtime pow.
- [COUPDAYSNC’s retained internal/public difference](FUNC.COUPDAYSNC.md); the published coupon-day quantity is not an unconditional replacement for PRICE’s internal complement.
