# The Last Bit — raw material from the W109 primitive-recovery campaign

Story-grade snippets captured during the 2026-07-16..18 sessions (G3-01/G3-02:
Excel's statistical-distribution substrate). Sources of record: OxFunc
docs/function-lane/W109_G3-01_GRATIO_IDENTIFICATION_20260716.md,
W109_GAMMALN_IDENTIFICATION_20260711.md, W109_CAMPAIGN_RESUME_20260718.md,
docs/notes/CHOPPED_EXP_IDENTIFICATION_STORY.md (full field diary), and the
private session transcripts (OxFunc-History, session d8a86d9c). Numbers below
are as banked; verify against the notes when drafting.

## 1. The expm1 nobody shipped (candidate: "The Function That Wasn't There")

> The expm1 lane is closed end-to-end and committed (278a416): identified,
> landed, and verified at 99.978% (17,996/18,000) in production — with the
> satisfying detail that the real hardware chain outscored the agent's
> idealized model by exactly the rows the microcode hypothesis predicted it
> would. EXPON.DIST's cdf, WEIBULL.DIST's cdf, and the GAMMA.DIST a=1 wrapper
> all shed their refuted 1−exp stagings, and the 4 remaining rows across
> 18,000 are the known idealization class plus one large-x edge.
>
> The story behind it is a gem for the eventual write-up: Excel 2010's CRT
> didn't have C99 expm1, so Microsoft's engineers hand-built one from the
> primitives they did have — Kahan's classic cancellation-free correction —
> and the identification recovered that decision from bit patterns alone,
> including proving the arithmetic is double (the ~0.5-ULP profile is the
> correction's double-rounding) and which of three algebraically-equivalent
> orderings they typed.

Supporting beats: msvcr100/110 export no C99 expm1 (ctypes-confirmed; only
msvcr120+ do). The formula: u = exp(t); t if u == 1; (u−1)·t/ln(u) for
|t| < 1; else u−1. The two alternate orderings of the Kahan quotient scored
82.9%/84.9% — the one they typed scored 99.956% idealized, 99.978% on real
hardware. All-extended arithmetic collapses to ≈CR (80.3%) — the error
profile IS the double rounding, which is what proves the precision. The
diagnostic that cracked it: off-CR density FLAT ~25% for x ∈ [1e-8, 0.1],
dropping to 0 at large x — a relative-accurate routine's signature, the
exact opposite of exp's grow-with-argument reduction error.

## 2. It wasn't a library, it was a rounding mode (candidate: "The 44% Clue")

Two sessions were spent asking "WHICH exp library rounds low?" — fdlibm,
Cephes, every loadable MSVC runtime, AMD's K8/x64 table exps transcribed
from ReactOS/Open64, even the exact msvcr90 9.0.30729 generation Office 2010
shipped, loaded via a Windows side-by-side manifest. Every one refuted; the
32-bit CRT exp rounds one-sided HIGH — the mirror image of Excel.

The break: Excel sat at CR−1 on 20 of 45 decoded rows. 44% ≈ half. What is
one ULP below correctly-rounded about half the time? A TRUNCATED result —
CR−1 exactly when CR rounded up. Not a worse approximation; a different
publication rounding. floor(true exp) scored 38/45 in a twenty-line race,
ten points clear of two sessions of library candidates. Lesson banked as a
standing method rule: race directed roundings, not just nearest.

Full narrative already written: docs/notes/CHOPPED_EXP_IDENTIFICATION_STORY.md
(includes the dead ends: wrong-bitness DLL probes, the hand-converted
constant table with exactly one wrong digit).

## 3. Full circle to the x87 (candidate: "Refuted, Then True")

The campaign's early verdict: "the 2010 stats rewrite is plain SSE2 double —
NOT x87" — reasoned from a race where the x87 chain's round-to-nearest
publication was CR-indistinguishable on 516 rows. Correct reasoning, wrong
hypothesis space: nobody had raced a truncating publication, because nobody
knew publication mode was a free variable yet.

The final identification: ONE x87 F2XM1 chain (2^(x·log2e), the naive
y−round(y) reduction, fldl2e constant bit-for-bit) behind every internal
exp — published round-to-nearest at wrapper sites, truncated at the series
site, handed an 80-bit extended argument on the erf path. The decisive
experiment: a 30,000-row POISSON corpus where POISSON(0,λ) publishes
exp(−λ) RAW — Excel's internal exp made directly visible. The chain
reproduced the 153 off-CR rows 153/153; a same-precision control WITHOUT the
f2xm1 op-graph reproduced 1/153. Same precision, wrong op-graph, one row.
That's what op-graph identification means. Removing the reduction
cancellation dropped it to 13/153 — the cancellation IS the error-growth
mechanism. Final sign-off: 34,000/34,000 held-out through the production
path, running the actual instruction.

The kernel arithmetic is genuinely SSE2 double; it calls the same old x87
transcendentals underneath. The 2010 rewrite adopted new Fortran structure
and linked the old primitives — architecturally obvious in hindsight, which
is how the best identifications feel.

## 4. GAMMALN's genealogy (candidate: "The 1967 Paper and the Literal 0.7")

The published GAMMALN was "a custom Microsoft rational, all published
sources ruled out" — until the structure resolved into a precise genealogy:
the FORM SET of Cody & Hillstrom 1967 (Math. Comp. 21(98)) with Microsoft
fingerprints all over it:
- Cody's PNT68 = 0.6796875 threshold replaced by a LITERAL 0.7 — the
  recurrence identity excel(x) == double(excel(x+1) − log(x)) flips between
  two ADJACENT DOUBLES: 0x3fe6666666666665 exact, 0x3fe6666666666666
  (= double(0.7)) direct. Someone typed `0.7` where Cody had a hex constant.
- GAMMALN(4) = CR(ln 6) EXACTLY — Cody's band-4 form publishing its stored
  anchor verbatim at x = 4. A one-row fingerprint of a code shape.
- "The Stirling switch is in (10.25, 11]" dissolved — there is NO switch at
  12: everything from x ≥ 8 is ONE formula whose coefficient vector is
  fdlibm's w1..w6 VERBATIM inside a staging fdlibm never used (fdlibm's own
  staging scores 46.8%; Excel's 99.83%). Microsoft borrowed the constants
  and rewrote the arithmetic.
- The user supplied the 1967 paper as a fuzzy scan; the OCR self-validated
  against the paper's own error table — each parsed coefficient set
  reproduced its published max-rel-error to a tenth of a digit (E = 1625.2
  vs 1625), confirming all 48 digits at once. The 1967 tables then LOST the
  race (Excel is a third, Microsoft-refit coefficient set) — but the forms
  won. Landed: 0/79 exact (worst 1,370 ULP) → 79% exact (worst 5) held-out.

## 5. The comb that wasn't there (candidate: "Aliasing, or The Grid Looks
Back")

Weeks of erf analysis tracked a "fine comb" — per-binade periods, a
non-monotonic table of them, config-differential partitions. Battery b18
scanned every binade at THREE matched relative resolutions: every "period"
rescaled ×10 whenever the grid did. All of it was the scan grid echoing its
own step through a 26-43% miss density. The banked rule: a period from a
dense scan is real only if a 10× finer grid reproduces it. The replacement
instrument (phase-gradients — miss probability vs position-in-ULP) is
alias-free and became the fingerprint that carried the exp identification.
Companion user steer, now a hard project rule: in a deterministic system
there is no "noise" — everything is signal; distributions are class
constraints; the per-row residual is what the op-graph must reproduce.

## 6. Small perfect experiments (assorted beats)

- The BRATIO port: 20,008/20,008 random+targeted points bit-identical to the
  Python identification spec before touching a single corpus — "the port IS
  the spec" as a verification philosophy (separate the wrong-algorithm
  question from the buggy-port question, always).
- The bgrat shared-z discriminator: probe batteries designed so the inner
  kernel is COMMON-MODE across an a-sweep (z bit-walked to land on the same
  double), turning an entangled two-unknown problem into a readable curve.
  It falsified every candidate family — including after the primitives were
  pinned — which is itself the finding: the wall is the body op-graph.
- The R6034 dialog: a probe harness left msvcr90.dll beside an exe; Windows
  raised the classic side-by-side loading error at the user mid-campaign.
  Reverse-engineering the 2010 CRT occasionally re-enacts 2010's bugs.
- BETAINV's worst error across the campaign: +1,910,580 ULP (an early-stop
  bisection artifact) down to +5. Six orders of magnitude in three landings.
- Cross-view algebra as a free instrument: probing the SAME y at eight
  integer-a slices makes the internal log solvable and the exp readable —
  multi-view identities turn one oracle into many.

## Candidate titles / angles

- "The Last Bit: The Function That Wasn't There" (expm1/Kahan)
- "The Last Bit: The 44% Clue" (chop discovery)
- "The Last Bit: Refuted, Then True" (x87 full circle; on priors and
  hypothesis spaces)
- "The Last Bit: The Literal 0.7" (GAMMALN genealogy; code archaeology from
  bits alone)
- "The Last Bit: The Grid Looks Back" (aliasing; measurement discipline)

## 7. Reading the source line off the bits (candidate: "Division First")

The WEIBULL.DIST pdf identification, four-lane sweep 2026-07-18. The naive
form matched every β=1 test block at 100% and failed a third of everything
else — including a β=2 block where every β-related operation is EXACT in
floating point. That "impossible" failure was the tell: the only way a
formula can be perfect at β=1 and wrong at β=2-with-exact-arithmetic is if
Excel computes the powers of x and β SEPARATELY — because pow(1, anything)
= exp(0·ln 1) = 1 exactly, β=1 makes both forms collapse to identical bits.
The textbook writes the Weibull density α/βᵅ·x^(α−1)·e^(−(x/β)^α), and
Microsoft's engineer typed it VERBATIM. The final race enumerated every
association order × every spill pattern: the winner — division first,
left-to-right, every op double-rounded through a spilled local — hit
1,600/1,600. You can reconstruct the C expression, token by token, from
rounding patterns alone. Held-out: 5,999/6,000.

## 8. The window that couldn't see (candidate: "Route-Blind")

A prior session had "proven" POISSON's pmf route via the k=0 window —
POISSON(0,λ) publishes exp(−λ) raw, 34,000/34,000. This session found the
proof was empty: at k=0, the direct product λ⁰·e^(−λ)/0! and the log-route
exp(0·lnλ − λ − ln 0!) are THE SAME EXPRESSION — 0·lnλ = 0 exactly. The
window validated the exp primitive perfectly and said nothing whatsoever
about the route. Method rule banked: a window that publishes a common
subexpression proves the subexpression, never the route. (The route turned
out to be two-headed: a direct product for k=1, and Loader's saddle-point
for k≥2 — one worksheet function, two algorithms.)

## 9. Recognizing an author (candidate: "The 0.1 Branch")

BINOM.DIST general-k refused every route family at the exact-bit level —
direct, recurrence, log-composed, 256 spill-mask graphs. The break came
from control flow, not arithmetic: Catherine Loader's dbinom (2000, the
algorithm R uses) computes the k=0 case as n·ln(q) — except when p < 0.1,
where it switches to a deviance form to protect accuracy. A 400-row capture
at p < 0.1: Excel switches formulas at exactly that boundary, 383/400.
You can identify an ALGORITHM'S AUTHOR from where its branches fall.
Excel 2010's BINOM.DIST is Loader's dbinom — and POISSON k≥2 was already
matching her dpois bit-for-bit through ±50-ULP saddle-point roundings.

## 10. Reverse-engineering ourselves (candidate: "The Shortcut")

The lane-3 re-score found the enemy within: OxFunc's own pre-campaign
integer-shape "fast path" for GAMMA.DIST — a tidy closed-form sum any
numerics textbook would endorse — had been silently overriding the
painstakingly identified GRATIO kernel for every integer shape parameter.
At small x it also cancels catastrophically: ±4,400 ULP. Months of corpus
scores had measured the identified kernel directly and never noticed that
production took a different road. The fix was deletion. Twin lesson from
the beta side: a capture proved Excel has NO integer fast path at all —
the 2010 rewrite trusts its continued fractions everywhere. Sometimes the
bug is the optimization you were proud of.

## 11. Passed the gate, lost the election (candidate: "Fresh Rows")

GAMMALN band-2, lane 4. The search produced a coefficient candidate that
passed every formal check: held-out total up (317>316), band score up,
worst-case improved, zero regressions. Textbook promotion material. But its
staging had been CHOSEN by peeking at those same held-out rows — so before
landing, 1,600 never-probed arguments were captured from live Excel. The
verdict: the gate-passing candidate scored WORSE than the incumbent
(505 vs 518), while the held-blind pick — selected purely by which op-graph
had the lowest residual noise floor, never by score — won at 549. The
noise-floor principle beat the scoreboard. Bonus archaeology: the fresh
gate also settled the op-graph — Excel's band 2 runs fully-continuous
x87 extended, one final rounding, the same evaluation class as band 4;
the code's two middle bands are siblings after all. (And the audit found
the original fit had quietly included 400 held-out rows — the
contamination that made the old model look better than it was.)

## 12. The bell that meant extended (candidate: "Two Functions, One Wall")

BINOM's exp-argument was recovered to a twentieth of a ULP by taking the
logarithm of every published probability at 200 digits. Racing 432 staging
candidates got 87.5% of rows inside the error window — and then the residual
histogram refused to behave: deviations at ±0.25 and ±0.5 of an argument-ULP.
Two IEEE doubles cannot differ by a quarter of an ULP. The argument reaching
the exponential was not a double at all. Model by model, the codegen story
fell out: Loader's `lc` and `lf` really are double locals, faithful to her C
source — but the final `lc − 0.5*lf` is evaluated on the x87 stack at 64-bit
and flows into the exponential unrounded. 439 of 600 rows then sat at
exactly d = 0.00. The end-to-end scoreboard still refused to close, and the
diagnosis of THAT refusal was the session's real prize: the one remaining
unknown — how the F2XM1 chain behaves when handed an 80-bit argument — is
the very same unknown that has held the erf lane at its 67% plateau for
weeks. Two functions, one wall. And BINOM brought the siege equipment: five
hundred rows where the extended argument is exactly known and the published
answer is visible — the oracle the erf lane never had.

## 13. The acquittal (candidate: "Sliding the Argument")

The extended-entry exponential had been the prime suspect for weeks — the
erf lane's 67% plateau was blamed on it, BINOM's 34% end-to-end was blamed
on it. The trial: take each BINOM row's recovered 80-bit argument and SLIDE
it, one 64-bit-ULP at a time, ±70 steps, asking the real hardware chain at
each step whether it produces Excel's exact published bits. If the chain
were guilty, no slide position would work. Verdict: for 76% of rows a
consistent argument EXISTS — the chain, given the right 80-bit input,
reproduces Excel perfectly. The suspect walked free. The real culprit is
eleven invisible bits: the argument's content BELOW double precision,
where the implied-argument decode cannot see but the chain's rounding can.
Every simple hypothesis for those bits failed in a different way, and the
holdouts cluster at extreme probabilities — the investigation continues,
but the wall has a new name and a much smaller cross-section. Best line in
the case file: the same acquittal likely applies to erf — weeks of
"chain microdetail" theories may all have been argument-side.

## 14. Number for number (candidate: "The Same Wall Twice")

The acquittal experiment was repeated at the other crime scene. The erf
lane's argument — half the logarithm of z², carried at 80 bits — was slid
one 64-bit-ULP at a time through the full C10r pipeline on 1,508
development rows. The verdict came back with numbers that could have been
photocopied from the BINOM case: a consistent argument exists for 76.5%
of rows (BINOM: 76%); 23.5% are unreachable by any argument shift
(BINOM: 24%); the argument deviations center on zero and scatter below
double precision. Two functions, written by different hands from
different papers — Loader's saddle-point and DiDonato-Morris's series —
publishing through the same 1990s exponential, blocked by the same eleven
invisible bits. Weeks of theories about the chain's microcode dissolved
in an afternoon: the chain was never wrong. What remains is a question
about compilers, not mathematics — how 32-bit code composes the last
line before calling exp — asked once, answerable everywhere.

## 15. The missing function, patched twice (candidate: "Two Logs")

The 2010 C runtime shipped without expm1 — and Excel's engineers built
Kahan's correction by hand (story #1). The same runtime also shipped
without log1p. This time the patch was plainer: where R's binomial code
says log1p(−x/n), Excel computes ln(n−k) − ln(n) — two separate hardware
logarithms. Nobody could see this for weeks: the difference hides below
double precision in the exponential's argument, one ULP of a term that
itself is only a correction. What betrayed it was a designed experiment —
six different (k,n) anchors steered by a 200-digit bisection to publish
from THE SAME argument, where any honest exponential must agree. The
bodies disagreed, anchor by anchor, in a pattern of exactly ±1 ULP of the
lf term: (0,0,0,−1,0,+1). Precisely one realization produces that vector.
The identification broke a three-lane plateau by eleven points in one
stroke, and closed windows that had scored 0/400 under ninety-six
candidate op-graphs to 400/400. Same missing header, two engineers, two
different repairs — both recovered from rounding patterns alone.

## 16. The parenthesis (candidate: "Order of Operations")

After the two-logs identification, one discrete mystery remained: on
certain rows — deterministically, row by row — Excel's five-term log-sum
landed one ULP away from every model. The hunt tested eleven ways to
parenthesize the subtraction chain. One of them, and only one, predicted
403 of the 475 observed flips with zero false alarms: Excel's code
computes ((s1 − s2) − (s3 + b1)) − b2 — someone grouped the third and
fourth terms in parentheses, perhaps for readability, perhaps by habit.
That invisible pair of brackets moved one bit in a billion-scale
computation often enough to be found, and its neighbors on the
parse tree lose by twenty points. Three source lines recovered in one
day, each from a different kind of shadow: a missing function, its
homemade replacement, and now a parenthesis.

## 18. The Days That Disagree (G6-03d, PRICE at Actual/360 — 2026-07-20)

Ask Excel how many days lie between settling a bond on 2020-09-20 and
its next coupon on 2021-01-01, and it answers: `=COUPDAYSNC(...)` says
103. Ask Excel to price that same bond on the Actual/360 basis and it
prices it as if the answer were 99.

Both numbers are defensible. 103 is the actual day count. 99 is what
you get if you insist the coupon period is exactly 180 days (360/2, the
whole point of a /360 basis) and subtract the 81 days that have already
accrued: 180 − 81 = 99. The Analysis ToolPak's PRICE routine apparently
contains a single line — `dsc = e − a` — that derives the
days-to-next-coupon from the period length and the accrued days, for
every basis. On 30/360 and actual/actual bases the derivation agrees
with the direct count, so nobody ever sees it. On Actual/360 and
Actual/365 — where the period length is a fiction (180 or 182.5 days)
but the accrual is real days — the derived number and the real number
part company, and the price moves by whole cents: 114.5887 against the
faithful-formula 114.5504. Not a last bit — a first-decimal
discrepancy, hiding in the two least-used day-count bases.

Every reimplementation we checked — including the F# library that
half the open-source world ports its bond math from — computes the
direct count, faithfully implementing the documented formula and
faithfully disagreeing with Excel by 3.8 cents per hundred. The
documentation shows a quantity called DSR; it does not show the
subtraction. And Excel itself publishes the direct count through
COUPDAYSNC while its own PRICE, in the same workbook, on the same
arguments, uses the derived one. The oracle disagrees with itself —
and both of its answers are deterministic, reproducible, and now,
bit for bit, ours.

(One hypothesis, one witness, first try: the guess `180 − 81` landed
within rounding of Excel's published 114.5887 before we had captured a
single new row. The 7,472-row settlement×yield lattice then confirmed
it at the bit level: 0/3,664 exact became 3,474/3,664 at basis 2,
0/3,664 became 3,446/3,664 at basis 3, residuals collapsing from
trillions of ULPs to ±4.)

## 19. The Same Pow Twice (G6-03d staging — 2026-07-20)

After the day-count semantics fell, ~400 bond prices still wobbled by a
bit or two. The culprit was the fractional discount power — and the fix
was already sitting in our own source tree. The x87 exponent chain we
had spent a week extracting from WEIBULL and the distribution family —
exp(RN53(RN64(y·ln x))), the 1993 CRT pow — turned out to be, byte for
byte, the routine pricing bonds. Two code paths written years apart in
Redmond, two reverse-engineering lanes months apart here, one shared
library function at the bottom of both. When the identification is
right, it stops being a per-function fact and starts being a fact about
the binary — and every later lane gets cheaper: this one took an
afternoon because the hard primitive was already on the shelf.

## 20. The Vanishing Coupons (G6-02, ACCRINT — 2026-07-20)

ACCRINT's last parameter, calc_method, is documented as a choice of
starting line: TRUE accrues from the issue date, FALSE from the first
interest date. The documentation is wrong on both counts. Both paths
start at issue. What FALSE actually selects — recovered from 145,620
live oracle rows — is an older, stranger arithmetic: one flat fraction,
no coupon schedule, except that when the issue date falls in an earlier
coupon period than the schedule's last pre-first-interest date, every
whole period in between simply vanishes from the sum. Bonds issued
three quarters early accrue LESS than bonds issued one quarter early.
For settlements early enough, the function returns NEGATIVE accrued
interest — interest that un-accrues — and Excel publishes it without
comment. We pinned one such witness in the regression suite:
0xc01c_4333_3333_3333, accrued interest of minus seven dollars.

And TRUE has its own tell: the per-period fractions are summed
BACKWARD, settlement first, issue last. Forward summation — the way
anyone would write it — lands one bit off on nine percent of rows.
Somewhere in a Redmond source file there is a loop that walks the
coupon schedule in reverse, probably because of how the dates were
generated, and twenty years later that iteration order is the
difference between matching a spreadsheet and not.

The held-out battery earned its keep here twice. The first model of the
vanishing-period rule measured the remainder from the wrong anchor and
scored 99.99% on everything it had seen; 14,025 fresh rows disagreed.
The revision scored 99.99% on data it had never touched. Same headline
number, entirely different epistemic weight.

## 21. The Order of Adjustments (G6-03c, DURATION — 2026-07-20)

Two functions in the same codebase both compute "30/360 days between
two dates," and both follow the same published convention: days on the
31st count as the 30th, February's end counts as the 30th. They differ
only in which adjustment they apply first. Ask them for the days from
February 28th, 2025 to March 31st, 2025 and one says 30, the other 31 —
because one collapses the 31 after noticing the start date was
month-end, and the other checks for the 31 while the start is still a
28. One integer apart, once a year, on month-end bonds only.

Excel's DURATION uses the second ordering. Our port used the first. The
error survived a 6,360-row identification corpus without appearing once
— none of those settlements landed on a month-end — and then detonated
on a held-out battery at twenty-five trillion ULPs, the largest
residual this campaign has measured, all from a day count of 30 where
Excel had 31. The fix was one line. The lesson wasn't the line; it was
that we only met this bug because the gate battery was designed by
asking "what date shapes has no corpus stressed yet?" — the same
question, asked one battery earlier, would have found nothing, and
asked one battery later, would have found it in production.

---

## §22 — The Siblings Who Disagree (PMT vs FV/PV; the metamorphic harvest)

PMT, FV, and PV all solve the same five-variable equation — the time-value-of-money
balance `fv + pv·(1+r)^n + pmt·(1+r·type)·((1+r)^n − 1)/r = 0`. Rearrange for `fv`,
you get FV; for `pv`, PV; for `pmt`, PMT. Same equation, three faces. A reasonable
person assumes one shared helper computes `(1+r)^n` and the annuity factor once, and
the three functions just divide it out differently.

Excel does not do that.

FV and PV are bit-for-bit exact when you compute `(1+r)^n` the naivest possible way:
`binexp` — square-and-multiply in plain double — then `(P−1)/r` for the annuity
factor. 149 out of 149. 48 out of 48. No transcendentals, no cleverness. The forward
factor is *pinned*: we can read Excel's exact internal `P` and `(1+r·type)·q` straight
off the FV oracle by asking `FV(r, n, pmt=1, pv=0)` and `FV(r, n, pmt=0, pv=1)`.

So we harvested Excel's own numbers and fed them into PMT's obvious formula,
`−(pv·P + fv)/(tf·q)` — using *Excel's* P, *Excel's* q. If PMT shared the helper, it
had to close. It scored zero out of a hundred and nine on every small-interest-rate
row. Zero.

That zero is the whole story. It means the programmer who wrote PMT looked at
`(P−1)/r` — which loses almost all its significant digits when `r` is a few
millionths and `P` is a hair above 1 — and refused to ship it. PMT was given a
*different, cancellation-safe algorithm* than its own siblings: a discount form built
on `expm1(−n·log1p(r))`, where the dangerous subtraction never happens. FV and PV
kept the naive form because, multiplying rather than dividing, they never felt the
cancellation. Three faces of one equation, and one of them was quietly rewritten by a
more careful hand.

The reverse-engineer's lesson is sharper than the trivia: **an intermediate you
cannot see inside function A may be sitting in plain sight inside its algebraic
sibling B.** FV's oracle is a window into the annuity factor. The window told us,
unambiguously, that PMT wasn't using it — and that single fact converted a five-year-
old "±1 ULP, cause unknown" into "the residual is entirely in one transcendental, and
here is which four routines it is *not*." The siblings disagree, and the disagreement
is the measurement.

---

## §23 — Quotient First (the PMT combine, and a proof from seven numbers)

For most of a day the payment function PMT would not yield its last bit. The
formula is not in doubt — a payment is present-value times rate divided by the
discount term, `pmt = pv·r/em`. The question was only the *order* the machine
does it in, because at the ±1-ulp level order is everything: `(pv·r)/em` and
`pv/em·r` round differently, and only one is Excel's.

We tried them all with the presumption everyone shares — you compute the numerator
`pv·r` first, then divide. Every such form, raced against 256 consecutive inputs at
a fixed rate, topped out around two-thirds and stuck. Shared divisor, extended
divisor, fused product, reciprocal-multiply — all the same wall. Two independent
searches, thousands of probes, and a growing suspicion the answer wasn't in the
family at all.

The break came from asking a different question: not "what constant fits?" but
"what does the *lineage* do?" Excel's financial functions descend from the
Visual-Basic/BASIC runtime, and that code has a tell — it writes the quotient
first and multiplies by the rate *last*: `(pv / em) * r`. Divide, then scale.

What made it certain wasn't the 256-point fit — though that went to 256 out of 256
at every rate the instant the order flipped. It was a proof from *seven numbers*.
At `pv = 1.0` exactly, the product `pv·r` is just `r` — a fixed value, with no
freedom. So in any product-first scheme the very first stage is *pinned*: it must
step through the grid at a slope the rate dictates, `{0, 2, 3, 5, 6, 8, 10}`, and
no divisor downstream can do anything but rescale that fixed staircase. The
observed staircase was `{0, 1, 3, 4, 6, 9, 11}` — and it contains a **single step
of three**. A one-rounding rescaling of a `{1,2}`-step sequence can never produce a
three. Two coarse, input-dependent roundings are *required* — which is exactly what
divide-then-multiply has and product-then-divide does not. Seven consecutive
payments, and the entire product-first family is dead on arrival, not by score but
by contradiction.

The lesson is the one this whole project keeps relearning: the target is
human-written, and humans wrote it in an order. When the parametrized search
plateaus, stop adding parameters and go read what the *lineage* actually did — then
prove the alternatives impossible from the cleanest handful of bits you have.

---

## §24 — The Skeleton Key (a power of two opens the black box)

To finish PMT we needed to see a number Excel never shows you: the internal
discount term `em = (1+r)^-n − 1`, computed deep inside the payment formula and
never surfaced. You cannot probe it. You can only probe `PMT` itself, and `PMT`
launders `em` through a divide and a multiply before you see the result:
`pmt = RN( RN(pv / em) · r )`. Two roundings stand between you and the number you
want, and either one can hide a ULP.

The key was to pick the rate. If `r` is a power of two — `2⁻⁵`, say — then the
final `· r` is *exact*: multiplying a double by `2⁻⁵` only decrements its exponent,
it cannot round. The two-rounding chain collapses to one. Now `pmt · 2⁵` recovers
`RN(pv / em)` bit-for-bit, and by walking `pv` through 256 consecutive doubles and
intersecting the constraints each one places on `em`, you pin Excel's private `em`
to under a hundredth of a ULP — without ever seeing inside the function.

It generalizes to a rule worth keeping: **when a hidden intermediate is trapped
behind a chain of roundings, find the input that makes one of those operations
exact, and the chain springs open.** A power-of-two rate turned out to be a
skeleton key to the whole annuity engine — it pinned `em`, and through `em` it
pinned the last primitive, `log1p`, one rate at a time.

## §25 — The Beautiful Dual That Wasn't, and the Bug at the Bottom

By the end, PMT was solved down to a single unknown: Excel's internal `log1p`. And
`log1p` turned out to be *wrong* — a faithful routine, good to about six-tenths of a
ULP, but not correctly rounded. After a full day spent chasing our own arithmetic,
the last thing standing between us and a bit-exact match was a rounding error inside
*Excel's* code that we now had to reproduce exactly. You do not get to fix it. The
whole point is fidelity: if Excel is a half-ULP high, you must be a half-ULP high, in
precisely the same places.

The prettiest wrong answer of the session came here. We already knew Excel's `expm1`
was Kahan's clever trick, `(u−1)·t/ln(u)`. There is a famous *dual* — Kahan's
companion `log1p`, `ln(u)·r/(u−1)`, the same idea run backwards — and any library
that ships one usually ships the other on the next line. Better still, its error has
a signature: the routine hinges on `ε`, the tiny rounding left over when you form
`1 + r`, and `ε` is a sawtooth whose period is exactly the binade — 256 consecutive
doubles at `2⁻⁸`, eight at `2⁻³`. Our dense measurements showed a smooth ramp of
period 256 at `2⁻⁸` and a ripple of period eight at `2⁻³`. The hypothesis *predicted
the data*.

And it was still wrong. The end-to-end test refuted it in one shot: every form of the
companion trick made the well-behaved cases *worse*. The tell was hiding in the
lattice. At the power-of-two rates where we'd measured the deviation, `1 + r` is
exact — `ε = 0` — so the companion's correction factor collapses to 1 and the trick
degenerates to a plain logarithm; those rates could never tell the two apart. On the
*other* rates, where `ε ≠ 0` and the trick actually does something, Excel is
correctly rounded and the companion is not. A period law that matched, a provenance
that fit, an algebraic elegance that begged to be true — and a control that killed it.
The routine is still unnamed. It is faithful, non-correctly-rounded, matches nothing
standard, and it is the one primitive left to fingerprint. The bug at the bottom of
the payment function is Excel's, and it kept its shape.

## §26 — Where a Function Refuses to Answer

`RATE` solves for an interest rate by iterating, and on some inputs it gives up and
returns `#NUM!`. That refusal is not noise — it is the iteration narrating its own
control flow. The *pattern* of which inputs converge and which fail is a fingerprint
of the solver, printed on the outside of the black box.

We first guessed the classic secant method, and on the plain numeric outputs it
looked plausible. The error basin said otherwise. A two-point secant, seeded a
particular way, false-converges on exactly the inputs where Excel returns `#NUM!` —
so a secant model *converges where Excel errors*, and errors nowhere near the right
places. Forward-difference Newton in rate-space, the sibling of the method Excel's
`IRR` uses, reproduced the failure boundary on all 116 test cases without a single
miss. Two models can agree on every answer a function gives and still be told apart
by the answers it *declines* to give. The shape of a function's silence is data.

## §27 — The Bug That Wasn't There

The section above this one ends with a confident verdict: the last unknown in the
payment function is Excel's own `log1p`, a faithful-but-not-correctly-rounded routine,
and the bug at the bottom is Excel's to keep. That verdict was wrong. There was no
log1p bug. This is the story of how a phantom survived a full day of careful work, and
what finally exposed it.

The mistake was structural, and it is worth naming because it is easy to make. We never
measured `log1p` directly. We measured the *payment* — a number that flows out of
`log1p`, then through `expm1`, then through three roundings of the combine — and we
back-solved for what `log1p` must have been. When the back-solved values disagreed with
a correctly-rounded logarithm, we wrote down "non-CR log1p." But a residual attributed
to the first link in a chain is only as trustworthy as your certainty about every link
after it. We had proven the *later* links on friendly inputs and assumed they held
everywhere. They did not.

The blade that cut the knot was almost embarrassingly simple. `log1p(r)` is `ln(1+r)` —
but only when `1+r` can be formed without error. Choose `r` so that `1+r` is *exactly
representable* — any `r` that is a small multiple of a power of two — and the two
functions coincide with nothing lost in between. And `ln` we can ask Excel about
*directly*: it is a worksheet function. So we asked. A hundred and forty-eight exactly
representable points, spanning the whole region where the payment function had looked
wrong, and for every single one Excel's `LN(1+r)` was correctly rounded. Not faithful-
but-off. Correct. The routine we had spent a day fingerprinting, the beautiful dual we
had refuted, the sawtooth we had tried to name — all of it was chasing a rounding error
that was never in the logarithm.

So where was it? We asked Excel for its exponential too, at the exact arguments the
payment function feeds it, and it matched our x87 model to the last bit — every one.
That left exactly one suspect standing: `expm1`, the cancellation-free correction, on
the narrow branch where its argument is small. And there, with the logarithm proven
correct and the exponential proven exact and the intermediate value known to the bit,
the plain double-precision Kahan form still misses about three inputs in ten — and *no*
rearrangement closes the gap. Not the classic association, not the companion, not the
extended-precision staging, not the hardware's own base-two shortcut. Twelve algebraic
faces of the same formula, and the best of them agrees with Excel seven times in ten.
The residual is real, it is a single ULP, and it lives in a place where every input is
observable and every operation is known — which means it is not a mystery of hidden
state but a genuine wall: Excel's `expm1` reaches its small-argument answers by a
sequence of doubles we have not yet reconstructed.

The lesson is the cheaper one, and it generalizes past this function. When a chain of
operations produces a wrong number, do not infer which link failed — *isolate the link
and ask it directly*. There is almost always an input that collapses the chain: a power
of two that makes a multiply exact, an integer that makes a rounding vanish, or — as
here — an argument that makes two functions become one so you can query the one you can
actually see. We had that key the whole time. It took a day of chasing our own tail to
pick it up. The bug at the bottom of the payment function is still Excel's, and it still
kept its shape — but it was never wearing the coat we'd hung on it.

## §28 — The Function That Keeps Its Own expm1

We had proven the last unknown was `expm1`, the routine that computes `e^x − 1` without
losing precision when `x` is small. The natural next question: which `expm1`? Excel has
one — surely the payment function just calls it. So we found a way to ask Excel for its
`expm1` directly. There is no `EXPM1` worksheet function, but there is a back door: the
exponential distribution's cumulative form, `EXPON.DIST(x, 1, TRUE)`, is exactly
`1 − e^−x`, which is `−expm1(−x)`. Feed it the arguments the payment function uses, negate,
and you are reading Excel's `expm1` with nothing in between.

Excel's `expm1`, read this way, was the textbook Kahan recipe we already had — it matched
our model on 232 of 234 points. Clean. And then the twist: the payment function's own
value, at the *same arguments*, matched that same Kahan model only 165 times. Two readings
of the same mathematical function, from the same spreadsheet, disagreeing. The only way
that happens is if the payment function does **not** call the `expm1` the rest of Excel
uses. It carries its own — a private copy, from an older layer of the code, subtly
different from the public one. The "expm1 we solved" three sessions ago was Excel's; it
just wasn't *this* function's.

That reframed the hunt, and we came at it from every side at once — a dozen investigators
in parallel, reading the source of every spreadsheet that ever cloned Excel's finances
(LibreOffice, Gnumeric, the old Visual Basic runtime), and grinding the number itself
through every arrangement of every operation. The verdicts converged, and every one was a
"no." Not the extended-precision hardware path. Not a polynomial. Not a truncation. Not
the correctly-rounded answer — Excel is, remarkably, *less* accurate than a correct `expm1`,
which means it is nobody's library routine but a bespoke one. And crucially: the error is
not random. Excel's payment `expm1` always leans the same way — it *underestimates*, every
time, pulling the magnitude a hair toward zero. We confirmed that lean survives when you
move off the tidy power-of-two rates onto ragged ones, so it is not an artifact of where we
sampled; it is the fingerprint of the routine itself.

So the payment function is closed down to one primitive, and that primitive is a small,
consistent, deliberate-looking imprecision in a copy of `expm1` that Microsoft wrote once,
by hand, before the standard library had the function — and then never touched again. It
is the oldest code in the room, and the last bit belongs to it. When we reproduce Excel, we
will reproduce this lean toward zero too, in exactly the places it leans, because that is
what fidelity to a thirty-year-old routine means: not the answer it should have given, but
the one it does.
