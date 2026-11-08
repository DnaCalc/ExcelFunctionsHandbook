# Sweep Questions 1 — Excel 16.0 Build 20527 Answers

Environment: Excel 16.0 build 20527 (`16.0.20527.20150`), Windows 64-bit, AMD Ryzen 7 4800H,
Windows `en-ZA` (`DecimalSeparator = "."`, `ThousandsSeparator = "\u00a0"`,
`UseSystemSeparators = true`), 2026-11-08.

---

## Q1: Subnormal and maximum-finite inputs (`Q1-subnormal-inputs.json`)

1. **Cell-write input retention (`Range.Value2`)**:
   - Every subnormal double written via `Range.Value2` — positive or negative (`±0x0000000000000001`,
     `±3min`, `±0x000fffffffffffff`, `±0x0008000000000000`, `±0x100`, `±0xdeadbeef`) — and `-0`
     (`0x8000000000000000`) is flushed to `+0` (`0x0000000000000000`) when stored into the cell.
   - Normal extremes `0x0010000000000000` (`2^-1022`) and `±0x7fefffffffffffff` (`±DBL_MAX`) read
     back intact (`input_kept: true`).

2. **Replay of the 430 bank rows (`q1a_bank_replay`)**:
   - **430 / 430** reproduce the bank's recorded build 20430 answer (`matches_bank: true`).
   - **179 rows** had a subnormal input (`0x0000000000000001` or `0x8000000000000001`); in all 179,
     the input cell reads back as `0x0000000000000000` (`input_kept: false`), and writing `+0`
     directly gives the identical answer on all 179 rows (`matches_zero_control: true`). That is why
     `SQRT(-min) = 0` (not `#NUM!`), `SIGN(±min) = 0`, `ODD(±min) = 1`, `EVEN(±min) = 0`,
     `ISEVEN(±min) = TRUE`, `ISODD(±min) = FALSE`, `GESTEP(±min) = 1`, `DELTA(±min) = 1`,
     `QUOTIENT(±min, ...) = 0`, `CEILING`/`FLOOR` with subnormal first arg `= 0`, and why a
     subnormal divisor/significance in `QUOTIENT`/`CEILING`/`FLOOR` flushes to `0` and gives
     `#DIV/0!` or `0`.
   - **251 rows** had `±0x7fefffffffffffff` (`±DBL_MAX`, plus normal second arguments); all 251 keep
     their exact input bits (`input_kept: true`) and return the bank answers (`ABS(±max) = max`,
     `SQRT(-max) = #NUM!`, `EVEN/ODD/INT(±max) = ±max`, `SIGN(±max) = ±1`, `ISEVEN(±max) = TRUE`,
     `ISODD(±max) = FALSE`).

3. **Formula literals and `VALUE` (`q1_literal_acceptance`, `q1b_formula_constructed`)**:
   - Excel refuses formula entry (`0x800A03EC`) for numeric literals below `2.2250738585072014E-308`
     (`1E-310`, `4.9406564584124654E-324`) and above `9.99999999999999E+307`
     (`1.7976931348623157E+308`).
   - `=2.2250738585072014E-308` and `=2.2250738585072009E-308` are accepted and parsed to `0`;
     `=2.3E-308` is kept (`0x00108b2a2c280291`).
   - `VALUE("1E-320")` and `VALUE("1E+308")` return `#VALUE!`.

4. **Arithmetic operators vs functions on subnormals (`q1b`, `q1c`, `q1d`)**:
   - `*`, `/`, and `^` flush subnormal results to `+0`: `2^-1074 * 3 = 0`, `2.3E-308 * 0.5 = 0`,
     `2^-1022 * 0.5 = 0`, `2^-1023 = 0`.
   - `+` and `-` do **not** flush subnormals: `2^-1022 * (1 + 2^-52) - 2^-1022 = 2^-1074`
     (`0x0000000000000001`), and `2^-1022 - 2^-1023 = 0x0008000000000000` remains normal because
     `2^-1023` flushed to `0` first (`2^-1022 - 0 = 2^-1022`).
   - **`EXP(-745)` produces a true subnormal in a cell** (`0x0000000000000001`), as do `EXP(-730)`
     (`0x00000000001c7ea3`) and `EXP(-709)` (`0x0008bfe58468888d`). That subnormal survives in the
     formula cell and through `+`, `-`, `SUM`, `IF`, `MIN`, `MAX`, `ROUND`, `N`, `ABS`:
     `EXP(-745) * 2^1000 = 2^-74` (`0x3b50000000000000`).
   - Negating a subnormal cell with `0 - cell` preserves the negative sign (`0x8000000000000001`);
     unary `-cell` loses the negative sign (returns `+cell`) for `EXP(-712)` (`0x00006f8498862828`)
     and smaller, while keeping the negative sign for `EXP(-710)` (`0x00033a961f2419c8`) and larger;
     `-1 * cell` and `1 * cell` flush to `0`.
   - When fed a true subnormal `x` from `EXP(-745)` / `EXP(-730)` / `EXP(-709)`:
     - `ABS(±x) = x`; `SQRT(+x) = 2^-537` (`0x1e60000000000000`), `SQRT(-x) = #NUM!`;
     - `EVEN(+x) = 2`, `EVEN(-x) = -2`, `ODD(+x) = 1`, `ODD(-x) = -1`;
     - `ISEVEN(±x) = TRUE`, `ISODD(±x) = FALSE`;
     - `CEILING(+x, 1) = 1`, `FLOOR(-x, 1) = -1`, `QUOTIENT(±x, 1) = 0`;
     - `DELTA(x, 0) = 0`, `DELTA(x, x) = 1`, `GESTEP(+x, 0) = 1`, `GESTEP(-x, 0) = 0`;
     - `SIGN` and `INT` have a threshold inside the subnormal range: `SIGN(-x) = -1` for all tested
       negative subnormals, `SIGN(+x) = 0` for `x <= 0x1c7ea3` and `1` for `0x0008bfe58468888d`;
       `INT(-x) = 0` for `x <= 0x1c7ea3` and `-1` for `0x0008bfe58468888d`.

---

## Q2: `FIXED` (`Q2-FIXED.json`, `Q2-excel-international.json`)

- All **989** bank rows have `no_commas = 1` (`TRUE`), so no grouping separator appears in any bank
  answer: 788 answers contain `"."` (`decimals > 0`) and 201 are integers (`decimals <= 0`).
- Replaying all 989 rows in build 20527 under `en-ZA` reproduces **989 / 989** bank strings
  identically (`matches_bank: true`).
- On this `en-ZA` host, `Application.DecimalSeparator = "."` (code 46),
  `Application.ThousandsSeparator = "\u00a0"` (code 160), while
  `Application.International(xlThousandsSeparator) = " "` (ASCII space, code 32) and
  `xlDecimalSeparator = "."`. When `no_commas = FALSE`, `FIXED(1234567.891, 2, FALSE)` produces
  `"1 234 567.89"` using ASCII space `0x20` (`TEXT(1234567.891, "#,##0.00")` produces
  `"1234,567.89"` because `,` is not the grouping symbol in `en-ZA`).

---

## Q3: `WEEKNUM` (`Q3-WEEKNUM.json`)

- **3,022** bank rows (`w111-date-boundaries` and `w111-date-conversion`) were replayed in five
  modes:
  - Multithreaded (8 threads), order A vs shuffled order B: **497 / 3,022** rows differ between A
    and B; even two identical order-A runs (`runA1` vs `runA2`) differ on **56 / 3,022** rows.
  - Single-threaded (`MultiThreadedCalculation.Enabled = false`), two identical order-A runs
    (`runA_st1` vs `runA_st2`): **0 / 3,022** differ (100% deterministic for a fixed evaluation
    order).
  - Single-threaded, order A vs shuffled order B (`runA_st1` vs `runB_st`): **471 / 3,022** differ.
- **Exact mechanism (`truth_table_probe`, `truncation_boundary_probe`)**:
  1. `return_type` `t` is truncated to an integer with a small positive offset (`floor(t + ~2^-22)`:
     `11 - 2.3e-7` becomes `11`, `11 - 2.4e-7` becomes `10`; `serial_number` truncates at
     `floor(s + ~2^-38)`).
  2. Valid return types `{1, 2, 11, 12, 13, 14, 15, 16, 17, 21}` are 100% deterministic across all
     runs and orders.
  3. Invalid return types in `{0, 3, 4, 5, 6, 7, 8, 9, 10, 18, 19, 20}` **always return `#NUM!`**
     (683/683 in the bank, and 100% in the truth-table probe regardless of the preceding call).
  4. Invalid return types `< 0` or `>= 22` (including `99`, `256`, `2^31`) return `#NUM!` **unless**
     the immediately preceding evaluated `WEEKNUM` call used:
     - `return_type` `1` or `17` -> the invalid call returns the **type-1 (Sunday)** week number of
       its own `serial_number`;
     - `return_type` `2`, `11`, or `21` -> the invalid call returns the **type-2 (Monday System 1)**
       week number of its own `serial_number` (even after `21`, it uses type-2, not ISO week 21);
     - another `< 0` or `>= 22` call -> preserves the state from before that call.
     - Any preceding call with `return_type` in `{12..16}` or `{0, 3..10, 18..20}` resets the state
       so `< 0` or `>= 22` returns `#NUM!`.

---

## Q4: `TEXTSPLIT` (`Q4-TEXTSPLIT.json`)

- In the original bank capture (`fn1-textsplit-004`, `fn1-textsplit-005`), the formula was placed in
  a cell with input cells immediately to its right, causing `#SPILL!` (`Error 2045`).
- Evaluated in a free row with empty spill cells to the right:
  - `=TEXTSPLIT("é-É-é","é")` spills `1x3`: `["", "-É-", ""]` (and with `ignore_empty=TRUE`: `1x1`
    `["-É-"]`; with `match_mode=1`: `1x4` `["", "-", "-", ""]`; with `ignore_empty=TRUE,
    match_mode=1`: `1x2` `["-", "-"]`).
  - `=TEXTSPLIT("a,,b",",")` spills `1x3`: `["a", "", "b"]` (and with `ignore_empty=TRUE`: `1x2`
    `["a", "b"]`). Empty elements are empty strings `""` (`kind: "string", value: ""`).

---

## Q5: `MINVERSE` (`Q5-MINVERSE.json`)

- The 112 `ExcelErrorType9` bank rows come from **28 invertible 4x4 matrices** queried as
  `INDEX(MINVERSE(m), r, 0)` for `r = 0, 1, 2, 3`, where `column_num = 0` returns a `1x4` row
  slice (or `4x4` for `r = 0`) that failed to spill (`#SPILL!`, `Error 2045`) in the bank's single-cell
  layout.
- Evaluated with unobstructed spill space, **all 28 / 28 matrices invert cleanly**:
  - All 448 input cells (including `±1e150`, `1e-150`) read back with exact bits (`input_kept: true`).
  - Every `MINVERSE(m)` spills a finite `4x4` matrix (all 16 element bits recorded), every
    `INDEX(MINVERSE(m), r, 0)` spills the matching `1x4` row, and `MDETERM(m)` is finite and
    non-zero.

---

## Q6: `BINOM.INV` / `CRITBINOM` (`Q6-BINOM-INV.json`)

- Across all 135 primary calls (15 `(n, p, k)` dyadic CDF boundaries × 9 `alpha` ulp offsets
  `-4..+4`) and 54 extended calls, **`BINOM.INV` and `CRITBINOM` are 100% identical** in every cell.
- The boundary transition from `k` to `k + 1` is **neither universally `CDF(k) >= alpha` (step at
  `+1` ulp) nor universally `CDF(k) > alpha` (step at `0` ulp)**; it depends on how Excel's internal
  CDF accumulation rounds for `(n, p, k)`:
  - At `k = 0`: `alpha == CDF(0)` steps to `1` (`first_step_ulp = 0`, i.e. strict `>`) for
    `n = 1, 2, 4, 5` with `p = 0.5, 0.25, 0.75` (`CDF = 1/2, 1/4, 1/16, 1/32, 9/16`), **except**
    `n = 3` (`p = 0.5, CDF = 1/8` and `p = 0.25, CDF = 27/64`), where `alpha == CDF(0)` returns `0`
    (`first_step_ulp = +1`).
  - At `k >= 1`: `alpha == CDF(k)` returns `k` (`first_step_ulp = +1`) for most cases, **except**:
    - `n = 4, p = 0.5, k = 1` (`CDF = 5/16`) and `n = 5, p = 0.5, k = 1` (`CDF = 3/16`), which step
      at **`+2` ulp** (`alpha = CDF + 1 ulp` still returns `1`);
    - `n = 5, p = 0.5, k = 2` (`CDF = 1/2`), which steps at **`-1` ulp** (`alpha = 0x3fdfffffffffffff`
      already returns `3`).
  - This proves Excel's `BINOM.INV`/`CRITBINOM` accumulates or recurrence-computes the binomial
    terms in floating point with ~1–2 ulp rounding drift relative to the exact dyadic rational sum,
    rather than comparing `alpha` against an exact dyadic CDF.
