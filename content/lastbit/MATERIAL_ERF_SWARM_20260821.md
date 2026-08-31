# The Last Bit — raw material from the ERF/GAUSS/NORMSDIST theory swarm

Story-grade snippets from the 2026-08-21 session (W109 error-function group:
eight rival-theory agents + adversarial verifiers, all offline). Source of
record: OxFunc docs/function-lane/W109_ERF_SWARM_RESULTS_20260821.md and the
handoff it corrects (W109_ERF_GAUSS_NORMSDIST_THEORY_HANDOFF_20260821.md).
Numbers below are as banked; verify against the results doc when drafting.

## 1. The instrument that measured itself (candidate: "876/1024")

The campaign's most-quoted "law" about Excel's erf — the tooth law: sawtooth
teeth dropping −0.85 ULP, ramping +0.145 per step, a mysterious "half-quantum
slip at mantissa 1.5," a bisected period of 2.996·2⁻³⁶ that was pointedly NOT
3·2⁻³⁶ — dissolved in one session. Every constant was the apparatus:

- The ramp +0.145 and drop −0.855 are frac(g·2⁴⁵) = 876/1024 — the low bits
  of the *reference model's own constant* g, beating against the scan ladder.
- The "3× spacing collapse" is the answer's ULP doubling at its binade edge:
  876/2048.
- The "m=1.5 slip" is the ordinary seventh gap of a 6.098-step rotation —
  its position is rotation phase, nothing happens at 1.5.
- The prized "2.996, not 3.000" period fell inside the bisection protocol's
  own bracket geometry: the seed brackets were laid 9 teeth apart, so the
  answer was constrained to [2.993, 2.997]·2⁻³⁶ before Excel said a word.

The kill shot was the campaign's own standing rule, finally executed: refine
the grid 10× (here 128×) and see if the structure survives. The 16,384-row
fine bank contained the old 128-row ladder as every-128th row — same oracle,
bit-identical on all 128 shared rows — and the slow sawtooth everyone had
been fitting was simply *absent* in between. Two sessions of tooth-hunting
had been reading the reflection of the model in the grid.

What survived is better than what died: the real residual is a deterministic
comb that is a function of the mantissa ONLY — row-by-row identical at
e=−30 and e=−40 (corr +0.9999) — which no ln-grid mechanism can produce,
and whose variance splits in two discrete regimes that pin an internal
complement cluster to plain double precision.

## 2. The constant that was a crowd (candidate: "Phantom Mantissa")

For a month the campaign hunted a specific 80-bit mantissa,
0x906eba8214db6c6f — "Excel's gam1(½)" — inferred from fitting the branch-190
path. An 8,192-config enumeration of the published rational never produced
it; the nearest public value was 57 units away. The handoff asked: why does
nothing hit it?

Answer: because it doesn't exist. Exact interval arithmetic (every observed
output bit converted into a rational inequality on g, intersected over
3,046 rows) proved NO single constant explains the tiny-GAUSS corpus — and
worse, three regions of the same function pin three mutually exclusive
constants: the dyadic inputs want g_x−337 ulp, the 1e-15 window wants
+564 and −496 *simultaneously* (empty), and the ERF tiny band wants g_x+6.
Every "measured" mantissa was a corpus-weighted average of a deterministic
±0.7-ULP residual — the fit handed back a different phantom for every diet
of inputs it was fed. (The b9train-era diet averaged to g_x itself, which
is why the number looked so solid.)

The real, measurable constant turned out to be one bit away from the
decade-old suspect: per-op-double evaluation of the published NSWC rational
gives h = 0x3fc06eba8214db6b — and an SSE single-rounded evaluation gives
the identical bits.

## 3. GAUSS is NORMSDIST minus a half — not the other way round
   (candidate: "The Wrapper Was Upside Down")

Every prior model had NORMSDIST = 0.5 + GAUSS (it scored 19/22 on sparse
probes — classic near-identity trap). A 16,495-row NORMSDIST bank already
on disk settled it structurally: 14,122 rows sit OFF the 2⁻⁵⁴ grid that any
0.5+GAUSS or half-(1±erf) publication forces, and 5,280 deep-tail rows are
nonzero full-mantissa values (down to 4×10⁻²⁸⁴) where both rivals force a
hard zero. The true graph publishes RN53(0.5·Q) / RN53(1−0.5·Q) directly —
and GAUSS is *derived* from it by appending a −0.5. The Sterbenz band
(|x| ≲ 0.674), where the subtraction is exact, is precisely where the sparse
probes had matched — the 19/22 was the exactness window of the wrong theory.

Bonus witnesses: the "quantized" GAUSS(2⁻⁴⁹) = 0x3cc8000000000000 that the
handoff flagged as a discriminator falls out as ties-to-even RNE(13/2)=6;
the non-odd GAUSS pairs at ±2⁻⁴⁴ are the positive/negative grid mismatch of
the trailing subtract. And the refutation converts the 16,495-row bank into
an implied-Q oracle (Q = 2·NS exactly) at z values no erfc capture has ever
touched — a free tail corpus.

## 4. The six misses were one tooth through a keyhole (candidate: "Keyhole")

Six stubborn +1-ULP residuals — five just above z = 1/8, one at √2/8 —
resisted 73,728 graph races. The structural lead ("they're all where z² lands
at a binade bottom") died properly: 118 control rows share the property and
are exact. The truth is stranger: internal P runs a ±1-ULP tooth everywhere,
but the GAUSS-decoded Q can only *see* it through a quarter-ULP parity
window that happens to be open in exactly one binade of the corpus. The
admissible shift, intersected over all six rows, is (−1.032, −0.990) —
the integer −1, not a fitted constant. And the frozen banks already held the
corroboration nobody had joined: ERF.PRECISE(1/8) sits exactly 1 ULP below
the plateau model while ERFC.PRECISE(1/8) equals it — the tooth, visible on
the P side, invisible on the Q side, at the same z.

## 5. Two thousand rows, one data point (candidate: "The Chorus That Was One Voice")

The tiny-GAUSS corpus seemed to say the model was 98.4% right on its largest
family — every power-of-two input, ~2,000 rows across 900 binades. But every
power of two shares the same mantissa after the √½ multiply, hence the same
phase of the residual comb: two thousand rows, one measurement, repeated.
The generic-mantissa rows missed at up to 72%. A reminder now banked as
method: count phases, not rows.

## 5b. Same-day answers (candidate: "Ask Excel Once")

The swarm finished its theory pack in the morning; by evening the capture
batch had run on the reference build and five of the seven questions came
back decided — every one landing on a predicted bit pattern, none on a third
value. The 480-way tie that had resisted 11,520 raced graphs broke 14-for-14
in one direction: the tiny route reuses its stored argument algebraically
(the x87-recovery rival predicted one ULP lower on every separator and lost
every row). The one-cell question at z = 1/8+1ulp came back `…8a`, not
`…8b` — the internal P really does sit one ULP below every raced model
there; a month of "six unexplainable residuals" was one tooth, confirmed by
one cell. The NORMSDIST ladder went 64/64 for the upside-down wrapper,
including the flourish row at x = −37.52 where NORMSDIST publishes +0 while
the erfc it is half of stays finite on the same sheet. And the four
"breaker" inputs — mined offline from 406,766 banked rows as the only
places two subtraction stagings could ever disagree — came back 4-for-4 on
the x87 side: the complement is a double-rounded x87 spill, detectable only
below 2⁻¹² and detected on the first try. One deviation, reported verbatim:
the tail separator answered 5 ULP off the best named rational — inside the
predicted noise envelope of the unsplit family, 20 ULP from the split
family. The staging died; the coefficients live to be hunted another day.

## 5c. The grid that wasn't there (candidate: "0.90 of a Bit")

The b26 battery was built as a two-way switch: walk the input across a
binade boundary where one candidate error grid doubles and the other glides
smoothly, and read which way the miss-width moves. It moved neither way.
Width 1.026 below, 1.014 above — no jump, and the rival ln-grid had already
been killed by the same data from another angle. What the switch actually
measured was a third thing nobody had put on the ballot: divide the width
by the result's mantissa and it is constant to 1% on both sides — the
perturbation is *relative*, a fixed grain of 0.90·2⁻⁵³ riding the exp
result at every scale. Ten weeks of asking "which grid?" ended with
"no grid — a proportion." And the number itself is strange in a useful
way: a faithful double routine would show a grain of exactly 1.0; Excel's
internal exp is measurably *tighter* than faithful but looser than correct
— 90% of a last bit, the signature of one specific software routine that
is now the sole remaining unidentified object in the error-function body.

Same reads, two more beats: the fresh-binade ladders confirmed the alias
story out-of-sample (predicted tooth rows [4,10,16,…,95] landed row-for-row,
zero shift) and settled a staging question nobody could reach before —
the complement cluster is variant A, `0.5+RN53(0.5−j)`, its rivals refuted
by breaks that failed to appear where they were predicted. And the tooth
map came back with one insubordinate row: √2/8 measures −2, not the
forced −1 — every campaign keeps one.

## 6. Meta (for the frame story)

One session, eight agents in parallel, each owning one rival theory and one
discriminator, each shadowed by an adversarial verifier that re-derived the
numbers with fresh code (three million tokens of checking; two material
misstatements caught, one vacuous discriminator sent back for re-scoping —
the agent had proposed "fresh" probe inputs that were already answered in
the frozen bank). Net: one theory killed (T1c), one law retired as an
artifact (§4.4), one phantom exorcised (gam1 mantissa), one wrapper
identified (NORMSDIST), one wrapper inverted (GAUSS), and seven capture
scripts queued, each with a bit-exact answer key that names its own death
condition before Excel is asked.
