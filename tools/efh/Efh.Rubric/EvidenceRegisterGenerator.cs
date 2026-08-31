using System.Text.Encodings.Web;
using System.Text.Json;
using System.Text.Json.Nodes;
using System.Text.RegularExpressions;

namespace Efh.Rubric;

/// <summary>
/// Rebuilds F5r as a fail-closed, deterministic projection of the F5 record directory.
/// Only typed fields used by the index are read from each source record.
/// </summary>
public static class EvidenceRegisterGenerator
{
    private static readonly Regex RecordIdPattern = new(
        @"^(?<prefix>EV-[A-Z-]+)-[0-9]{4}$",
        RegexOptions.Compiled | RegexOptions.CultureInvariant);

    private static readonly JsonSerializerOptions JsonOptions = new()
    {
        WriteIndented = true,
        Encoder = JavaScriptEncoder.UnsafeRelaxedJsonEscaping,
    };

    public static string Generate(string handbookRoot)
    {
        var recordsDirectory = Path.Combine(handbookRoot, "content", "evidence", "records");
        if (!Directory.Exists(recordsDirectory))
            throw new DirectoryNotFoundException(recordsDirectory);

        var records = Directory.EnumerateFiles(recordsDirectory, "*.json")
            .OrderBy(Path.GetFileName, StringComparer.Ordinal)
            .Select(LoadRecord)
            .OrderBy(record => record.RecordId, StringComparer.Ordinal)
            .ToArray();
        if (records.Length == 0)
            throw new InvalidDataException("content/evidence/records contains no JSON records");

        var duplicateIds = records.GroupBy(record => record.RecordId, StringComparer.Ordinal)
            .Where(group => group.Count() > 1)
            .Select(group => group.Key)
            .ToArray();
        if (duplicateIds.Length > 0)
            throw new InvalidDataException(
                "duplicate evidence record ids: " + string.Join(", ", duplicateIds));

        var basisRepo = SingleValue(records.Select(record => record.BasisRepo), "basis_repo");
        var basisCommit = SingleValue(records.Select(record => record.BasisCommit), "basis_commit");

        var root = LoadHeader(handbookRoot, basisRepo, basisCommit);

        var prefixes = new JsonObject();
        foreach (var group in records.GroupBy(record => record.Prefix, StringComparer.Ordinal)
                     .OrderBy(group => group.Key, StringComparer.Ordinal))
            prefixes[group.Key] = group.Count();
        root["record_id_prefixes_present"] = prefixes;
        root["records_indexed"] = records.Length;
        root["distinct_subjects_referenced"] = records
            .SelectMany(record => record.Subjects)
            .Distinct(StringComparer.Ordinal)
            .Count();
        root["distinct_excel_builds_named_across_all_counts"] = StringArray(records
            .SelectMany(record => record.Builds)
            .Distinct(StringComparer.Ordinal)
            .OrderBy(value => value, StringComparer.Ordinal));

        // A malformed input aborts before this object is written. The complete register never
        // silently publishes a partial directory scan.
        root["malformed_files_skipped"] = new JsonArray();

        var entries = new JsonArray();
        foreach (var record in records)
        {
            entries.Add(new JsonObject
            {
                ["record_id"] = record.RecordId,
                ["path"] = record.Path,
                ["class"] = record.Class,
                ["title"] = record.Title,
                ["subjects"] = StringArray(record.Subjects),
                ["upstream_row_id"] = record.UpstreamRowId,
                ["upstream_register"] = record.UpstreamRegister,
                ["upstream_anchor_file"] = record.UpstreamAnchorFile,
                ["counts_declared"] = record.CountsDeclared,
                ["counts_with_a_published_numerator"] = record.CountsWithPublishedNumerator,
                ["counts_with_a_published_denominator"] = record.CountsWithPublishedDenominator,
                ["axes_present"] = StringArray(record.Axes),
                ["has_group_scoped_count"] = record.HasGroupScopedCount,
                ["has_repair_target_count"] = record.HasRepairTargetCount,
                ["has_retraction"] = record.HasRetraction,
                ["evidence_locality"] = record.EvidenceLocality,
                ["record_version"] = record.RecordVersion,
                ["handbook_reverified"] = false,
            });
        }
        root["records"] = entries;

        return root.ToJsonString(JsonOptions).ReplaceLineEndings("\n") + "\n";
    }

    public static void Emit(string handbookRoot)
    {
        var text = Generate(handbookRoot);
        var path = Path.Combine(handbookRoot, "content", "evidence", "REGISTER.json");
        File.WriteAllText(path, text, new System.Text.UTF8Encoding(false));
    }

    private static JsonObject LoadHeader(
        string handbookRoot,
        string basisRepo,
        string basisCommit)
    {
        var path = Path.Combine(
            handbookRoot, "tools", "efh", "Efh.Rubric", "EvidenceRegisterHeader.template.json");
        if (JsonNode.Parse(File.ReadAllText(path)) is not JsonObject root)
            throw new InvalidDataException("evidence-register header template is not a JSON object");
        root["basis_repo"] = basisRepo;
        root["basis_commit"] = basisCommit;
        return root;
    }

    private static SourceRecord LoadRecord(string path)
    {
        try
        {
            using var document = JsonDocument.Parse(File.ReadAllText(path));
            var root = document.RootElement;
            if (RequiredString(root, "schema") != "efh.evidence-record/v2")
                throw new InvalidDataException("schema is not efh.evidence-record/v2");

            var recordId = RequiredString(root, "record_id");
            var expectedId = Path.GetFileNameWithoutExtension(path);
            if (!StringComparer.Ordinal.Equals(recordId, expectedId))
                throw new InvalidDataException(
                    $"record_id {recordId} does not match filename {expectedId}");

            var idMatch = RecordIdPattern.Match(recordId);
            if (!idMatch.Success)
                throw new InvalidDataException($"record_id {recordId} is not canonical");

            var reverified = RequiredBool(root, "handbook_reverified");
            if (reverified)
                throw new InvalidDataException(
                    "handbook_reverified is true in a pre-ledger evidence record");

            var oracle = RequiredProperty(root, "oracle");
            if (oracle.ValueKind != JsonValueKind.Null)
                throw new InvalidDataException(
                    "record-level oracle is non-null; the register forbids inherited oracle fields");

            var subjects = StringValues(RequiredArray(root, "subjects"), "subjects");
            var counts = RequiredArray(root, "counts");
            var axes = new SortedSet<string>(StringComparer.Ordinal);
            var builds = new SortedSet<string>(StringComparer.Ordinal);
            var publishedNumerators = 0;
            var publishedDenominators = 0;
            var hasGroupScopedCount = false;
            var hasRepairTargetCount = false;

            foreach (var count in counts.EnumerateArray())
            {
                if (count.ValueKind != JsonValueKind.Object)
                    throw new InvalidDataException("counts[] contains a non-object");

                axes.Add(RequiredString(count, "axis"));
                if (RequiredString(count, "count_scope") == "group")
                    hasGroupScopedCount = true;
                if (IsRepairTarget(count))
                    hasRepairTargetCount = true;

                if (RequiredProperty(count, "passed").ValueKind != JsonValueKind.Null)
                    publishedNumerators++;
                if (RequiredProperty(count, "total").ValueKind != JsonValueKind.Null)
                    publishedDenominators++;

                foreach (var build in StringValues(RequiredArray(count, "builds"), "counts[].builds"))
                    builds.Add(build);

                // These are the fields promised by reading_rules. Reading them establishes their
                // presence without interpreting or inheriting their values.
                _ = RequiredString(count, "build_ambiguity");
                _ = RequiredString(count, "arch");
                _ = RequiredString(count, "cpu");
                _ = RequiredString(count, "excel_bitness");
                _ = RequiredString(count, "workbook_mode");
                _ = RequiredString(count, "plumbing");
            }

            var anchor = RequiredProperty(root, "upstream_anchor");
            if (anchor.ValueKind != JsonValueKind.Object)
                throw new InvalidDataException("upstream_anchor is not an object");

            return new SourceRecord(
                RecordId: recordId,
                Prefix: idMatch.Groups["prefix"].Value,
                Path: "content/evidence/records/" + Path.GetFileName(path),
                Class: RequiredString(root, "class"),
                Title: RequiredString(root, "title"),
                Subjects: subjects,
                UpstreamRowId: OptionalString(root, "upstream_row_id"),
                UpstreamRegister: RequiredString(root, "upstream_register"),
                UpstreamAnchorFile: RequiredString(anchor, "file"),
                CountsDeclared: counts.GetArrayLength(),
                CountsWithPublishedNumerator: publishedNumerators,
                CountsWithPublishedDenominator: publishedDenominators,
                Axes: axes.ToArray(),
                Builds: builds.ToArray(),
                HasGroupScopedCount: hasGroupScopedCount,
                HasRepairTargetCount: hasRepairTargetCount,
                HasRetraction: RequiredArray(root, "retractions").GetArrayLength() > 0,
                EvidenceLocality: RequiredString(root, "evidence_locality"),
                RecordVersion: RequiredInt(root, "record_version"),
                BasisRepo: RequiredString(root, "basis_repo"),
                BasisCommit: RequiredString(root, "basis_commit"));
        }
        catch (Exception exception) when (exception is JsonException or InvalidDataException)
        {
            throw new InvalidDataException(
                $"cannot generate a complete evidence register from {Path.GetFileName(path)}: "
                + exception.Message,
                exception);
        }
    }

    private static JsonElement RequiredProperty(JsonElement value, string name)
    {
        if (value.ValueKind != JsonValueKind.Object || !value.TryGetProperty(name, out var property))
            throw new InvalidDataException($"missing required property {name}");
        return property;
    }

    private static string RequiredString(JsonElement value, string name)
    {
        var property = RequiredProperty(value, name);
        if (property.ValueKind != JsonValueKind.String)
            throw new InvalidDataException($"{name} is not a string");
        return property.GetString()!;
    }

    private static string? OptionalString(JsonElement value, string name)
    {
        var property = RequiredProperty(value, name);
        return property.ValueKind switch
        {
            JsonValueKind.Null => null,
            JsonValueKind.String => property.GetString(),
            _ => throw new InvalidDataException($"{name} is neither a string nor null"),
        };
    }

    private static bool RequiredBool(JsonElement value, string name)
    {
        var property = RequiredProperty(value, name);
        if (property.ValueKind is not (JsonValueKind.True or JsonValueKind.False))
            throw new InvalidDataException($"{name} is not a Boolean");
        return property.GetBoolean();
    }

    private static bool IsRepairTarget(JsonElement count)
    {
        var property = RequiredProperty(count, "corpus_was_repair_target");
        return property.ValueKind switch
        {
            JsonValueKind.True => true,
            JsonValueKind.False => false,
            JsonValueKind.String when property.GetString() == "source-does-not-state" => false,
            _ => throw new InvalidDataException(
                "corpus_was_repair_target is neither a Boolean nor source-does-not-state"),
        };
    }

    private static int RequiredInt(JsonElement value, string name)
    {
        var property = RequiredProperty(value, name);
        if (property.ValueKind != JsonValueKind.Number || !property.TryGetInt32(out var result))
            throw new InvalidDataException($"{name} is not an integer");
        return result;
    }

    private static JsonElement RequiredArray(JsonElement value, string name)
    {
        var property = RequiredProperty(value, name);
        if (property.ValueKind != JsonValueKind.Array)
            throw new InvalidDataException($"{name} is not an array");
        return property;
    }

    private static string[] StringValues(JsonElement array, string name)
    {
        var result = new List<string>();
        foreach (var item in array.EnumerateArray())
        {
            if (item.ValueKind != JsonValueKind.String)
                throw new InvalidDataException($"{name} contains a non-string");
            result.Add(item.GetString()!);
        }
        return result.ToArray();
    }

    private static string SingleValue(IEnumerable<string> values, string name)
    {
        var distinct = values.Distinct(StringComparer.Ordinal).ToArray();
        if (distinct.Length != 1)
            throw new InvalidDataException(
                $"evidence records do not have one shared {name}: {string.Join(", ", distinct)}");
        return distinct[0];
    }

    private static JsonArray StringArray(IEnumerable<string> values)
    {
        var result = new JsonArray();
        foreach (var value in values)
            result.Add(value);
        return result;
    }

    private sealed record SourceRecord(
        string RecordId,
        string Prefix,
        string Path,
        string Class,
        string Title,
        IReadOnlyList<string> Subjects,
        string? UpstreamRowId,
        string UpstreamRegister,
        string UpstreamAnchorFile,
        int CountsDeclared,
        int CountsWithPublishedNumerator,
        int CountsWithPublishedDenominator,
        IReadOnlyList<string> Axes,
        IReadOnlyList<string> Builds,
        bool HasGroupScopedCount,
        bool HasRepairTargetCount,
        bool HasRetraction,
        string EvidenceLocality,
        int RecordVersion,
        string BasisRepo,
        string BasisCommit);
}
