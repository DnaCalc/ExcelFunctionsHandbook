using System.Text.Json;

namespace Efh.Rubric;

/// <summary>
/// Reads the three mechanical organs and the curated evidence records.
///
/// G-9 is enforced here structurally: the forbidden display property is never requested from any
/// JsonElement. There is no field on <see cref="CountRow"/> to hold it. Its spelling is isolated
/// in the guard type.
/// </summary>
public sealed class Loader
{
    private readonly string _root;
    public HygieneReport Hygiene { get; } = new();

    public Loader(string handbookRoot) => _root = handbookRoot.Replace('\\', '/').TrimEnd('/');

    public string Root => _root;

    // ------------------------------------------------------------------ helpers

    private static JsonDocument Parse(string path) =>
        JsonDocument.Parse(File.ReadAllText(path));

    private static string? Str(JsonElement o, string name) =>
        o.TryGetProperty(name, out var v) && v.ValueKind == JsonValueKind.String ? v.GetString() : null;

    private static int? Int(JsonElement o, string name)
    {
        if (!o.TryGetProperty(name, out var v)) return null;
        return v.ValueKind == JsonValueKind.Number && v.TryGetInt32(out var i) ? i : null;
    }

    private static bool Bool(JsonElement o, string name, bool fallback = false)
    {
        if (!o.TryGetProperty(name, out var v)) return fallback;
        return v.ValueKind switch
        {
            JsonValueKind.True => true,
            JsonValueKind.False => false,
            JsonValueKind.String => string.Equals(v.GetString(), "true", StringComparison.OrdinalIgnoreCase),
            _ => fallback
        };
    }

    private static List<string> StrList(JsonElement o, string name)
    {
        var result = new List<string>();
        if (o.TryGetProperty(name, out var v) && v.ValueKind == JsonValueKind.Array)
            foreach (var e in v.EnumerateArray())
                if (e.ValueKind == JsonValueKind.String)
                {
                    var s = e.GetString();
                    if (s is not null) result.Add(s);
                }
        return result;
    }

    // ------------------------------------------------------------------ functions

    public IReadOnlyList<FunctionRow> LoadFunctions()
    {
        var dir = Path.Combine(_root, "data", "functions");
        var rows = new List<FunctionRow>();
        foreach (var path in Directory.EnumerateFiles(dir, "*.json").OrderBy(p => p, StringComparer.Ordinal))
        {
            using var doc = Parse(path);
            var o = doc.RootElement;
            rows.Add(new FunctionRow
            {
                FunctionId = Str(o, "function_id") ?? Path.GetFileNameWithoutExtension(path),
                SurfaceName = Str(o, "surface_name") ?? "",
                EntryKind = Str(o, "entry_kind") ?? "",
                Category = Str(o, "category") ?? "",
            });
        }
        return rows.OrderBy(r => r.FunctionId, StringComparer.Ordinal).ToList();
    }

    // ------------------------------------------------------------------ presence

    public IReadOnlyDictionary<string, PresenceRow> LoadPresence()
    {
        var dir = Path.Combine(_root, "data", "presence");
        var map = new Dictionary<string, PresenceRow>(StringComparer.Ordinal);
        foreach (var path in Directory.EnumerateFiles(dir, "*.json").OrderBy(p => p, StringComparer.Ordinal))
        {
            var name = Path.GetFileNameWithoutExtension(path);
            if (name == "index") continue;
            using var doc = Parse(path);
            var o = doc.RootElement;

            var mtms = new Dictionary<string, int>(StringComparer.Ordinal);
            if (o.TryGetProperty("module_tests_minus_sibling_count", out var m) && m.ValueKind == JsonValueKind.Object)
                foreach (var p in m.EnumerateObject())
                    if (p.Value.ValueKind == JsonValueKind.Number) mtms[p.Name] = p.Value.GetInt32();

            var tpm = new Dictionary<string, int>(StringComparer.Ordinal);
            if (o.TryGetProperty("tests_per_module", out var t) && t.ValueKind == JsonValueKind.Object)
                foreach (var p in t.EnumerateObject())
                    if (p.Value.ValueKind == JsonValueKind.Number) tpm[p.Name] = p.Value.GetInt32();

            var streams = new List<string>();
            if (o.TryGetProperty("doc_mention_counts", out var dmc) && dmc.ValueKind == JsonValueKind.Object)
                streams = StrList(dmc, "bug_stream_files");

            map[name] = new PresenceRow
            {
                FunctionId = Str(o, "function_id") ?? name,
                SurfaceName = Str(o, "surface_name") ?? "",
                ImplModules = StrList(o, "impl_modules"),
                TestsInImplModules = Int(o, "tests_in_impl_modules") ?? 0,
                ModuleSharedByCount = Int(o, "module_shared_by_count") ?? 0,
                ModuleSharedBy = StrList(o, "module_shared_by"),
                ModuleTestsMinusSiblingCount = mtms,
                TestsPerModule = tpm,
                NameMatchConfidence = Str(o, "name_match_confidence") ?? "high",
                DeclaredArtifactsPresent = Bool(o, "declared_artifacts_present"),
                BugStreamFiles = streams,
                OxfuncTreeClean = Bool(o, "oxfunc_tree_clean", true),
            };
        }
        return map;
    }

    // ------------------------------------------------------------------ battery

    public IReadOnlySet<string> LoadBatteryIds()
    {
        var dir = Path.Combine(_root, "data", "battery");
        var set = new SortedSet<string>(StringComparer.Ordinal);
        if (!Directory.Exists(dir)) return set;
        foreach (var path in Directory.EnumerateFiles(dir, "*.json"))
        {
            var name = Path.GetFileNameWithoutExtension(path);
            if (name == "index") continue;
            set.Add(name);
        }
        return set;
    }

    // ------------------------------------------------------------------ evidence

    public IReadOnlyList<EvidenceRecord> LoadEvidence()
    {
        var dir = Path.Combine(_root, "content", "evidence", "records");
        var list = new List<EvidenceRecord>();
        if (!Directory.Exists(dir)) return list;

        foreach (var path in Directory.EnumerateFiles(dir, "*.json").OrderBy(p => p, StringComparer.Ordinal))
        {
            using var doc = Parse(path);
            var o = doc.RootElement;
            var recordId = Str(o, "record_id") ?? Path.GetFileNameWithoutExtension(path);

            var roles = new Dictionary<string, string>(StringComparer.Ordinal);
            if (o.TryGetProperty("subject_role", out var sr) && sr.ValueKind == JsonValueKind.Object)
                foreach (var p in sr.EnumerateObject())
                    if (p.Value.ValueKind == JsonValueKind.String) roles[p.Name] = p.Value.GetString()!;

            var siep = new Dictionary<string, string>(StringComparer.Ordinal);
            if (o.TryGetProperty("subject_internal_equality_predicate", out var sp) && sp.ValueKind == JsonValueKind.Object)
                foreach (var pr in sp.EnumerateObject())
                    if (pr.Value.ValueKind == JsonValueKind.String) siep[pr.Name] = pr.Value.GetString()!;

            var counts = new List<CountRow>();
            if (o.TryGetProperty("counts", out var cs) && cs.ValueKind == JsonValueKind.Array)
            {
                int i = 0;
                foreach (var c in cs.EnumerateArray())
                    counts.Add(ReadCount(recordId, i++, c));
            }

            list.Add(new EvidenceRecord
            {
                RecordId = recordId,
                RecordVersion = Int(o, "record_version") ?? 1,
                Class = Str(o, "class") ?? "",
                Subjects = StrList(o, "subjects"),
                SubjectRole = roles,
                Title = Str(o, "title") ?? "",
                UpstreamRowId = Str(o, "upstream_row_id"),
                UpstreamRegister = Str(o, "upstream_register") ?? "none",
                UpstreamStatusVerbatim = Str(o, "upstream_status_verbatim"),
                Substrate = Str(o, "substrate"),
                ReaderWarning = Str(o, "reader_warning"),
                HandbookReverified = Bool(o, "handbook_reverified"),
                Counts = counts,
                SourcePath = "content/evidence/records/" + Path.GetFileName(path),
                SubjectInternalEqualityPredicate = siep,
                MustNotRenderAlone = Bool(o, "must_not_render_alone"),
                RenderTogetherWith = StrList(o, "render_together_with"),
            });
        }
        return list.OrderBy(r => r.RecordId, StringComparer.Ordinal).ToList();
    }

    private CountRow ReadCount(string recordId, int index, JsonElement c)
    {
        // ---- held_out : BOTH JSON boolean and JSON string appear in the corpus.
        string heldOut;
        if (c.TryGetProperty("held_out", out var ho))
        {
            switch (ho.ValueKind)
            {
                case JsonValueKind.True:
                    heldOut = HeldOut.True; break;
                case JsonValueKind.False:
                    heldOut = HeldOut.False; break;
                case JsonValueKind.String:
                    var raw = ho.GetString() ?? "";
                    heldOut = raw switch
                    {
                        "true" or "True" or "TRUE" => HeldOut.True,
                        "false" or "False" or "FALSE" => HeldOut.False,
                        HeldOut.Partial => HeldOut.Partial,
                        HeldOut.SourceDoesNotState => HeldOut.SourceDoesNotState,
                        _ => HeldOut.SourceDoesNotState
                    };
                    if (raw is "true" or "True" or "TRUE" or "false" or "False" or "FALSE")
                        Hygiene.Add(HygieneReport.Kinds.HeldOutStringBoolean, recordId, index, "held_out",
                            "\"" + raw + "\"", heldOut,
                            "Normalised to the enum value. The string \"false\" is NOT truthy; it is read as not-held-out, " +
                            "and the string \"true\" is read as held-out. A naive truthiness test would invert 'false'.");
                    else if (raw is not (HeldOut.Partial or HeldOut.SourceDoesNotState))
                        Hygiene.Add(HygieneReport.Kinds.HeldOutStringBoolean, recordId, index, "held_out",
                            "\"" + raw + "\"", HeldOut.SourceDoesNotState,
                            "Unrecognised held_out value; demoted to source-does-not-state (the weaker reading).");
                    break;
                default:
                    heldOut = HeldOut.SourceDoesNotState; break;
            }
        }
        else
        {
            heldOut = HeldOut.SourceDoesNotState;
            Hygiene.Add(HygieneReport.Kinds.MissingOptionalField, recordId, index, "held_out",
                "absent", HeldOut.SourceDoesNotState, "Field absent; defaulted to the weaker reading.");
        }

        // ---- build_ambiguity : variant spelling
        var buildAmb = Str(c, "build_ambiguity") ?? BuildAmbiguity.NotStated;
        if (buildAmb == "build-not-stated-in-this-record")
        {
            Hygiene.Add(HygieneReport.Kinds.BuildAmbiguityVariantSpelling, recordId, index, "build_ambiguity",
                buildAmb, BuildAmbiguity.NotStated,
                "Variant spelling folded onto the canonical value. Both mean: this record names no Excel build.");
            buildAmb = BuildAmbiguity.NotStated;
        }

        // ---- currency : ABSENCE IS NOT "current"
        string currency;
        if (c.TryGetProperty("currency", out var cur) && cur.ValueKind == JsonValueKind.String)
        {
            currency = cur.GetString()!;
        }
        else
        {
            currency = Currency.NotAnnotated;
            Hygiene.Add(HygieneReport.Kinds.CurrencyAbsent, recordId, index, "currency",
                "absent", Currency.NotAnnotated,
                "The record does not annotate whether this count is the current one. Absence is given its own " +
                "value and is never read as \"current\"; every label derived from such a count carries the " +
                "not-annotated clause and the count is tallied separately.");
        }

        // ---- corpus_was_repair_target : bool | "source-does-not-state"
        string repairTarget;
        if (c.TryGetProperty("corpus_was_repair_target", out var rt))
        {
            repairTarget = rt.ValueKind switch
            {
                JsonValueKind.True => "true",
                JsonValueKind.False => "false",
                JsonValueKind.String => rt.GetString() ?? "source-does-not-state",
                _ => "source-does-not-state"
            };
            if (rt.ValueKind == JsonValueKind.String &&
                (rt.GetString() == "true" || rt.GetString() == "false"))
                Hygiene.Add(HygieneReport.Kinds.RepairTargetStringBoolean, recordId, index,
                    "corpus_was_repair_target", "\"" + rt.GetString() + "\"", repairTarget,
                    "Normalised to the enum value; the string is not read by truthiness.");
        }
        else
        {
            repairTarget = "source-does-not-state";
        }

        var heldOutRows = Int(c, "held_out_rows");
        if (heldOutRows.HasValue && heldOut is not (HeldOut.True or HeldOut.Partial))
            Hygiene.Add(HygieneReport.Kinds.HeldOutRowsWithoutHeldOut, recordId, index, "held_out_rows",
                heldOutRows.Value.ToString(), "ignored",
                "held_out_rows is populated while held_out is not true/partial. The row count is not used; " +
                "the entry is not counted as holding a held-out corpus.");

        var explicitSubjects = StrList(c, "subject_ids");
        if (explicitSubjects.Count == 0) explicitSubjects = StrList(c, "subjects");
        if (explicitSubjects.Count == 0)
        {
            var one = Str(c, "subject");
            if (one is not null) explicitSubjects = new List<string> { one };
        }

        return new CountRow
        {
            RecordId = recordId,
            Index = index,
            Passed = Int(c, "passed"),
            Total = Int(c, "total"),
            AxisValue = Str(c, "axis") ?? Axis.Numeric,
            ComparisonPredicate = Str(c, "comparison_predicate") ?? "exact-typed-bit-match",
            CountScope = Str(c, "count_scope") ?? "per-surface",
            GroupMembers = StrList(c, "group_members"),
            AttributionValue = Str(c, "attribution") ?? Attribution.Unclear,
            MeasurementSubject = Str(c, "measurement_subject") ?? Subject.Unclear,
            HeldOutValue = heldOut,
            HeldOutRows = heldOutRows,
            CorpusWasRepairTarget = repairTarget,
            ResidualAttribution = Str(c, "residual_attribution"),
            MeasurementFound = Bool(c, "measurement_found", true),
            DivergenceMeasured = Bool(c, "divergence_measured"),
            FullPassOnly = Bool(c, "full_pass_only"),
            Builds = StrList(c, "builds"),
            BuildAmbiguityValue = buildAmb,
            BuildNote = Str(c, "build_note"),
            Arch = Str(c, "arch") ?? "not-stated-in-this-record",
            Cpu = Str(c, "cpu") ?? "not-stated-in-this-record",
            CpuScoped = Bool(c, "cpu_scoped"),
            CorpusOrBuild = Str(c, "corpus_or_build") ?? "not-stated-in-this-record",
            CorpusTracked = Bool(c, "corpus_tracked"),
            MeasuredAsOf = Str(c, "measured_as_of"),
            CurrencyValue = currency,
            AmbiguityNote = Str(c, "ambiguity_note"),
            Citation = Str(c, "citation") ?? "",
            ExplicitSubjects = explicitSubjects,
        };
    }

    // ------------------------------------------------------------------ curation organs

    public int CountFiles(string relativeDir, string pattern)
    {
        var dir = Path.Combine(_root, relativeDir.Replace('/', Path.DirectorySeparatorChar));
        return Directory.Exists(dir) ? Directory.EnumerateFiles(dir, pattern).Count() : 0;
    }

    public int CountVectorSuites()
    {
        var dir = Path.Combine(_root, "vectors");
        if (!Directory.Exists(dir)) return 0;
        return Directory.EnumerateFiles(dir, "MANIFEST.json", SearchOption.AllDirectories).Count();
    }

    public int CountAdmittedImplementations()
    {
        var dir = Path.Combine(_root, "implementations");
        if (!Directory.Exists(dir)) return 0;
        return Directory.EnumerateDirectories(dir).Count();
    }
}
