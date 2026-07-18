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
