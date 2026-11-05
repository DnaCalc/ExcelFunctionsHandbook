---
schema: efh.function-page/v1
function_id: FUNC.GAUSS
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-DIST-0023
  - EV-DIST-0018
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Gauss method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gauss"
    role: "the entire documented surface: a one-line description, one parameter, a Double return, and no Remarks section"
  - work: "Microsoft Support — GAUSS function"
    locator: "https://support.microsoft.com/en-us/office/gauss-function-069f1b4e-7dee-4d6a-a71f-4b69044a6b33"
    role: "the worksheet-surface documentation page; not retrievable at curation time (the host refused the request)"
  - work: "Abramowitz & Stegun, Handbook of Mathematical Functions, chapters 7 and 26"
    locator: "7.1 — the error function; 26.2 — the normal probability function and its relation to erf"
    role: "the mathematics of the normal integral this function returns half of"
  - work: "W. J. Cody, 'Rational Chebyshev approximation for the error function' (1969) and ACM Algorithm 715 CALERF"
    locator: null
    role: "the classical erf/erfc implementation family"
  - work: "G. W. Hitchcox / W. Cody, and the fdlibm s_erf.c treatment"
    locator: null
    role: "the reference open-source error-function implementation"
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
family: gauss_fn
role_in_family: >-
  Sole member of its module, but not of its substrate: a thin wrapper over the shared normal-CDF
  kernel, and the surface whose open discrepancy is explicitly blocked on the error function.
---

## What it computes

`GAUSS(z)` returns the probability that a standard normal variable falls between 0 and z —
that is, the standard normal cumulative distribution function shifted so that the origin maps to
zero. Microsoft's one-line description says it exactly: "Returns 0.5 less than the standard
normal cumulative distribution."

*The quoted one-line description is Microsoft's text, from its support article <https://support.microsoft.com/en-us/office/gauss-function-069f1b4e-7dee-4d6a-a71f-4b69044a6b33> and the VBA reference <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gauss>.*

    GAUSS(z)  =  Φ(z) − ½  =  (1/√(2π)) ∫₀^z e^{−t²/2} dt  =  ½ · erf( z / √2 )

- **Domain:** the whole real line.
- **Range:** the open interval (−½, ½). The bounds are approached but never attained
  mathematically, and are attained in floating point once |z| is beyond about 8.3.
- **Symmetry:** odd. GAUSS(−z) = −GAUSS(z), and GAUSS(0) = 0 exactly. This is the structural
  reason the function exists: Φ itself is not odd, and the ½ subtraction is precisely what makes
  the symmetry manifest.
- **Derivative:** the standard normal density φ(z) = e^{−z²/2}/√(2π), which is 1/√(2π) ≈ 0.3989
  at the origin and decays super-exponentially.
- **Series at the origin:** GAUSS(z) = φ(0)·(z − z³/6 + z⁵/40 − …), so for small z the function is
  z/√(2π) to first order.

This is the function classically tabulated in the back of statistics textbooks — the "area under
the normal curve from 0 to z" table — and that is the whole reason it has a worksheet surface of
its own. A&S §26.2 defines the same object and relates it to the error function of §7.1.

**The identity that governs the implementation** is the last one above: GAUSS is half an error
function of a scaled argument. Everything numerical about this surface is therefore a statement
about erf, and the Handbook's evidence for it says so in as many words.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `z` | The value for which you want the distribution. Required. | — |

One argument; the reference engine declares an arity of exactly 1, and classifies the surface as
a plain number-to-number kernel with elementwise array lifting. The slot is numeric under
ordinary to-number coercion; see [Coercion and lifting](../model/02-coercion-and-lifting.md).

There is no mean, no standard deviation, and no cumulative flag. `GAUSS` is the standardised
form only; a non-standard normal must be standardised first — `GAUSS((x − μ)/σ)` — or computed
with `NORM.DIST`.

## Result and edge cases

Returns `Number`, always in [−½, ½].

- **z = 0** gives exactly 0, and the signed zero is worth preserving.
- **Small z** gives z/√(2π) to within a rounding. Subnormals cannot be asked through a cell,
  which reads them back as 0 (FN-ERF.PRECISE-001). There is no cancellation here, unlike in the
  ½-subtracted form discussed below — which is the point.
- **Large |z|** saturates at ±½. In binary64 the difference between Φ(z) and 1 falls below the
  representable spacing at ½ once |z| exceeds roughly 8.3, so beyond that the function returns
  exactly ±½ and carries no further information.
- **Odd symmetry** is a free self-check: `GAUSS(z) + GAUSS(−z)` should be exactly zero for every
  z. An implementation that computes Φ(z) − ½ separately for each sign will generally fail this,
  and failing it is a defect provable without any oracle.
- **Array arguments** lift elementwise.

The projected battery beside this page records the reference engine's own answers on a fixed
probe list; no Excel was involved in producing them.

## Errors

**Microsoft's VBA reference page for this method has no Remarks section and documents no error
condition at all.** Like `GAMMA`, it is a description, a parameter row and a return type. The
worksheet-surface page at `support.microsoft.com` is the fuller source and was not retrievable at
curation time, so this page does not quote it and does not assert its contents from memory.

That is a documentation gap and it is the honest statement for this section. What the reference
engine does: the kernel is total on the reals and returns a number for every finite input; a
non-coercible argument surfaces `#VALUE!` under the shared coercion rules, and an error value in
the argument propagates. The reference engine declares `arg_domain_guard=none` on its
real-result axis, consistent with a function that has no domain to guard.

## Relationships

- **`NORM.S.DIST(z, TRUE)`** — the unshifted standard normal CDF. `GAUSS(z)` and
  `NORM.S.DIST(z, TRUE) − 0.5` are the same number mathematically. They are *not* the same
  computation in general, and the difference matters: forming Φ(z) and then subtracting ½ loses
  precision for small z, because Φ(z) ≈ ½ + small and the subtraction cancels the leading digits
  of the answer. `GAUSS` exists so that the small quantity never has to be recovered from a
  difference. Below 0.5 Excel's `GAUSS` uses the ERF rational of FN-ERF.PRECISE-001; from 0.5 up,
  whether it avoids that path is an open question here.
- **`PHI(z)`** — the standard normal *density*, not a probability. Naming collision with the
  common notation Φ for the CDF; Excel's `PHI` is φ, the derivative of `NORM.S.DIST`. The two
  were introduced together in Excel 2013 and are documented on separate pages.
- **`NORM.DIST` / `NORM.INV` / `NORM.S.INV`** — the general and inverse normal surfaces.
- **`ERF` and `ERF.PRECISE`** — the substrate. GAUSS(z) = ½·erf(z/√2) exactly, so any statement
  about `GAUSS`'s last bits is a statement about the error function Excel provides. The
  Handbook's own evidence names this dependency explicitly.
- **`STANDARDIZE`** — the usual preprocessing step, (x − μ)/σ.
- **`CONFIDENCE.NORM`, `Z.TEST`** — consumers of the same normal-tail machinery.

## Numerical notes

**The whole function is one erf evaluation, and erf is a solved problem.** The classical
implementation is Cody's rational Chebyshev approximation (1969), refined into ACM Algorithm 715
(`CALERF`), which splits the line into three regions — a small-argument rational in z², a middle
region computing erfc directly, and a large-argument asymptotic form — and is the ancestor of
fdlibm's `erf`/`erfc` and of most library implementations. Boost.Math and Cephes are in the same
lineage with different coefficient sets. The mathematics has been settled for fifty years; what
differs between implementations is coefficients, region boundaries, and the order of the final
arithmetic.

**The reason `GAUSS` is a separate function is cancellation.** Writing GAUSS(z) as Φ(z) − ½ is
the obvious implementation and the wrong one for small z: the true answer is small, Φ(z) is close
to ½, and the subtraction discards the leading significant digits. Computing ½·erf(z/√2) instead
keeps the small quantity small throughout. This is the same lesson as `log1p` versus
`LN(1+x)` and `expm1` versus `EXP(x)−1`, applied to the normal integral — and it is the reason
the function has a name of its own rather than being left to the reader to spell out. An
implementation that quietly routes `GAUSS` through the CDF has thrown away the function's
purpose while returning plausible answers everywhere except where it matters.

**The scaling by √2 is not free.** z/√2 must be computed with a correctly rounded division (or by
multiplying by a correctly rounded 1/√2 — which is *not* the same, and the two differ in the last
bit for many z). For a compatibility-oriented implementation this is a specification detail, not
an optimisation: the choice is observable.

**Saturation.** The threshold beyond which the result is exactly ±½ is an implementation
fingerprint, determined by where erf(z/√2) rounds to 1 rather than by any mathematical boundary.

**What the Handbook's evidence says about the substrate.** The open-discrepancy record for this
surface diagnoses the disagreement as needing the error-function/CDF substrate. That is an
unusually specific statement of *why* a surface is unresolved: not "we have not looked", but "the
layer beneath it has not been identified". FN-ERF.PRECISE-001 has since identified that layer below
0.5: a stored (4, 5) rational, which `GAUSS` uses too, with coefficients read off Excel's answers
and no published table matched. A separate record in the same family identifies the `.PRECISE`
error-function surfaces as branches of an incomplete-gamma routine rather than as a standalone
coefficient table; below 0.5 that note finds a coefficient table.

## What has not been checked

Two records name `GAUSS` as a subject, and both are small.

`EV-DIST-0023` is an open-discrepancy record whose entire evidence is **one witness point**, with
the local and Excel results recorded against each other on a named build, plus the diagnosis that
the surface is blocked on the error-function substrate. The record states plainly that **no count
exists** for this surface. `EV-DIST-0018` is a ten-witness re-sweep under cell-reference plumbing
on a named build — the record is explicit that it is one pinned row per surface and *not* a
corpus pass rate — in which this surface is one of the two that drifted while the rest matched.
Both records' figures render mechanically beside this page; this prose deliberately does not
restate them.

So those two records amount to **a single point, twice, on one build, and it disagrees.** That is
enough to know the surface is not resolved and nowhere near enough to characterise it.
FN-ERF.PRECISE-001 goes further: `GAUSS` uses the ERF rational below 0.5 and matches every recorded
row in that range. There is no Handbook vector suite for `GAUSS`, no measurement anywhere in the
tails, and no identification of the algorithm from 0.5 up. Still open from that note: the last bits
of the coefficients of order 3 and above, the source table, which branch owns 0.5, and subnormal
arguments.

Inputs I would probe first, and why:

1. **A ladder of small z** — 2⁻¹⁰ down to the subnormal edge — against the correctly rounded
   ½·erf(z/√2). Answered below 0.5: `GAUSS` uses the ERF rational, and every recorded row in that
   range matches (FN-ERF.PRECISE-001). Subnormal arguments cannot be asked through a cell.
2. **`GAUSS(z) + GAUSS(−z)`** across the range, which must be exactly zero and needs no oracle.
   A non-zero result proves the symmetry is not being exploited.
3. **`GAUSS(z)` against `NORM.S.DIST(z, TRUE) − 0.5` and against `ERF(z/SQRT(2))/2`**, on the
   same arguments. Three surfaces, one mathematical quantity: any disagreement identifies which
   pairs share code in Excel, and that is structural information no bit comparison alone gives.
4. **The saturation boundary**: the largest z with a result strictly below ½, and the smallest
   with exactly ½. The threshold is a fingerprint.
5. **The witness point already in the record**, re-run on a different build, to establish whether
   the recorded drift is build-scoped.
6. **The moderate range 0.5 ≤ z ≤ 3**, densely — the region where the erf implementations in the
   literature switch between their approximation regions, and therefore where a seam would show.
7. **`GAUSS(TRUE)` and `GAUSS("1")`** — the coercion path, undocumented for this surface.

## Notes

<!-- efh:notes begin: generated by `efh notes render` from content/notes/; edits here are overwritten -->

Recorded behaviour of `GAUSS` that an implementer matching Excel, or a reader explaining an
unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer
and where it was recorded; "measured" means the statement is read off recorded Excel answers,
"inferred" marks what goes beyond them. Where older text on this page says a behaviour was not
measured, these notes are the later record. All notes: [Function notes](../notes/NOTES.md).

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

<!-- efh:environments begin: generated by `efh environments render` from content/evidence/environments/FUNC.GAUSS.json; edits here are overwritten -->

Readings of GAUSS with a recorded Excel build: 9,770 input(s) over 3 recorded environment(s),
Excel 16.0 build(s) 19929, 20228, 20430, all on Windows; none of these captures records the
CPU. 1 same-input comparison(s) between environments over 3 input-pair(s): no result differs.
No reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.

Readings of `GAUSS` recorded here come from:

| Excel | Platform | CPU | Inputs read | Observed |
|---|---|---|---:|---|
| 16.0 build 19929 | Windows, bitness not recorded | CPU not recorded | 42 | date not recorded |
| 16.0 build 20228 | Windows, 64-bit | CPU not recorded | 9,216 | 2026-08-09 |
| 16.0 build 20430 | Windows, 64-bit | CPU not recorded | 512 | 2026-09-29 |

Compared on the same inputs:

| Check | Differs in | Environments | Inputs compared | Inputs with different results |
|---|---|---|---:|---:|
| XC-GAUSS-001 | excel-build | 16.0 build 19929 vs 16.0 build 20228 | 3 | 0 |

Differences observed: none.

Evidence for every row: [`content/evidence/environments/FUNC.GAUSS.json`](../../content/evidence/environments/FUNC.GAUSS.json). All known differences: [Excel environment differences](../evidence/EXCEL-ENVIRONMENTS.md).

<!-- efh:environments end -->

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| Φ (Phi) | The standard normal cumulative distribution function; GAUSS is Φ − ½ |
| φ (phi) | The standard normal density, which Excel exposes as `PHI` |
| erf | The error function; GAUSS(z) = ½·erf(z/√2) exactly |
| cancellation | Loss of leading digits when Φ(z) − ½ is formed for small z |
| saturation | The point beyond which the result is exactly ±½ |
| substrate | The lower-level function (here erf) whose behaviour determines this surface's |

## Sources

- Microsoft Learn, "WorksheetFunction.Gauss method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.gauss>
  (the description "Returns 0.5 less than the standard normal cumulative distribution", the
  single parameter, and the `Double` return type; **no Remarks and no documented error
  condition**). The worksheet-surface page at `support.microsoft.com` was not retrievable at
  curation time.
- Abramowitz & Stegun, *Handbook of Mathematical Functions*, §7.1 (error function) and §26.2
  (the normal probability function and its erf representation).
- W. J. Cody, "Rational Chebyshev approximation for the error function", *Math. Comp.* 23 (1969);
  ACM Algorithm 715 `CALERF`; fdlibm `s_erf.c`; Boost.Math; Cephes — the implementation lineage
  discussed under **Numerical notes**.
- Handbook evidence records `EV-DIST-0023` (the single witness, the absence of any count, and the
  erf-substrate diagnosis) and `EV-DIST-0018` (the ten-witness re-sweep, explicitly one pinned
  row per surface rather than a corpus rate).
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md).
- Handbook projections `data/functions/FUNC.GAUSS.json` (arity, `NumToNum` kernel class, unary
  numeric scalar-or-array lift, `arg_domain_guard=none`) and `data/presence/FUNC.GAUSS.json`
  (the `gauss_fn` module, with no sibling surfaces).
- OxFunc `crates/oxfunc_core/src/functions/gauss_fn.rs` at commit `473efa3` — the total kernel
  delegating to the module's shared normal-CDF helper.
