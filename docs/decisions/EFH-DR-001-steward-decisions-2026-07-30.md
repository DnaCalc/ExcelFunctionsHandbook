# EFH-DR-001 — The eight steward decisions, and the curation scope

Status: `decided`
Date: 2026-07-30
Decider: steward
Argument of record: the `STEWARD-DECISIONS.md` dossier (session-archived), which states the
options, the recommendation, and the consequence of each option for all eight decisions.

Nine questions were open. Eight of them (SD-1 … SD-8) each blocked something concrete in the build
specification; the ninth (curation scope) decided how many entries get a curated page. All nine are
decided below. Each entry names the option taken, why, and what it changed in the repository.

Only SD-1, SD-2, SD-3 and SD-7 change a governing document; those changes are listed in section 10
so a reader can check them without reading nine records.

---

## 1. SD-1 — the coverage denominator

**Decided: 541 entries is the coverage denominator.**

`CHARTER.md` and `content/model/06-claim-language.md` used **534**; `data/index.json`, both mockups
and `README.md` used **541**. The Handbook generates 541 pages and ratchets 541 rows, so a
denominator that does not count the thing being counted is the kind of small dishonesty that
compounds. 534 survives as what it actually is — Microsoft's published-row count.

Every coverage statement now carries the reconciliation adjacent:
**541 entries = 518 functions + 23 operators = 534 published Excel rows + 7 split byte-variant
rows.** The seven split rows are the pairs the export ships as one row (`FIND, FINDB` and its six
siblings) and the OxFunc runtime registry models as fourteen separate ids.

**Changed.** `CHARTER.md` section 3 clause 2 (comprehensiveness) and section 7 clause 3 (coverage
counting) now divide by 541. `CHARTER.md` section 4 clause 1 keeps 534 and says explicitly that it
is the published-row statement and not a coverage denominator.
`content/model/06-claim-language.md` scoping rule 3 now reads "the full 541-entry function
surface". `README.md` line 6 said "534 source functions plus 7 operators", which was wrong twice —
there are 23 operators, and the 7 are the split byte-variant rows — and now carries the
reconciliation sentence instead.

## 2. SD-2 — `data/` regeneration requires a built OxFunc

**Decided: take both — delta D-1 (the live registry) and organ F16 (the reference-engine
battery).**

The alternative preserved a doctrine by publishing a falsehood: 115 entries whose classification
OxFunc holds today would keep rendering "OxFunc has not classified this function". That inverts the
priority. The cost is real and is now a published cost rather than a hidden one: `data/`
regeneration needs a Rust toolchain and a built `oxfunc_core`, and some battery rows are host-scoped
because certain OxFunc kernels route through hardware x87.

**Changed.** `OPERATIONS.md` section 4 clause 1 now reads "byte-stable for a fixed OxFunc source
state **built at a named commit with the build inputs recorded**", names the toolchain dependency,
and requires host-scoped rows to publish `host_scoped`. A new "Mixed-vintage disclosure for `data/`"
block in section 4 publishes the two "as of" bases (section 4 of this record).

## 3. SD-3 — CHARTER section 7 clause 2: keep it, ban the phrase

**Decided: keep clause 7.2 unamended and forbid the words.**

Clause 7.2 requires three axes on every exactness claim — Excel build, platform, suite version.
`vectors/` holds zero suites, so the third axis is unavailable for every claim the Handbook can
presently make: **as of today no claim in the Handbook satisfies clause 7.2 as written.** The
choice was to relax the rule to fit what we can prove, or to keep the rule and drop the words. We
kept the rule. Amending 7.2 would make it satisfiable immediately and permanently weaker, and would
remove the pressure to build the suite mechanism before the first exactness claim ships.

**Changed.** `CHARTER.md` section 7 gains clause 8: "bit-exact" and "bit-for-bit" may not state a
Handbook claim, in prose or in a rendered label, until `vectors/` publishes a suite. Two
appearances are permitted — naming the phrases in order to forbid them, and quoting an upstream
record inside a captioned quotation carrying that record's own axes.
`content/model/06-claim-language.md` mirrors the rule as scoping rule 6 and marks the
`suite-exact, vN` and `characterized bit-exact` vocabulary rows unavailable until a suite
publishes. Existing prose occurrences in `CHARTER.md` section 1 clause 7, `README.md`, and
`OPERATIONS.md` section 5 clause 3 were reworded. The flavour identifier `excel-bitexact` names an
implementation's target rather than a demonstrated result and stays in use.

**Not done here.** `content/model/07-implementation-options.md` still contains the forbidden phrases
in its prose and its page-vocabulary table. That file was outside this change's write scope; it is
the first thing a rule-8 sweep must fix.

## 4. SD-4 — legacy aliases

**Decided: demoted.**

`WEIBULL`, `EXPONDIST`, `GAMMALN.PRECISE` and `OP_POWER` (`^`) each appear in an evidence record
whose count was measured on their modern sibling, whose kernel they share. They carry
`attribution: alias-sibling-inherited`, render at warrant W3 with
`why_no_count: named-in-a-group-record-not-itself-measured`, and are never credited with the
sibling's count.

The case for crediting them is genuinely strong — `WEIBULL` and `WEIBULL.DIST` route through the
same kernel. The case against is that "close to a measurement" is exactly the reasoning that
produced every laundered figure the audits found, and OxFunc has repeatedly had to *prove* such
identities rather than assume them (`CHIDIST` versus `CHISQ.DIST.RT`, and the multi-view collapse
work, exist for that reason). Cost: four entries read as less established than a reader might think
fair, and the counted set is 92 rather than 96.

**Changed.** Nothing in a governing document. This decision binds the evidence-import rubric.

## 5. SD-5 — PMT's in-sample 2,304/2,304

**Decided: publish it, with its flag and its open state in the same visual unit.**

PMT's combsweep corpus scored 2,304/2,304 against live Excel, and that corpus is the one the
combine fix was pinned against — the score is in-sample. Withholding a real live-Excel measurement
because it flatters is as much a distortion as publishing one that flatters without its caveat. So
it publishes carrying `corpus_was_repair_target: true`, none held out, alongside its numeric state
N1 (open, measured divergence).

**Implementation condition.** Warrant and both Excel-relationship states must render in one visual
unit, and overclaim test OT-5 must pass, before PMT publishes. If either mechanism is dropped, this
decision becomes an overclaim and is void.

**Changed.** Nothing in a governing document. This decision binds the page contract.

## 6. SD-6 — who reviews the attribution table

**Decided: resolved by rebuilding the evidence layer from the primary sources.**

The question was whether one agent pass over the attribution table could stand unreviewed. It was
overtaken: an independent re-derivation from primary OxFunc sources measured the harvest's
field-level error rate at 65.2% over 92 counted entries and found three fabricated figures. The
answer is therefore neither "human review" nor "ship it" but "rebuild from the primary
derivations". The record of that is `EFH-DR-002`.

**Changed.** Nothing in a governing document. The evidence layer's basis changed.

## 7. SD-7 — four `OPERATIONS.md` amendments

**Decided: all four.** Each is a factual correction to a governing document, not a design change.

1. **Section 4 clause 2** said "The ledger rebuilds byte-stably from its canonical JSONL export."
   Not satisfiable. `Gneiss.Cell`'s public API (`GneissLedger`: `Create`, `Open`, `HighWater`,
   `Append`, `DeclarePredicate`, `DeclareContext`, `Ask`, `Why`, `CheckStale`, `Note`,
   `ExportLedgerJsonl`, `Dispose`) contains no import operation, and `ExportLedgerJsonl()` has no
   inverse. The export is also not a complete representation of the ledger — Gneiss's own
   `CONTRACT-V01.md` section 6 records that notes "are not part of `ExportLedgerJsonl()` (which
   covers only tx/assrt/dec/just)". The clause was unsatisfiable in two independent ways. It now
   reads: the export is byte-stable and is the reviewable twin, verified by byte-equality on every
   build.
2. **Section 4 clause 1** — the toolchain and host-scoping sentence required by SD-2.
3. **Section 2 ownership table** — the table described `ledger/` as tool-owned and append-only
   while the organ contained a hand-edited Markdown file (`VOCABULARY.md`). The table now
   distinguishes the ledger database and its export from the curated Markdown beside them.
4. **Section 3 sync doctrine** — the `upstream_anchor` re-check, now clause 6. This is the least
   visible amendment and the one that matters most: OxFunc **deletes** a discrepancy row when it
   signs the discrepancy off, so a Handbook that never re-checks its anchors publishes stale open
   discrepancies indefinitely. The design was well armoured against publishing false agreement and
   completely unarmoured against publishing false disagreement, which for a reference work aimed at
   Excel users is the reputationally worse error.

## 8. SD-8 — does the ledger gate publication?

**Decided: no. The ledger does not gate publication.**

The pre-ledger answer envelope (`basis_mode: "pre-ledger"`, seven typed door limitations, no
receipt) is fully specified and publishes today; the ledger flips `basis_mode` afterwards without a
schema change, because the envelope was designed for exactly that. Publication is therefore not
hostage to the scaling behaviour of a library the Handbook does not own and cannot fix.

The cost is stated rather than hidden: the first published site will say, on every page, that the
Handbook's claims plane does not exist yet and that there is no receipt. That is true, and it is a
weaker artifact than the Charter describes.

**Changed.** Nothing in a governing document. The milestone order changed.

## 9. Curation scope

**Decided: a curated page for all 541 entries. A later workflow writes them.**

A concern was raised against this and **overruled**, and the concern was correct on its facts: for
several hundred entries there is no OxFunc evidence to report at all. The measured figures are
**221** entries carrying no harvested evidence record of any kind, and **426** carrying no
numeric-axis comparison record (build specification section 3.6). Their curated value cannot come
from evidence, because there is none. It has to come from mathematics, from the literature, and
from stating the absence honestly — what the function computes, how it is defined, what is known
about implementing it well, and the plain sentence that the Handbook has measured nothing here.

That is a harder writing job than the evidence-backed pages, and it is the one most likely to drift
into padding. The decision is recorded with the objection attached so that a later reviewer can
weigh it again: if those pages turn out to be filler, this decision, not the writing, is where the
fault started.

---

## 10. What changed in the governing documents

| Document | Location | Change | Decision |
|---|---|---|---|
| `CHARTER.md` | section 1 clause 7 | "bit-exact behavior" reworded | SD-3 |
| `CHARTER.md` | section 3 clause 2 | denominator → 541 entries + reconciliation | SD-1 |
| `CHARTER.md` | section 4 clause 1 | 534 kept, marked as the published-row statement | SD-1 |
| `CHARTER.md` | section 7 clause 3 | denominator → 541 entries + reconciliation | SD-1 |
| `CHARTER.md` | section 7 clause 8 (new) | the phrase prohibition | SD-3 |
| `OPERATIONS.md` | section 2 table + clause 1 | `ledger/*.md` ownership | SD-7.3 |
| `OPERATIONS.md` | section 3 clause 6 (new) | `upstream_anchor` re-check | SD-7.4 |
| `OPERATIONS.md` | section 4 clause 1 | built-at-a-named-commit; host-scoped rows | SD-2, SD-7.2 |
| `OPERATIONS.md` | section 4 clause 2 | reviewable twin replaces rebuild-from-export | SD-7.1 |
| `OPERATIONS.md` | section 4, disclosure block (new) | mixed-vintage disclosure | SD-2 |
| `OPERATIONS.md` | section 5 clause 3 | "bit-exactness" reworded, points at rule 8 | SD-3 |
| `OPERATIONS.md` | section 10 | H3/H4 → `in progress`; H5 evidence added | honest register |
| `README.md` | line 6 | operator count and split-row description corrected | SD-1 |
| `README.md` | Status | H3/H4 `in progress`; rule-7.8 statement | SD-3, register |
| `content/model/06-claim-language.md` | scoping rule 2 | wording; labels marked unavailable | SD-3 |
| `content/model/06-claim-language.md` | scoping rule 3 | denominator → 541 entries | SD-1 |
| `content/model/06-claim-language.md` | scoping rule 6 (new) | the phrase prohibition, mirrored | SD-3 |
| `content/model/06-claim-language.md` | page vocabulary | two rows marked unavailable | SD-3 |

## 11. Known incompleteness of this application

1. `content/model/07-implementation-options.md` still uses "bit-for-bit" and "characterized
   bit-exact" in prose and in its vocabulary table. It was outside this change's write scope.
2. `site/mockups/` was not swept for the forbidden phrases or for the 534 denominator.
3. `GNEISS-PROFILE.md` and `gneiss-profile.json` were not re-read against SD-8; the profile's
   "Known Pre-Implementation Limits" list predates the pre-ledger publication decision.
4. No CI check enforces clause 7.8 yet. Until one exists, the prohibition is a documented rule with
   no mechanical defence.
