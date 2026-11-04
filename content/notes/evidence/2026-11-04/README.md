# Research evidence behind the notes of 2026-11-04

Copies of write-ups and Excel readings that existed only in a working folder when the notes were
written. Bank rows they cite are in OxFunc-Evidence. "The bank" in these files means the recorded
Excel answers held there. Scripts and traces they name by relative path were not archived.

These are cleaned copies: machine paths, host names, process ids and one tool name were removed from
the original write-ups before publication, and nothing else was changed. A statement in a function
note takes precedence over these files. Two known differences: the first ERF write-up's Cody reading
is withdrawn by its round 2, and `annuity-EVIDENCE.txt` finding F4 says PMT - IPMT equals PPMT in 4 of
12 periods and PMT - PPMT equals IPMT in 8 of 12; recomputing from the capture gives 2 and 7, the
figures FN-PPMT-001 states.

- `rounding-EVIDENCE.md`: ROUND, ROUNDUP, ROUNDDOWN, TRUNC, MROUND, MOD.
- `erf-EVIDENCE.md`, `erf-EVIDENCE-round2.md`, `erf-EVIDENCE-round3.md`: ERF below 0.5 and ERFC above it.
  Later rounds supersede earlier ones.
- `annuity-EVIDENCE.txt`: PMT, IPMT, PPMT, CUMIPMT, CUMPRINC.
- `gamma-EVIDENCE.md`, `gamma-readings.tsv`: GAMMA, GAMMALN, GAMMA.DIST, and the readings the notes
  cite (Excel build 20527).
- `round-nan-downstream.json`, `round-nan-downstream-sheet1.xml`, `round-nan-downstream.ps1`: how
  Excel treats the NaN bit patterns ROUND returns near DBL_MAX, the sheet as Excel saved it, and the
  script that read them (Excel build 20527, 2026-11-04).