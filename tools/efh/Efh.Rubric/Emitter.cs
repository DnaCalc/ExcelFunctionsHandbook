using System.Globalization;
using System.Text.Encodings.Web;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace Efh.Rubric;

public static class Emitter
{
    private static readonly JsonSerializerOptions Opts = new()
    {
        WriteIndented = true,
        Encoder = JavaScriptEncoder.UnsafeRelaxedJsonEscaping,
    };

    private static void Write(string path, JsonNode node)
    {
        Directory.CreateDirectory(Path.GetDirectoryName(path)!);
        var text = node.ToJsonString(Opts).ReplaceLineEndings("\n");
        File.WriteAllText(path, text + "\n", new System.Text.UTF8Encoding(false));
    }

    private static JsonArray Arr(IEnumerable<string> items)
    {
        var a = new JsonArray();
        foreach (var i in items) a.Add(i);
        return a;
    }

    private static JsonArray TraceArr(IReadOnlyList<DerivationStep> steps)
    {
        var a = new JsonArray();
        foreach (var s in steps)
            a.Add(new JsonObject
            {
                ["rung"] = s.Rung,
                ["rule"] = s.Rule,
                ["records"] = Arr(s.Records),
                ["fields_read"] = Arr(s.Fields),
            });
        return a;
    }

    private static JsonObject AxisObj(AxisState a) => new()
    {
        ["state"] = a.State,
        ["label"] = a.Label,
        ["primary_count"] = a.PrimaryCountKey,
        ["passed"] = a.Passed,
        ["total"] = a.Total,
        ["comparison_predicate"] = a.ComparisonPredicate,
        ["derivation_trace"] = TraceArr(a.Trace),
    };

    private static JsonObject FlagsObj(Flags f) => new()
    {
        ["has_any_evidence_record"] = f.HasAnyEvidenceRecord,
        ["has_counted_record"] = f.HasCountedRecord,
        ["has_per_surface_count"] = f.HasPerSurfaceCount,
        ["has_numeric_counted_record"] = f.HasNumericCountedRecord,
        ["has_per_surface_numeric_count"] = f.HasPerSurfaceNumericCount,
        ["has_structural_counted_record"] = f.HasStructuralCountedRecord,
        ["only_counted_axis_is_structural"] = f.OnlyCountedAxisIsStructural,
        ["only_counted_evidence_is_group_total"] = f.OnlyCountedEvidenceIsGroupTotal,
        ["has_held_out_counted_record"] = f.HasHeldOutCountedRecord,
        ["corpus_was_repair_target"] = f.CorpusWasRepairTarget,
        ["cpu_not_stated_on_a_counted_record"] = f.CpuNotStatedOnACountedRecord,
        ["non_single_build_on_a_counted_record"] = f.NonSingleBuildOnACountedRecord,
        ["currency_not_annotated_on_a_counted_record"] = f.CurrencyNotAnnotatedOnACountedRecord,
        ["dropped_from_counted_by_attribution_gate"] = f.DroppedFromCountedByAttributionGate,
        ["dropped_from_counted_by_measurement_subject_gate"] = f.DroppedFromCountedByMeasurementSubjectGate,
        ["has_unbindable_per_surface_count"] = f.HasUnbindablePerSurfaceCount,
        ["unbindable_per_surface_count_rows"] = f.UnbindablePerSurfaceCountRows,
        ["has_handbook_recomputation_count"] = f.HasHandbookRecomputationCount,
        ["has_open_defect_stream_record"] = f.HasOpenDefectStreamRecord,
        ["named_in_a_defect_stream_file_any_status"] = f.NamedInADefectStreamFileAnyStatus,
        ["has_open_catalogue_row"] = f.HasOpenCatalogueRow,
        ["has_residual_register_row"] = f.HasResidualRegisterRow,
        ["numeric_clean_no_open_row_and_no_defect_stream_file_names_it"] = f.NumericCleanNoOpenRowNoStream,
        ["numeric_clean_no_open_row_and_no_open_defect_stream_record"] = f.NumericCleanNoOpenRowNoOpenStreamRecord,
        ["numeric_clean_no_open_row"] = f.NumericCleanNoOpenRow,
        ["open_row_classifier_used_fallback"] = f.OpenRowClassifierUsedFallback,
        ["impl_module_absent"] = f.ImplModuleAbsent,
        ["capped_by_g1"] = f.CappedByG1,
        ["has_battery"] = f.HasBattery,
        ["declared_artifacts_present"] = f.DeclaredArtifactsPresent,
        ["low_name_match_confidence"] = f.LowNameMatchConfidence,
        ["in_pigeonhole_set"] = f.InPigeonholeSet,
    };

    // ================================================================ rubric.json

    public static void EmitRubric(string path, RubricEngine e, Counters c, Loader loader,
                                  string basisMode = "pre-ledger")
    {
        var entries = new JsonArray();
        foreach (var a in e.Assignments.OrderBy(x => x.FunctionId, StringComparer.Ordinal))
        {
            entries.Add(new JsonObject
            {
                ["function_id"] = a.FunctionId,
                ["surface_name"] = a.SurfaceName,
                ["rubric_version"] = a.RubricVersion,
                ["warrant"] = new JsonObject
                {
                    ["level"] = a.Warrant,
                    ["label"] = a.WarrantLabel,
                    ["derivation_trace"] = TraceArr(a.WarrantTrace),
                },
                ["test_depth"] = new JsonObject
                {
                    ["tier"] = a.Depth,
                    ["label"] = a.DepthLabel,
                    ["derivation_trace"] = TraceArr(a.DepthTrace),
                },
                ["excel_relationship"] = new JsonObject
                {
                    ["axis_order_worst_first"] = Arr(a.AxisOrderWorstFirst),
                    ["numeric_bits"] = AxisObj(a.Numeric),
                    ["structural_admission"] = AxisObj(a.Structural),
                },
                ["internal_equality_predicate"] = a.InternalEqualityPredicate,
                ["internal_equality_predicate_label"] = a.InternalEqualityPredicateLabel,
                ["records_that_must_not_render_alone"] = Arr(a.RecordsThatMustNotRenderAlone),
                ["companion_records"] = Arr(a.CompanionRecords),
                ["why_no_count"] = a.WhyNoCount,
                ["why_no_count_all_applicable"] = Arr(a.WhyNoCountAllApplicable),
                ["evidence_records"] = Arr(a.EvidenceRecords),
                ["open_defect_stream_records"] = Arr(a.OpenDefectStreamRecords),
                ["defect_stream_files_naming_this_surface"] = Arr(a.DefectStreamFilesNamingThisSurface),
                ["limitations"] = Arr(a.Limitations),
                ["flags"] = FlagsObj(a.Flags),
            });
        }

        var counters = new JsonObject();
        foreach (var (k, v) in c.HonestyCounters().OrderBy(x => x.Key, StringComparer.Ordinal))
            counters[k] = v;

        var comparison = new JsonArray();
        foreach (var r in c.Comparison())
            comparison.Add(new JsonObject
            {
                ["fact"] = r.Fact,
                ["derived_now"] = r.Derived,
                ["foundation_3"] = r.Foundation,
                ["attribution_corrections_4"] = r.AttributionCorrections,
                ["difference_vs_foundation"] = r.Foundation is null ? null : r.Derived - r.Foundation,
                ["explanation"] = r.Explanation,
            });

        var root = new JsonObject
        {
            ["schema"] = "efh.rubric/v1",
            ["rubric_version"] = RubricEngine.RubricVersion,
            ["basis_mode"] = basisMode,
            ["denominator_sentence"] = Counters.DenominatorSentence,
            ["entries_total"] = e.Assignments.Count,
            ["evidence_records_read"] = e.Records.Count,
            ["evidence_layer_is_still_being_written"] = true,
            ["standing"] = new JsonArray
            {
                "No level on this page is asserted. Every warrant level, depth tier and axis state carries "
                + "rubric_version and a derivation trace naming the records and the fields that produced it.",
                "handbook_reverified is false on every evidence record read. The Handbook has re-run nothing.",
                "basis_mode is pre-ledger: no claim has been recorded in a ledger, so the seven doors render "
                + "in reduced form and the Why door shows this derivation rather than an assertion graph.",
                Counters.DenominatorSentence + ".",
            },
            ["honesty_counters"] = counters,
            ["comparison_against_prior_documents"] = comparison,
            ["entries"] = entries,
        };

        Write(path, root);
    }

    // ================================================================ hygiene report

    public static void EmitHygiene(string path, Loader loader, RubricEngine e)
    {
        var byKind = new JsonArray();
        foreach (var g in loader.Hygiene.ByKind())
        {
            var occurrences = new JsonArray();
            foreach (var x in g.OrderBy(x => x.RecordId, StringComparer.Ordinal).ThenBy(x => x.CountIndex))
                occurrences.Add(new JsonObject
                {
                    ["record_id"] = x.RecordId,
                    ["count_index"] = x.CountIndex,
                    ["field"] = x.Field,
                    ["observed"] = x.Observed,
                    ["normalised_to"] = x.Normalised,
                    ["note"] = x.Note,
                });
            byKind.Add(new JsonObject
            {
                ["kind"] = g.Key,
                ["occurrences"] = g.Count(),
                ["records_affected"] = g.Select(x => x.RecordId).Distinct(StringComparer.Ordinal).Count(),
                ["detail"] = occurrences,
            });
        }

        var unbindable = new JsonArray();
        foreach (var u in e.Unbindable.OrderBy(u => u.RecordId, StringComparer.Ordinal).ThenBy(u => u.CountIndex))
            unbindable.Add(new JsonObject
            {
                ["record_id"] = u.RecordId,
                ["count_index"] = u.CountIndex,
                ["axis"] = u.AxisValue,
                ["record_subject_count"] = u.SubjectCount,
                ["subjects"] = Arr(u.Subjects),
                ["reason"] = u.Reason,
            });

        var root = new JsonObject
        {
            ["schema"] = "efh.rubric-hygiene/v1",
            ["rubric_version"] = RubricEngine.RubricVersion,
            ["what_this_is"] =
                "Every normalisation the rubric applied while reading content/evidence/records, and every "
                + "count it refused to bind. Published so the repairs are visible rather than buried in a parser.",
            ["normalisations"] = byKind,
            ["binding_defect"] = new JsonObject
            {
                ["headline"] =
                    "The counts[] schema carries no field naming which subject a per-surface count belongs to. "
                    + "In a multi-subject record the binding is carried only by the prose inside the display text / "
                    + "source_sentence, and guard G-9 forbids the rubric from parsing those. Positional binding "
                    + "would be a guess, and a wrong guess puts one surface's shortfall on another surface's page.",
                ["handling"] =
                    "An unbindable per-surface count contributes \"a comparison is on record\" (W3, N7/S7) and "
                    + "never a counted warrant (W4/W5) or a matched-Excel state (N5/N6/S5/S6).",
                ["repair"] =
                    "Add counts[].subject (or counts[].subject_ids) to the evidence-record schema. The rubric "
                    + "already reads it and will bind immediately on the next run.",
                ["unbindable_count_rows"] = e.Unbindable.Count,
                ["records_affected"] = e.Unbindable.Select(u => u.RecordId).Distinct(StringComparer.Ordinal).Count(),
                ["entries_affected"] = e.UnbindableByFunction.Count,
                ["detail"] = unbindable,
            },
        };

        Write(path, root);
    }

    // ================================================================ ratchets

    public static void EmitRatchets(string path, RubricEngine e, Counters c, Loader loader)
    {
        var h = c.HonestyCounters();

        JsonObject R(string id, string counter, int today, string direction, string note) => new()
        {
            ["id"] = id,
            ["counter"] = counter,
            ["today"] = today,
            ["direction"] = direction,
            ["note"] = note,
        };

        var ratcheted = new JsonArray
        {
            R("RC-1", "entries with curated Handbook prose (content/functions/*.md)",
              h["entries_with_curated_handbook_prose"], "up", "CI-red if it falls."),
            R("RC-2", "curated family pages (content/families/*.md)",
              h["curated_family_pages"], "up", "CI-red if it falls."),
            R("RC-3", "evidence records (content/evidence/records/*.json)",
              h["evidence_records"], "up", "CI-red if it falls."),
            R("RC-4", "published vector suites (vectors/**/MANIFEST.json)",
              h["published_vector_suites"], "up", "CI-red if it falls."),
            R("RC-5", "admitted Handbook implementations (implementations/*)",
              h["admitted_handbook_implementations"], "up", "CI-red if it falls."),
            R("RC-6", "entries with a numeric-bits counted record",
              h["entries_with_a_numeric_bits_counted_record"], "up",
              "Handbook-controlled: it moves only when the Handbook extracts or binds another count."),
            R("RC-7", "entries with a held-out counted record",
              h["entries_with_a_held_out_counted_record"], "up", "CI-red if it falls."),
            R("RC-8", "entries with a per-surface count",
              h["of_those_with_a_per_surface_count"], "up", "CI-red if it falls."),
            R("RC-9", "entries with a battery row set (data/battery)",
              h["entries_with_a_battery_row_set"], "up", "CI-red if it falls."),
            R("RC-10", "evidence records with an unresolved upstream_changed marker",
              0, "down-to-zero", "Must return to zero. No upstream_changed marker exists in the records today."),
        };

        var reported = new JsonArray();
        void Rep(string id, string counter, int today, string note) =>
            reported.Add(new JsonObject
            {
                ["id"] = id,
                ["counter"] = counter,
                ["today"] = today,
                ["on_rise"] = "review note",
                ["never"] = "a red build",
                ["note"] = note,
            });

        Rep("RP-W0", "entries at W0", c.Warrant("W0"), "A fact about the read-only OxFunc tree.");
        Rep("RP-W1", "entries at W1", c.Warrant("W1"), "A fact about the read-only OxFunc tree.");
        Rep("RP-W2", "entries at W2", c.Warrant("W2"), "A fact about the read-only OxFunc tree.");
        Rep("RP-W3", "entries at W3", c.Warrant("W3"), "A fact about OxFunc's evidence, as harvested.");
        Rep("RP-W3R", "entries at W3R", c.Warrant("W3R"), "Handbook recomputations over stored OxFunc corpora.");
        Rep("RP-W4", "entries at W4", c.Warrant("W4"), "A fact about OxFunc's measurements.");
        Rep("RP-W5", "entries at W5", c.Warrant("W5"), "A fact about OxFunc's measurements.");
        Rep("RP-D0D1", "entries at D0 or D1", c.Depth("D0") + c.Depth("D1"), "A fact about OxFunc's test layout.");
        Rep("RP-D2", "entries at D2", c.Depth("D2"), "A fact about OxFunc's module layout.");
        Rep("RP-D3", "entries at D3", c.Depth("D3"), "A fact about OxFunc's module layout.");
        Rep("RP-D4", "entries at D4", c.Depth("D4"), "A fact about OxFunc's module layout.");
        Rep("RP-N1", "entries at N1", c.NumericState("N1"), "A fact about OxFunc's open discrepancy register.");
        Rep("RP-N2", "entries at N2", c.NumericState("N2"), "A fact about OxFunc's open discrepancy register.");
        Rep("RP-N8", "entries at N8", c.NumericState("N8"), "A fact about what OxFunc has measured.");
        Rep("RP-S8", "entries at S8", c.StructuralState("S8"), "A fact about what OxFunc has measured.");
        Rep("RP-GROUP", "entries whose only counted evidence is a group total",
            c.Flag(f => f.OnlyCountedEvidenceIsGroupTotal), "A fact about how OxFunc scoped its corpora.");
        Rep("RP-STREAMS-REC", "entries with an open OxFunc defect-stream record",
            c.Flag(f => f.HasOpenDefectStreamRecord), "A fact about OxFunc's own defect tracker.");
        Rep("RP-STREAMS-FILE", "entries named in an OxFunc defect-stream file (any status)",
            c.Flag(f => f.NamedInADefectStreamFileAnyStatus),
            "data/presence records the stream file path and not its status, so this counter cannot distinguish open from closed.");
        Rep("RP-NORECORD", "entries with no evidence record of any kind",
            c.Flag(f => !f.HasAnyEvidenceRecord), "Falls as the evidence layer is written; a rise is a review note.");
        Rep("RP-ARTIFACTS", "entries with declared artifacts present",
            c.Flag(f => f.DeclaredArtifactsPresent), "A fact about the read-only OxFunc tree.");
        Rep("RP-REPAIRTARGET", "entries whose counted corpus was the repair target",
            c.Flag(f => f.CorpusWasRepairTarget), "A fact about how OxFunc fitted its repairs.");
        Rep("RP-NOCPU", "entries holding a counted record whose host CPU is not stated",
            c.Flag(f => f.CpuNotStatedOnACountedRecord), "A fact about OxFunc's disclosure.");
        Rep("RP-NOBUILD", "entries holding a counted record with a non-single build",
            c.Flag(f => f.NonSingleBuildOnACountedRecord), "A fact about OxFunc's disclosure.");

        var tooling = new JsonArray
        {
            new JsonObject
            {
                ["id"] = "TM-1",
                ["counter"] = "entries with name_match_confidence low",
                ["today"] = c.Flag(f => f.LowNameMatchConfidence),
                ["note"] = "A tooling metric, not an evidence state. It is constant unless the matching rule changes.",
            },
        };

        var root = new JsonObject
        {
            ["schema"] = "efh.ratchets/v1",
            ["rubric_version"] = RubricEngine.RubricVersion,
            ["denominator_sentence"] = Counters.DenominatorSentence,
            ["denominator"] = e.Assignments.Count,
            ["evidence_records_at_baseline"] = e.Records.Count,
            ["baseline_caveat"] =
                "This baseline was taken while content/evidence/records was still being written. "
                + "Regenerate with 'efh rubric emit' once the evidence layer is complete; the RATCHETED "
                + "counters may only rise, so a baseline taken too early is safe, and a baseline taken from "
                + "an over-complete state would wedge the build.",
            ["ratcheted"] = ratcheted,
            ["ratcheted_contract"] =
                "Handbook-controlled counters. A fall is a red build.",
            ["reported"] = reported,
            ["reported_contract"] =
                "Facts about the read-only OxFunc repository and about what OxFunc has measured. "
                + "A rise produces a review note and never a red build: the Handbook does not control them, "
                + "and ratcheting a fact about somebody else's repository would either be vacuous or would "
                + "make an honest re-derivation fail CI.",
            ["tooling_metrics"] = tooling,
        };

        Write(path, root);
    }
}
