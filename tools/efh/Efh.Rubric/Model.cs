namespace Efh.Rubric;

// ---------------------------------------------------------------- vocabulary

public static class Axis
{
    public const string Numeric = "numeric-bits";
    public const string Structural = "structural-admission";
}

public static class Attribution
{
    public const string Measured = "measured-for-this-surface";
    public const string AliasInherited = "alias-sibling-inherited";
    public const string NamedNotMeasured = "named-but-not-measured";
    public const string Disclaimed = "disclaimed-by-source";
    public const string ModelOrCandidate = "model-or-candidate-score";
    public const string Unclear = "attribution-unclear";
}

public static class Subject
{
    public const string Production = "production-oxfunc";
    public const string Research = "research-model-or-candidate";
    public const string ExcelVsTruth = "excel-vs-truth";
    public const string ExcelVsExcel = "excel-vs-excel-identity";
    public const string InstrumentValidation = "instrument-validation";
    public const string InternalRegression = "internal-regression";
    public const string Sibling = "sibling";
    public const string Unclear = "attribution-unclear";

    /// <summary>
    /// A computation the Handbook performed over a corpus of stored bits. NOT a live-Excel
    /// measurement. It may never produce an N5/N6/S5/S6 "matched Excel" state; it produces the
    /// weaker NR/SR states and the W3R warrant.
    /// </summary>
    public const string HandbookRecomputation = "handbook-recomputation-over-cached-corpus";
}

public static class Currency
{
    public const string Current = "current";
    public const string CurrentAndOnly = "current-and-only";
    public const string Superseded = "superseded";
    public const string PreRepairBaseline = "pre-repair-baseline";
    public const string StaleInSource = "stale-in-source";
    public const string Withdrawn = "withdrawn";

    /// <summary>
    /// The record carries no <c>currency</c> field for this count. Absence is NOT "current".
    /// It is its own state: the record does not say whether this count is the current one.
    /// Counts in this state are eligible to derive a state, but every label built on one carries
    /// the not-annotated clause, and they are counted separately in the honesty counters.
    /// </summary>
    public const string NotAnnotated = "not-annotated";

    public static bool IsUsableForState(string c) =>
        c is Current or CurrentAndOnly or NotAnnotated;
}

public static class HeldOut
{
    public const string True = "true";
    public const string False = "false";
    public const string Partial = "partial";
    public const string SourceDoesNotState = "source-does-not-state";
}

public static class BuildAmbiguity
{
    public const string SingleBuild = "single-build";
    public const string NotRestated = "single-build-not-restated-on-the-scored-line";
    public const string TwoBuilds = "two-builds-named";
    public const string SourcesContradict = "sources-contradict";
    public const string NotStated = "not-stated-in-this-record";
}

public static class Binding
{
    public const string ExplicitField = "explicit-subject-field-on-the-count";
    public const string SingleSubjectRecord = "single-subject-record";
    public const string HeaderRole = "sole-header-subject-role";
    public const string GroupMembers = "group-members-list";
    public const string GroupAllSubjects = "group-scope-over-all-record-subjects";

    /// <summary>
    /// A per-surface count inside a multi-subject record where the schema carries no field naming
    /// which subject the count belongs to. The only carrier of that binding in the source is the
    /// prose in the display text / <c>source_sentence</c>, and G-9 forbids parsing it. Such a count
    /// may not produce a counted warrant or a "matched Excel" state for anybody.
    /// </summary>
    public const string Unbindable = "unbindable-no-subject-field-in-schema";
}

// ---------------------------------------------------------------- loaded shapes

public sealed record CountRow
{
    public required string RecordId { get; init; }
    public required int Index { get; init; }

    public int? Passed { get; init; }
    public int? Total { get; init; }
    public required string AxisValue { get; init; }
    public required string ComparisonPredicate { get; init; }
    public required string CountScope { get; init; }
    public required IReadOnlyList<string> GroupMembers { get; init; }
    public required string AttributionValue { get; init; }
    public required string MeasurementSubject { get; init; }
    public required string HeldOutValue { get; init; }
    public int? HeldOutRows { get; init; }
    public required string CorpusWasRepairTarget { get; init; }
    public string? ResidualAttribution { get; init; }
    public required bool MeasurementFound { get; init; }
    public required bool DivergenceMeasured { get; init; }
    public required bool FullPassOnly { get; init; }
    public required IReadOnlyList<string> Builds { get; init; }
    public required string BuildAmbiguityValue { get; init; }
    public string? BuildNote { get; init; }
    public required string Arch { get; init; }
    public required string Cpu { get; init; }
    public required bool CpuScoped { get; init; }
    public required string CorpusOrBuild { get; init; }
    public required bool CorpusTracked { get; init; }
    public string? MeasuredAsOf { get; init; }
    public required string CurrencyValue { get; init; }
    public string? AmbiguityNote { get; init; }
    public required string Citation { get; init; }

    /// <summary>Subject ids named directly on the count, when the record carries them. Usually empty.</summary>
    public required IReadOnlyList<string> ExplicitSubjects { get; init; }

    public bool IsNumeric => AxisValue == Axis.Numeric;
    public bool IsStructural => AxisValue == Axis.Structural;
    public bool HasNumerator => Passed.HasValue && Total.HasValue;
    public bool IsHeldOut => HeldOutValue is HeldOut.True or HeldOut.Partial;
    public bool IsResidual => HasNumerator && Passed!.Value < Total!.Value;
    public bool IsClean => HasNumerator && Passed!.Value == Total!.Value;
    public bool UsableForState => Currency.IsUsableForState(CurrencyValue);
    public string Key => $"{RecordId}#{Index}";
}

public sealed record EvidenceRecord
{
    public required string RecordId { get; init; }
    public required int RecordVersion { get; init; }
    public required string Class { get; init; }
    public required IReadOnlyList<string> Subjects { get; init; }
    public required IReadOnlyDictionary<string, string> SubjectRole { get; init; }
    public required string Title { get; init; }
    public string? UpstreamRowId { get; init; }
    public required string UpstreamRegister { get; init; }
    public string? UpstreamStatusVerbatim { get; init; }
    public string? Substrate { get; init; }
    public string? ReaderWarning { get; init; }
    public required bool HandbookReverified { get; init; }
    public required IReadOnlyList<CountRow> Counts { get; init; }
    public required string SourcePath { get; init; }

    /// <summary>
    /// Per-subject: what predicate that surface's own implementation uses when it compares two
    /// numbers. This is NOT the comparison predicate of the measurement; it is a property of the
    /// surface. MATCH / XMATCH / DELTA carry raw-ieee-equality and are the exact-match control arm;
    /// SWITCH / COUNTIF / SUMIF and the comparison operators carry quantise-then-compare.
    /// </summary>
    public required IReadOnlyDictionary<string, string> SubjectInternalEqualityPredicate { get; init; }

    public required bool MustNotRenderAlone { get; init; }
    public required IReadOnlyList<string> RenderTogetherWith { get; init; }
}

public sealed record PresenceRow
{
    public required string FunctionId { get; init; }
    public required string SurfaceName { get; init; }
    public required IReadOnlyList<string> ImplModules { get; init; }
    public required int TestsInImplModules { get; init; }
    public required int ModuleSharedByCount { get; init; }
    public required IReadOnlyList<string> ModuleSharedBy { get; init; }
    public required IReadOnlyDictionary<string, int> ModuleTestsMinusSiblingCount { get; init; }
    public required IReadOnlyDictionary<string, int> TestsPerModule { get; init; }
    public required string NameMatchConfidence { get; init; }
    public required bool DeclaredArtifactsPresent { get; init; }
    public required IReadOnlyList<string> BugStreamFiles { get; init; }
    public required bool OxfuncTreeClean { get; init; }
}

public sealed record FunctionRow
{
    public required string FunctionId { get; init; }
    public required string SurfaceName { get; init; }
    public required string EntryKind { get; init; }
    public required string Category { get; init; }
}
