# The Handbook of Excel Functions — Charter

Status: `active`
Established: 2026-07-17

## 1. Mission

**The Handbook of Excel Functions** is a comprehensive, honest, evidence-backed public reference
for every Excel worksheet function and operator — the Abramowitz & Stegun of Excel functions.

For each function it aims to provide:
1. a uniform description of how the function is called (admission, coercion, evaluation, adaptation),
2. identity and metadata: signature, category, behavioral axes, XLL identity, localized names, history,
3. links to the official Microsoft documentation (English and localized) — and an honest account of
   where empirical behavior agrees with or diverges from it,
4. implementations in four flavours, in multiple languages, showing only what actually exists,
5. versioned, downloadable, independently checkable test suites,
6. curated external references (books, papers, code sources),
7. the research record of how bit-exact behavior was identified ("The Last Bit" series),
8. visual plates: function graphs and signed-ULP residual portraits (the ULP Atlas).

## 2. Identity and Relationship

1. The Handbook is its **own knowledge system**. It is not a wiki, and it is not a derived
   presentation of OxFunc.
2. **OxFunc is the strongest evidence source, not the definition of the Handbook.** The Handbook
   describes Excel functions as computational and mathematical objects, drawing on OxFunc,
   Microsoft documentation, the Welinder/Gnumeric record, published numerical literature, and
   community-submitted evidence. It carries facts OxFunc does not own (history, pedagogy,
   external implementations, natural-best algorithms).
3. The Handbook may disagree with any source, including OxFunc. Disagreement is rendered as a
   contested claim with evidence on both sides — that is the system working as intended.
4. The claims plane is a **Gneiss ledger** (`Gneiss.Cell`): append-only testimony, versioned
   contexts, deterministic belief views, receipts. The published site is generated from the
   ledger plus the content organs.

## 3. Doctrine

1. **Honesty.** Show what exists; promise nothing. Uncurated, placeholder, and deferred entries
   are labeled as such, plainly. Claim language is always qualified (see section 7). Contested
   and stale are first-class published states, never suppressed.
2. **Comprehensiveness.** All 534 published rows (511 functions + 23 operators) are present from
   the first publication, each honestly labeled with its curation depth.
3. **Evidence.** Every load-bearing fact is a claim in the ledger with typed provenance, or
   curated content that cites its sources. Bulk evidence lives in its organ (Git, suites,
   external archives); the ledger records claims and precise references.
4. **Independent checkability.** Test suites are versioned, hash-manifested, downloadable, and
   oracle-provenance-stamped, so any claim can be re-verified anywhere — including on platforms
   with no Excel available.
5. **Synchronization, not regeneration.** Only the `data/` projection organ is mechanically
   rebuilt from OxFunc. Sync produces a diff of proposed claim updates, decided by the steward.
   Handbook-owned organs are never touched by sync. See `OPERATIONS.md` section 3.
6. **Interaction toward improvement.** The Handbook welcomes evidence: counterexample vectors,
   Excel-build observations, implementation proposals. Submissions enter the record as proposed
   claims — reviewed, decided, and credited. There is no anonymous direct editing.

## 4. Scope

1. The published Excel worksheet function/operator surface: 534 rows on the current reading
   (511 functions + 23 operators), including the 17 currently deferred rows, which appear with
   their deferral reasons.
2. Four implementation flavours per function:
   - `excel-bitexact` — returns exactly what Excel returns, quirks included; always scoped to
     Excel build and platform,
   - `natural-best` — the balanced state-of-the-art implementation of the intended function;
     the recommended default for new code,
   - `portable-reproducible` — bit-identical across CPUs and compilers,
   - `math-correct` — correctly rounded reference where feasible; the yardstick.
3. Out of scope: formula grammar and cell/workbook semantics (OxFml/OxCalc lanes), and any
   duplication of OxFunc's role as the semantic implementation lane.

## 5. Organs and Ownership

Three organs (detail in `OPERATIONS.md` section 2):
1. **Content/evidence organ** (Git): `data/` (mechanically synced projections), `content/`
   (curated prose: call-model chapters, function pages, "The Last Bit" series, exhibits),
   `implementations/`, `vectors/`.
2. **Claims plane**: `ledger/` — the Gneiss ledger, its canonical JSONL export, and the declared
   `efh.*` vocabulary.
3. **Presentation**: `site/` — generated, dependency-free static HTML plus static JSON API and
   downloadable suites.

## 6. Licenses

1. Code (tools, implementations, tests): MIT, copyright DNA Kode — consistent with the DnaCalc
   repo family.
2. Content (prose, plates, suites, data projections): Creative Commons Attribution 4.0
   (CC BY 4.0).
3. See `LICENSE.md` and `LICENSE-CONTENT.md`.

## 7. Claim Language Rules

1. No unqualified "done", "complete", or "verified". Every such claim names its scope.
2. Bit-exactness claims always carry three axes: Excel build, platform, and suite version.
3. Coverage statements are counts against the full 534-row surface, never percentages of an
   undisclosed base.
4. An implementation is listed only after passing the function's versioned test suite; the badge
   names the suite version.
5. Where published documentation and empirical behavior differ, both are shown and the
   divergence is stated with evidence.

## 8. Non-goals

1. Not a wiki: no anonymous or direct public editing.
2. Not a second function-semantics catalog: OxFunc owns the semantic implementation lane;
   the Handbook cites and enriches it.
3. No speculative content presented as fact; open questions are published as open problems.
4. No engagement mechanics: no comments sections, votes, or gamification. Evidence only.
