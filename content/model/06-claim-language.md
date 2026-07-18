# Claim language and honesty

Status: draft (H1) · Sources: Handbook CHARTER, Gneiss public-surface vocabulary

## Why this chapter exists

Most references assert; this one claims. The difference is accountability. A claim in the
Handbook names its scope, carries its evidence, and can be challenged. This chapter defines the
words the Handbook uses when it tells you something, so you always know exactly how much is
being said — and how much is not.

## What a claim is

A claim is a recorded statement with:

1. a **subject** — the thing the statement is about (a function, one implementation, one test
   suite),
2. a **scope** — the explicit conditions under which it holds,
3. a **basis** — where it comes from (a verification run, an observation of Excel, a document,
   a decision),
4. a **status** — how the Handbook currently regards it.

Claims live in an append-only record. Corrections never rewrite history; they are new claims
that supersede old ones, with reasons. Every page that renders a claim also renders enough of
its basis to audit it ("Basis: … · receipt … · why?").

## Status is several independent questions

The Handbook does not compress epistemic state, domain warrant, evidence survival, and content
depth into one badge. A displayed answer may carry values on each of these independent dimensions:

| Dimension | Examples | Question answered |
|---|---|---|
| testimony/admission | proposed, admitted, not admitted | Has this testimony entered the published view? |
| current belief | accepted, defeated, contested, missing | What does this view conclude? |
| sensitivity | fresh, stale | Did a consumed input or watched frontier change? |
| domain warrant | documented, observed, suite-exact, characterized | How was the Excel-domain claim supported? |
| evidence survival | grounded, archive-backed, attested, basis lost | What material still survives for inspection or replay? |
| content depth | projected, curated, deep, open problem | How much Handbook work exists here? |

Supersession is decision/history, not a universal current-status label. An open problem is curated
content describing an unresolved question, not a substitute for Gneiss typed missingness or
contest. Contested and stale are ordinary published results, never embarrassments.

## Scoping rules

1. **No unqualified completeness.** "Done", "complete", "verified", and "exact" always name
   their scope. A statement with no scope is a defect.
2. **Compatibility warrant is named.** Passing a finite suite is rendered as `suite-exact`, not
   as universal bit-exactness. It names the observation context (Excel build/channel, workbook
   mode, locale, platform/CPU where material) and verification artifact (suite version, manifest
   hash, capture route). Stronger `characterized bit-exact` or `bit-exact over declared domain`
   wording requires the additional domain/mechanism evidence stated by the claim.
3. **Coverage is counted, not gestured.** Coverage statements are absolute counts against the
   full 534-row function surface ("12 of 534 functions curated"), never percentages of an
   undisclosed base.
4. **Verification is named.** An implementation is shown only after passing its function's
   versioned test suite, and its badge names that suite version.
5. **Documentation is cited, behavior is evidenced.** Where Microsoft's documentation and
   observed behavior differ, the page shows both and states the divergence with evidence.
   Neither is silently preferred.

## Two reading registers

The default **Working** lens uses compact domain terminology. Every working label is true without
being expanded. The **Explicit** lens expands registered qualifications inline with their plain
meaning, scope, and exclusions. A reader may also expand one working term in place.

Working and Explicit are presentation lenses, not evaluation contexts. They render the same
Gneiss answer, context, result hash, and receipt. Changing lenses must not strengthen or weaken a
claim. Evidence and record detail remain one level beneath both through Why, Sources, As of,
Rules, History, Limits, and Replay.

Substantive explanations never exist only in a hover tooltip or transient pop-up. Definitions and
templates live in the versioned `content/vocabulary/working-terms.json` registry so pages cannot
quietly drift into different meanings for the same working term.

## What the Handbook does not do

1. It does not present speculation as fact. Unresolved questions are published as open
   problems, often with the current best hypothesis labeled as such.
2. It does not auto-admit anything. Contributor evidence, automated research output, and
   mechanical synchronization from upstream sources all enter as proposed claims; a steward
   decision admits them.
3. It does not delete inconvenient history. Superseded claims remain in the record.

## Challenging a claim

Every claim-backed fact carries an affordance to challenge it with evidence: a counterexample
vector, an observation from a specific Excel build, or a better implementation. Submissions are
structured (function, challenged claim, evidence kind, reproduction), enter the record as
proposed claims, and are decided and credited. The Handbook is not collaboratively edited, but
it is collaboratively corrected.

## Page vocabulary

| Label | Meaning |
|---|---|
| reference engine: available | OxFunc currently admits the function to its implementation surface |
| reference engine: deferred | OxFunc intentionally defers the function; the reason is shown |
| not yet curated | The entry exists with projected metadata only; no curated content yet |
| placeholder | A metadata field (e.g. a signature) is not yet real and is suppressed, not faked |
| suite-exact, vN | Matched the named oracle bit-for-bit for every vector in suite vN; no claim outside the suite |
| characterized bit-exact | Mechanism and declared input domain are characterized under the named observation context |
| current baseline | The pinned observation context used by the `public-current` view; the complete scope is inspectable |
| open problem | A published, explicitly unresolved behavior question |

## Sources

- Handbook `CHARTER.md` sections 3 and 7 (doctrine and claim-language rules).
- Gneiss public-surface vocabulary (answer, claim, basis, receipt, contested, stale) —
  the accountable-knowledge substrate the Handbook's record runs on.
- The completeness-reporting discipline originates in the OxFunc project charter
  (scope-qualified completion claims); the Handbook adopts its spirit for public claims.
