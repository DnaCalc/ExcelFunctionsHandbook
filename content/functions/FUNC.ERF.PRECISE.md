---
schema: efh.function-page/v1
function_id: FUNC.ERF.PRECISE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0024
  - EV-DIST-0026
open_problems: []
references:
  - work: "Microsoft Support — ERF.PRECISE function"
    locator: "https://support.microsoft.com/en-us/office/erf-precise-function-9a349593-705c-4278-9a98-e4122831a8e0"
    role: "syntax, the single #VALUE! remark and the worked examples"
  - work: "NIST Digital Library of Mathematical Functions, chapter 7"
    locator: "https://dlmf.nist.gov/7.2"
    role: "definition of the error function"
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Notes
  - Excel versions, platforms and CPUs
  - Page vocabulary
  - Sources
family: special_dist_family
role_in_family: >-
  The one-argument error function, and the surface with the largest recorded population in the
  family: the rational the notes describe was scored on its rows.
---

# ERF.PRECISE

## What it computes

`ERF.PRECISE(x)` returns the **error function** integrated from 0 to x:

    ERF.PRECISE(x)  =  erf(x)  =  (2/√π) ∫₀^x e^(−t²) dt

It is [ERF](FUNC.ERF.md) without the second limit. The mathematics, the shape of the function, and
its uses are on that page and are not repeated here.

**The name does not promise more accuracy.** The rational described below was read from this
function's rows, and the notes below treat `ERF` as sharing it, so the recorded evidence gives no
sign of a more accurate routine behind the longer name.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `x` | The point the integral runs to from 0. Required. | — |

One argument; the projection declares an arity of exactly 1.

Microsoft's syntax section calls the argument `x` and describes it as the lower bound for the
integration, and its one remark calls it `lower_limit`. Both are carried over from `ERF`. It is
mathematically the upper limit of an integral starting at 0.

Argument handling matches `ERF`. Numeric text is accepted, `ERF.PRECISE("2")` giving the same value
as `ERF.PRECISE(2)`; a logical is `#VALUE!`, typed or held in a cell; an empty cell reads as 0; and
an array lifts, `ERF.PRECISE({2;3})` returning a 2-by-1 array (builds 20026 and 20527).

## Result and edge cases

Returns `Number`.

- **The documented examples** `ERF.PRECISE(0.745)` and `ERF.PRECISE(1)` are printed by Microsoft as
  `0.70792892` and `0.84270079`, and the recorded `ERF.PRECISE(1)` agrees with the correctly rounded
  value in every digit (build 20131).
- **Saturation.** The result is below 1 at x = 5.875 and exactly 1 at 5.999999999999999 (build
  20131). The true crossing, where erf rounds to 1 in binary64, is near 5.92.
- **Tiny arguments** give about 1.128·x. One unit high is normal here, as the notes below show.
- **Subnormal arguments** cannot be asked through a cell: Excel reads a subnormal written to a cell
  back as 0.

## Errors

As documented on Microsoft's page:

| Error | Condition |
|---|---|
| `#VALUE!` | the argument is nonnumeric |

As recorded on build 20026, adding to that: `ERF.PRECISE("")` and `ERF.PRECISE(TRUE)` are `#VALUE!`,
and `ERF.PRECISE(NA())` is `#N/A`. No `#NUM!` is documented, and none appears in the recorded rows.

## Relationships

- **[ERF](FUNC.ERF.md)** — the same function, with an optional second limit. Use `ERF` for an
  integral between two points.
- **`ERFC.PRECISE`** — the complement. Below 0.5 the notes below record that it is 1 less this
  value. From 0.5 up the relation runs the other way, as *Numerical notes* describes.
- **[GAUSS](FUNC.GAUSS.md)** and **[NORM.S.DIST](FUNC.NORM.S.DIST.md)** — the normal distribution in
  this function's terms.
- **[GAMMA.DIST](FUNC.GAMMA.DIST.md)** — mathematically, erf(x) = P(½, x²) for x ≥ 0, where P is
  the regularized lower incomplete gamma function. Evidence record `EV-DIST-0026` holds the
  Excel-against-Excel comparison: `GAMMA.DIST(k²/1024, 1/2, 1, TRUE)` against `ERF.PRECISE(k/32)`.
  That comparison does not identify this function's source; see *Numerical notes*.

## Numerical notes

**Below 0.5.** The notes below give the stored (4, 5) rational and its full staging. All 618,225
recorded rows with |x| < 0.5 match it. Up to 3 units separate it from a correctly rounded erf, and
400 of the 654 rows in [0.25, 0.5) on build 20131 differ from that value. The source table is not
identified.

**From 0.5 up.** The notes below do not cover this range for this function. On the 187 inputs from
0.5 to 6 at which `ERF.PRECISE` and `ERFC.PRECISE` were both read, `ERF.PRECISE(x)` equals 1 minus
`ERFC.PRECISE(x)` computed in binary64, every time (build 20131, AMD Ryzen 7 4800H). That was
checked while writing this page. Of the 212 recorded rows in [0.5, 6], 192 are correctly rounded
and the other 20 are within 2 units. See [ERF](FUNC.ERF.md) for why the complement hides the noise
of `ERFC`.

**`EV-DIST-0026` is out of date on this function's source.** The record predates the notes. It
attributes `ERF.PRECISE` and `ERFC.PRECISE` to the a < 1 branches of the NSWC `GRATIO` routine and
says no coefficient table exists. Below 0.5 FN-ERF.PRECISE-001 replaces that attribution: Excel
uses a stored (4, 5) rational from a table not yet identified. From 0.5 up the notes name no source
for this function, so nothing on this page supports the record's attribution there either. The
record is kept unchanged as the upstream statement of its date. Its figures remain what its own
warning says they are: comparisons of Excel with mathematical truth, Excel with Excel, and research
models. None is a pass rate for any implementation.

**Reference engine.** At the commit named under Sources it calls a general-purpose library `erf`,
and does not implement the rational.

## What has not been checked

No Handbook vector suite exists for `ERF.PRECISE`; `vectors/` publishes nothing at this revision, so
no suite-scoped claim exists for it.

The rational below 0.5 is the best-measured result in this family. What is still open is what the
notes below list: the last bits of the coefficients of order 3 and above, the source table (Hart's
tables have not been checked), which branch owns x = 0.5, and subnormal arguments. Two fresh
random sets on build 20527 show a small residue of unexplained rows, mostly in [0.25, 0.5).

Beyond those:

1. **The range from 0.5 up is thinly recorded.** Apart from the 2,000 consecutive doubles from 0.5
   up that FN-ERF.PRECISE-001 reads to place the cut, only 212 rows on record lie there, stopping
   at about 6, and the factor that the complement depends on is unidentified, as the notes on `ERFC` record.
2. **Negative arguments above 0.5.** No such row is on record for this surface. The rational below
   0.5 is odd by construction, and `ERF` has a few negative rows on build 19929.
3. **Other platforms.** The recorded answers are from Windows, on builds 19929 to 20527. No reading
   from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is on record.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `ERF.PRECISE` that an implementer matching Excel, or a reader explaining
an unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's
answer and where it was recorded; "measured" means the statement is read off recorded Excel
answers, "inferred" marks what goes beyond them. Where older text on this page says a behaviour
was not measured, these notes are the later record. All notes: [Function
notes](../notes/NOTES.md).

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
- OxFunc-Evidence `cache/oracle/build-20131/ERF.PRECISE.jsonl.gz`, line 1219, key ERF.PRECISE|0x3fdcf55555555555 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle/build-20131/ERF.PRECISE.jsonl.gz`, line 2115, key ERF.PRECISE|0x3d70000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle/build-20131/ERF.PRECISE.jsonl.gz`, line 209, key ERF.PRECISE|0x3ddb7cdfd9d7bdb9 at `e26b7bddb8` (readings)

<!-- efh:notes end -->

## Excel versions, platforms and CPUs

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.ERF.PRECISE.json; edits here are overwritten -->

Readings of ERF.PRECISE with a recorded Excel build: 409,255 input(s) over 2 recorded
environment(s), Excel 16.0 build(s) 19929, 20131, all on Windows; recorded CPUs: AMD Zen 2.
614,343 input(s) were also read in captures that record no build (some may repeat inputs
counted above); they are not counted. 1 same-input comparison(s) between environments over 1
input-pair(s): no result differs. No reading from a Mac, Excel for the web, 32-bit Excel or an
Intel CPU is recorded.

Readings of `ERF.PRECISE` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 42 | date not recorded |
| 16.0 build 20131 | Windows, bitness not recorded | AMD Ryzen 7 4800H with Radeon Graphics (Zen 2) | 409,213 | 2026-07-16 to 2026-07-17 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-ERF.PRECISE-001 | excel-build | 16.0 build 19929 vs 16.0 build 20131 | 1 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.ERF.PRECISE.json`](../../content/evidence/environments/FUNC.ERF.PRECISE.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| erf | The mathematical error function |
| staging | The order of operations and the precision each result is held in |
| unit | One unit in the last place of a binary64 result |

## Sources

- Microsoft, "ERF.PRECISE function" —
  <https://support.microsoft.com/en-us/office/erf-precise-function-9a349593-705c-4278-9a98-e4122831a8e0>
  (the description, the argument, the single `#VALUE!` remark and the two examples).
- NIST Digital Library of Mathematical Functions, §7.2 and §7.11 — <https://dlmf.nist.gov/7.2>
  (definition and the incomplete-gamma relation).
- The recorded Excel answers the notes below draw on, and those this page adds to them: argument
  probes on Excel 16.0 build 20026 and build 20527, and the `ERF.PRECISE` and `ERFC.PRECISE` answers
  on build 20131, AMD Ryzen 7 4800H. The statements on this page that go beyond the notes were read
  from those files and compared with mpmath at 300 bits while this page was written; each is marked
  where it appears.
- Handbook evidence records `EV-DIST-0024` (the rejection of logicals) and `EV-DIST-0026` (what the
  older figures for this surface are, and are not).
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [Claim language and honesty](../model/06-claim-language.md).
- Handbook projections `data/functions/FUNC.ERF.PRECISE.json` and
  `data/presence/FUNC.ERF.PRECISE.json` (arity, classification axes, implementing module and sibling
  set).
- OxFunc `crates/oxfunc_core/src/functions/special_dist_family.rs` and
  `crates/oxfunc_core/src/functions/normal_dist_common.rs` at commit `473efa3` (the reference
  engine's `erf_precise_kernel` and `erf_approx`).
