# EFH-DR-002 — The evidence layer is rebuilt from primary sources

Status: `decided`
Date: 2026-07-30
Decider: steward
Supersedes on this question: SD-6 ("who reviews the attribution table before it becomes 541 pages")
Sources of record (session-archived dossier): `ATTRIBUTION-CORRECTIONS.md` and its machine twin
`ATTRIBUTION-CORRECTIONS.json`; the eight primary re-derivations `RV-w108-elementary.md`,
`RV-gamma-lgamma.md`, `RV-distributions.md`, `RV-normal-discrete.md`, `RV-financial-annuity.md`,
`RV-financial-bond.md`, `RV-structural-and-ops.md`, `RV-bessel-matrix-misc.md`.

## 1. The decision

The Handbook's evidence layer is **rebuilt from the eight primary-source re-derivations**, not
reviewed from the existing harvest. Where the re-derivations disagree with the build specification,
the re-derivations win. Every published figure must trace to a quoted OxFunc sentence with a
`path:line` citation, and a figure that cannot be traced is omitted and its omission recorded.

## 2. Why: the pipeline's error rate, measured

The build specification's own section 7.2 admitted that "the base rate of error in this pipeline is
not zero and is not measured". It is now measured, and not by sampling: the eight re-derivations
were each written from the OxFunc primary sources **before** the reviewer opened the specification,
and between them they independently re-derived **all 92 of the 92 entries that hold a counted
comparison record**. This is a census of the counted set, not an estimate.

Quoted verbatim from `ATTRIBUTION-CORRECTIONS.md` section 5.1, "Over the 92 counted entries":

| Field | Wrong | of 92 | Direction of the error |
|---|---:|---:|---|
| the **figure** (value, corpus scope, or which surface it belongs to) | 12 | **13.0%** | 5 overstate, 5 land on a surface that was not measured, 2 mis-scope a battery/candidate |
| `held_out` | 11 | **12.0%** | **10 of 11 understate** the hold-out; 1 (POWER) overstates it |
| `corpus_was_repair_target` | 12 | **13.0%** | **12 of 12 in the flattering direction** — all say `false` where the corpus was the fitting set |
| `cpu_stated` | 21 | **22.8%** | **21 of 21 in the flattering direction** — all claim a CPU that no source names |
| `build_ambiguity` | ≥36 | **≥39%** | flattering: flat `single-build` where the build is not restated on the scored line |
| **at least one field wrong** | 60 | **65.2%** | — |
| **no correction of any kind** | 32 | **34.8%** | — |

Sixty of ninety-two counted entries carried at least one wrong field. Thirty-two survived untouched:
ACOTH, CHIDIST, DURATION, FACTDOUBLE, FORECAST, GAMMA, INTERCEPT, MDURATION, NORMSDIST, NORMSINV,
PERMUT, RATE, REGEXTEST, SLOPE, SUMIF, TBILLYIELD, WEIBULL.DIST and the 15 measured operators.

Over the attribution apparatus itself — 42 discrete judgements — **10 were corrected (23.8%)**, or 11
(26.2%) counting one judgement confirmed for the wrong reason.

A 65.2% field-level error rate is not a reason to review harder. It is a reason to change the basis.

## 3. Three fabricated figures

**3 of the 92 counted entries (3.3%) carry a fraction that appears in no OxFunc source:**

| Entry | Published figure | Status |
|---|---|---|
| ATANH | `338/344` | Appears in no OxFunc source. The harvest record for ATANH says `344/350`. |
| NPER | `1293/1293` | Appears in no OxFunc source. |
| XNPV | `1705/1705` | Appears in no OxFunc source. |

All three were composed at the specification/assignment layer rather than inherited from the
harvest: `grep -l "338/344"` across the entire dossier returns the specification alone. **Two of the
three are sums that merge a numeric count with an error-row count** — that is, two numbers that
existed were added into a third number that did not.

This is the failure mode the specification's own weakness section could not see, and it is the one a
reviewer working from the harvest is least able to catch, because the harvest is right and the
composition is downstream of it. It is also the direct source of the standing rule that now governs
every evidence agent: never invent a number, and never sum two figures into a new one.

## 4. The directional bias

Of the 91 disagreements raised: **38 reduce a published claim, 35 increase it, 18 are cosmetic.**
Superficially symmetric. It is not symmetric in kind.

1. The reduce-side errors cluster in **figures and warrants** — the things a reader quotes.
2. The increase-side errors cluster in **absent records** — things a reader cannot know are missing.
3. **Every single one of the 33 disclosure-field errors** (`corpus_was_repair_target`, `cpu_stated`,
   `build_ambiguity`) **points the same way: towards making the evidence base look cleaner and
   better-pinned than it is.** The 33 is 12 of 12 on `corpus_was_repair_target` plus 21 of 21 on
   `cpu_stated`; the `build_ambiguity` set (≥36) is named in the same finding, is flattering in the
   same direction, and is not inside that total. The source's own phrasing carries this tension and
   it is reproduced here rather than reconciled by arithmetic.

A random transcription process does not produce 33 errors that all point one way. The pipeline has a
directional bias in exactly the fields that describe how trustworthy its own evidence is. That
finding, and not the raw error rate, is what makes review-in-place insufficient: a reviewer checking
figures would have passed the disclosure fields, because each one individually looks plausible.

## 5. What this record is for

The Handbook is a knowledge system that asks its readers to trust published figures about numerical
behaviour. It therefore has to be willing to publish the error rate of the pipeline that produced
its own figures — 65.2% field-level over a census of 92, three fabrications, and a one-directional
disclosure bias — rather than only the corrected result. This record exists so that number is on
the record before the corrected figures are, and so a later reader can see what the correction was
correcting.

The same discipline applies forward. When the rebuilt evidence layer is itself re-derived, whatever
that measurement says gets published here too.

## 6. What is not settled

1. The corrections document records **18 judgements that are genuinely ambiguous in the source**.
   The standing rule applies: publish the weaker reading and record the ambiguity. None of the 18
   is resolved by this decision.
2. The warrant-ladder totals were **not** recomputed under the corrections. At least 24 entries
   change warrant level, but a correct W2/W3 boundary needs a full 541-entry pass that has not been
   run. Any ladder count quoted before that pass is stale.
3. The N1–N8 numeric-state totals were **not** re-totalled: five of the eight re-derivations did not
   cover the 426 entries in N8.
4. The re-derivations are themselves one pass by one reviewer over the primary sources. They are a
   stronger basis than the harvest, not a verified one. Nothing in the Handbook has been checked
   against live Excel by the Handbook.
