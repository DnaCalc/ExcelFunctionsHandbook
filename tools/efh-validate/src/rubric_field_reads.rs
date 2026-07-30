//! Every field the FOUNDATION 3.7 decision procedure reads, enumerated by hand, each with the
//! FOUNDATION line it is read at and the sentence that reads it.
//!
//! THIS FILE IS THE POINT OF T1.
//!
//! Audit finding A3-F1 (FOUNDATION line 51) was: "D2's procedures read fields D1's schemas do not
//! define." Its disposition names this test: "6 T1's acceptance test asserts field-by-field that
//! every field the rubric reads exists in the schema it reads it from."
//!
//! A field read that no schema declares cannot be caught by reading either document alone. It is
//! caught here, mechanically, by listing the reads next to the schemas and checking one against the
//! other. Every entry cites a line of
//! scratchpad/dossier/FOUNDATION.md so the list can be re-derived from the source rather than
//! trusted.
//!
//! Line numbers are 1-based into FOUNDATION.md as of the build this crate was written against
//! (1361 lines). The `quote` field carries the text, so a line drift is detectable.

/// What the rubric does with a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadKind {
    /// The rubric reads it. It must exist in the named schema with a compatible type.
    Read,
    /// The field exists and a tool may NEVER parse it. Guard G-9. It must exist in the schema and
    /// carry the `efhForbiddenRead` annotation.
    ForbiddenRead,
    /// The field exists for audit or rendering but is explicitly NOT an input to the warrant or
    /// depth predicate. It must exist; the note records what would go wrong if it were read.
    PresentButNotRubricInput,
    /// The field must NOT exist in the schema at all, so that no tool can read it by accident.
    MustNotExist,
    /// The pseudocode names a quantity in prose and NO schema field carries it. Reported, counted,
    /// and ratcheted against EXPECTED_UNRESOLVED so a new one cannot appear silently.
    NarrativeUnresolved,
}

/// Where in FOUNDATION the read comes from. Kept explicit so a reader can tell a literal line of
/// pseudocode from a label template or a steward decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadSource {
    Pseudocode37,
    Invariant37,
    LabelTemplate,
    RenderingMandate,
    StewardDecision,
    OverclaimTest,
}

pub struct FieldRead {
    /// How the pseudocode names it, e.g. "P.impl_modules" or "c.passed".
    pub reader: &'static str,
    /// The schema file the field must be declared in.
    pub schema_file: &'static str,
    /// Dotted path into the schema's declared properties. `[]` steps into `items`.
    pub path: &'static str,
    /// JSON types the reader requires the field to be able to carry. Every one of these must be
    /// permitted by the schema, or the read is unsafe.
    pub expect_types: &'static [&'static str],
    pub kind: ReadKind,
    pub source: ReadSource,
    /// FOUNDATION.md lines, 1-based.
    pub foundation_lines: &'static str,
    /// The text that performs the read.
    pub quote: &'static str,
    /// Anything a reader needs in order to check the entry.
    pub note: &'static str,
}

/// The number of `NarrativeUnresolved` entries expected. A rise means the rubric acquired a read
/// with no schema field behind it, which is finding A3-F1 recurring; the selftest fails on any
/// change, in either direction.
pub const EXPECTED_UNRESOLVED: usize = 5;

pub const FIELD_READS: &[FieldRead] = &[
    // ============================================================ F2, data/presence/<id>.json
    FieldRead {
        reader: "P.surface_name",
        schema_file: "f2-presence.schema.json",
        path: "surface_name",
        expect_types: &["string"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "672",
        quote: "s  := P.surface_name",
        note: "`s` is then read at line 714 ('any comparison record naming s') and rendered into \
               absence strings A-36, A-40, A-54, A-55 and A-56.",
    },
    FieldRead {
        reader: "P.impl_modules",
        schema_file: "f2-presence.schema.json",
        path: "impl_modules",
        expect_types: &["array"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "716, 721, 725",
        quote: "if P.impl_modules == [] -> W0 ... if P.impl_modules == [] -> D0",
        note: "FOUNDATION 3.7 line 763: 'impl_modules, NEVER mention_modules - the latter puts ~525 \
               siblings on almost every entry.' Guard G-1 also forbids a warrant above W0 or a \
               depth above D0 when this is empty.",
    },
    FieldRead {
        reader: "P.tests_in_impl_modules",
        schema_file: "f2-presence.schema.json",
        path: "tests_in_impl_modules",
        expect_types: &["integer"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "717, 718, 726",
        quote: "if P.tests_in_impl_modules == 0 -> W1 ... -> W2 (+ n = P.tests_in_impl_modules ...)",
        note: "FOUNDATION 3.7 line 761: 'The test predicate reads tests_in_impl_modules, NEVER the \
               raw unit_tests_direct.' See the MustNotExist entry for unit_tests_direct below.",
    },
    FieldRead {
        reader: "P.module_tests_minus_sibling_count",
        schema_file: "f2-presence.schema.json",
        path: "module_tests_minus_sibling_count",
        expect_types: &["object"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "719",
        quote: "-> W2 (+ n = P.tests_in_impl_modules, + P.module_tests_minus_sibling_count)",
        note: "A1-F2 (line 44) makes it a required RENDERED field, and overclaim test OT-24 (line \
               796) requires it on every entry in the 202-entry pigeonhole set while forbidding the \
               words tested, unit-tested and test coverage there.",
    },
    FieldRead {
        reader: "P.module_shared_by_count",
        schema_file: "f2-presence.schema.json",
        path: "module_shared_by_count",
        expect_types: &["integer"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "727, 728",
        quote: "if P.module_shared_by_count >= 5 -> D2 / if P.module_shared_by_count >= 1 -> D3",
        note: "The whole test-depth tier below D5 turns on this one integer.",
    },
    FieldRead {
        reader: "P.mention_modules",
        schema_file: "f2-presence.schema.json",
        path: "mention_modules",
        expect_types: &["array"],
        kind: ReadKind::PresentButNotRubricInput,
        source: ReadSource::Pseudocode37,
        foundation_lines: "763-764",
        quote: "(b) `impl_modules`, never `mention_modules` - the latter puts ~525 siblings on \
                almost every entry.",
        note: "Must exist (F2 line 313 declares it R, 'unfiltered set, for audit') and must never \
               reach the warrant or depth predicate.",
    },
    FieldRead {
        reader: "P.unit_tests_direct",
        schema_file: "f2-presence.schema.json",
        path: "unit_tests_direct",
        expect_types: &[],
        kind: ReadKind::MustNotExist,
        source: ReadSource::Pseudocode37,
        foundation_lines: "761-763",
        quote: "(a) The test predicate reads `tests_in_impl_modules`, never the raw \
                `unit_tests_direct` - the latter includes `surface_dispatch.rs`'s 110 own tests and \
                is inflated for 526 of 541 entries.",
        note: "The safest way to honour 'never' is for the field to have no schema slot at all. If \
               a future F2 revision adds it, this entry fails and the reason is on the line above.",
    },
    FieldRead {
        reader: "P.oxfunc_tree_clean",
        schema_file: "f2-presence.schema.json",
        path: "oxfunc_tree_clean",
        expect_types: &["boolean"],
        kind: ReadKind::Read,
        source: ReadSource::RenderingMandate,
        foundation_lines: "308",
        quote: "`oxfunc_tree_clean` | bool | R | `false` blocks publication",
        note: "T2 acceptance test (e) at line 1182: 'oxfunc_tree_clean == false aborts with a \
               non-zero exit and writes nothing.'",
    },
    FieldRead {
        reader: "P.scan_inventory",
        schema_file: "f2-presence.schema.json",
        path: "scan_inventory",
        expect_types: &["object"],
        kind: ReadKind::Read,
        source: ReadSource::RenderingMandate,
        foundation_lines: "330",
        quote: "required rendering field for W0/W1/W2 and D0/D1; IT HAD NO SCHEMA SLOT (A3-F2)",
        note: "This entry is the archetype the whole file exists for: A3-F2 found a REQUIRED \
               RENDERING FIELD with no schema slot. OT-11 (line 783) requires LAMBDA's page to \
               render the scan inventory.",
    },
    FieldRead {
        reader: "P.searched_where",
        schema_file: "f2-presence.schema.json",
        path: "searched_where",
        expect_types: &["array"],
        kind: ReadKind::Read,
        source: ReadSource::OverclaimTest,
        foundation_lines: "331, 782",
        quote: "`searched_where[]` ... the human-readable list the absence block renders / OT-10 \
                STDEV must render ... the searched-in list",
        note: "Second field A3-F2 found with a required rendering and no schema slot.",
    },
    FieldRead {
        reader: "P.limits",
        schema_file: "f2-presence.schema.json",
        path: "limits",
        expect_types: &["array"],
        kind: ReadKind::Read,
        source: ReadSource::RenderingMandate,
        foundation_lines: "332, 1209",
        quote: "limit **ids** [\"L1\"..\"L9\"]; prose in F14 (A3-S7) / G-12 Every data/presence \
                limit id is defined in F14, and every F14 id is referenced.",
        note: "A3-S7: ids here, prose in F14, because two blocks of authored prose were being \
               duplicated 541 times inside the organ that may not be hand-edited.",
    },
    FieldRead {
        reader: "P.doc_mention_classification",
        schema_file: "f2-presence.schema.json",
        path: "doc_mention_classification",
        expect_types: &["array"],
        kind: ReadKind::Read,
        source: ReadSource::OverclaimTest,
        foundation_lines: "327, 784",
        quote: "This is what makes HARMEAN's best sentence generated rather than typed (A3-F2) / \
                OT-12 HARMEAN must render the generated doc_mention_classification sentence",
        note: "T2 acceptance test (f) at line 1182 pins it: HARMEAN yields 10 files, all \
               bulk-inventory.",
    },
    FieldRead {
        reader: "P.name_match_confidence",
        schema_file: "f2-presence.schema.json",
        path: "name_match_confidence",
        expect_types: &["string"],
        kind: ReadKind::Read,
        source: ReadSource::RenderingMandate,
        foundation_lines: "309, 821-822",
        quote: "Tooling metric, labelled as such: entries with name_match_confidence: low (53) - \
                this is constant unless the matching rule changes, so it guards a tooling decision, \
                not an evidence state.",
        note: "A3-S9 demoted RA-15 to a tooling metric; the field is read by /coverage/ and must be \
               labelled as tooling, never as evidence.",
    },
    // ============================================================ F5, evidence records
    FieldRead {
        reader: "r.class",
        schema_file: "f5-evidence-record.schema.json",
        path: "class",
        expect_types: &["string"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "677, 699, 705, 707",
        quote: "for r in E if r.class in {live-verification, structural-verification} ... \
                open_rows := [r in E if r.class == \"open-discrepancy\"] ... \
                xmd := [r in E if r.class == \"excel-math-deviation\"] ... \
                streams := [r in E if r.class == \"defect-stream\" ...]",
        note: "Four separate rungs read this one enum. A1-S7 (line 64) made the defect-stream class \
               a first-class input: 99 surfaces carry an OPEN or repair-landed-unsigned stream and \
               35 of them would otherwise render 'matched' or 'no record' on both axes.",
    },
    FieldRead {
        reader: "r.subjects",
        schema_file: "f5-evidence-record.schema.json",
        path: "subjects",
        expect_types: &["array"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "670-671, 714",
        quote: "E : the set of content/evidence/records/*.json whose subjects contain this id ... \
                if named_only or open_rows or xmd_witness or any comparison record naming s -> W3",
        note: "The join that selects E. Every id must exist in data/functions/ or the build fails \
               (2.5 line 194).",
    },
    FieldRead {
        reader: "r.counts",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts",
        expect_types: &["array"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "678, 706",
        quote: "for c in r.counts ... xmd_witness := any(len(r.counts) > 0 or r.substrate != null \
                for r in xmd)",
        note: "May be empty (2.5 line 205).",
    },
    FieldRead {
        reader: "r.substrate",
        schema_file: "f5-evidence-record.schema.json",
        path: "substrate",
        expect_types: &["string", "null"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "706",
        quote: "xmd_witness := any(len(r.counts) > 0 or r.substrate != null for r in xmd)",
        note: "Drives N4 ('matches Excel by reproducing a documented Excel departure; a witness \
               exists, no count'), which 13 entries occupy.",
    },
    FieldRead {
        reader: "r.upstream_status_verbatim",
        schema_file: "f5-evidence-record.schema.json",
        path: "upstream_status_verbatim",
        expect_types: &["string", "null"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "708-709",
        quote: "streams := [r in E if r.class == \"defect-stream\" and r.upstream_status_verbatim \
                normalises to {open, repair-landed-unsigned}]",
        note: "The verbatim string is normalised, never rewritten in place. Rendered as \
               <status_verbatim> in absence string A-52 (line 901).",
    },
    FieldRead {
        reader: "r.upstream_row_id",
        schema_file: "f5-evidence-record.schema.json",
        path: "upstream_row_id",
        expect_types: &["string", "null"],
        kind: ReadKind::Read,
        source: ReadSource::Invariant37,
        foundation_lines: "755-756, 901",
        quote: "ASSERT streams == [] or the page renders open_defect_streams[] adjacent to every \
                state whose label startswith \"matched\" / A-52: Open defect stream in the \
                reference implementation: <stream_id> - <title> (<status_verbatim>).",
        note: "<stream_id> in A-52 is this field. OT-16 and OT-17 (lines 788-789) both turn on it: \
               MMULT must render its open stream, and NPER must not render N5 without the stream.",
    },
    FieldRead {
        reader: "r.title",
        schema_file: "f5-evidence-record.schema.json",
        path: "title",
        expect_types: &["string"],
        kind: ReadKind::Read,
        source: ReadSource::Invariant37,
        foundation_lines: "755-756, 901",
        quote: "A-52: Open defect stream in the reference implementation: <stream_id> - <title> \
                (<status_verbatim>).",
        note: "<title> in A-52. Capped at 90 chars with no completeness words (2.5 line 196).",
    },
    FieldRead {
        reader: "r.oracle.builds",
        schema_file: "f5-evidence-record.schema.json",
        path: "oracle.builds",
        expect_types: &["array"],
        kind: ReadKind::Read,
        source: ReadSource::Invariant37,
        foundation_lines: "758",
        quote: "ASSERT every counted label contains a build string, an arch string and a cpu string",
        note: "Guard G-7 (line 1204) enforces the same thing against the generated site. A1-S6: \
               builds[] is a normalised ARRAY because five records name two builds for one count, \
               and {build} may never render from free text.",
    },
    FieldRead {
        reader: "r.oracle.arch",
        schema_file: "f5-evidence-record.schema.json",
        path: "oracle.arch",
        expect_types: &["string"],
        kind: ReadKind::Read,
        source: ReadSource::Invariant37,
        foundation_lines: "758",
        quote: "ASSERT every counted label contains a build string, an arch string and a cpu string",
        note: "Carries the explicit value 'not-stated-in-this-record' rather than null, so a page \
               cannot inherit an arch it was never told.",
    },
    FieldRead {
        reader: "r.oracle.cpu",
        schema_file: "f5-evidence-record.schema.json",
        path: "oracle.cpu",
        expect_types: &["string"],
        kind: ReadKind::Read,
        source: ReadSource::Invariant37,
        foundation_lines: "758",
        quote: "ASSERT every counted label contains a build string, an arch string and a cpu string",
        note: "A1-S5 (line 62): 22 entries hold a counted record whose CPU is not stated; their \
               pages render 'host CPU not restated in this record' and NEVER inherit Zen2.",
    },
    // ============================================================ F5 counts[] elements
    FieldRead {
        reader: "c.attribution",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].attribution",
        expect_types: &["string"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "679, 680, 684-687",
        quote: "if c.attribution == \"measured-for-this-surface\" ... # attribution gate -> A1-F4 \
                ... named_only := [c ... if c.attribution in {alias-sibling-inherited, \
                named-but-not-measured, disclaimed-by-source, model-or-candidate-score}]",
        note: "The gate that drops 42 entries out of 'counted' (line 496). STEWARD DECISION SD-4: \
               legacy aliases are DEMOTED to alias-sibling-inherited and never credited with a \
               sibling's count.",
    },
    FieldRead {
        reader: "c.measurement_subject",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].measurement_subject",
        expect_types: &["string"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "679, 681, 700-702",
        quote: "and c.measurement_subject == \"production-oxfunc\" # subject gate -> A1-F7 ... \
                open_class := \"measured\" if any(r has a production-oxfunc count with passed<total \
                ...)",
        note: "A1-F7 (line 49) replaced a classifier that read `figure` and ignored what was being \
                measured. FOUNDATION 7.4 (line 1274) is candid that this field 're-labels an \
                agent's tag, not a fact about the world', and that 61 non-production figures exist.",
    },
    FieldRead {
        reader: "c.passed",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].passed",
        expect_types: &["integer", "null"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "690, 695, 696, 700",
        quote: "if c.passed != null and c.total != null ... residual(axis) := [c in counted(axis) \
                if c.passed < c.total] ... clean(axis) := [c in counted(axis) if c.passed == c.total]",
        note: "A1-F5 (line 47): residual is tested BEFORE clean, and a hard invariant forbids any \
               label beginning 'matched Excel' where passed < total on that axis.",
    },
    FieldRead {
        reader: "c.total",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].total",
        expect_types: &["integer", "null"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "690, 695, 696",
        quote: "if c.passed != null and c.total != null ... c.passed < c.total ... c.passed == c.total",
        note: "null only when the source publishes no denominator (2.5 line 227).",
    },
    FieldRead {
        reader: "c.axis",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].axis",
        expect_types: &["string"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "691, 738-748",
        quote: "and (axis == null or c.axis == axis) ... if residual(numeric-bits) -> N6 ... \
                if residual(structural-admission) -> S6",
        note: "A1-F3 (line 45) destroyed the single X ladder and replaced it with two independent \
               per-axis states. Without this field an entry renders one state where it must render \
               two.",
    },
    FieldRead {
        reader: "c.count_scope",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].count_scope",
        expect_types: &["string"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "692-693, 757",
        quote: "# per-surface counts shadow group counts for the same (record, surface): \
                counted(axis) := prefer_per_surface(counted(axis)) ... ASSERT count_scope == \
                \"group\" implies label contains \"covering {k} functions jointly\"",
        note: "A1-S4 (line 61): without per-surface shadowing, record 13's group 73 reaches SLOPE, \
               whose own figure is 4/4. OT-2 (line 774) forbids the string 73 on SLOPE's page.",
    },
    FieldRead {
        reader: "c.group_members",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].group_members",
        expect_types: &["array"],
        kind: ReadKind::Read,
        source: ReadSource::Invariant37,
        foundation_lines: "757, 231",
        quote: "ASSERT count_scope == \"group\" implies label contains \"covering {k} functions \
                jointly\" / group_members[] | string[] | empty iff count_scope == per-surface",
        note: "{k} is len(group_members). Guard G-3a (line 1199) fails the build on a group-scoped \
               counted label lacking the clause, in either lens. 48 of the 92 counted entries have \
               no per-surface count at all (line 505).",
    },
    FieldRead {
        reader: "c.held_out",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].held_out",
        expect_types: &["string"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "697, 712",
        quote: "held_out := any(c.held_out in {true, partial} for c in counted(null)) ... \
                if counted(null) and held_out -> W5",
        note: "STRING enum, not boolean: the four values are true, false, partial and \
               source-does-not-state (2.5 line 234). A schema that typed this as a boolean would \
               lose 'source-does-not-state' and turn an unknown into a false.",
    },
    FieldRead {
        reader: "c.held_out_rows",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].held_out_rows",
        expect_types: &["integer", "null"],
        kind: ReadKind::Read,
        source: ReadSource::LabelTemplate,
        foundation_lines: "453, 235, 65",
        quote: "W5 | compared to Excel on {n} counted rows{, covering {k} functions jointly}, of \
                which {h} were held out",
        note: "A1-S8 (line 65): W5's single {n} next to 'including a held-out corpus' read as {n} \
               held-out rows, so the template carries both numbers. CORRECTION R25 \
               (ATTRIBUTION-CORRECTIONS 2.1 lines 168-171) changes POWER's values - held_out false, \
               corpus_was_repair_target true - and requires OT-13 to stop demanding '715 counted \
               rows of which 400 were held out'. The FIELD is unchanged; the VALUES are.",
    },
    FieldRead {
        reader: "c.comparison_predicate",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].comparison_predicate",
        expect_types: &["string"],
        kind: ReadKind::Read,
        source: ReadSource::LabelTemplate,
        foundation_lines: "45, 229, 452-453, 605-607",
        quote: "A mandatory `comparison_predicate` in {exact-typed-bit-match, \
                15-significant-digit-truncation} travels with every count.",
        note: "CORRECTION R22-R24 (ATTRIBUTION-CORRECTIONS 2.1 lines 150-164) INVERTS FOUNDATION \
               3.4 lines 605-607 for three of the six surfaces it names. MATCH, XMATCH and DELTA \
               use RAW IEEE == and are the source's deliberately excluded EXACT-MATCH CONTROL GROUP \
               (delta_fn.rs:26 `if lhs == rhs`; xmatch.rs:240 `a == b`; match_fn.rs shares xmatch's \
               comparator; none imports excel_numeric_compare; negative-control tests exist by \
               name). They carry exact-typed-bit-match and subject_role contrast. Only SWITCH, \
               COUNTIF and SUMIF carry 15-significant-digit-truncation, and the label must call it \
               QUANTISE-THEN-COMPARE, never 'tolerant': the helper truncates each operand \
               independently to 15 significant digits and compares exactly, so values arbitrarily \
               close but astride a bucket boundary compare UNEQUAL. Overclaim test OT-15 is \
               inverted and must be rewritten.",
    },
    FieldRead {
        reader: "r.subject_role",
        schema_file: "f5-evidence-record.schema.json",
        path: "subject_role",
        expect_types: &["object"],
        kind: ReadKind::Read,
        source: ReadSource::LabelTemplate,
        foundation_lines: "195",
        quote: "subject_role | object | O | per subject: header | body-only | inherited | contrast",
        note: "CORRECTION R22-R24 requires MATCH, XMATCH and DELTA to carry subject_role \
               'contrast'. ATTRIBUTION-CORRECTIONS line 160 observes that FOUNDATION 2.5 'already \
               defines an unused contrast subject_role for exactly this'. The enum is asserted \
               present by the corrections cross-check in this crate.",
    },
    FieldRead {
        reader: "c.residual_attribution",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].residual_attribution",
        expect_types: &["string", "null"],
        kind: ReadKind::Read,
        source: ReadSource::LabelTemplate,
        foundation_lines: "563, 237, 593-596",
        quote: "S6 | structural check matched on {p} of {n} counted rows{; the single miss belongs \
                to {residual_attribution}}",
        note: "OT-1 (line 773): GCD must render residual_attribution naming TBILLYIELD, which is \
               NOT one of the 14 surfaces on record 39's group 46/47.",
    },
    FieldRead {
        reader: "c.corpus_was_repair_target",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].corpus_was_repair_target",
        expect_types: &["boolean", "string"],
        kind: ReadKind::Read,
        source: ReadSource::StewardDecision,
        foundation_lines: "66, 236, 294",
        quote: "corpus_was_repair_target in {true, false, source-does-not-state} is required and \
                rendered. 18 entries hold a counted record whose corpus was the repair target.",
        note: "STEWARD DECISION SD-5: PMT's in-sample 2,304/2,304 IS published, carrying \
               corpus_was_repair_target: true and its N1 open-divergence state in the SAME VISUAL \
               UNIT. FOUNDATION 7.1 (lines 1240-1243) is explicit that dropping either mechanism \
               turns it into an overclaim. The union type (boolean or the string \
               'source-does-not-state') is why both types are expected here.",
    },
    FieldRead {
        reader: "c.measurement_found",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].measurement_found",
        expect_types: &["boolean"],
        kind: ReadKind::Read,
        source: ReadSource::RenderingMandate,
        foundation_lines: "238",
        quote: "measurement_found | bool | explicit boolean, replaces prose parsing (A1-F7, A3-F1)",
        note: "One of the three booleans that exist so no tool has to parse `figure`.",
    },
    FieldRead {
        reader: "c.divergence_measured",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].divergence_measured",
        expect_types: &["boolean"],
        kind: ReadKind::Read,
        source: ReadSource::RenderingMandate,
        foundation_lines: "239",
        quote: "divergence_measured | bool | ditto",
        note: "Feeds open_class == 'measured' without reading prose (see the NarrativeUnresolved \
               entry for 'a stated drift magnitude', which this field does NOT cover: it records \
               THAT a divergence was measured, not its magnitude).",
    },
    FieldRead {
        reader: "c.full_pass_only",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].full_pass_only",
        expect_types: &["boolean"],
        kind: ReadKind::Read,
        source: ReadSource::Pseudocode37,
        foundation_lines: "240, 702",
        quote: "full_pass_only | bool | ditto / open_class := ... \"no-non-match-measured\" if \
                any(r has only full-pass production counts)",
        note: "Drives N7 via line 742 ('or open_class == \"no-non-match-measured\"').",
    },
    FieldRead {
        reader: "c.figure",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].figure",
        expect_types: &["string"],
        kind: ReadKind::ForbiddenRead,
        source: ReadSource::Pseudocode37,
        foundation_lines: "682, 225, 1206",
        quote: "# NOTE: c.figure is NEVER read. Guard G-9 fails the build on any read of it. / \
                Display only. Guard G-9 fails the build if any tool parses it.",
        note: "The field must EXIST (it is the verbatim upstream string a page displays) and must \
               carry the efhForbiddenRead annotation so the prohibition travels with the schema. \
               T5 acceptance test (d) at line 1185 implements the same guard in C# as an \
               [Obsolete(error: true)] accessor plus a source grep.",
    },
    // ============================================================ F1, for the basis line
    FieldRead {
        reader: "F1.oxfunc_tree_clean",
        schema_file: "f1-function.schema.json",
        path: "oxfunc_tree_clean",
        expect_types: &["boolean"],
        kind: ReadKind::Read,
        source: ReadSource::RenderingMandate,
        foundation_lines: "179, 85",
        quote: "D-8 new oxfunc_tree_clean (bool, R) + vintage (object, R) | false blocks \
                publication of this file.",
        note: "A3-S12: F1 gains oxfunc_tree_clean (publication-blocking, as F2 has).",
    },
    FieldRead {
        reader: "F1.vintage",
        schema_file: "f1-function.schema.json",
        path: "vintage",
        expect_types: &["object"],
        kind: ReadKind::Read,
        source: ReadSource::RenderingMandate,
        foundation_lines: "179, 849-854",
        quote: "A-09 replaced - permanent basis clause (A3-S12): Field vintages on this page: \
                <field group> from OxFunc <state> ... Two or more OxFunc states contribute to this \
                page and the basis line names which.",
        note: "The old A-09 said 115 entries and would render on 0 after delta D-1, removing the \
               warning while keeping the condition. The replacement is ALWAYS rendered in S4.",
    },
    // ============================================================ NARRATIVE READS WITH NO FIELD
    FieldRead {
        reader: "(a stated drift magnitude)",
        schema_file: "f5-evidence-record.schema.json",
        path: "<no field carries this>",
        expect_types: &[],
        kind: ReadKind::NarrativeUnresolved,
        source: ReadSource::Pseudocode37,
        foundation_lines: "700-701",
        quote: "open_class := \"measured\" if any(r has a production-oxfunc count with passed<total \
                OR A STATED DRIFT MAGNITUDE)",
        note: "No field in the F5 counts[] table (2.5 lines 221-244) carries a drift magnitude. The \
               only place a magnitude appears is `figure`, which guard G-9 forbids any tool from \
               parsing. So this disjunct of the N1 predicate is unimplementable against the schema \
               as specified. Corroborated independently: ATTRIBUTION-CORRECTIONS 6 item 23 says \
               'The >16 ULP tail. NO FIELD RECORDS IT. PMT's production replay has 32 rows above 16 \
               ULP; IPMT has 75; CUMPRINC reaches 5.27e14 and CUMIPMT 2.45e13. A page can read \
               \"measured shortfall\" without any hint of the magnitude.' Resolving this needs a \
               spec amendment, not a guess by this tool.",
    },
    FieldRead {
        reader: "why_no_count",
        schema_file: "<none>",
        path: "<no schema declares it>",
        expect_types: &[],
        kind: ReadKind::NarrativeUnresolved,
        source: ReadSource::Pseudocode37,
        foundation_lines: "715, 458-461, 43",
        quote: "-> W3 (+ why_no_count, enumerated) / why_no_count (required at W3, enumerated): \
                count-published-in-prose-not-extracted, only-percentages-published, \
                named-in-a-group-record-not-itself-measured, figure-disclaimed-by-source, \
                figure-is-a-model-or-candidate-score, qualitative-verdict-only, \
                open-row-with-no-figure",
        note: "A1-F1's disposition (line 43) says 'why_no_count BECOMES AN ENUMERATED REQUIRED \
               FIELD (3.2)', but 3.2 states the enum without giving it a home in any 2 schema \
               table, and no F5 or F2 field carries it. Three of its seven values map onto \
               counts[].attribution (disclaimed-by-source, model-or-candidate-score, \
               named-but-not-measured); the other four - count-published-in-prose-not-extracted, \
               only-percentages-published, qualitative-verdict-only, open-row-with-no-figure - are \
               derivable from no declared field. It is recorded here as a gap rather than \
               materialised into a schema this tool has no authority to write.",
    },
    FieldRead {
        reader: "an efh.evidence claim of kind handbook-declared-module",
        schema_file: "<none>",
        path: "<ledger, not a section 2 schema>",
        expect_types: &[],
        kind: ReadKind::NarrativeUnresolved,
        source: ReadSource::Invariant37,
        foundation_lines: "721-722",
        quote: "ASSERT P.impl_modules != [] or warrant == W0 or an efh.evidence claim of kind \
                handbook-declared-module exists  # latent-hole guard, A1-M6",
        note: "This assert resolves against the Gneiss ledger (FOUNDATION 5), not against any \
               section 2 file family, and T1's scope is the section 2 schema set. FOUNDATION line \
               101-102 records that A1-M6 'is applied as a CI ASSERTION rather than a procedure \
               change because no entry occupies it today'. Listed so the read is visible, not \
               silently out of scope.",
    },
    FieldRead {
        reader: "a suite manifest exists on disk",
        schema_file: "<none>",
        path: "vectors/<function_id>/vN/MANIFEST.json",
        expect_types: &[],
        kind: ReadKind::NarrativeUnresolved,
        source: ReadSource::Invariant37,
        foundation_lines: "730-731, 143",
        quote: "# D5 requires vectors/<id>/vN/MANIFEST.json. Zero today. / ASSERT depth != D5 or a \
                suite manifest exists on disk",
        note: "FOUNDATION 2.2 line 143 lists vectors/<function_id>/vN/{suite.jsonl,MANIFEST.json} \
               in the tree and NO section gives it a schema table. Zero suites exist (ratchet RC-4 \
               = 0), so nothing is currently mis-validated, but D5 is unreachable-by-construction \
               until the family is specified. Steward decision SD-3 turns on the same absence: \
               'bit-exact' is forbidden in Handbook-voice prose until vectors/ publishes.",
    },
    FieldRead {
        reader: "label(N-state) / label(S-state)",
        schema_file: "<none>",
        path: "<rubric output, not a stored field>",
        expect_types: &[],
        kind: ReadKind::NarrativeUnresolved,
        source: ReadSource::Invariant37,
        foundation_lines: "752-754",
        quote: "ASSERT not (label(N-state) startswith \"matched\" and residual(numeric-bits)) ... \
                ASSERT \"bit\" not in label(S-state) and \"exact\" not in label(S-state)",
        note: "The three string invariants read a LABEL the rubric computes from the templates in \
               3.2 and 3.4, not a field in any file. They belong to T5 (tools/efh/rubric) and to \
               guards G-3 and G-8 against the generated site, and are listed here so that 'the \
               schema set does not cover them' is a recorded fact rather than an oversight. \
               STEWARD DECISION SD-3 reinforces G-8: 'bit-exact'/'bit-for-bit' are FORBIDDEN in \
               Handbook-voice prose until vectors/ publishes, and CHARTER 7.2 is NOT amended.",
    },
];

// =====================================================================================
// Acceptance test (c): no field declared in a content/ schema is a total function of a
// data/ schema. A3-S8, FOUNDATION line 81:
//   "chip_suppression, member_count, display_label_differs_from_ingested are pure functions of
//    data/ frozen in content/ ... All three are deleted from content/ and computed at build time.
//    Review rule: no field in content/ may be a total function of data/; the T1 acceptance test
//    enumerates and asserts it."
// =====================================================================================

pub struct TotalFunctionField {
    /// The field name that must not be declared.
    pub field: &'static str,
    /// The content/ schema it used to sit in.
    pub content_schema: &'static str,
    /// The data/ inputs that compute it, so a reader can check the "total function" claim.
    pub computed_from: &'static str,
    pub foundation_lines: &'static str,
    pub quote: &'static str,
}

pub const TOTAL_FUNCTION_ALLOW_LIST: &[TotalFunctionField] = &[
    TotalFunctionField {
        field: "chip_suppression",
        content_schema: "f4-axis-glossary.schema.json",
        computed_from: "F3 axes[].modal_share >= 0.90 (data/axes/vocabulary.json), F1 \
                        axis_provenance.<key>.oxfunc_tier in {B, C} (data/functions/*.json), and \
                        the vacuity rule over F1 classification.coercion_lift_profile",
        foundation_lines: "81, 340-343",
        quote: "F4 content/model/axis-glossary.json - as D1 2.4 MINUS chip_suppression, which is \
                now derived at build time from F3 modal_share >= 0.90, oxfunc_tier in {B, C}, and \
                the vacuity rule (A3-S8).",
    },
    TotalFunctionField {
        field: "member_count",
        content_schema: "f8-category-page.schema.json",
        computed_from: "the number of F0 rows[] (data/index.json) whose `category` equals this \
                        page's category_string",
        foundation_lines: "81, 374",
        quote: "F8 loses member_count and display_label_differs_from_ingested (derived - A3-S8)",
    },
    TotalFunctionField {
        field: "display_label_differs_from_ingested",
        content_schema: "f8-category-page.schema.json",
        computed_from: "display_label != category_string, where category_string is itself a \
                        verbatim copy of F1 `category` (data/functions/*.json)",
        foundation_lines: "81, 374",
        quote: "F8 loses member_count and display_label_differs_from_ingested (derived - A3-S8)",
    },
];

// =====================================================================================
// Cross-checks that ATTRIBUTION-CORRECTIONS 7 forces onto the schema set. Where the
// corrections disagree with FOUNDATION, the corrections win.
// =====================================================================================

pub struct CorrectionCheck {
    pub id: &'static str,
    pub schema_file: &'static str,
    /// Dotted path to a node whose `enum` must contain `must_contain`.
    pub path: &'static str,
    pub must_contain: &'static str,
    pub why: &'static str,
}

pub const CORRECTION_CHECKS: &[CorrectionCheck] = &[
    CorrectionCheck {
        id: "R22-R24 / OT-15 (INVERTED)",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].comparison_predicate",
        must_contain: "exact-typed-bit-match",
        why: "ATTRIBUTION-CORRECTIONS 7 records OT-15 as INVERTED: MATCH, XMATCH and DELTA use raw \
              IEEE == and are the source's exact-match control group, so the schema must be able to \
              carry exact-typed-bit-match for them. FOUNDATION 3.4 lines 605-607, which calls all \
              six surfaces tolerant, is superseded on this point.",
    },
    CorrectionCheck {
        id: "R22-R24 / OT-15 (retarget)",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].comparison_predicate",
        must_contain: "15-significant-digit-truncation",
        why: "The tolerant value survives, retargeted to SWITCH, COUNTIF and SUMIF only, and its \
              label must read quantise-then-compare rather than tolerant.",
    },
    CorrectionCheck {
        id: "R22-R24 / subject_role contrast",
        schema_file: "f5-evidence-record.schema.json",
        path: "subject_role.<any>",
        must_contain: "contrast",
        why: "The corrections require MATCH, XMATCH and DELTA to carry subject_role 'contrast', and \
              note that FOUNDATION 2.5 already defines the value for exactly this purpose.",
    },
    CorrectionCheck {
        id: "R25 / OT-13",
        schema_file: "f5-evidence-record.schema.json",
        path: "counts[].held_out",
        must_contain: "false",
        why: "CORRECTION R25: POWER's held_out becomes false with corpus_was_repair_target true, \
              because neither half of the 715 rows is cleanly held out for the (a)+(b) algorithm \
              the figure describes. The schema must be able to carry the corrected value.",
    },
];
