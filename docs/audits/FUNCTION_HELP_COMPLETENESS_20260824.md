# Function-help completeness audit — 2026-08-24

Status: `current working-tree audit`

This is an inventory, not a publication claim. It records what the Handbook can support from its
checked-in sources after the function-page curation and localization sweep. It does not assert that
the Gneiss ledger exists, that any evidence figure was independently re-measured, or that the site
has published function pages.

## Source basis

- The currently decided canonical denominator is 541 entries: 518 worksheet functions and 23
  operators, represented by 534 published rows plus seven split byte-function rows. The live-source
  reconciliation below found four official functions outside that spine, so 541 is not claimed as
  the final Microsoft-function denominator.
- The data projection was regenerated from a clean standalone checkout of OxFunc commit
  `473efa37db60e565ce04241d7cedf74fb3227777`, the basis already declared by the Handbook's mixed-
  vintage contract. The user's newer OxFunc working tree was not treated as a clean publication
  source.
- The localized source is OxFunc
  `docs/function-lane/W28_FUNCTION_NAME_LOCALIZATION_LIBRARY_SEED.csv`, SHA-256
  `b248fbe16fc977c8287ed99b3996218393543c1855d08c1e8436470b28193231`.
- That source is a Microsoft Support alphabetical-index harvest: localized function name, article
  link, category, version marker, and one-line description. It is not a harvest of full translated
  article bodies, signatures, parameter descriptions, remarks, or examples.
- Foundation's earlier `function_catalog_full.csv` has 500 English rows and no localized corpus.
  It is useful provenance, but it is superseded for current identity/help coverage by OxFunc's
  509-row English index and the 518-function corrected catalogue.
- Microsoft's live English alphabetical table was also re-read on 2026-08-24. After splitting the
  seven combined rows and correcting `BETA.INVn` only for identity comparison, it contains 521
  function names. Relative to the Handbook spine it adds four `FORECAST.ETS*` functions and omits
  `COPILOT`; the relevant union is therefore 522 functions. The four missing rows were already in
  Foundation's raw February fetch but were dropped from its generated catalogue and OxFunc's spine.
  See [EFH-HO-002](../handoffs/oxfunc/EFH-HO-002-forecast-ets-inventory-gap.md).

## Measured coverage

| Surface | Present | Honest interpretation |
|---|---:|---|
| Canonical entries | 541 / 541 | The current spine is internally reconciled; it is not claimed as the final Microsoft-function denominator. |
| Curated F6 pages | 375 / 541 | 352 function pages plus all 23 operator pages; 166 function pages remain unauthored. |
| F6 contract-valid curated pages | 375 / 375 | Front matter, declared/authored H2 order, data/evidence joins, and final sections pass. |
| Broken relative links in curated page bodies | 0 | The sweep repaired 86 links in 44 pages. |
| Official function-help URLs in F1 | 518 / 522 relevant functions | Four `FORECAST.ETS*` rows are documented in EFH-HO-002 but cannot enter F1 until the upstream spine is repaired. |
| Official English one-line help in F1 | 518 / 522 relevant functions | COPILOT and JIS are supplied from the English W28 row; the four outside-spine summaries are retained in EFH-HO-002. |
| Official functions outside the canonical spine | 4 | FORECAST.ETS, FORECAST.ETS.CONFINT, FORECAST.ETS.SEASONALITY, FORECAST.ETS.STAT. |
| Raw localized index rows | 20,360 | Exactly 40 locales × 509 rows, preserved in F18 with only relative URLs made absolute. |
| Directly resolved localized-name pages | 492 / 518 functions | 491 pages use an article-unique association; FORECAST uses the sole exact English-name row on its shared article. No split is invented. |
| Non-direct localization associations | 26 / 518 functions | 14 combined-row pages, 10 shared-article pages, and 2 missing function-specific source rows. |
| Non-placeholder signatures | 296 / 541 entries | 245 entries have no publishable signature object. |
| Named parameters | 624 | Only 29 carry a parameter description in the local source. |
| Long descriptions | 8 / 541 entries | The local source does not contain full article help for the remainder. |
| Evidence records on disk | 117 | `handbook_reverified` remains false; record presence is not independent verification. |
| Versioned vector suites | 0 | No exactness wording is available to Handbook voice. |
| Admitted implementations | 0 | Presence in OxFunc is not an admitted Handbook implementation. |
| Generated site function pages | 0 | The rubric API is pre-ledger; site v1 and `/coverage` do not exist yet. |

The machine-readable sources for these counts are
[`data/index.json`](../../data/index.json),
[`data/localization/function-name-index.json`](../../data/localization/function-name-index.json),
and the 541 records under [`data/functions/`](../../data/functions/).

## Localization cases that cannot be split honestly

- Shared published rows (14 pages): FIND, FINDB, LEFT, LEFTB, LEN, LENB, MID, MIDB, REPLACE,
  REPLACEB, RIGHT, RIGHTB, SEARCH, SEARCHB.
- Shared article GUID (10 pages): ISBLANK, ISERR, ISERROR, ISLOGICAL, ISNA, ISNONTEXT,
  ISNUMBER, ISODD, ISREF, ISTEXT.
- No function-specific W28 row (2 pages): FORECAST.LINEAR and IMAGINARY.

All underlying localized names and one-line descriptions remain available in F18. The per-function
records deliberately leave `localized_names` empty in these cases and state the association status,
rather than copying a combined label onto multiple functions or multiplying shared-article rows.

## Canonical function pages still to author

These 166 functions have complete identity, official URL, and official English one-line help in
`data/functions/`, but no curated F6 Markdown page. This is the remaining page-curation inventory;
no placeholder pages were fabricated during this sweep.

The four `FORECAST.ETS*` functions in EFH-HO-002 are additional to this list: they first require
canonical data records, so the honest total page backlog is 170 functions (166 in-spine plus 4
upstream inventory gaps).

### Compatibility (40)

BETADIST, BETAINV, BINOMDIST, CHIDIST, CHIINV, CHITEST, CONFIDENCE, COVAR, CRITBINOM,
EXPONDIST, FDIST, FINV, FLOOR, FTEST, GAMMADIST, GAMMAINV, HYPGEOMDIST, JIS, LOGINV,
LOGNORMDIST, MODE, NEGBINOMDIST, NORMDIST, NORMINV, NORMSDIST, NORMSINV, PERCENTILE,
PERCENTRANK, POISSON, QUARTILE, RANK, STDEV, STDEVP, TDIST, TINV, TTEST, VAR, VARP,
WEIBULL, ZTEST.

### Engineering functions (54)

BESSELI, BESSELJ, BESSELK, BESSELY, BIN2DEC, BIN2HEX, BIN2OCT, BITAND, BITLSHIFT, BITOR,
BITRSHIFT, BITXOR, COMPLEX, CONVERT, DEC2BIN, DEC2HEX, DEC2OCT, DELTA, ERF, ERF.PRECISE,
ERFC, ERFC.PRECISE, GESTEP, HEX2BIN, HEX2DEC, HEX2OCT, IMABS, IMAGINARY, IMARGUMENT,
IMCONJUGATE, IMCOS, IMCOSH, IMCOT, IMCSC, IMCSCH, IMDIV, IMEXP, IMLN, IMLOG10, IMLOG2,
IMPOWER, IMPRODUCT, IMREAL, IMSEC, IMSECH, IMSIN, IMSINH, IMSQRT, IMSUB, IMSUM, IMTAN,
OCT2BIN, OCT2DEC, OCT2HEX.

### Financial functions (27)

NPER, NPV, ODDFPRICE, ODDFYIELD, ODDLPRICE, ODDLYIELD, PDURATION, PMT, PPMT, PRICE,
PRICEDISC, PRICEMAT, PV, RATE, RECEIVED, RRI, SLN, SYD, TBILLEQ, TBILLPRICE, TBILLYIELD,
VDB, XIRR, XNPV, YIELD, YIELDDISC, YIELDMAT.

### Logical functions (2)

MAKEARRAY, SCAN.

### Lookup and reference functions (4)

FORMULATEXT, GROUPBY, HYPERLINK, PIVOTBY.

### Math and trigonometry functions (8)

CEILING, CEILING.MATH, CEILING.PRECISE, FLOOR.PRECISE, MULTINOMIAL, PERCENTOF, SEQUENCE,
SERIESSUM.

### Our 10 featured functions (4)

COUNT, COUNTIFS, SUM, SUMIFS.

### Statistical functions (27)

BINOM.INV, CHISQ.DIST, CHISQ.DIST.RT, CHISQ.INV, CHISQ.INV.RT, CHISQ.TEST, COUNTIF,
COVARIANCE.P, COVARIANCE.S, DEVSQ, EXPON.DIST, F.DIST, F.DIST.RT, F.INV, F.INV.RT,
F.TEST, GROWTH, HARMEAN, HYPGEOM.DIST, MODE.MULT, PERCENTILE.EXC, PERCENTRANK.EXC,
PERCENTRANK.INC, VAR.P, VAR.S, VARA, VARPA.

## Remaining information gaps

The local OxFunc/Foundation fetches do not contain enough source material to fill these honestly:

- full localized article bodies, remarks, examples, and parameter-by-parameter help;
- 245 missing signatures and descriptions for 595 of 624 named parameters;
- curated pages for the 166 in-spine functions listed above, plus canonical intake and pages for the
  four `FORECAST.ETS*` functions missing from the upstream spine;
- category pages, family pages, bibliography, open-problem records, and operator-syntax organ;
- versioned test vectors, admitted multi-language implementations, ULP Atlas plates, and Last Bit
  episode registry;
- a rendered site and coverage page.

Those are backlog statements, not evidence that the missing information does not exist in Excel or
Microsoft's live documentation. Acquiring new web material or asserting semantics beyond the local
sources requires a separately provenance-recorded intake.

## Verification boundary

The F0/F1/F12/F15/F18/F5r subset validates with zero schema findings; the 79 F6,
localization, evidence-register, rubric, and ratchet tests pass; the importer has six passing unit
tests; and the battery verifier accepts 541 files / 6,492 rows. The repository-wide organ validator
routes 1,749 JSON files but still reports 4,291 legacy findings in F3, F5, and F14. Its non-zero
result must not be reported as a clean full-organ validation. The presence scanner's 16 unit tests
pass; its cross-repository publication acceptance remains intentionally closed while the user's
current OxFunc working tree is dirty.
