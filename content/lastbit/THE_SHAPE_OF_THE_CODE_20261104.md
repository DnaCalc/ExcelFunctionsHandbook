# The Last Bit — The shape of the code

Draft for the series, 4 November 2026.

Scope of every figure below: answers recorded from Excel 16.0 builds 20131 to 20527, 64-bit, on
Windows on an AMD Ryzen 7 4800H, compared with DNA Calc's calculation source at the commits named.
"Matches" means the same 64 bits on those recorded answers. No test suite has been published yet,
and nothing here is a claim about inputs nobody has asked Excel about.

## The hypothesis

Suppose you want to know how a closed program computes `ERF.PRECISE`. You can ask it, as many times
as you like, for an answer. You cannot look inside. The space of programs that could produce those
answers is enormous, and a search that treats it as enormous fails: you end up fitting a smooth
surface through sparse points, and the fit tells you nothing about the program.

The way out is a guess about the authors, and it is a guess. Suppose most of Excel's numerical core
was written in the 1990s, by hand, by people with a textbook open, for machines where every multiply
cost something. Then the code had to be short and fit in someone's head. So the computation tree you are looking for is
small and direct: a Horner loop, a recurrence, the formula from a statistics textbook, one guard
for the awkward case. Nothing is correctly rounded, because nobody could afford that, and the
rounding errors of that small tree are exactly what you see in the last bit.

We called this the code-shape hypothesis. It changes the question from "what function fits these
numbers?" to "what would someone have typed?". A candidate is ranked by how plausibly it was
written, not by how accurate it is, and being inaccurate in the right way is the evidence.

On 4 November 2026 it paid off in five unrelated corners of the function suite in one day.

## The common step

The recurring operation needs almost no notation. Let `RN64` round to the x87 register's 64-bit
significand and `RN53` round to a binary64 double. In C, with an 80-bit `long double`:

```c
static inline double st(long double v) { volatile double d = (double)v; return d; }
#define MUL(a,b) st((long double)(a) * (b))
#define ADD(a,b) st((long double)(a) + (b))
#define SUB(a,b) st((long double)(a) - (b))
#define DIV(a,b) st((long double)(a) / (b))
```

Every operation is `RN53(RN64(a op b))`: rounded twice, then written to memory before the next
step reads it. That is what `x = a * b;` does on an x87 with the precision bits set to 64 and a
compiler, or a programmer, who keeps variables in memory. Most of the time double rounding changes
nothing. Sometimes it moves the last bit, and those moments are a fingerprint.

## Five small programs

**Error function, |x| < 0.5.** A rational function of degrees 4 and 5 in `t = x*x`.

```c
double horner(const double *c, int n, double t) {   /* highest power first */
    double v = c[0];
    for (int i = 1; i < n; i++) v = ADD(MUL(v, t), c[i]);
    return v;
}
double erf_small(double x) {
    double t = MUL(x, x);
    double n = horner(ERF_N, 5, t);
    double d = horner(ERF_D, 6, t);
    return DIV(MUL(x, n), d);
}
```

618,419 of 618,437 recorded ERF.PRECISE rows match, and all 18 misses are at |x| >= 0.5, where the
routine stops applying. On 60,000 fresh answers, asked after the constants were fixed, 59,982
match. The constants are measured, and the table they came from is still unidentified: it has the
shape of Cephes' erf but not its values, and an early identification with Cody's 1969 rational was
withdrawn when the linear terms were measured and disagreed.

**Variance.** The formula on a pocket calculator, not the one in a numerical analysis book.

```c
for (i = 0; i < n; i++) { sx = ADD(sx, x[i]); sxx = ADD(sxx, MUL(x[i], x[i])); }
s2 = MUL(sx, sx);
sample = DIV(SUB(sxx, DIV(s2, n)), n - 1);            /* VAR.S */
popn   = DIV(SUB(MUL(n, sxx), s2), MUL(n, n));        /* VAR.P: a different store order */

/* two guards fall back to a centred pass over x[i] - mean */
if (DIV(DIV(s2, n), sxx) > 0.99 || MUL(variance, 1000.0) <= fabs(mean)) { ... }
```

All 14,679 recorded answers in the frozen variance family match, and all 232 corpus mismatches in
the family cleared. The two variances are not one formula with a different divisor: they store in
different places, and six recorded rows tell the stored steps from ordinary double arithmetic.
Thirty years of advice on computing variance stably would suggest a running update. The evidence
says the calculator formula with two plain guards, and a 1% cancellation test is just what someone
would add after seeing one bad answer. How the new guard edges carry over to ranges and arrays is
still an open question.

**Annuity payment (PMT, end of period, no future value).**

```c
double log1p_excel(double r) {          /* plain double arithmetic, not x87 */
    double s = r / (2.0 + r), s2 = s * s, t = 1.0 / 23.0;
    for (int k = 10; k >= 1; k--) t = t * s2 + 1.0 / (2 * k + 1);
    double two_s = 2.0 * s;
    return two_s + two_s * (s2 * t);
}
double pmt0(double r, double n, double pv) {
    double em = expm1_x87(-(n * log1p_excel(r)));
    double q  = DIV(pv, em);
    return MUL(q, r);
}
```

This is the textbook `ln(1+x) = 2 atanh(x/(2+x))` (Abramowitz and Stegun 4.1.27). A correctly
rounded logarithm reproduces the logarithm Excel used for 667 of 1,184 rates; this series
reproduces 1,181. 720 of 720 fresh Excel answers in the shape match. Payments at the start of the
period and payments with a future value use other paths we have not identified.

**Gamma, below 10.** A product from the top, each step stored, ending on a seed value.

```c
double gamma_small(double x) {
    double y = x - floor(x);
    double p = x - 1.0;
    for (double f = p - 1.0; f >= 1.0 + y; f -= 1.0) p = MUL(p, f);
    return MUL(p, gamma_seed(1.0 + y));   /* the seed itself is unidentified */
}
```

Given Excel's own seed values, 14,000 of 14,000 answers match. `GAMMA.DIST`'s long-standing
"chopped exponential" turned out to be nothing of the kind: it was a stored `GAMMALN(a+1)`, a
constant error per shape that had looked like a strange exponential.

**ROUND.** The one that isn't floating-point at heart. `num_digits` is a 32-bit integer, it keeps
`digits + exponent + 1` significant decimal digits, and that sum wraps. Excel works on a 15-digit
decimal reading of the number, so `ROUNDUP` is `ROUNDDOWN` plus one unit, added in binary:

```text
ROUNDUP(-1.125, 2) = -1.1300000000000001
```

Which is not the closest double to -1.13, and is exactly what the two-step program produces.

## What the shape does not say

The hypothesis is not "everything is x87". PMT's logarithm above runs in plain double, and the
rounding functions think in decimal. There is more than one population of code here.

The figures come from 64-bit Excel. As far as we know, Microsoft's x64 compiler does not generate
x87 instructions for ordinary `double` code, so the x87 population may be hand-written assembly or
carried-over routines, kept so that 32-bit and 64-bit Excel agree, with plain-double code being
ordinary compiled C. This is inference from behaviour alone and has not been tested. It would fit
a routine written in the 1990s and hand-tuned later.

Identifying a program's shape is also not the same as having its constants. Where a published
table matches, we cite it. Where none does, the constants are labelled as measured.

## The instruments

Three methods did the work, and all of them follow from taking the shape seriously.

**Ask for consecutive doubles.** Sample a function sparsely and you get a smooth curve. Ask for
4,096 consecutive doubles and you get a sawtooth, and each stored quantity adds its own line to the
spectrum. The lines tell you how many stages the program has before you have written a candidate.

**Bisect to the edge.** A stored value changes at a precise input. Bisecting Excel to one input step
finds that edge, which pins a constant directly, with no fitting. For ERF, the two denominator edges
agree to 1e-15 only if the double rounding is present, and in the fourth digit they disagree
without it.

**Let mathematics check.** The measured ERF coefficients give `N1/N0 - D1/D0 = -1/3` to 1.2e-16.
That is erf's Taylor coefficient, and nothing in the measurement imposed it. A constant read from
bisection that also agrees with a Taylor series is not a coincidence.

## A warning from our own history

In August 2026 this project spent two sessions fitting a "tooth law" for Excel's erf, down to a
prized period of 2.996, and every constant in it turned out to be our own apparatus: the low bits of
the reference model's constant, beating against the scan ladder. Refining the grid made the law
vanish. Today's stored steps are the opposite kind of result. Refining the grid makes them sharper.

The second lesson came on the same day. A rule that no candidate may lose any match looks rigorous.
It rejected correct pieces of algorithms, because a right step can expose an old error that a wrong
step had been cancelling. Variance is the example: the first correct commit lost 451 rows, every
one of them with a tiny sum of squares, and that pattern pointed straight at the missing path.
Losses are information. Our data is tiny next to the space of inputs, and the goal is the program,
not the score.

## Where it stands

Still open at this writing: ERFC above 0.5, NORM.S.DIST's body, GAMMALN below 3 and the gamma seed,
LINEST's rank and QR arithmetic, GEOMEAN, the Bessel functions, BINOM.DIST, and the two PMT paths.
The evidence for each result is in the function notes (`content/notes/NOTES.md`) and in the
DnaCalc commits named there.
