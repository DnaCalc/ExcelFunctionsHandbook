# H1 findings: value universe and coercion sources

Scope: issues noticed while drafting `content/model/01-value-universe.md` and
`content/model/02-coercion-and-lifting.md` from OxFunc snapshot 937f198. Read-only review;
nothing in OxFunc was modified. Paths below are relative to the OxFunc repo unless noted.

1. **Scan policy diverges from Excel's documented range-scan rule.**
   What: `AggregateScanPolicy::IgnoreTextAndEmpty` in `crates/oxfunc_core/src/coercion.rs`
   (`aggregate_scan_sum`, lines 113-130) skips only *unparsable* text (`NonNumericText`),
   Empty, and Missing. Numeric-looking text (`"2"`) and logicals coerce successfully via
   `coerce_arg_to_number` and are summed. Excel's documented range-scan behavior for SUM
   ignores all text and logicals in scanned ranges regardless of parseability, so the seed
   scenario CO4-011 (range containing `1`, `"2"`, `"asd"`) would produce 3 under this policy
   where Excel produces 1. The flagship direct-vs-scan contrast the decision table is built
   around is not reproduced by the code as written.
   Where: `crates/oxfunc_core/src/coercion.rs`; contrast scenarios CO4-011/CO4-012 in
   `docs/function-lane/COERCION_SCENARIO_MANIFEST_SEED.csv`; row CO-POL-006 in
   `docs/function-lane/COERCION_DECISION_TABLE.csv`.
   Why it matters: chapter 02 presents the direct-vs-scan asymmetry as Excel behavior (which
   is correct) but the cited executable baseline currently implements a different scan rule.
   Either the policy is a placeholder awaiting the per-family matrix or it is a bug; the
   chapter's Sources note should not be read as "code reproduces Excel here".

2. **Empty-through-reference asymmetry in the to-number path.**
   What: a direct Empty argument maps to `CoercionError::EmptyCell`
   (`coerce_arg_to_number` / `coerce_calc_scalar_to_number`), but a reference that resolves
   to an Empty value re-enters `coerce_eval_to_number`, whose catch-all arm returns
   `UnsupportedValueKind("unsupported_value")` (lines 59-65) instead of `EmptyCell`.
   Where: `crates/oxfunc_core/src/coercion.rs`.
   Why it matters: family policies keyed on the EmptyCell outcome (empty maps to zero in
   scalar numeric contexts; aggregates skip empty cells) cannot observe emptiness when it
   arrives through a reference — yet that is the most common way emptiness arrives. Excel's
   `=SIN(A1)` with empty `A1` is 0; this path cannot express that mapping.

3. **Naming drift: `EvalValue` in the specs versus `CalcValue` in code.**
   What: `docs/function-lane/VALUE_UNIVERSE_PRELIM_SPEC.md` section 2 (interpretation rule 3)
   still says "the current Rust `EvalValue` type", section 9A says `ValueWithPresentation`
   "carries an ordinary `EvalValue`", and `COERCION_AND_CONVERSION_PRELIM_SPEC.md` section 2
   lists `EvalValue` as a typed source domain — but the code
   (`crates/oxfunc_value_types/src/lib.rs`) has adopted the two-tier `CalcValue`/`CoreValue`
   carrier, and presentation hints are a rich-payload variant, not a wrapper type. Section 2A
   of the value-universe spec documents the new direction, so the document is internally
   half-migrated.
   Why it matters: a reader cross-checking spec against code finds types that no longer
   exist; the chapters standardize on the current code vocabulary.

4. **Tag vocabulary mismatch between spec algebra and `ValueTag`.**
   What: the spec's 12-tag algebra names `lambda_value`, `rich_value`, `extended_wrapper`;
   the Rust `ValueTag` has 13 variants naming `Callable`, `RichValue`, `Presentation`,
   `ErrorMetadata` — i.e. `extended_wrapper` has been split into two tags in code while spec
   section 9.5 still presents it as one live bucket, and no updated tag table reflects the
   split.
   Where: `docs/function-lane/VALUE_UNIVERSE_PRELIM_SPEC.md` sections 3 and 9;
   `crates/oxfunc_value_types/src/lib.rs` (`ValueTag`).
   Why it matters: the machine vocabulary a public page displays comes from the enum; the
   spec's names would not round-trip. Chapters use the enum names.

5. **Callable admission tension at the published-result boundary.**
   What: the value-universe spec (section 8, item 4) and the research notes (section 3, item
   5) place lambda/callable values in the intermediate evaluation domain, "not ordinary
   cell-result scalar domain" — yet `ValueBoundary::PublishedFormulaResult.allows` admits
   `ValueTag::Callable` (and `RawFunctionReturn` does too).
   Where: `crates/oxfunc_value_types/src/lib.rs` (`ValueBoundary::allows`);
   `docs/function-lane/VALUE_UNIVERSE_PRELIM_SPEC.md`.
   Why it matters: it is ambiguous whether a callable is publishable as such or only via its
   `#CALC!` core projection. Chapter 01 reports the admission matrix as pinned and separately
   explains the `#CALC!` projection, but the intended semantics should be decided and stated
   in one place.

6. **Boundary-set count mismatch: seven in spec, six in code.**
   What: spec section 2 lists seven boundary sets including `RichValueData`; the Rust
   `ValueBoundary` enum has six variants. Rich-value data admission is realized structurally
   (the `RichObjectData` enum, which admits scalars/arrays/nested objects but not Missing or
   Reference) rather than as a boundary.
   Where: `docs/function-lane/VALUE_UNIVERSE_PRELIM_SPEC.md` section 2;
   `crates/oxfunc_value_types/src/lib.rs`.
   Why it matters: harmless as implementation choice, but the spec's "boundary" framing and
   the code's structural framing should be reconciled so the admission story has one shape.
   Chapter 01 follows the code (six boundaries) and describes rich-data admission textually.

7. **`#BUSY!` and `#GETTING_DATA` are unclassified in the error registry split.**
   What: `WorksheetErrorCode` has 14 codes including `Busy` and `GettingData`, but the spec's
   provisional registry split (section 6) lists the legacy seven and gives only examples for
   the extended family; the transient placeholder codes appear in neither list, and the
   research notes' error anchors do not cover them. Newer codes (e.g. `#PYTHON!`) are absent
   from the enum entirely, presumably deferred under the version-scoping rule, but no row
   records that deferral.
   Where: `crates/oxfunc_value_types/src/lib.rs`;
   `docs/function-lane/VALUE_UNIVERSE_PRELIM_SPEC.md` section 6.
   Why it matters: chapter 01 had to label `#BUSY!`/`#GETTING_DATA` "extended" by inference
   and flags the classification as not evidence-pinned; the registry should classify all 14
   and record the deferral policy for codes outside the enum.

8. **The text-to-number recognizer is a minimal baseline, not Excel's grammar.**
   What: `parse_excel_number` in `crates/oxfunc_core/src/coercion.rs` is
   `str::trim` + Rust `str::parse::<f64>` + a finiteness filter. Excel's coercion recognizer
   additionally accepts locale-dependent forms (thousands separators, currency symbols,
   percent suffixes, date/time text, fraction text) and its exact grammar is pinned nowhere
   in the read sources; probe scenarios CO4-001/CO4-005 cover only plain and scientific
   decimals.
   Why it matters: text-to-number is among the most divergence-prone behaviors across
   engines; chapter 02 flags the gap explicitly. A dedicated recognizer-grammar
   characterization (with a locale axis) is missing from the source set.

9. **Spec-required primitives `to_text`, `to_logical`, `array_lift_map` are not centralized.**
   What: `COERCION_AND_CONVERSION_PRELIM_SPEC.md` section 2 requires six primitives; the
   central `crates/oxfunc_core/src/coercion.rs` implements only the to-number family,
   error propagation, and the reference seam. Text/logical conversion logic exists scattered
   across `crates/oxfunc_core/src/functions/*` (28 files match to_text/to_logical-style
   helpers).
   Why it matters: chapter 02's to-text/to-logical sections could only be written at outline
   level with an explicit caveat; centralizing (or at least indexing) these primitives would
   let per-function pages cite one pinned rule instead of per-function code.

10. **CSV hygiene: unquoted comma in `COERCION_DECISION_TABLE.csv` row CO-POL-002.**
    What: the notes field reads `Runtime coercion error class, not admission failure.`
    without quoting, so naive parsers see 13 fields against a 12-column header.
    Where: `docs/function-lane/COERCION_DECISION_TABLE.csv` line 3.
    Why it matters: any tooling that ingests the matrix (including future Handbook data
    pipelines) will mis-split this row.

11. **Legacy stub `Value` enum still exported beside `CalcValue`.**
    What: `crates/oxfunc_value_types/src/lib.rs` still publicly exports a two-variant
    `Value { Number, Error(EvalError) }` with `EvalError::ArityMismatch` at the top of the
    same file that defines the full `CalcValue` model.
    Why it matters: two exported value types in one crate invite wrong imports; if the stub
    is load-bearing for older call sites it deserves a doc comment saying so, otherwise it
    looks removable.

12. **The raw-return-to-published normalization map is doctrine, not code.**
    What: spec section 12 pins (from add-in probe evidence) that a raw nil/empty scalar
    return normalizes to numeric-zero semantics before outer binding and publication, and
    that nil array elements persist until scalarization — but no artifact in the read set
    implements a `RawFunctionReturn -> PublishedFormulaResult` normalization step; spec
    section 13 (open point 5) acknowledges this.
    Where: `docs/function-lane/VALUE_UNIVERSE_PRELIM_SPEC.md` sections 12-13.
    Why it matters: chapter 01 presents the rule as empirically pinned model doctrine; when
    the normalization lands in code, the admission matrix tests should grow to cover it, and
    the chapter's Sources note can be upgraded.

13. **Admission-failure evidence is Excel-automation-shaped.**
    What: the entry-time rejection of `=SIN()` (scenario CO4-004) encodes its expectation as
    a COM automation error code (`0x800A03EC`) in the scenario manifest — an
    automation-surface observation standing in for the interactive "Excel refuses the
    formula" behavior. The coercion spec (section 4) says worksheet entry is the normative
    admission surface and automation entry is contextual, so the evidence for the normative
    claim is currently drawn from the contextual surface.
    Where: `docs/function-lane/COERCION_SCENARIO_MANIFEST_SEED.csv` (CO4-004);
    `docs/function-lane/COERCION_AND_CONVERSION_PRELIM_SPEC.md` section 4.
    Why it matters: chapter 02 states the entry-time rejection as observed; if interactive
    entry and automation entry ever diverge on admission, the normative claim needs a
    worksheet-surface observation to stand on.
