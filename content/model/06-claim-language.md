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

## The status vocabulary

| Status | Meaning |
|---|---|
| accepted | Reviewed and admitted to the published view; evidence on record |
| proposed | Submitted (by research or by a contributor) but not yet decided |
| contested | Credible evidence points both ways; both sides are shown |
| stale | The claim's inputs have changed since it was verified; re-affirmation pending |
| superseded | Replaced by a later claim; kept in the record for history |
| open problem | An explicitly unresolved question, published as such |

Contested and stale are ordinary published states, not embarrassments. When two sources
disagree — Excel's documentation against observed behavior, or a contributor's counterexample
against our verification — the page says so.

## Scoping rules

1. **No unqualified completeness.** "Done", "complete", "verified", and "exact" always name
   their scope. A statement with no scope is a defect.
2. **Bit-exactness has three mandatory axes.** Any claim that an implementation returns
   exactly Excel's bits names the Excel build, the platform, and the test-suite version it was
   verified against. See the version-axes chapter for why all three are load-bearing.
3. **Coverage is counted, not gestured.** Coverage statements are absolute counts against the
   full 534-row function surface ("12 of 534 functions curated"), never percentages of an
   undisclosed base.
4. **Verification is named.** An implementation is shown only after passing its function's
   versioned test suite, and its badge names that suite version.
5. **Documentation is cited, behavior is evidenced.** Where Microsoft's documentation and
   observed behavior differ, the page shows both and states the divergence with evidence.
   Neither is silently preferred.

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
| supported | The function's core behavior is characterized and implemented against the current baseline |
| deferred | Intentionally out of the current implementation surface; the reason is shown |
| not yet curated | The entry exists with projected metadata only; no curated content yet |
| placeholder | A metadata field (e.g. a signature) is not yet real and is suppressed, not faked |
| verified, suite vN | Passed every vector of that suite version; badge links to the claim |
| bit-exact (build, platform, suite vN) | Bitwise equality with Excel under exactly those three scopes |
| current baseline | The reference Excel build and platform the Handbook currently verifies against |
| open problem | A published, explicitly unresolved behavior question |

## Sources

- Handbook `CHARTER.md` sections 3 and 7 (doctrine and claim-language rules).
- Gneiss public-surface vocabulary (answer, claim, basis, receipt, contested, stale) —
  the accountable-knowledge substrate the Handbook's record runs on.
- The completeness-reporting discipline originates in the OxFunc project charter
  (scope-qualified completion claims); the Handbook adopts its spirit for public claims.
