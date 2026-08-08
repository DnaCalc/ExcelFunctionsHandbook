namespace Efh.Rubric;

public sealed record HygieneEntry(
    string Kind,
    string RecordId,
    int CountIndex,
    string Field,
    string Observed,
    string Normalised,
    string Note);

/// <summary>
/// Every normalisation the loader applies to the evidence records, recorded so the repair is
/// visible in the published output instead of buried in a parser.
/// </summary>
public sealed class HygieneReport
{
    private readonly List<HygieneEntry> _entries = new();

    public IReadOnlyList<HygieneEntry> Entries => _entries;

    public void Add(string kind, string recordId, int countIndex, string field,
                    string observed, string normalised, string note) =>
        _entries.Add(new HygieneEntry(kind, recordId, countIndex, field, observed, normalised, note));

    public IEnumerable<IGrouping<string, HygieneEntry>> ByKind() =>
        _entries.GroupBy(e => e.Kind).OrderBy(g => g.Key, StringComparer.Ordinal);

    public int CountOf(string kind) => _entries.Count(e => e.Kind == kind);

    public IReadOnlyList<string> RecordsAffected(string kind) =>
        _entries.Where(e => e.Kind == kind)
                .Select(e => e.RecordId)
                .Distinct(StringComparer.Ordinal)
                .OrderBy(x => x, StringComparer.Ordinal)
                .ToList();

    public static class Kinds
    {
        public const string HeldOutStringBoolean =
            "held_out-carried-as-a-JSON-string-instead-of-a-boolean";
        public const string BuildAmbiguityVariantSpelling =
            "build_ambiguity-variant-spelling-build-not-stated-in-this-record";
        public const string CurrencyAbsent =
            "currency-absent-recorded-as-not-annotated-never-as-current";
        public const string RepairTargetStringBoolean =
            "corpus_was_repair_target-carried-as-a-JSON-string-instead-of-a-boolean";
        public const string HeldOutRowsWithoutHeldOut =
            "held_out_rows-populated-while-held_out-is-false-or-unstated";
        public const string MissingOptionalField =
            "count-field-absent-defaulted";
    }
}
