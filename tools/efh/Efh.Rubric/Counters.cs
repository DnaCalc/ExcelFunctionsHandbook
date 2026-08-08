namespace Efh.Rubric;

public sealed record ComparisonRow(
    string Fact,
    int Derived,
    int? Foundation,
    int? AttributionCorrections,
    string Explanation);

/// <summary>
/// Populations derived from the organs as they stand on disk. Nothing here is copied from
/// FOUNDATION 3 or from ATTRIBUTION-CORRECTIONS 4; those two are carried only as comparison
/// columns so every difference is visible.
/// </summary>
public sealed class Counters
{
    public const string DenominatorSentence =
        "541 entries = 518 functions + 23 operators = 534 published Excel rows + 7 split byte-variant rows";

    private readonly RubricEngine _e;
    private readonly Loader _loader;

    public Counters(RubricEngine engine, Loader loader) { _e = engine; _loader = loader; }

    public int Total => _e.Assignments.Count;

    public int Warrant(string level) => _e.Assignments.Count(a => a.Warrant == level);
    public int Depth(string tier) => _e.Assignments.Count(a => a.Depth == tier);
    public int NumericState(string s) => _e.Assignments.Count(a => a.Numeric.State == s);
    public int StructuralState(string s) => _e.Assignments.Count(a => a.Structural.State == s);
    public int Flag(Func<Flags, bool> f) => _e.Assignments.Count(a => f(a.Flags));

    /// <summary>Entries — not files — that carry a curated page named for their function id.</summary>
    public int CuratedPages()
    {
        var dir = Path.Combine(_loader.Root, "content", "functions");
        if (!Directory.Exists(dir)) return 0;
        var have = Directory.EnumerateFiles(dir, "*.md")
                            .Select(Path.GetFileNameWithoutExtension)
                            .Where(x => x is not null)
                            .ToHashSet(StringComparer.Ordinal)!;
        return _e.Assignments.Count(a => have.Contains(a.FunctionId));
    }

    public IReadOnlyList<string> Members(Func<Assignment, bool> f) =>
        _e.Assignments.Where(f).Select(a => a.SurfaceName).OrderBy(x => x, StringComparer.Ordinal).ToList();

    // ---------------------------------------------------------------- honesty counters

    public Dictionary<string, int> HonestyCounters() => new(StringComparer.Ordinal)
    {
        ["entries_total"] = Total,
        ["entries_with_any_evidence_record"] = Flag(f => f.HasAnyEvidenceRecord),
        ["entries_with_no_evidence_record_of_any_kind"] = Flag(f => !f.HasAnyEvidenceRecord),
        ["entries_with_a_counted_comparison_record"] = Flag(f => f.HasCountedRecord),
        ["of_those_with_a_per_surface_count"] = Flag(f => f.HasCountedRecord && f.HasPerSurfaceCount),
        ["of_those_whose_only_counted_evidence_is_a_group_total"] = Flag(f => f.OnlyCountedEvidenceIsGroupTotal),
        ["entries_with_a_numeric_bits_counted_record"] = Flag(f => f.HasNumericCountedRecord),
        ["of_those_with_a_per_surface_numeric_count"] = Flag(f => f.HasPerSurfaceNumericCount),
        ["entries_whose_only_counted_axis_is_structural_admission"] = Flag(f => f.OnlyCountedAxisIsStructural),
        ["entries_with_a_held_out_counted_record"] = Flag(f => f.HasHeldOutCountedRecord),
        ["entries_whose_counted_corpus_was_the_repair_target"] = Flag(f => f.CorpusWasRepairTarget),
        ["entries_holding_a_counted_record_whose_host_cpu_is_not_stated"] = Flag(f => f.CpuNotStatedOnACountedRecord),
        ["entries_holding_a_counted_record_with_a_non_single_build"] = Flag(f => f.NonSingleBuildOnACountedRecord),
        ["entries_holding_a_counted_record_with_no_currency_annotation"] = Flag(f => f.CurrencyNotAnnotatedOnACountedRecord),
        ["entries_dropped_from_counted_by_the_attribution_gate"] = Flag(f => f.DroppedFromCountedByAttributionGate),
        ["entries_dropped_from_counted_by_the_measurement_subject_gate"] = Flag(f => f.DroppedFromCountedByMeasurementSubjectGate),
        ["entries_holding_an_unbindable_per_surface_count"] = Flag(f => f.HasUnbindablePerSurfaceCount),
        ["entries_holding_a_handbook_recomputation_count"] = Flag(f => f.HasHandbookRecomputationCount),
        ["entries_whose_only_counted_evidence_is_a_handbook_recomputation"] = Flag(f => f.HasHandbookRecomputationCount && !f.HasCountedRecord),
        ["entries_with_an_open_defect_stream_record"] = Flag(f => f.HasOpenDefectStreamRecord),
        ["entries_named_in_a_defect_stream_file_any_status"] = Flag(f => f.NamedInADefectStreamFileAnyStatus),
        ["entries_with_an_open_catalogue_row"] = Flag(f => f.HasOpenCatalogueRow),
        ["entries_with_a_residual_register_row"] = Flag(f => f.HasResidualRegisterRow),
        ["entries_numeric_clean_no_open_row_and_no_defect_stream_file_names_it"] = Flag(f => f.NumericCleanNoOpenRowNoStream),
        ["entries_numeric_clean_no_open_row_and_no_open_defect_stream_record"] = Flag(f => f.NumericCleanNoOpenRowNoOpenStreamRecord),
        ["entries_numeric_clean_no_open_row_streams_ignored"] = Flag(f => f.NumericCleanNoOpenRow),
        ["entries_whose_open_row_classifier_used_the_fallback"] = Flag(f => f.OpenRowClassifierUsedFallback),
        ["entries_with_no_implementation_module"] = Flag(f => f.ImplModuleAbsent),
        ["entries_whose_warrant_was_capped_by_G1"] = Flag(f => f.CappedByG1),
        ["entries_with_a_battery_row_set"] = Flag(f => f.HasBattery),
        ["entries_in_the_pigeonhole_set"] = Flag(f => f.InPigeonholeSet),
        ["entries_with_low_name_match_confidence"] = Flag(f => f.LowNameMatchConfidence),
        ["entries_with_curated_handbook_prose"] = CuratedPages(),
        ["curated_family_pages"] = _loader.CountFiles("content/families", "*.md"),
        ["evidence_records"] = _e.Records.Count,
        ["published_vector_suites"] = _loader.CountVectorSuites(),
        ["admitted_handbook_implementations"] = _loader.CountAdmittedImplementations(),
    };

    // ---------------------------------------------------------------- comparison table

    public IReadOnlyList<ComparisonRow> Comparison()
    {
        var h = HonestyCounters();
        var rows = new List<ComparisonRow>();

        void Row(string fact, int derived, int? found, int? ac, string expl) =>
            rows.Add(new ComparisonRow(fact, derived, found, ac, expl));

        // ---- warrant ladder (FOUNDATION 3.2; AC 4 could not recompute it)
        Row("W0", Warrant("W0"), 15, null, "");
        Row("W1", Warrant("W1"), 42, null, "");
        Row("W2", Warrant("W2"), 325, null, "");
        Row("W3", Warrant("W3"), 67, null, "");
        Row("W3R (new level: Handbook recomputation only)", Warrant("W3R"), null, null,
            "FOUNDATION has no level for measurement_subject handbook-recomputation-over-cached-corpus, which did not exist when it was written.");
        Row("W4", Warrant("W4"), 73, null, "");
        Row("W5", Warrant("W5"), 19, null, "");

        // ---- depth
        Row("D0", Depth("D0"), 15, null, "");
        Row("D1", Depth("D1"), 44, null, "");
        Row("D2", Depth("D2"), 256, null, "");
        Row("D3", Depth("D3"), 106, null, "");
        Row("D4", Depth("D4"), 120, null, "");
        Row("D5", Depth("D5"), 0, null, "");

        // ---- numeric axis
        Row("N1", NumericState("N1"), 51, null, "");
        Row("N2", NumericState("N2"), 2, null, "");
        Row("N3", NumericState("N3"), 10, null, "");
        Row("N4", NumericState("N4"), 13, null, "");
        Row("N5", NumericState("N5"), 16, null, "");
        Row("N6", NumericState("N6"), 6, null, "");
        Row("NR (new state: Handbook recomputation)", NumericState("NR"), null, null,
            "New state. FOUNDATION had no place to put a recomputation over stored bits and would have rendered it as a live-Excel match.");
        Row("N7", NumericState("N7"), 17, null, "");
        Row("N8", NumericState("N8"), 426, null, "");

        // ---- structural axis
        Row("S5", StructuralState("S5"), 29, null, "");
        Row("S6", StructuralState("S6"), 14, null, "");
        Row("SR (new state: Handbook recomputation)", StructuralState("SR"), null, null, "New state, as NR.");
        Row("S7", StructuralState("S7"), 4, null, "");
        Row("S8", StructuralState("S8"), 494, null, "");

        // ---- honesty counters (FOUNDATION 3.6 / AC 4)
        Row("entries with >=1 counted comparison record", h["entries_with_a_counted_comparison_record"], 92, 117, "");
        Row("— of those, with >=1 per-surface count", h["of_those_with_a_per_surface_count"], 44, 65, "");
        Row("— of those, whose only counted evidence is a group total", h["of_those_whose_only_counted_evidence_is_a_group_total"], 48, 52, "");
        Row("entries with >=1 numeric-bits counted record", h["entries_with_a_numeric_bits_counted_record"], 50, 73, "");
        Row("— of those, with a per-surface numeric count", h["of_those_with_a_per_surface_numeric_count"], 43, 64, "");
        Row("entries whose only counted axis is structural-admission", h["entries_whose_only_counted_axis_is_structural_admission"], 42, 44, "");
        Row("entries with >=1 held-out counted record", h["entries_with_a_held_out_counted_record"], 19, 29, "");
        Row("entries whose counted corpus was the repair target", h["entries_whose_counted_corpus_was_the_repair_target"], 18, 34, "");
        Row("entries holding a counted record whose host CPU is not stated", h["entries_holding_a_counted_record_whose_host_cpu_is_not_stated"], 22, 75, "");
        Row("entries holding a counted record with a non-single build", h["entries_holding_a_counted_record_with_a_non_single_build"], 20, 80, "");
        Row("entries numeric-clean, no open catalogue row, no defect-stream file names it", h["entries_numeric_clean_no_open_row_and_no_defect_stream_file_names_it"], 12, 12, "");
        Row("entries numeric-clean, no open catalogue row (streams ignored)", h["entries_numeric_clean_no_open_row_streams_ignored"], 25, 27, "");
        Row("entries with any evidence record", h["entries_with_any_evidence_record"], 320, null, "");
        Row("entries with no evidence record of any kind", h["entries_with_no_evidence_record_of_any_kind"], 221, null, "");
        Row("entries named in a defect-stream file (any status)", h["entries_named_in_a_defect_stream_file_any_status"], 288, null,
            "FOUNDATION 3.6's 288 is 'surfaces named in any OxFunc defect stream, any status'. Its separate 99 counts OPEN or repair-landed-unsigned streams; data/presence records the stream FILE PATH and not the stream's status, so the rubric cannot derive 99 from the organs it reads and does not pretend to.");
        Row("entries with curated Handbook prose", h["entries_with_curated_handbook_prose"], 0, null, "");
        Row("published vector suites", h["published_vector_suites"], 0, null, "");
        Row("admitted Handbook implementations", h["admitted_handbook_implementations"], 0, null, "");
        Row("entries with a battery row set", h["entries_with_a_battery_row_set"], 0, null,
            "FOUNDATION RC-9 recorded 0 because data/battery did not exist when it was written.");
        Row("entries in the pigeonhole set", h["entries_in_the_pigeonhole_set"], 202, null, "");

        return rows;
    }
}
