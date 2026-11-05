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
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Numerical notes
  - What has not been checked
  - Notes
  - Page vocabulary
  - Sources
family: special_dist_family
role_in_family: >-
  The original, two-limit error function: the only member of the family that integrates between
  two points, and the surface for which the shared body is inferred, not scored.
---

# ERF

## What it computes

`ERF(lower_limit, [upper_limit])` returns the **error function**, integrated either from 0 to one
point or between two.

With one argument it is erf(x), the standard error function:

    ERF(x)  =  erf(x)  =  (2/√π) ∫₀^x e^(−t²) dt

With two arguments it is the same integrand between the two limits:

    ERF(a, b)  =  (2/√π) ∫ₐ^b e^(−t²) dt  =  erf(b) − erf(a)

The factor 2/√π is chosen so that erf(∞) = 1. The series and the behaviour at the ends are short
to state:

    erf(x)  =  (2/√π) Σ_{k≥0}  (−1)^k x^(2k+1) / ( k! (2k+1) )
    erf(x)  ≈  (2/√π) x                        for small x
    erfc(x) =  1 − erf(x)  ≈  e^(−x²) / (x√π)  for large x

**Shape.** erf is odd, strictly increasing, and bounded by −1 and 1, which it approaches and never
reaches. In binary64 it does reach them: erf(x) rounds to exactly 1 once x passes about 5.92. It
has no poles and no branch cuts on the real line.

**What it is for.** Up to a scaling of x it is the normal distribution: the probability that a
normal variable lies within z standard deviations of its mean is erf(z/√2). Diffusion, heat
conduction and signal detection use it in the same way.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `lower_limit` | The lower bound for the integration. Required. | — |
| `upper_limit` | The upper bound. Optional. | If omitted, the integral runs from 0 to `lower_limit` |

One or two arguments; the projection declares an arity of 1 to 2.

**The naming misleads in the one-argument form.** Microsoft's text is exact about what happens, but
in `ERF(x)` the argument called `lower_limit` is mathematically the upper limit of an integral
from 0. `ERF(0.745)` and `ERF(0, 0.745)` are the same integral. Two limits in either order are
meaningful: `ERF(b, a)` should be the negative of `ERF(a, b)`, a point that has not been read.

**Argument handling is recorded, and it is stricter than the ordinary numeric slot.**

- **Numeric text is accepted.** `ERF("2")` gives the same value as `ERF(2)`, while `ERF("")` is
  `#VALUE!` (Excel 16.0 build 20026).
- **Logicals are rejected,** in either position, typed or held in a cell. `ERF(TRUE)`, `ERF(2,TRUE)`
  and `ERF(TRUE,3)` are `#VALUE!`. The notes below record this for build 20527, alongside `DELTA`
  and `GESTEP`. Many numeric slots convert TRUE to 1, and the gamma functions in the same implementing
  module do.
- **An empty cell reads as 0.** `ERF(B1, 3)` with `B1` empty gives erf(3).
- **Arrays lift.** `ERF({2;3})` returns a 2-by-1 array of the two values (build 20026).

## Result and edge cases

Returns `Number`. The behaviour below is read from recorded Excel answers unless marked otherwise.

- **`ERF(0)`** is exactly 0.
- **Negative limits are ordinary.** `ERF(-1.00001276867384825)` is about −0.8427, and small negative
  arguments return small negative values (build 19929). Microsoft's page states no condition on the
  sign of either limit.
- **Saturation.** `ERF(6.49517072256603711)` and every larger argument read are exactly 1, and
  the large negative arguments read are exactly −1 (build 19929). The answer `ERF.PRECISE(5.875)` is
  still below 1, and `ERF.PRECISE(5.999999999999999)` is 1 (build 20131).
- **The documented examples**, `ERF(0.745)` and `ERF(1)`, are printed by Microsoft to eight places
  as `0.70792892` and `0.84270079`, and the true values agree.
- **Tiny arguments** give about 1.128·x. `ERF(1.00000000000000003E-300)` is about 1.128E-300. The
  last digit does not always land where a correctly rounded erf would put it; see *Numerical
  notes*.
- **Equal limits, reversed limits, and limits beyond saturation** have not been read. The last
  case matters, and is taken up under *Numerical notes*.

## Errors

As documented on Microsoft's page:

| Error | Condition |
|---|---|
| `#VALUE!` | `lower_limit` is nonnumeric |
| `#VALUE!` | `upper_limit` is nonnumeric |

As recorded, adding to that:

| Input | Result | Recorded on |
|---|---|---|
| Empty text in the slot, `ERF("")` | `#VALUE!` | build 20026 |
| An error value, `ERF(NA())` | that error | build 20026 |
| A logical in either position | `#VALUE!` | builds 20026 and 20527 |
| Two bad arguments | the left one decides: `ERF(TRUE,#REF!)` is `#VALUE!`, `ERF(#REF!,TRUE)` is `#REF!` | build 20527 |

The last row is the opposite of `DELTA` and `GESTEP`, which read their second argument first, as
the notes below record. No `#NUM!` is documented, and none appears in the recorded answers. The
reference engine raises `#NUM!` for a non-finite argument, which a worksheet cell cannot supply.

## Relationships

- **[ERF.PRECISE](FUNC.ERF.PRECISE.md)** — the one-argument form under a later name. The name does
  not mean more accurate. The notes below treat them as one body. ERF.PRECISE is the one with the
  large recorded population, so most of what is known about this function was measured there.
- **`ERFC` and `ERFC.PRECISE`** — the complement, 1 − erf(x). Excel uses that relation in both
  directions, as *Numerical notes* describes. For anything in the tail use `ERFC`, because it
  keeps digits that `ERF` has already lost to 1.
- **[GAUSS](FUNC.GAUSS.md)** — GAUSS(z) = ½·erf(z/√2), and the notes below record that it uses the
  same rational at small argument.
- **[NORM.S.DIST](FUNC.NORM.S.DIST.md)** — the standard normal CDF, Φ(z) = ½(1 + erf(z/√2)).
- **[GAMMA.DIST](FUNC.GAMMA.DIST.md)** — erf(x) = P(½, x²) for x ≥ 0, where P is the regularized
  lower incomplete gamma function, so `GAMMA.DIST(x*x, 1/2, 1, TRUE)` is the same quantity.
- **`DELTA` and `GESTEP`** — in the same Engineering category, with the same refusal of logicals
  and the opposite order of reading arguments.
- **Module siblings**: `ERF.PRECISE`, `ERFC`, `ERFC.PRECISE`, `GAMMA`, `GAMMALN`, `GAMMALN.PRECISE`,
  `WEIBULL` and `WEIBULL.DIST` share the reference engine's implementing module. That is an
  implementation fact and implies nothing about Excel's code.

## Numerical notes

**Below 0.5 it is a stored rational.** The notes below give the full reading: for |x| < 0.5 the
answer is x·N(x²)/D(x²), with N of degree 4 and D of degree 5, and every step rounded on the x87
unit and then stored to binary64. The coefficients were read off Excel's answers and are listed
there. **Their source is not identified.** Cody's rational of 1969 is refuted by its linear terms,
Cephes has the same shape with different ratios, and no published table has been matched.

**So the last digits are not correctly rounded, and this is not a defect of precision.** The
rational itself equals erf to about 9E-17. The rounding at each step is what moves the answer, by
up to 3 units in [0.25, 0.5), and by 1 unit at very small x. A correctly rounded erf is therefore the
wrong target for anyone matching Excel, and the reference engine, at the commit named under Sources,
calls a general-purpose library `erf`. It differs from Excel by a unit on some small arguments.

**From 0.5 up it is the complement.** On the 187 inputs from 0.5 to 6 at which both functions were
read, `ERF.PRECISE(x)` equals 1 minus `ERFC.PRECISE(x)` computed in binary64, every time (build
20131, AMD Ryzen 7 4800H). That was checked while writing this page; it is not yet a note. The ERFC
side is in the [function notes](../notes/NOTES.md) under `ERFC`: its answers drift by many units in
the far tail, and one factor in them is unidentified. Almost none of that reaches `ERF`, because
subtracting a small number from 1 discards its last digits. Of 212 `ERF.PRECISE` rows in [0.5, 6],
192 are correctly rounded and the other 20 are within 2 units.

**The two-limit form has a cancellation problem by construction.** If the integral is computed as
erf(b) − erf(a), as the formula invites and as the reference engine does, then both terms round
to near 1 as the limits grow, and the difference loses everything. The true value of the integral
from 6 to 7 is about 2.15E-17, and erf(6) and erf(7) are both 1 once rounded to binary64, so that
route returns 0. `ERFC(a)-ERFC(b)` does not have this problem. Whether Excel subtracts, and how, is
unread.

## What has not been checked

No Handbook vector suite exists for `ERF`; `vectors/` publishes nothing at this revision, so no
suite-scoped claim exists for it.

The evidence records on this page are narrow. `EV-DIST-0018` is one pinned point for this surface
in a ten-surface re-sweep, not a corpus rate, and `EV-DIST-0024` concerns which argument types are
accepted and says nothing about numeric results. Their figures render beside this page. The
reader's guide to numeric behaviour is the notes below.

What is still unmeasured:

1. **Two numeric limits.** No recorded answer has two non-zero limits. The only two-argument number
   on record is `ERF(B1, 3)` with an empty `B1`. So the formula erf(b) − erf(a) is Microsoft's
   description and the mathematics, not an observation. The first probes are `ERF(0.2, 0.7)`,
   `ERF(6, 7)`, `ERF(1, 0)` and `ERF(1, 1)`.
2. **Whether this surface shares the ERF.PRECISE body.** The notes below mark that as inferred,
   since their scored population is `ERF.PRECISE`, `ERFC`, `ERFC.PRECISE` and `GAUSS`. A spot check
   while writing this page used the few dozen single-argument `ERF` answers that survive from build
   19929. Nine are non-zero, below 0.5 and have their exact input on record, and all nine equal the
   rational, including the four where it differs from a correctly rounded erf. That supports the
   inference. It is a handful of rows on one older build, and no population has been scored.
3. **The open items in the notes below**: the last bits of the coefficients of order 3 and above,
   the source table, which branch owns x = 0.5, and subnormal arguments.
4. **The complement above 0.5 on this surface.** The relation described under *Numerical notes* was
   checked on `ERF.PRECISE`, not on `ERF`.
5. **Ranges and arrays with two arguments.** Only a single array in the first slot has been read.
   How `ERF({1;2},{3;4})` pairs its arguments, and how a multi-cell range behaves, is unread.
6. **Other platforms.** The recorded answers are from Windows. No reading from a Mac, Excel for the
   web, 32-bit Excel or an Intel CPU is on record for this surface.

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
- OxFunc-Evidence `cache/oracle/build-20131/ERF.PRECISE.jsonl.gz`, line 1219, key ERF.PRECISE|0x3fdcf55555555555 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle/build-20131/ERF.PRECISE.jsonl.gz`, line 2115, key ERF.PRECISE|0x3d70000000000000 at `e26b7bddb8` (readings)
- OxFunc-Evidence `cache/oracle/build-20131/ERF.PRECISE.jsonl.gz`, line 209, key ERF.PRECISE|0x3ddb7cdfd9d7bdb9 at `e26b7bddb8` (readings)

<!-- efh:notes end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| erf | The mathematical error function; `ERF(x)` with one argument |
| erfc | The complementary error function, 1 − erf(x) |
| saturation | The point beyond which the result is exactly ±1 in binary64 |
| cancellation | Loss of leading digits when two nearly equal numbers are subtracted |
| unit | One unit in the last place of a binary64 result |
| x87 unit | The floating-point unit of x86 processors, which computes in an 80-bit format |

## Sources

- Microsoft, "ERF function" —
  <https://support.microsoft.com/en-us/office/erf-function-c53c7e7b-5482-4b6c-883e-56df3c9af349>
  (the description, syntax, argument meanings, the two `#VALUE!` remarks and the two examples).
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, §7.1; NIST Digital Library of
  Mathematical Functions, §7.2, §7.4, §7.6, §7.11 and §7.12 — <https://dlmf.nist.gov/7.2>
  (definition, symmetry, series, the incomplete-gamma relation and asymptotics).
- The recorded Excel answers the notes below draw on, and those this page adds to them: argument
  probes on Excel 16.0 build 20026 and build 20527; the single-argument `ERF` answers on build
  19929; and the `ERF.PRECISE` and `ERFC.PRECISE` answers on build 20131, AMD Ryzen 7 4800H. The
  statements on this page that go beyond the notes were read from those files and compared with
  mpmath at 300 bits while this page was written; each is marked where it appears.
- Handbook evidence records `EV-DIST-0018` (one pinned point in a ten-surface re-sweep) and
  `EV-DIST-0024` (the rejection of logicals).
- Handbook, [The value universe](../model/01-value-universe.md),
  [Coercion and lifting](../model/02-coercion-and-lifting.md) and
  [Claim language and honesty](../model/06-claim-language.md).
- Handbook projections `data/functions/FUNC.ERF.json` and `data/presence/FUNC.ERF.json` (arity,
  classification axes, implementing module and sibling set).
- OxFunc `crates/oxfunc_core/src/functions/special_dist_family.rs` and
  `crates/oxfunc_core/src/functions/normal_dist_common.rs` at commit `473efa3` (the reference
  engine's `erf_kernel`, `erf_interval`, its logical-rejecting coercion, and `erf_approx`).
