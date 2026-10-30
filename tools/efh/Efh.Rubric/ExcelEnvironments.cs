using System.Text;
using System.Text.Json;

namespace Efh.Rubric;

/// <summary>
/// F19: per-function records of the Excel environments (build, platform, bitness, CPU) behind the
/// Handbook's readings, the cross-environment comparisons and the observed differences
/// (content/evidence/environments/*.json). Renders the index page
/// content/evidence/EXCEL-ENVIRONMENTS.md and a generated section on each affected function page.
/// Deterministic: the same records always render the same bytes.
/// </summary>
public static class ExcelEnvironments
{
    public const string SectionTitle = "Excel versions, platforms and CPUs";
    public const string IndexRelativePath = "content/evidence/EXCEL-ENVIRONMENTS.md";
    public const string RecordsRelativeDirectory = "content/evidence/environments";
    private const string BeginMarker = "<!-- efh:environments begin";
    private const string EndMarker = "<!-- efh:environments end -->";

    public sealed record Evidence(
        string Repository, string Commit, string Path, string? Locator, string? Sha256, string? Note);

    public sealed record Environment(
        string EnvId, string ExcelVersion, string ExcelBuild, string Platform, string Os,
        string ExcelBitness, string Cpu, string CpuVendor, string CpuMicroarchitecture,
        string WorkbookCompatibility, string UpdateChannel, string Basis);

    public sealed record Reading(
        string EnvId, long? InputsRead, string? FirstObserved, string? LastObserved,
        IReadOnlyList<Evidence> Evidence);

    public sealed record Comparison(
        string ComparisonId, IReadOnlyList<string> Axis, IReadOnlyList<string> EnvIds,
        long InputsCompared, long InputsDiffering, string Method, IReadOnlyList<Evidence> Evidence);

    public sealed record Value(string Kind, string Display, string? Bits);

    public sealed record Observation(string EnvId, Value Result, string? ObservedOn, Evidence Evidence);

    public sealed record Difference(
        string DifferenceId, IReadOnlyList<string> Axis, string? Formula, IReadOnlyList<Value> Inputs,
        IReadOnlyList<Observation> Observations, string RecheckStatus, string? RecheckedOn,
        string? RecheckNote, string Note);

    public sealed record Sensitivity(
        string SensitivityId, IReadOnlyList<string> Axis, string Status, string Statement,
        IReadOnlyList<Evidence> Evidence);

    public sealed record Record(
        string Path, string FunctionId, string SurfaceName, string StatusPlain,
        IReadOnlyList<Environment> Environments, IReadOnlyList<Reading> Readings,
        IReadOnlyList<Comparison> Comparisons, IReadOnlyList<Difference> Differences,
        IReadOnlyList<Sensitivity> Sensitivities)
    {
        public Environment Env(string id) => Environments.First(e => e.EnvId == id);

        /// <summary>A function page carries the generated section when there is something beyond bare
        /// readings to say: a comparison, a difference, or a stated sensitivity.</summary>
        public bool HasPageSection => Comparisons.Count > 0 || Differences.Count > 0 || Sensitivities.Count > 0;

        public string Standing =>
            Differences.Count > 0 ? "differences observed"
            : Comparisons.Count > 0 ? "compared, no difference found"
            : Readings.Count == 0 ? "no reading with a recorded build"
            : Readings.Select(r => r.EnvId).Distinct().Count() > 1
                ? "read in several environments, not compared on the same inputs"
                : "read in one environment only";
    }

    // ------------------------------------------------------------------ loading

    public static IReadOnlyList<Record> Load(string root)
    {
        var dir = System.IO.Path.Combine(root, "content", "evidence", "environments");
        if (!Directory.Exists(dir))
            return [];
        return Directory.EnumerateFiles(dir, "*.json")
            .OrderBy(p => System.IO.Path.GetFileName(p), StringComparer.Ordinal)
            .Select(p => Parse(root, p))
            .ToArray();
    }

    private static Record Parse(string root, string path)
    {
        using var doc = JsonDocument.Parse(File.ReadAllText(path));
        var r = doc.RootElement;
        var rel = System.IO.Path.GetRelativePath(root, path).Replace('\\', '/');
        return new Record(
            rel,
            Str(r, "function_id"),
            Str(r, "surface_name"),
            Str(r, "status_plain"),
            r.GetProperty("environments").EnumerateArray().Select(e => new Environment(
                Str(e, "env_id"), Str(e, "excel_version"), Str(e, "excel_build"), Str(e, "platform"),
                Str(e, "os"), Str(e, "excel_bitness"), Str(e, "cpu"), Str(e, "cpu_vendor"),
                Str(e, "cpu_microarchitecture"), Str(e, "workbook_compatibility"),
                Str(e, "update_channel"), Str(e, "basis"))).ToArray(),
            r.GetProperty("readings").EnumerateArray().Select(e => new Reading(
                Str(e, "env_id"), Long(e, "inputs_read"), NStr(e, "first_observed"),
                NStr(e, "last_observed"), EvidenceList(e.GetProperty("evidence")))).ToArray(),
            r.GetProperty("comparisons").EnumerateArray().Select(e => new Comparison(
                Str(e, "comparison_id"), StrList(e, "axis"), StrList(e, "env_ids"),
                e.GetProperty("inputs_compared").GetInt64(), e.GetProperty("inputs_differing").GetInt64(),
                Str(e, "method"), EvidenceList(e.GetProperty("evidence")))).ToArray(),
            r.GetProperty("differences").EnumerateArray().Select(e =>
            {
                var recheck = e.GetProperty("recheck");
                return new Difference(
                    Str(e, "difference_id"), StrList(e, "axis"), NStr(e, "formula"),
                    e.GetProperty("inputs").EnumerateArray().Select(ParseValue).ToArray(),
                    e.GetProperty("observations").EnumerateArray().Select(o => new Observation(
                        Str(o, "env_id"), ParseValue(o.GetProperty("result")), NStr(o, "observed_on"),
                        ParseEvidence(o.GetProperty("evidence")))).ToArray(),
                    Str(recheck, "status"), NStr(recheck, "rechecked_on"), NStr(recheck, "note"),
                    Str(e, "note"));
            }).ToArray(),
            r.GetProperty("sensitivities").EnumerateArray().Select(e => new Sensitivity(
                Str(e, "sensitivity_id"), StrList(e, "axis"), Str(e, "status"), Str(e, "statement"),
                EvidenceList(e.GetProperty("evidence")))).ToArray());
    }

    private static Value ParseValue(JsonElement e) => new(Str(e, "kind"), Str(e, "display"), NStr(e, "bits"));

    private static Evidence ParseEvidence(JsonElement e) => new(
        Str(e, "repository"), Str(e, "commit"), Str(e, "path"), NStr(e, "locator"), NStr(e, "sha256"),
        NStr(e, "note"));

    private static IReadOnlyList<Evidence> EvidenceList(JsonElement a) =>
        a.EnumerateArray().Select(ParseEvidence).ToArray();

    private static string Str(JsonElement e, string name) =>
        e.GetProperty(name).GetString() ?? throw new InvalidDataException($"{name} is null");

    private static string? NStr(JsonElement e, string name) =>
        e.TryGetProperty(name, out var v) && v.ValueKind == JsonValueKind.String ? v.GetString() : null;

    private static long? Long(JsonElement e, string name) =>
        e.TryGetProperty(name, out var v) && v.ValueKind == JsonValueKind.Number ? v.GetInt64() : null;

    private static IReadOnlyList<string> StrList(JsonElement e, string name) =>
        e.GetProperty(name).EnumerateArray().Select(x => x.GetString()!).ToArray();

    // ------------------------------------------------------------------ cross-reference checks

    /// <summary>Checks the schema cannot express. Returns one message per problem.</summary>
    public static IReadOnlyList<string> Check(string root, IReadOnlyList<Record> records)
    {
        var errors = new List<string>();
        foreach (var r in records)
        {
            var p = r.Path;
            var fileId = System.IO.Path.GetFileNameWithoutExtension(p);
            if (fileId != r.FunctionId)
                errors.Add($"{p}: function_id {r.FunctionId} does not match the file name");
            if (!File.Exists(System.IO.Path.Combine(root, "data", "functions", r.FunctionId + ".json")))
                errors.Add($"{p}: {r.FunctionId} is not in data/functions");
            var envIds = r.Environments.Select(e => e.EnvId).ToArray();
            if (envIds.Distinct(StringComparer.Ordinal).Count() != envIds.Length)
                errors.Add($"{p}: duplicate env_id");
            if (!envIds.SequenceEqual(envIds.OrderBy(x => x, StringComparer.Ordinal)))
                errors.Add($"{p}: environments[] is not sorted by env_id");
            var known = envIds.ToHashSet(StringComparer.Ordinal);
            var used = new HashSet<string>(StringComparer.Ordinal);
            void Ref(string id, string where)
            {
                used.Add(id);
                if (!known.Contains(id))
                    errors.Add($"{p}: {where} names unknown env_id {id}");
            }
            foreach (var rd in r.Readings) Ref(rd.EnvId, "readings[]");
            if (r.Readings.Select(x => x.EnvId).Distinct().Count() != r.Readings.Count)
                errors.Add($"{p}: more than one reading for one environment");
            var stem = r.FunctionId[(r.FunctionId.IndexOf('.') + 1)..];
            foreach (var c in r.Comparisons)
            {
                foreach (var id in c.EnvIds) Ref(id, c.ComparisonId);
                if (!c.ComparisonId.StartsWith("XC-" + stem + "-", StringComparison.Ordinal))
                    errors.Add($"{p}: {c.ComparisonId} does not carry the function stem {stem}");
                if (c.InputsDiffering > c.InputsCompared)
                    errors.Add($"{p}: {c.ComparisonId} has more differing than compared inputs");
                var listed = r.Differences.Count(d =>
                    d.Observations.Select(o => o.EnvId).Intersect(c.EnvIds).Count() >= 2);
                if (c.InputsDiffering > 0 && listed == 0)
                    errors.Add($"{p}: {c.ComparisonId} reports differing inputs but lists no difference");
            }
            foreach (var d in r.Differences)
            {
                foreach (var o in d.Observations) Ref(o.EnvId, d.DifferenceId);
                if (!d.DifferenceId.StartsWith("XD-" + stem + "-", StringComparison.Ordinal))
                    errors.Add($"{p}: {d.DifferenceId} does not carry the function stem {stem}");
                if (d.Observations.Select(o => o.EnvId).Distinct().Count() < 2)
                    errors.Add($"{p}: {d.DifferenceId} is observed in fewer than two environments");
                if (d.Observations.Select(o => (o.Result.Kind, o.Result.Display, o.Result.Bits)).Distinct().Count() < 2)
                    errors.Add($"{p}: {d.DifferenceId} lists no two distinct results");
                foreach (var v in d.Inputs.Concat(d.Observations.Select(o => o.Result)))
                    if ((v.Kind == "number") != (v.Bits is not null))
                        errors.Add($"{p}: {d.DifferenceId}: a number carries bits and nothing else does");
            }
            foreach (var s in r.Sensitivities)
                if (!s.SensitivityId.StartsWith("XS-" + stem + "-", StringComparison.Ordinal))
                    errors.Add($"{p}: {s.SensitivityId} does not carry the function stem {stem}");
            foreach (var id in envIds.Where(id => !used.Contains(id)))
                errors.Add($"{p}: environment {id} is named by nothing in the file");
            CheckSortedIds(p, r.Comparisons.Select(c => c.ComparisonId), "comparisons[]", errors);
            CheckSortedIds(p, r.Differences.Select(c => c.DifferenceId), "differences[]", errors);
            CheckSortedIds(p, r.Sensitivities.Select(c => c.SensitivityId), "sensitivities[]", errors);
        }
        return errors;
    }

    private static void CheckSortedIds(string p, IEnumerable<string> ids, string what, List<string> errors)
    {
        var a = ids.ToArray();
        if (a.Distinct(StringComparer.Ordinal).Count() != a.Length)
            errors.Add($"{p}: duplicate id in {what}");
        if (!a.SequenceEqual(a.OrderBy(x => x, StringComparer.Ordinal)))
            errors.Add($"{p}: {what} is not sorted by id");
    }

    // ------------------------------------------------------------------ rendering helpers

    private static string EnvShort(Environment e)
    {
        var platform = e.Platform switch
        {
            "windows" => "Windows",
            "mac" => "Mac",
            "web" => "web",
            "not-recorded" => "platform not recorded",
            var x => x,
        };
        var bitness = e.ExcelBitness == "not-recorded" ? "bitness not recorded" : e.ExcelBitness;
        return $"Excel {Build(e)}, {platform} {bitness}, {CpuShort(e)}";
    }

    private static string Build(Environment e) =>
        e.ExcelBuild == "not-recorded" ? "build not recorded" : $"{e.ExcelVersion} build {e.ExcelBuild}";

    private static string CpuShort(Environment e)
    {
        if (e.Cpu == "not-recorded")
            return "CPU not recorded";
        return e.CpuMicroarchitecture == "not-recorded" ? e.Cpu : $"{e.Cpu} ({e.CpuMicroarchitecture})";
    }

    /// <summary>Hard-wraps a prose paragraph at 95 columns (F6 body style).</summary>
    public static string Wrap(string text, int width = 95)
    {
        var sb = new StringBuilder();
        var line = 0;
        foreach (var word in text.Split(' ', StringSplitOptions.RemoveEmptyEntries))
        {
            if (line > 0 && line + 1 + word.Length > width)
            {
                sb.Append('\n');
                line = 0;
            }
            else if (line > 0)
            {
                sb.Append(' ');
                line++;
            }
            sb.Append(word);
            line += word.Length;
        }
        return sb.ToString();
    }

    private static string Cell(string s) => s.Replace("|", "\\|").Replace("\r", " ").Replace("\n", " ");

    private static string Val(Value v) => v.Bits is null ? $"`{v.Display}`" : $"`{v.Display}` (`{v.Bits}`)";

    private static string Count(long? n) => n is null ? "not counted" : n.Value.ToString("N0", System.Globalization.CultureInfo.InvariantCulture);

    private static string Ev(Evidence e, string linkPrefix)
    {
        var shortCommit = e.Commit[..10];
        var loc = e.Locator is null ? "" : $" ({e.Locator})";
        return e.Repository == "ExcelFunctionsHandbook"
            ? $"[`{e.Path}`]({linkPrefix}{e.Path}){loc}"
            : $"{e.Repository} `{e.Path}`{loc} at `{shortCommit}`";
    }

    private static string Observed(string? first, string? last) =>
        first is null && last is null ? "date not recorded"
        : first == last || last is null ? first!
        : first is null ? last
        : $"{first} to {last}";

    // ------------------------------------------------------------------ the function-page section

    /// <summary>The generated section, from its H2 through the end marker, with a trailing newline.</summary>
    public static string RenderSection(Record r)
    {
        var sb = new StringBuilder();
        sb.Append("## ").Append(SectionTitle).Append('\n');
        sb.Append('\n');
        sb.Append(BeginMarker).Append(": generated by `efh environments render` from ")
          .Append(r.Path).Append("; edits here are overwritten -->\n");
        sb.Append('\n');
        sb.Append(Wrap(r.StatusPlain)).Append('\n');
        sb.Append('\n');
        if (r.Readings.Count == 0)
        {
            sb.Append("No reading of `").Append(r.SurfaceName).Append("` with a recorded Excel build is held in this record.\n");
        }
        else
        {
            sb.Append("Readings of `").Append(r.SurfaceName).Append("` recorded here come from:\n");
            sb.Append('\n');
            sb.Append("| Excel | Platform | CPU | Inputs read | Observed |\n");
            sb.Append("|---|---|---|---:|---|\n");
            foreach (var rd in r.Readings)
            {
                var e = r.Env(rd.EnvId);
                sb.Append($"| {Cell(Build(e))} | {Cell(PlatformText(e))} | {Cell(CpuShort(e))} | {Count(rd.InputsRead)} | {Observed(rd.FirstObserved, rd.LastObserved)} |\n");
            }
        }
        sb.Append('\n');
        if (r.Comparisons.Count > 0)
        {
            sb.Append("Compared on the same inputs:\n");
            sb.Append('\n');
            sb.Append("| Check | Differs in | Environments | Inputs compared | Inputs with different results |\n");
            sb.Append("|---|---|---|---:|---:|\n");
            foreach (var c in r.Comparisons)
            {
                var envs = string.Join(" vs ", c.EnvIds.Select(id => Build(r.Env(id))));
                sb.Append($"| {c.ComparisonId} | {string.Join(", ", c.Axis)} | {Cell(envs)} | {Count(c.InputsCompared)} | {Count(c.InputsDiffering)} |\n");
            }
            sb.Append('\n');
        }
        else
        {
            sb.Append("No check has read the same inputs in two environments, so the absence of a listed ")
              .Append("difference says nothing about other builds, platforms or CPUs.\n");
            sb.Append('\n');
        }
        if (r.Differences.Count > 0)
        {
            sb.Append("Differences observed:\n");
            sb.Append('\n');
            sb.Append("| Difference | Inputs | Excel | Result | Rechecked |\n");
            sb.Append("|---|---|---|---|---|\n");
            foreach (var d in r.Differences)
            {
                var inputs = string.Join(", ", d.Inputs.Select(Val));
                foreach (var o in d.Observations)
                {
                    var e = r.Env(o.EnvId);
                    sb.Append($"| {d.DifferenceId} | {Cell(inputs)} | {Cell(EnvShort(e))} | {Cell(Val(o.Result))} | {d.RecheckStatus} |\n");
                }
            }
            sb.Append('\n');
            foreach (var d in r.Differences)
                sb.Append($"- {d.DifferenceId}: {d.Note}\n");
            sb.Append('\n');
        }
        else
        {
            sb.Append("Differences observed: none.\n");
            sb.Append('\n');
        }
        foreach (var s in r.Sensitivities)
        {
            var label = s.Status switch
            {
                "expected-untested" => "Expected, not yet tested",
                "tested-no-difference-found" => "Tested, no difference found",
                _ => "Observed",
            };
            sb.Append(Wrap($"{label} ({s.SensitivityId}, {string.Join(", ", s.Axis)}): {s.Statement}")).Append('\n');
            sb.Append('\n');
        }
        sb.Append("Evidence for every row: [`").Append(r.Path).Append("`](../../")
          .Append(r.Path).Append("). All known differences: [Excel environment differences](../evidence/")
          .Append(System.IO.Path.GetFileName(IndexRelativePath)).Append(").\n");
        sb.Append('\n');
        sb.Append(EndMarker).Append('\n');
        return sb.ToString();
    }

    private static string PlatformText(Environment e)
    {
        var platform = e.Platform switch
        {
            "windows" => "Windows",
            "mac" => "Mac",
            "web" => "web",
            "not-recorded" => "not recorded",
            var x => x,
        };
        return e.ExcelBitness == "not-recorded" ? $"{platform}, bitness not recorded" : $"{platform}, {e.ExcelBitness}";
    }

    /// <summary>Returns the page text with the generated section inserted, replaced or removed, and
    /// body_sections kept in step. Pure function of the inputs.</summary>
    public static string ApplyToPage(string pageText, Record? record)
    {
        var lines = pageText.Split('\n').ToList();
        // remove any existing generated section (its H2 through the end marker and one blank line)
        var start = lines.FindIndex(l => l == "## " + SectionTitle);
        if (start >= 0)
        {
            var end = lines.FindIndex(start, l => l == EndMarker);
            if (end < 0)
                throw new InvalidDataException($"'## {SectionTitle}' without '{EndMarker}'");
            var count = end - start + 1;
            if (end + 1 < lines.Count && lines[end + 1].Length == 0)
                count++;
            lines.RemoveRange(start, count);
        }
        // body_sections
        var fmEnd = lines.FindIndex(1, l => l == "---");
        var bs = lines.FindIndex(l => l == "body_sections:");
        if (bs < 0 || bs > fmEnd)
            throw new InvalidDataException("no body_sections in the front matter");
        var existing = lines.FindIndex(bs, l => l == "  - " + SectionTitle || l == "  - \"" + SectionTitle + "\"");
        if (existing >= 0 && existing < fmEnd)
        {
            lines.RemoveAt(existing);
            fmEnd--;
        }
        if (record is not null && record.HasPageSection)
        {
            var pv = lines.FindIndex(bs, l => l == "  - Page vocabulary" || l == "  - \"Page vocabulary\"");
            if (pv < 0 || pv > fmEnd)
                throw new InvalidDataException("body_sections lacks Page vocabulary");
            var quoted = lines[pv].Contains('"');
            lines.Insert(pv, quoted ? "  - \"" + SectionTitle + "\"" : "  - " + SectionTitle);
            var h2 = lines.FindIndex(l => l == "## Page vocabulary");
            if (h2 < 0)
                throw new InvalidDataException("no '## Page vocabulary' heading");
            var block = RenderSection(record).TrimEnd('\n').Split('\n');
            lines.InsertRange(h2, block.Append(""));
        }
        return string.Join('\n', lines);
    }

    // ------------------------------------------------------------------ the index page

    public static string RenderIndex(string root, IReadOnlyList<Record> records)
    {
        var sb = new StringBuilder();
        var pages = new HashSet<string>(
            Directory.Exists(System.IO.Path.Combine(root, "content", "functions"))
                ? Directory.EnumerateFiles(System.IO.Path.Combine(root, "content", "functions"), "*.md")
                    .Select(System.IO.Path.GetFileNameWithoutExtension)!
                : [],
            StringComparer.Ordinal);
        string FnLink(Record r) => pages.Contains(r.FunctionId)
            ? $"[`{r.SurfaceName}`](../functions/{r.FunctionId}.md)"
            : $"`{r.SurfaceName}` (no page yet)";
        string RecLink(Record r) => $"[record](environments/{System.IO.Path.GetFileName(r.Path)})";

        var diffs = records.SelectMany(r => r.Differences.Select(d => (r, d))).ToArray();
        var comps = records.SelectMany(r => r.Comparisons.Select(c => (r, c))).ToArray();
        var sens = records.SelectMany(r => r.Sensitivities.Select(s => (r, s))).ToArray();
        var envs = records.SelectMany(r => r.Environments)
            .GroupBy(e => e.EnvId, StringComparer.Ordinal)
            .OrderBy(g => g.Key, StringComparer.Ordinal)
            .Select(g => g.First())
            .ToArray();

        sb.Append("# Excel environment differences\n\n");
        sb.Append("<!-- generated by `efh environments render` from content/evidence/environments/*.json; edits here are overwritten -->\n\n");
        sb.Append("Excel's worksheet functions are assumed to give the same result for the same inputs in every ")
          .Append("Excel build, on every platform (Windows, Mac, web; 32- or 64-bit) and on every CPU. Every reading ")
          .Append("the Handbook records names the Excel build, platform and CPU it was taken in, where these were ")
          .Append("recorded. When two environments give different results for the same inputs, the difference is ")
          .Append("rechecked and recorded here, function by function, with the exact inputs (binary64 bits as well as ")
          .Append("a decimal display), each result, and where each observation is kept.\n\n");
        sb.Append("This page is generated from the per-function records in ")
          .Append("[`content/evidence/environments/`](environments/); the format is described in ")
          .Append("[`data/README.md`](../../data/README.md#excel-environment-differences) and ")
          .Append("[`tools/schemas/f19-excel-environments.schema.json`](../../tools/schemas/f19-excel-environments.schema.json).\n\n");

        var writeUps = records
            .SelectMany(r => r.Readings.SelectMany(x => x.Evidence)
                .Concat(r.Comparisons.SelectMany(x => x.Evidence))
                .Concat(r.Sensitivities.SelectMany(x => x.Evidence)))
            .Where(e => e.Repository == "ExcelFunctionsHandbook"
                        && e.Path.EndsWith(".pdf", StringComparison.OrdinalIgnoreCase))
            .GroupBy(e => e.Path, StringComparer.Ordinal)
            .OrderBy(g => g.Key, StringComparer.Ordinal)
            .Select(g => $"[{g.First().Note ?? System.IO.Path.GetFileName(g.Key)}](../../{g.Key})")
            .ToArray();
        if (writeUps.Length > 0)
            sb.Append("Write-ups cited by the records: ").Append(string.Join(", ", writeUps)).Append(".\n\n");

        sb.Append("## Summary\n\n");
        sb.Append($"- Functions with recorded readings: {records.Count} (of the 541 entries = 518 functions + 23 operators).\n");
        sb.Append($"- Excel environments named: {envs.Length}.\n");
        sb.Append($"- Same-input comparisons between environments: {comps.Length}, over {records.Count(r => r.Comparisons.Count > 0)} functions.\n");
        sb.Append($"- Differences observed: {diffs.Length}, in {records.Count(r => r.Differences.Count > 0)} functions.\n");
        sb.Append($"- Expected but untested sensitivities: {sens.Count(x => x.s.Status == "expected-untested")}, over {sens.Where(x => x.s.Status == "expected-untested").Select(x => x.r.FunctionId).Distinct().Count()} functions.\n");
        sb.Append($"- Functions read in one environment only: {records.Count(r => r.Standing == "read in one environment only")}.\n\n");
        sb.Append("A function without a record here has no reading recorded in this family. A function read in one ")
          .Append("environment only has no known difference because nothing has been compared, not because its results ")
          .Append("are known to agree elsewhere.\n\n");

        sb.Append("## Known differences\n\n");
        if (diffs.Length == 0)
        {
            sb.Append("None. No reading in the seeded evidence shows Excel giving different results for the same ")
              .Append("inputs in two environments. The comparisons below say how far that has been tested.\n\n");
        }
        else
        {
            sb.Append("| Function | Difference | Inputs | Excel | Result | Rechecked |\n");
            sb.Append("|---|---|---|---|---|---|\n");
            foreach (var (r, d) in diffs)
            {
                var inputs = string.Join(", ", d.Inputs.Select(Val));
                foreach (var o in d.Observations)
                    sb.Append($"| {FnLink(r)} | {d.DifferenceId} | {Cell(inputs)} | {Cell(EnvShort(r.Env(o.EnvId)))} | {Cell(Val(o.Result))} | {d.RecheckStatus} |\n");
            }
            sb.Append('\n');
            foreach (var (r, d) in diffs)
                sb.Append($"- {d.DifferenceId} ({r.SurfaceName}): {d.Note}\n");
            sb.Append('\n');
        }

        sb.Append("## Comparisons on the same inputs\n\n");
        if (comps.Length == 0)
            sb.Append("None recorded.\n\n");
        else
        {
            sb.Append("| Function | Check | Differs in | Environments | Inputs compared | Inputs with different results | Evidence |\n");
            sb.Append("|---|---|---|---|---:|---:|---|\n");
            foreach (var (r, c) in comps)
            {
                var envText = string.Join(" vs ", c.EnvIds.Select(id => EnvShort(r.Env(id))));
                sb.Append($"| {FnLink(r)} | {c.ComparisonId} | {string.Join(", ", c.Axis)} | {Cell(envText)} | {Count(c.InputsCompared)} | {Count(c.InputsDiffering)} | {Cell(string.Join("; ", c.Evidence.Select(e => Ev(e, "../../"))))} |\n");
            }
            sb.Append('\n');
        }

        sb.Append("## Expected but untested\n\n");
        var untested = sens.Where(x => x.s.Status != "observed").ToArray();
        if (untested.Length == 0)
            sb.Append("None recorded.\n\n");
        else
        {
            sb.Append("| Function | Sensitivity | Differs in | Status | Statement |\n");
            sb.Append("|---|---|---|---|---|\n");
            foreach (var (r, s) in untested)
                sb.Append($"| {FnLink(r)} | {s.SensitivityId} | {string.Join(", ", s.Axis)} | {s.Status} | {Cell(s.Statement)} |\n");
            sb.Append('\n');
        }

        sb.Append("## Where each function's readings come from\n\n");
        sb.Append("| Function | Environments read (inputs) | Standing | Record |\n");
        sb.Append("|---|---|---|---|\n");
        foreach (var r in records)
        {
            var read = string.Join("; ", r.Readings.Select(rd =>
            {
                var e = r.Env(rd.EnvId);
                return $"{Build(e)}, {CpuVendorShort(e)} ({Count(rd.InputsRead)})";
            }));
            sb.Append($"| {FnLink(r)} | {Cell(read)} | {r.Standing} | {RecLink(r)} |\n");
        }
        sb.Append('\n');

        sb.Append("## Environments\n\n");
        sb.Append("| Environment id | Excel | Platform | OS | CPU | Workbook compatibility | Update channel |\n");
        sb.Append("|---|---|---|---|---|---|---|\n");
        foreach (var e in envs)
            sb.Append($"| `{e.EnvId}` | {Cell(Build(e))} | {Cell(PlatformText(e))} | {Cell(e.Os)} | {Cell(CpuShort(e))} | {Cell(e.WorkbookCompatibility)} | {Cell(e.UpdateChannel)} |\n");
        sb.Append('\n');
        // hard-wrap prose paragraphs; tables, headings, list items and comments stay on one line
        return string.Join('\n', sb.ToString().Split('\n').Select(line =>
            line.Length > 95 && !line.StartsWith('|') && !line.StartsWith('#')
                && !line.StartsWith("- ") && !line.StartsWith("<!--")
                ? Wrap(line)
                : line));
    }

    private static string CpuVendorShort(Environment e) =>
        e.Cpu == "not-recorded" ? "CPU not recorded"
        : e.CpuMicroarchitecture == "not-recorded" ? e.CpuVendor
        : $"{e.CpuVendor} {e.CpuMicroarchitecture}";

    // ------------------------------------------------------------------ write / check

    /// <summary>Every file this generator owns, with the bytes it should hold.</summary>
    public static IReadOnlyDictionary<string, string> Plan(string root)
    {
        var records = Load(root);
        var errors = Check(root, records);
        if (errors.Count > 0)
            throw new InvalidDataException("F19 records fail their cross-reference checks:\n" + string.Join("\n", errors));
        var byId = records.ToDictionary(r => r.FunctionId, StringComparer.Ordinal);
        var plan = new SortedDictionary<string, string>(StringComparer.Ordinal)
        {
            [IndexRelativePath] = RenderIndex(root, records),
        };
        var pageDir = System.IO.Path.Combine(root, "content", "functions");
        foreach (var page in Directory.EnumerateFiles(pageDir, "*.md").OrderBy(p => p, StringComparer.Ordinal))
        {
            var id = System.IO.Path.GetFileNameWithoutExtension(page);
            var text = File.ReadAllText(page);
            byId.TryGetValue(id, out var rec);
            if (rec is null && !text.Contains("## " + SectionTitle + "\n", StringComparison.Ordinal))
                continue;
            plan["content/functions/" + id + ".md"] = ApplyToPage(text, rec);
        }
        return plan;
    }

    /// <summary>Writes the plan; returns the paths whose bytes changed.</summary>
    public static IReadOnlyList<string> Render(string root)
    {
        var changed = new List<string>();
        foreach (var (rel, text) in Plan(root))
        {
            var path = System.IO.Path.Combine(root, rel.Replace('/', System.IO.Path.DirectorySeparatorChar));
            var bytes = new UTF8Encoding(false).GetBytes(text);
            if (File.Exists(path) && File.ReadAllBytes(path).AsSpan().SequenceEqual(bytes))
                continue;
            File.WriteAllBytes(path, bytes);
            changed.Add(rel);
        }
        return changed;
    }

    /// <summary>The paths whose bytes on disk differ from what render would write.</summary>
    public static IReadOnlyList<string> Stale(string root)
    {
        var stale = new List<string>();
        foreach (var (rel, text) in Plan(root))
        {
            var path = System.IO.Path.Combine(root, rel.Replace('/', System.IO.Path.DirectorySeparatorChar));
            var bytes = new UTF8Encoding(false).GetBytes(text);
            if (!File.Exists(path) || !File.ReadAllBytes(path).AsSpan().SequenceEqual(bytes))
                stale.Add(rel);
        }
        return stale;
    }
}
