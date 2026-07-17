# H1 findings: version and platform axes (chapter 05)

Findings from the OxFunc source pass (commit 937f198) behind
`content/model/05-version-axes.md`. Each item is an ambiguity, contradiction, stale statement,
or open question observed in the sources; none block the chapter draft, but several should be
resolved before the chapter leaves draft status.

1. **"Current baseline" build drifts across artifacts with no central registry.** Different
   OxFunc documents each pin a different Excel build as the current reference:
   `docs/function-lane/DATE_SERIAL_SYSTEM_AND_WORKBOOK_MODE_NOTES.md` pins 16.0 build 19725;
   `docs/EXCEL_MATH_DEVIATION_CATALOG.md` entries cite build 20026; bug streams and
   `docs/KNOWN_EXACTNESS_DEVIATIONS.md` cite build 19929;
   `docs/function-lane/DISCREPANCY_RECONNAISSANCE_PASS_20260710.md` cites build 20131. This is
   consistent with a baseline that advances over time, but no single artifact defines what the
   current baseline is right now. The chapter describes "current baseline" as a concept; the
   Handbook will need an actual pinned registry page.

2. **The workbook compatibility-version policy is an open decision, not a ratified schema.**
   `docs/function-lane/EXCEL_FUNCTION_DEFINITION_DISCUSSION.md` D-010 still lists its outputs
   (per-function compatibility-version policy schema, replay-matrix contract) as "decision
   output needed". Recording "workbook Compatibility Version 2" in test records is established
   practice, but the axis's contract is not finalized. Chapter statements about this axis rest
   on practice, not on a closed policy.

3. **The meaning of Compatibility Version value "2" is nowhere defined.** Dozens of bug streams
   and worksheets record "workbook Compatibility Version `2`" as environment fact, but no
   source document explains the value's semantics (presumably the modern/dynamic-array mode) or
   enumerates the other possible values. The chapter avoids asserting what "2" means. Open
   question for the Handbook's workbook-mode tag vocabulary.

4. **x87 bit-exactness is verified on a single CPU family/host; cross-vendor scope is
   asserted, not measured.** `crates/oxfunc_core/src/excel_numeric/x87.rs` states the backend
   reproduces Excel "on the x86-64 machine this was validated against" and that on ~1-in-2000
   hardest inputs the result is "in principle CPU-dependent". No cross-vendor (Intel vs AMD)
   empirical sweep is cited in the sources read. The chapter's claim that last-bit behavior is
   CPU-family-scoped is faithful to the source, but the boundary (do Intel and AMD microcode
   actually differ on real witnesses?) is an open empirical question.

5. **Stale accuracy paragraph in `excel_numeric/mod.rs`.** The module's "Accuracy" section
   still describes the portable core as matching "Excel's own ~0.502-ULP bespoke routine" with
   residual near-midpoint rows as "the tracked W108 discrepancy" — language predating the x87
   identification recorded in the same file's "Backend dispatch" section (which states the
   public entry points are bit-exact via x87). The paragraph is not wrong about the portable
   fallback, but its framing of Excel's routine and of the discrepancy as open reads stale.

6. **`docs/KNOWN_EXACTNESS_DEVIATIONS.md` is superseded for tracking but still the assigned
   evidence source.** The file is marked `superseded_for_tracking` in favor of
   `docs/OXFUNC_EXCEL_DISCREPANCY_CATALOG.md`. It remains valid as the detailed evidence
   record (and is cited that way in the chapter), but future Handbook passes should source
   open-discrepancy state from the catalog, not from this file.

7. **The general `_xlfn.` story is grounded only via the SINGLE case.** OxFunc sources document
   the `_xlfn.SINGLE(...)` / `@` round-trip thoroughly, but the broader mechanism (new
   functions generally serializing as `_xlfn.NAME` in older-format contexts) is public Excel
   knowledge not independently evidenced in the sources read. The chapter states the general
   mechanism briefly and uses SINGLE as its concrete example; a later pass could add an OxFunc-
   or suite-backed witness for an ordinary `_xlfn.`-prefixed function (e.g. a dotted 2010
   statistical name in a legacy workbook).

8. **Systematic scrape artifact in the localization seed CSV.**
   `docs/function-lane/W28_FUNCTION_NAME_LOCALIZATION_LIBRARY_SEED.csv` records the BETA.INV
   function's name as `BETA.INVn` in all 36 locale rows that carry it (no clean `BETA.INV` row
   exists); the trailing `n` looks like a footnote marker glued onto the name during scraping
   of Microsoft's function-list pages. Any Handbook consumption of this CSV as a
   localized-name source needs a cleaning pass; other names may carry the same artifact.

9. **The 1904 workbook mode is recognized but unexercised in the evidence base.**
   `DATE_SERIAL_SYSTEM_AND_WORKBOOK_MODE_NOTES.md` explicitly defers 1904-system empirical
   replay until the evaluation-context seam is wired. All date-function evidence is therefore
   1900-system-scoped. The chapter presents the date system as a workbook-mode axis (correct),
   but the Handbook currently has observed evidence for only one of the two positions on that
   axis.

10. **"Suite version" is a Handbook construct without a direct OxFunc counterpart.** The
    chapter's third scope tag (test-suite version) maps onto OxFunc's practice of citing
    specific run artifacts and dated evidence records, but OxFunc has no single versioned
    suite identifier. The Handbook will need to define how its suite version is minted and how
    it maps back to the underlying run artifacts.

11. **Update-channel identification is inconsistent in the sources.** Where a channel is
    recorded at all, it appears as a raw CDN URL
    (`DATE_SERIAL_SYSTEM_AND_WORKBOOK_MODE_NOTES.md`); most build citations carry no channel.
    The chapter's "channel tag" is therefore weakly grounded — the Handbook should decide on a
    canonical human-readable channel vocabulary and whether channel is required or optional
    scope on a build tag.
