# Research evidence archived for the 2026-11-22 function notes batch

Cleaned provenance and live-Excel measurements supporting the 2026-11-22 Function Notes batch
(`DnaCalc` bead `dc-k2a3p`, covering rounds `cb4`–`cb11`, `nx3`–`nx9`, `gp1`, and the last-bit
articles in `DnaCalc/docs/articles/`).

- `frozen-holdouts-provenance.tsv`: pre-ask SHA-256 hashes, row counts and `matched / asked`
  scores for the prospective frozen prediction suites across `cb4`–`cb11` and `nx3`–`nx8`. The
  underlying Excel 16.0.20527 readings are archived in `CalcFunc-Evidence` at
  `readings/excel-20527/studies/{cb4..cb9,cb11,nx3..nx8}.json.gz` and
  `readings/excel-20527/discrepancies/{cb4..cb9,cb11,nx3..nx7}.json.gz` (commit
  `4ef2c244d73c6c4ca9b69641f9d95b72c806ed7e`).
- `nx9-GAMMA.DIST-witnesses.json`: 79 differing Excel `16.0.20527.20196` (Windows 64-bit,
  AMD Ryzen 7 4800H) `GAMMA.DIST` readings from `nx9_probe1` (`A_rexp_3way_gt015`,
  `B_rexp_poly_vs_expm1_le015`, `C_a0875_zln2_consec`), cleaned of local paths.
- `nx9-probe-summary.md`: concise summary of the `nx9` `expm1` proof and witness-bank counts
  for `GAMMADIST`, `GAMMA.DIST`, `GAMMA.INV` and `GAMMAINV`.
