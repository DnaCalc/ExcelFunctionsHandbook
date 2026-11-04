# GAMMA lane evidence (Excel 16.0.20527, private instance, Value2 doubles)

Method: read Excel on windows of consecutive doubles. The signed error against the true function,
transformed, shows a line for each stored quantity t that is a smooth function of x:
f = |t'(x)| * ulp(x) / ulp(t) mod 1, amplitude = its weight in the answer. Means over a window show
frozen rounding: the steps of the mean are stage binade crossings.

Probe files: probes/*.txt (questions), probes/*.ans (answers with bits). probe.ps1 is the runner.

## GAMMALN
- rec3 (36,000) rec4 (12,000): 6,000 random y on a 2^-49 grid, x = y .. 7+y.
- E(3+y) = st(st(ln(y+2)) + E(2+y)): 6000/6000. E(4+y) with z = st((y+2)(y+3)): 6000/6000.
  n = 5,6,7 ascending z with every product stored: 6000/6000 each. Descending order: 5430-5527.
- E(3) = 0x3fe62e42fefa39f0 is one ulp above st(ln 2): 3 itself is on the rational band.
- E(2+y) is not st(ln(1+y) + E(1+y)): 1546/6000, and noise uncorrelated.
- Stair near 1, 2, 4 (stair1, 30,000): noise at x = 1 + k 2^-52 reaches 4.7 units of 2^-53 at k=1.
  A polynomial in x-1 cannot do that. Lines near 1 pair across 1 (f at step 2^-53 doubles to f at
  step 2^-52), so one polynomial pair spans 1. Lines near 2 do not pair across 2: separate bands.
- scan5, scan6, scan7: mean of the error is piecewise constant with steps of 0.5 and 1 unit.
  Reproduced in a plain Horner simulation: a stored product plus a smaller coefficient rounds the
  coefficient's low bits, which is frozen until a stage crosses a power of two. Excel steps on
  [2,3]: 2.2731 2.3434 2.3981 2.4665 2.6032 2.6306 2.6599 2.7048 2.777.
  No rescaled Cody-Hillstrom n=7 [1.5,4) table reproduces these (best 3 of 9, chance level).

## GAMMA
- g1 (44,000): 2,000 y, x = y-3 ... y+13, 20, 40, 80, 120, 160.
- G(2+y) = (1+y)*G(1+y): 2000/2000.
- G(i+y) = st(t * G(1+y)), t descending stored: 2000/2000 for i = 3..9. Fails at i = 10.
- Noise jumps at 10. rms = 0.225 x above 20.
- g2, g3: dominant line f = frac(2m) (m < sqrt 2) or frac(m): stored x^2, weight x/2.
  Then 8(m-1), 32(m-1), 128(m-1): stored x^4 = z*z, x^8, x^16 (frequency of a stored square of a
  stored value, derived and matched). Right-to-left binary powering of floor(x/2).
- Line at L' dx/ulp(L) with weight 2*frac(x/2): stored ln x inside x^(frac(x/2)).
- Mean bias +0.66 = rounding of binary64 sqrt(2 pi) (+0.6586).
- Stage search, 59,200 answers x in [36,170): 69.5% exact: v = st(p*st(exp(st(f*st(ln x)))));
  u = st(v*em/sqrt(x)) with em stored, sqrt unstored; y = st(u*v); A = st(SP*y); r = st(A*w).
- Line at 1/(12 m x): the stored series w. Below x = 30 w differs from the true series.
- Seed on (1,2): rms 1, smooth mean 0.62 -> 1.0, about five lines, no frozen steps, noise at x = 1+tiny:
  a chain of products from the constant. Families tested and refuted by line scan: ln(x+c), 1/(x+c),
  (x/2) ln(x+c), (x-.5) ln(x+c), exp(+-(x+c)), (x+c)^(x-.5), (x+c)^(x/2), Lanczos exponent q, exp q,
  sqrt(x+c). Curve A: frac(phi) = 0.125 at x = 1, phi crossing an integer at 1.4248, delta(x) close to
  -0.64 (sqrt x - 1) or 8/(x+4). Lines C and E have stationary points at 1.28 and 1.6.
  Cody's rational: 33%, rms 1.2 ulp.

## GAMMA.DIST integer shape, series arm (gd1 windows, bank rows)
- Line at L'/ulp(L) with weight a: stored ln x. Line at f/4 for a=3: stored a*L.
- Stored t - x is refuted: a slow line of amplitude 5 appears. exp(t - x) with t stored: noise 13 -> 1.5.
- The sum is the stored forward sum already in the product.
- Even a behave as a chopped quotient, odd a as a nearest one: a=2 15094 vs 10109, a=3 9337 vs 14988,
  a=4 899 vs 466, a=5 74 vs 227, a=6 532 vs 104. Unexplained: open.

## GAMMA.DIST, the parity resolved
A constant factor per shape, scanned in steps of 1/8 of 2^-53, peaks at -0.875 (a=2), +0.25 (3),
-1.25 (4), +1.625 (5), -2 or below (6). The first is the error of GAMMALN(3), which Excel returns one
ulp above stored ln 2 (+0.79). So the exponent holds the stored GAMMALN(a+1):
r = st(exp(t - x - GAMMALN(a+1))), t = st(a st(ln x)), answer st(r S).
Rows: a=2 15,364 of 26,776, a=3 15,588 of 26,770, a=4 1,088 of 2,051, a=5 654 of 1,142, a=6 691 of 1,151.
The earlier "chopped exponential" and the parity were this constant.
Density: st(r a) / x with the same r, 305 of 750 at a = 2..7, beta 1, against 138 for the density's own
form. With y = x/beta for scale.

## Tables tried for GAMMALN's rationals
My Remez (c/remez.py) reproduces Cody-Hillstrom n=7 on [0.5,1.5] to 8 digits and E = 5.58e-17.
Fresh minimax tables on [0.7,1.5] with (n,m) = (7,7) (6,6) (7,6) (6,7) (5,6) (6,5) (5,5), monic Q: lines near
1 and the steps of the mean do not match. Refuted, within this normalisation.
G(1+y) = st(exp(GAMMALN(1+y))): 618 of 2,000, refuted.

## Round 2: netlib, soft-x87, GAMMALN [2,3]
- netlib.org/specfun/gamma, SHA-256 b469054273fd03c7ee13998a828138578906c10b1fc2398c34b98d1d52b7fb52:
  Cody and Stoltz, October 12, 1989. All 16 P and Q literals in gammaln.rs compared by program: equal.
- soft-x87 at 5165d50c: cargo test -p calc-func --features soft-x87: 586 passed, 0 failed.
- Excel service, x in [2,3], 3 x 4096 at 21 anchors plus runs of 20,000: tracks of line frequency against x
  are polynomials in u = x-2:
    T1 .3071 + .049u + .002u^2   amp .31, whole band
    T2 .1979 + .128u + .021u^2   T3 .0944 - .219u - .012u^2   T4 .4079 + .403u + .026u^2
    T5 .4058 + .613u + .082u^2   T6 .1890 + .906u + .165u^2   T7 .0733 + 1.009u + .275u^2
    T8 .4966 - .189u, L3 .2016 + .0185u (degree 2 stage)
  Low-degree stages are visible, with weights near .3. Horner relation for a stage in variable w:
  2^D phi'_next = w phi' + 2 phi, 2^D phi''_next/2 = w phi''/2 + 1.5 phi'. It links T4 -> T6 and T5 -> T7 with
  w = 2, D = 2, phi4(2) = 1.4079, phi5(2) = 1.4058, so polynomials in x itself, two chains, with
  T4 about -2.06 and T5 about -2.52 in units of 2^(e-1).
- Fresh Remez tables on [2,3] for (n,m) from (4,4) to (7,7), Horner variable x-c for c in {-1,0,.5,1,1.5,2,2.5,3},
  four normalisations: at best 3 of 12 lines. Refuted.
- Next: sawtooth phase gives t(x) mod ulp(t) to about .015 of a unit. Phase-coherent tracking along a
  track, with spacing grown from the frequency fit, reads each stage to its last bit, hence the
  coefficients, without any table.
