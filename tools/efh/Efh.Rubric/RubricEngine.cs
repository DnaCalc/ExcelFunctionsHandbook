namespace Efh.Rubric;

public sealed record DerivationStep(
    string Rung,
    string Rule,
    IReadOnlyList<string> Records,
    IReadOnlyList<string> Fields);

public sealed record AxisState(
    string State,
    string Label,
    string? PrimaryCountKey,
    int? Passed,
    int? Total,
    string? ComparisonPredicate,
    IReadOnlyList<DerivationStep> Trace);

public sealed record Assignment
{
    public required string FunctionId { get; init; }
    public required string SurfaceName { get; init; }
    public required string RubricVersion { get; init; }

    public required string Warrant { get; init; }
    public required string WarrantLabel { get; init; }
    public required IReadOnlyList<DerivationStep> WarrantTrace { get; init; }

    public required string Depth { get; init; }
    public required string DepthLabel { get; init; }
    public required IReadOnlyList<DerivationStep> DepthTrace { get; init; }

    public required AxisState Numeric { get; init; }
    public required AxisState Structural { get; init; }
    public required IReadOnlyList<string> AxisOrderWorstFirst { get; init; }

    public required string? WhyNoCount { get; init; }
    public required IReadOnlyList<string> WhyNoCountAllApplicable { get; init; }

    /// <summary>
    /// What this surface's own implementation does when it compares two numbers, as recorded per
    /// subject in the evidence. Distinct from the measurement's comparison_predicate. Null when no
    /// record states it.
    /// </summary>
    public required string? InternalEqualityPredicate { get; init; }

    public required string? InternalEqualityPredicateLabel { get; init; }

    /// <summary>Records that may not be rendered alone, with the records they must render beside.</summary>
    public required IReadOnlyList<string> RecordsThatMustNotRenderAlone { get; init; }
    public required IReadOnlyList<string> CompanionRecords { get; init; }

    public required IReadOnlyList<string> EvidenceRecords { get; init; }
    public required IReadOnlyList<string> OpenDefectStreamRecords { get; init; }
    public required IReadOnlyList<string> DefectStreamFilesNamingThisSurface { get; init; }
    public required IReadOnlyList<string> Limitations { get; init; }
    public required Flags Flags { get; init; }
}

public sealed record Flags
{
    public required bool HasAnyEvidenceRecord { get; init; }
    public required bool HasCountedRecord { get; init; }
    public required bool HasPerSurfaceCount { get; init; }
    public required bool HasNumericCountedRecord { get; init; }
    public required bool HasPerSurfaceNumericCount { get; init; }
    public required bool HasStructuralCountedRecord { get; init; }
    public required bool OnlyCountedAxisIsStructural { get; init; }
    public required bool OnlyCountedEvidenceIsGroupTotal { get; init; }
    public required bool HasHeldOutCountedRecord { get; init; }
    public required bool CorpusWasRepairTarget { get; init; }
    public required bool CpuNotStatedOnACountedRecord { get; init; }
    public required bool NonSingleBuildOnACountedRecord { get; init; }
    public required bool CurrencyNotAnnotatedOnACountedRecord { get; init; }
    public required bool DroppedFromCountedByAttributionGate { get; init; }
    public required bool DroppedFromCountedByMeasurementSubjectGate { get; init; }
    public required bool HasUnbindablePerSurfaceCount { get; init; }
    public required int UnbindablePerSurfaceCountRows { get; init; }
    public required bool HasHandbookRecomputationCount { get; init; }
    /// <summary>An evidence record of class defect-stream whose upstream status reads open or repair-landed-unsigned.</summary>
    public required bool HasOpenDefectStreamRecord { get; init; }

    /// <summary>
    /// data/presence names at least one docs/bugs/streams/ file mentioning this surface. The presence
    /// organ records the file path and NOT the stream's status, so this flag cannot distinguish an
    /// open stream from a closed one. It is deliberately named for what it is.
    /// </summary>
    public required bool NamedInADefectStreamFileAnyStatus { get; init; }

    public required bool HasOpenCatalogueRow { get; init; }
    public required bool HasResidualRegisterRow { get; init; }

    /// <summary>The conservative reading: numeric-clean, no open row, and no defect-stream file names it at all.</summary>
    public required bool NumericCleanNoOpenRowNoStream { get; init; }

    /// <summary>The narrower reading: numeric-clean, no open row, and no defect-stream RECORD carries an open status.</summary>
    public required bool NumericCleanNoOpenRowNoOpenStreamRecord { get; init; }

    public required bool NumericCleanNoOpenRow { get; init; }
    public required bool OpenRowClassifierUsedFallback { get; init; }
    public required bool ImplModuleAbsent { get; init; }
    public required bool CappedByG1 { get; init; }
    public required bool HasBattery { get; init; }
    public required bool DeclaredArtifactsPresent { get; init; }
    public required bool LowNameMatchConfidence { get; init; }
    public required bool InPigeonholeSet { get; init; }
}

public sealed class RubricEngine
{
    public const string RubricVersion = "rubric-derivation/v1.0.0";

    private static readonly string[] ComparisonClasses =
    {
        "live-verification", "structural-verification", "local-verification",
        "open-discrepancy", "excel-math-deviation", "substrate-identification",
        "alias-pairing", "defect-stream",
    };

    private static readonly string[] WarrantOrder = { "W0", "W1", "W2", "W3", "W3R", "W4", "W5" };

    public IReadOnlyList<FunctionRow> Functions { get; }
    public IReadOnlyDictionary<string, PresenceRow> Presence { get; }
    public IReadOnlyList<EvidenceRecord> Records { get; }
    public IReadOnlySet<string> BatteryIds { get; }
    public IReadOnlyList<UnbindableCount> Unbindable { get; }
    public IReadOnlyDictionary<string, List<BoundCount>> BoundByFunction { get; }
    public IReadOnlyDictionary<string, List<UnbindableCount>> UnbindableByFunction { get; }
    public IReadOnlyList<Assignment> Assignments { get; }

    public RubricEngine(
        IReadOnlyList<FunctionRow> functions,
        IReadOnlyDictionary<string, PresenceRow> presence,
        IReadOnlyList<EvidenceRecord> records,
        IReadOnlySet<string> batteryIds)
    {
        Functions = functions;
        Presence = presence;
        Records = records;
        BatteryIds = batteryIds;

        var (bound, unbindable) = Binder.Bind(records);
        BoundByFunction = bound;
        Unbindable = unbindable;
        UnbindableByFunction = Binder.UnbindableByFunction(unbindable);

        var pigeonhole = ComputePigeonholeSet(presence);

        var list = new List<Assignment>();
        foreach (var f in functions)
            list.Add(Derive(f, pigeonhole));
        Assignments = list;
    }

    private static HashSet<string> ComputePigeonholeSet(IReadOnlyDictionary<string, PresenceRow> presence)
    {
        var set = new HashSet<string>(StringComparer.Ordinal);
        foreach (var (id, p) in presence)
            if (p.ModuleTestsMinusSiblingCount.Values.Any(v => v < 0))
                set.Add(id);
        return set;
    }

    public static int WarrantRank(string w) => Array.IndexOf(WarrantOrder, w);

    // ==================================================================== derivation

    private Assignment Derive(FunctionRow f, HashSet<string> pigeonhole)
    {
        var id = f.FunctionId;
        Presence.TryGetValue(id, out var p);
        var recs = Records.Where(r => r.Subjects.Contains(id)).ToList();
        var bound = BoundByFunction.TryGetValue(id, out var b) ? b : new List<BoundCount>();
        UnbindableByFunction.TryGetValue(id, out var unbound);
        unbound ??= new List<UnbindableCount>();

        // ---- currency gate: superseded / withdrawn / pre-repair counts never derive a state.
        var usable = bound.Where(x => x.Count.UsableForState).ToList();

        // ---- attribution gate (A1-F4) and measurement-subject gate (A1-F7).
        var measured = usable.Where(x => x.Count.AttributionValue == Attribution.Measured
                                      && x.Count.MeasurementFound).ToList();
        var prod = measured.Where(x => x.Count.MeasurementSubject == Subject.Production).ToList();
        var recomp = measured.Where(x => x.Count.MeasurementSubject == Subject.HandbookRecomputation).ToList();

        var namedOnly = usable.Where(x => x.Count.AttributionValue is Attribution.AliasInherited
                                          or Attribution.NamedNotMeasured
                                          or Attribution.Disclaimed
                                          or Attribution.ModelOrCandidate).ToList();

        var countedProd = ShadowGroupCounts(prod.Where(x => x.Count.HasNumerator).ToList());
        var countedRecomp = ShadowGroupCounts(recomp.Where(x => x.Count.HasNumerator).ToList());

        var countedProdNum = countedProd.Where(x => x.Count.IsNumeric).ToList();
        var countedProdStr = countedProd.Where(x => x.Count.IsStructural).ToList();
        var countedRecompNum = countedRecomp.Where(x => x.Count.IsNumeric).ToList();
        var countedRecompStr = countedRecomp.Where(x => x.Count.IsStructural).ToList();

        // ---- open rows, math-deviation rows, defect streams
        var openRecs = recs.Where(r => r.Class == "open-discrepancy").ToList();
        var xmdRecs = recs.Where(r => r.Class == "excel-math-deviation").ToList();
        var xmdWitness = xmdRecs.Any(r => r.Counts.Count > 0 || !string.IsNullOrEmpty(r.Substrate));
        var streamRecs = recs.Where(r => r.Class == "defect-stream" && IsOpenStream(r)).ToList();
        var presenceStreams = p?.BugStreamFiles ?? Array.Empty<string>();

        var (openClass, usedFallback, openTrace) = ClassifyOpenRows(id, openRecs, bound);

        // ================================================================ warrant
        var wTrace = new List<DerivationStep>();
        string warrant;
        string warrantLabel;
        BoundCount? warrantCount = null;

        var implAbsent = p is null || p.ImplModules.Count == 0;
        var tests = p?.TestsInImplModules ?? 0;

        var whyAll = WhyNoCount(namedOnly, bound, unbound, openRecs, recs);
        var why = whyAll.Count > 0 ? whyAll[0] : "qualitative-verdict-only";

        if (countedProd.Count > 0 && countedProd.Any(x => x.Count.IsHeldOut))
        {
            warrantCount = Primary(countedProd.Where(x => x.Count.IsHeldOut).ToList());
            warrant = "W5";
            warrantLabel = Labels.W5Label(warrantCount!);
            wTrace.Add(Step("warrant/W5",
                "a counted, held-out comparison exists whose attribution is measured-for-this-surface and whose measurement_subject is production-oxfunc",
                countedProd.Where(x => x.Count.IsHeldOut), "passed", "total", "attribution", "measurement_subject", "held_out", "held_out_rows"));
        }
        else if (countedProd.Count > 0)
        {
            warrantCount = Primary(countedProd);
            warrant = "W4";
            warrantLabel = Labels.W4Label(warrantCount!);
            wTrace.Add(Step("warrant/W4",
                "a counted comparison exists whose attribution is measured-for-this-surface and whose measurement_subject is production-oxfunc; none of the counted rows is held out",
                countedProd, "passed", "total", "attribution", "measurement_subject", "held_out"));
        }
        else if (countedRecomp.Count > 0)
        {
            warrantCount = Primary(countedRecomp);
            warrant = "W3R";
            warrantLabel = Labels.W3RLabel(warrantCount!);
            wTrace.Add(Step("warrant/W3R",
                "the only counted evidence is a Handbook recomputation over a stored corpus; measurement_subject is handbook-recomputation-over-cached-corpus, which is not a live-Excel oracle run and may not raise W4/W5",
                countedRecomp, "passed", "total", "attribution", "measurement_subject"));
        }
        else if (namedOnly.Count > 0 || openRecs.Count > 0 || xmdWitness || unbound.Count > 0
                 || recs.Any(r => ComparisonClasses.Contains(r.Class)))
        {
            warrant = "W3";
            warrantLabel = Labels.W3Label(why);
            wTrace.Add(Step("warrant/W3",
                "a comparison record names this surface but no count survives the attribution, measurement-subject, currency and binding gates",
                recs.Where(r => ComparisonClasses.Contains(r.Class)), "class", "attribution", "measurement_subject", "currency", "count_scope"));
        }
        else if (implAbsent)
        {
            warrant = "W0";
            warrantLabel = Labels.W0;
            wTrace.Add(Step("warrant/W0", "data/presence impl_modules is empty", Array.Empty<EvidenceRecord>(), "impl_modules"));
        }
        else if (tests == 0)
        {
            warrant = "W1";
            warrantLabel = Labels.W1;
            wTrace.Add(Step("warrant/W1", "a module was located and it contains zero #[test] functions", Array.Empty<EvidenceRecord>(), "impl_modules", "tests_in_impl_modules"));
        }
        else
        {
            warrant = "W2";
            warrantLabel = Labels.W2Label(tests);
            wTrace.Add(Step("warrant/W2", "a module was located and it contains tests; no per-function attribution of those tests exists", Array.Empty<EvidenceRecord>(), "impl_modules", "tests_in_impl_modules", "module_tests_minus_sibling_count"));
        }

        // ---- G-1 cap: no implementation module => no warrant above W0, no depth above D0.
        var capped = false;
        if (implAbsent && warrant != "W0")
        {
            wTrace.Add(Step("guard/G-1",
                $"warrant {warrant} was capped to W0: data/presence records no implementation module for this entry, so no warrant above W0 may stand",
                Array.Empty<EvidenceRecord>(), "impl_modules"));
            warrant = "W0";
            warrantLabel = Labels.W0;
            warrantCount = null;
            capped = true;
        }

        // ================================================================ depth
        var dTrace = new List<DerivationStep>();
        string depth, depthLabel;
        var shared = p?.ModuleSharedByCount ?? 0;
        if (implAbsent) { depth = "D0"; depthLabel = Labels.D0; dTrace.Add(Step("depth/D0", "impl_modules is empty", Array.Empty<EvidenceRecord>(), "impl_modules")); }
        else if (tests == 0) { depth = "D1"; depthLabel = Labels.D1; dTrace.Add(Step("depth/D1", "tests_in_impl_modules is zero", Array.Empty<EvidenceRecord>(), "tests_in_impl_modules")); }
        else if (shared >= 5) { depth = "D2"; depthLabel = Labels.D2Label(shared, tests); dTrace.Add(Step("depth/D2", "module_shared_by_count >= 5", Array.Empty<EvidenceRecord>(), "module_shared_by_count", "tests_in_impl_modules")); }
        else if (shared >= 1) { depth = "D3"; depthLabel = Labels.D3Label(shared, tests); dTrace.Add(Step("depth/D3", "module_shared_by_count is 1..4", Array.Empty<EvidenceRecord>(), "module_shared_by_count", "tests_in_impl_modules")); }
        else { depth = "D4"; depthLabel = Labels.D4Label(tests); dTrace.Add(Step("depth/D4", "module_shared_by_count is zero", Array.Empty<EvidenceRecord>(), "module_shared_by_count", "tests_in_impl_modules")); }

        // ================================================================ numeric axis
        var nTrace = new List<DerivationStep>(openTrace);
        var residNum = countedProdNum.Where(x => x.Count.IsResidual).ToList();
        var cleanNum = countedProdNum.Where(x => x.Count.IsClean).ToList();
        AxisState numeric;

        if (openClass == "measured")
        {
            nTrace.Add(Step("numeric/N1", "an open discrepancy row for this surface carries a measured shortfall or a measured divergence", openRecs, "class", "passed", "total", "divergence_measured"));
            numeric = new AxisState("N1", Labels.N1, null, null, null, null, nTrace);
        }
        else if (residNum.Count > 0)
        {
            var pc = Primary(residNum)!;
            nTrace.Add(Step("numeric/N6", "residual is tested before clean: a counted production comparison on the numeric axis has passed < total, so no label on this axis may begin \"matched\"", residNum, "passed", "total", "axis"));
            numeric = new AxisState("N6", Labels.N6Label(pc), pc.Count.Key, pc.Count.Passed, pc.Count.Total, pc.Count.ComparisonPredicate, nTrace);
        }
        else if (openClass == "unmeasured")
        {
            nTrace.Add(Step("numeric/N2", "an open discrepancy row names this surface and publishes no measurement of it", openRecs, "class", "passed", "total"));
            numeric = new AxisState("N2", Labels.N2, null, null, null, null, nTrace);
        }
        else if (xmdRecs.Count > 0 && cleanNum.Count > 0)
        {
            var pc = Primary(cleanNum)!;
            nTrace.Add(Step("numeric/N3", "an Excel-math-deviation record names this surface and a counted production comparison on the numeric axis passed on every row", xmdRecs, "class", "passed", "total"));
            numeric = new AxisState("N3", Labels.N3Label(pc), pc.Count.Key, pc.Count.Passed, pc.Count.Total, pc.Count.ComparisonPredicate, nTrace);
        }
        else if (xmdRecs.Count > 0 && xmdWitness)
        {
            nTrace.Add(Step("numeric/N4", "an Excel-math-deviation record names this surface and carries a witness, but no extracted row count", xmdRecs, "class", "substrate", "counts"));
            numeric = new AxisState("N4", Labels.N4, null, null, null, null, nTrace);
        }
        else if (cleanNum.Count > 0)
        {
            var pc = Primary(cleanNum)!;
            nTrace.Add(Step("numeric/N5", "every counted production comparison on the numeric axis passed on every row, and no open row, defect stream or residual count contradicts it", cleanNum, "passed", "total", "axis", "attribution", "measurement_subject"));
            numeric = new AxisState("N5", Labels.N5Label(pc), pc.Count.Key, pc.Count.Passed, pc.Count.Total, pc.Count.ComparisonPredicate, nTrace);
        }
        else if (countedRecompNum.Count > 0)
        {
            var pc = Primary(countedRecompNum)!;
            nTrace.Add(Step("numeric/NR", "the only counted numeric evidence is a Handbook recomputation over a stored corpus; measurement_subject handbook-recomputation-over-cached-corpus may never produce an N5 or N6 matched-Excel state", countedRecompNum, "passed", "total", "measurement_subject"));
            numeric = new AxisState("NR", Labels.NRLabel(pc), pc.Count.Key, pc.Count.Passed, pc.Count.Total, pc.Count.ComparisonPredicate, nTrace);
        }
        else if (bound.Any(x => x.Count.IsNumeric)
                 || unbound.Any(u => u.AxisValue == Axis.Numeric)
                 || recs.Any(r => r.Class is "live-verification" or "excel-math-deviation" or "substrate-identification")
                 || openClass == "no-non-match-measured")
        {
            nTrace.Add(Step("numeric/N7", "a numeric comparison names this surface but no count survives the gates", recs, "axis", "attribution", "measurement_subject", "currency", "count_scope"));
            numeric = new AxisState("N7", Labels.N7Label(why), null, null, null, null, nTrace);
        }
        else
        {
            nTrace.Add(Step("numeric/N8", "no record in the sources listed under Sources carries a numeric comparison naming this surface", Array.Empty<EvidenceRecord>(), "counts"));
            numeric = new AxisState("N8", Labels.N8, null, null, null, null, nTrace);
        }

        // ================================================================ structural axis
        var sTrace = new List<DerivationStep>();
        var residStr = countedProdStr.Where(x => x.Count.IsResidual).ToList();
        var cleanStr = countedProdStr.Where(x => x.Count.IsClean).ToList();
        AxisState structural;

        if (residStr.Count > 0)
        {
            var pc = Primary(residStr)!;
            sTrace.Add(Step("structural/S6", "residual is tested before clean: a counted production comparison on the structural axis has passed < total", residStr, "passed", "total", "axis", "residual_attribution"));
            structural = new AxisState("S6", Labels.S6Label(pc), pc.Count.Key, pc.Count.Passed, pc.Count.Total, pc.Count.ComparisonPredicate, sTrace);
        }
        else if (cleanStr.Count > 0)
        {
            var pc = Primary(cleanStr)!;
            sTrace.Add(Step("structural/S5", "every counted production comparison on the structural axis passed on every row", cleanStr, "passed", "total", "axis", "attribution", "measurement_subject"));
            structural = new AxisState("S5", Labels.S5Label(pc), pc.Count.Key, pc.Count.Passed, pc.Count.Total, pc.Count.ComparisonPredicate, sTrace);
        }
        else if (countedRecompStr.Count > 0)
        {
            var pc = Primary(countedRecompStr)!;
            sTrace.Add(Step("structural/SR", "the only counted structural evidence is a Handbook recomputation over a stored corpus", countedRecompStr, "passed", "total", "measurement_subject"));
            structural = new AxisState("SR", Labels.SRLabel(pc), pc.Count.Key, pc.Count.Passed, pc.Count.Total, pc.Count.ComparisonPredicate, sTrace);
        }
        else if (bound.Any(x => x.Count.IsStructural)
                 || unbound.Any(u => u.AxisValue == Axis.Structural)
                 || recs.Any(r => r.Class is "structural-verification" or "local-verification"))
        {
            sTrace.Add(Step("structural/S7", "a structural comparison names this surface but no count survives the gates", recs, "axis", "attribution", "measurement_subject", "currency"));
            structural = new AxisState("S7", Labels.S7Label(why), null, null, null, null, sTrace);
        }
        else
        {
            sTrace.Add(Step("structural/S8", "no record in the sources listed under Sources carries a structural comparison naming this surface", Array.Empty<EvidenceRecord>(), "counts"));
            structural = new AxisState("S8", Labels.S8, null, null, null, null, sTrace);
        }

        // ================================================================ invariants
        if (residNum.Count > 0 && numeric.Label.StartsWith("matched", StringComparison.Ordinal))
            throw new InvalidOperationException($"INVARIANT: {id} numeric label begins \"matched\" while holding a residual numeric count.");
        if (residStr.Count > 0 && structural.Label.StartsWith("matched", StringComparison.Ordinal))
            throw new InvalidOperationException($"INVARIANT: {id} structural label begins \"matched\" while holding a residual structural count.");

        // ================================================================ flags
        var countedAny = countedProd.Count > 0;
        var perSurface = countedProd.Any(x => x.IsPerSurface);
        var flags = new Flags
        {
            HasAnyEvidenceRecord = recs.Count > 0,
            HasCountedRecord = countedAny,
            HasPerSurfaceCount = perSurface,
            HasNumericCountedRecord = countedProdNum.Count > 0,
            HasPerSurfaceNumericCount = countedProdNum.Any(x => x.IsPerSurface),
            HasStructuralCountedRecord = countedProdStr.Count > 0,
            OnlyCountedAxisIsStructural = countedProdStr.Count > 0 && countedProdNum.Count == 0,
            OnlyCountedEvidenceIsGroupTotal = countedAny && !perSurface,
            HasHeldOutCountedRecord = countedProd.Any(x => x.Count.IsHeldOut),
            CorpusWasRepairTarget = countedProd.Any(x => x.Count.CorpusWasRepairTarget == "true"),
            CpuNotStatedOnACountedRecord = countedProd.Any(x => x.Count.Cpu == "not-stated-in-this-record"),
            NonSingleBuildOnACountedRecord = countedProd.Any(x => x.Count.BuildAmbiguityValue != BuildAmbiguity.SingleBuild),
            CurrencyNotAnnotatedOnACountedRecord = countedProd.Any(x => x.Count.CurrencyValue == Currency.NotAnnotated),
            DroppedFromCountedByAttributionGate = !countedAny && bound.Any(x => x.Count.HasNumerator && x.Count.AttributionValue != Attribution.Measured),
            DroppedFromCountedByMeasurementSubjectGate = !countedAny && bound.Any(x => x.Count.HasNumerator && x.Count.AttributionValue == Attribution.Measured && x.Count.MeasurementSubject != Subject.Production),
            HasUnbindablePerSurfaceCount = unbound.Count > 0,
            UnbindablePerSurfaceCountRows = unbound.Count,
            HasHandbookRecomputationCount = countedRecomp.Count > 0,
            HasOpenDefectStreamRecord = streamRecs.Count > 0,
            NamedInADefectStreamFileAnyStatus = presenceStreams.Count > 0,
            HasOpenCatalogueRow = openRecs.Any(r => r.UpstreamRegister == "catalogue"),
            HasResidualRegisterRow = openRecs.Any(r => r.UpstreamRegister == "residual-register"),
            NumericCleanNoOpenRowNoStream = numeric.State == "N5" && openRecs.Count == 0
                                            && streamRecs.Count == 0 && presenceStreams.Count == 0,
            NumericCleanNoOpenRowNoOpenStreamRecord = numeric.State == "N5" && openRecs.Count == 0
                                            && streamRecs.Count == 0,
            NumericCleanNoOpenRow = numeric.State == "N5" && openRecs.Count == 0,
            OpenRowClassifierUsedFallback = usedFallback,
            ImplModuleAbsent = implAbsent,
            CappedByG1 = capped,
            HasBattery = BatteryIds.Contains(id),
            DeclaredArtifactsPresent = p?.DeclaredArtifactsPresent ?? false,
            LowNameMatchConfidence = p?.NameMatchConfidence == "low",
            InPigeonholeSet = pigeonhole.Contains(id),
        };

        // ================================================================ limitations
        var limitations = new List<string>();
        if (capped)
            limitations.Add("G-1: no implementation module is recorded for this entry in data/presence, so the warrant is capped at W0 whatever evidence names it.");
        if (unbound.Count > 0)
            limitations.Add($"{unbound.Count} per-surface count(s) in record(s) naming this surface cannot be bound to a surface by any machine-readable field. They contribute \"a comparison is on record\" and nothing stronger.");
        if (flags.CurrencyNotAnnotatedOnACountedRecord)
            limitations.Add("At least one count on this entry carries no currency annotation. Absence is not read as \"current\".");
        if (flags.CpuNotStatedOnACountedRecord)
            limitations.Add("The counted record does not state a host CPU. No CPU is inherited from any other record.");
        if (flags.NonSingleBuildOnACountedRecord)
            limitations.Add("The counted record does not carry a single restated Excel build on the scored line. No build is inherited.");
        if (flags.CorpusWasRepairTarget)
            limitations.Add("A counted corpus on this entry was the target of the repair it scores.");
        if (flags.HasHandbookRecomputationCount)
            limitations.Add("A count on this entry is a Handbook recomputation over a stored corpus, not a live-Excel measurement.");
        if (usedFallback)
            limitations.Add("The open-row classifier found no production-oxfunc count on at least one open row and fell back to all measurements on that row.");
        if ((numeric.Label.StartsWith("matched", StringComparison.Ordinal)
             || structural.Label.StartsWith("argument shape", StringComparison.Ordinal))
            && (streamRecs.Count > 0 || presenceStreams.Count > 0))
            limitations.Add("A matched-Excel state on this entry sits beside an OxFunc defect-stream file naming this surface. Both are true and both are scoped; they are usually about different axes or different corpora.");
        if (numeric.State == "N8" && structural.State == "S8")
            limitations.Add("Nothing on this entry has been compared to a running copy of Excel in the sources the Handbook has read.");
        limitations.Add("basis_mode pre-ledger: no claim has been recorded in a ledger, so this assignment carries a derivation trace rather than an assertion graph.");

        var iepValues = recs.Where(r => r.SubjectInternalEqualityPredicate.ContainsKey(id))
                            .Select(r => r.SubjectInternalEqualityPredicate[id])
                            .Distinct(StringComparer.Ordinal)
                            .OrderBy(x => x, StringComparer.Ordinal)
                            .ToList();
        var iep = iepValues.Count == 1 ? iepValues[0]
                : iepValues.Count > 1 ? "sources-disagree: " + string.Join(" / ", iepValues)
                : null;

        var order = AxisRank(numeric.State) <= AxisRank(structural.State)
            ? new[] { "numeric-bits", "structural-admission" }
            : new[] { "structural-admission", "numeric-bits" };

        return new Assignment
        {
            FunctionId = id,
            SurfaceName = f.SurfaceName,
            RubricVersion = RubricVersion,
            Warrant = warrant,
            WarrantLabel = warrantLabel,
            WarrantTrace = wTrace,
            Depth = depth,
            DepthLabel = depthLabel,
            DepthTrace = dTrace,
            Numeric = numeric,
            Structural = structural,
            AxisOrderWorstFirst = order,
            InternalEqualityPredicate = iep,
            InternalEqualityPredicateLabel = iep is null ? null : iep switch
            {
                "raw-ieee-equality" =>
                    "this surface compares numbers with raw IEEE equality; it is the exact control arm, outside quantise-then-compare",
                "15-significant-digit-truncation-then-exact-compare" =>
                    "this surface truncates each operand to 15 significant digits and then compares the truncated values exactly — quantise-then-compare: two values arbitrarily close but astride a bucket boundary compare unequal",
                _ => "this surface's internal number-comparison predicate is recorded as " + iep,
            },
            RecordsThatMustNotRenderAlone = recs.Where(r => r.MustNotRenderAlone)
                                                .Select(r => r.RecordId)
                                                .OrderBy(x => x, StringComparer.Ordinal).ToList(),
            CompanionRecords = recs.Where(r => r.MustNotRenderAlone)
                                   .SelectMany(r => r.RenderTogetherWith)
                                   .Distinct(StringComparer.Ordinal)
                                   .OrderBy(x => x, StringComparer.Ordinal).ToList(),
            WhyNoCount = warrant is "W3" || numeric.State == "N7" || structural.State == "S7" ? why : null,
            WhyNoCountAllApplicable = whyAll,
            EvidenceRecords = recs.Select(r => r.RecordId).OrderBy(x => x, StringComparer.Ordinal).ToList(),
            OpenDefectStreamRecords = streamRecs.Select(r => r.RecordId)
                                          .Distinct(StringComparer.Ordinal)
                                          .OrderBy(x => x, StringComparer.Ordinal).ToList(),
            DefectStreamFilesNamingThisSurface = presenceStreams
                                          .Distinct(StringComparer.Ordinal)
                                          .OrderBy(x => x, StringComparer.Ordinal).ToList(),
            Limitations = limitations,
            Flags = flags,
        };
    }

    // ==================================================================== helpers

    /// <summary>Worst-first ranking. Lower rank = worse = rendered first.</summary>
    private static int AxisRank(string state) => state switch
    {
        "N1" => 0, "N2" => 1, "N6" => 2, "S6" => 2,
        "N8" => 3, "S8" => 3,
        "N7" => 4, "S7" => 4,
        "NR" => 5, "SR" => 5,
        "N4" => 6, "N3" => 7,
        "N5" => 8, "S5" => 8,
        _ => 9,
    };

    private static bool IsOpenStream(EvidenceRecord r)
    {
        var s = (r.UpstreamStatusVerbatim ?? "").ToLowerInvariant();
        return s.Contains("open") || s.Contains("repair_landed_unsigned") || s.Contains("repair-landed-unsigned")
            || s.Contains("unsigned");
    }

    /// <summary>Per-surface counts shadow group counts from the same record on the same axis.</summary>
    private static List<BoundCount> ShadowGroupCounts(List<BoundCount> counts)
    {
        var shadowed = new HashSet<string>(StringComparer.Ordinal);
        foreach (var c in counts.Where(x => x.IsPerSurface))
            shadowed.Add(c.Record.RecordId + "|" + c.Count.AxisValue);
        return counts.Where(c => c.IsPerSurface || !shadowed.Contains(c.Record.RecordId + "|" + c.Count.AxisValue))
                     .ToList();
    }

    /// <summary>Deterministic choice of the count a label renders: per-surface first, then largest denominator, then ordinal key.</summary>
    private static BoundCount? Primary(IReadOnlyList<BoundCount> counts) =>
        counts.OrderByDescending(x => x.IsPerSurface)
              .ThenByDescending(x => x.Count.Total ?? 0)
              .ThenBy(x => x.Count.Key, StringComparer.Ordinal)
              .FirstOrDefault();

    private static DerivationStep Step(string rung, string rule, IEnumerable<BoundCount> counts, params string[] fields) =>
        new(rung, rule,
            counts.Select(c => c.Record.RecordId + "#" + c.Count.Index).Distinct(StringComparer.Ordinal)
                  .OrderBy(x => x, StringComparer.Ordinal).ToList(),
            fields);

    private static DerivationStep Step(string rung, string rule, IEnumerable<EvidenceRecord> recs, params string[] fields) =>
        new(rung, rule,
            recs.Select(r => r.RecordId).Distinct(StringComparer.Ordinal)
                .OrderBy(x => x, StringComparer.Ordinal).ToList(),
            fields);

    /// <summary>
    /// A1-F7 / the task's open-row clause. The classifier gates on measurement_subject ==
    /// production-oxfunc. Only when an open row publishes no production count at all does it fall
    /// back to every measurement on that row, and the fallback is recorded per row.
    /// </summary>
    private static (string? OpenClass, bool UsedFallback, List<DerivationStep> Trace)
        ClassifyOpenRows(string functionId, List<EvidenceRecord> openRecs, List<BoundCount> bound)
    {
        if (openRecs.Count == 0) return (null, false, new List<DerivationStep>());

        var trace = new List<DerivationStep>();
        var usedFallback = false;
        var anyMeasured = false;
        var anyFullPassOnly = false;

        foreach (var r in openRecs)
        {
            var rowCounts = bound.Where(x => x.Record.RecordId == r.RecordId && x.Count.UsableForState).ToList();
            var prodCounts = rowCounts.Where(x => x.Count.MeasurementSubject == Subject.Production).ToList();
            List<BoundCount> used;
            string basis;
            if (prodCounts.Count > 0) { used = prodCounts; basis = "production-oxfunc counts only"; }
            else if (rowCounts.Count > 0)
            {
                used = rowCounts; basis = "FALLBACK: this open row publishes no production-oxfunc count, so all measurements on it were used";
                usedFallback = true;
            }
            else { used = new List<BoundCount>(); basis = "this open row publishes no measurement bound to this surface"; }

            var measuredHere = used.Any(x => x.Count.IsResidual || x.Count.DivergenceMeasured);
            var fullPassHere = used.Count > 0 && used.All(x => x.Count.IsClean || x.Count.FullPassOnly);
            anyMeasured |= measuredHere;
            anyFullPassOnly |= fullPassHere && !measuredHere;

            trace.Add(new DerivationStep(
                "open-row/" + r.RecordId,
                $"open row {r.UpstreamRowId ?? "(no id)"} in register {r.UpstreamRegister}; classifier basis: {basis}; " +
                (measuredHere ? "a shortfall or divergence was measured on this row"
                              : fullPassHere ? "only full-pass counts on this row"
                              : "no measurement of this surface on this row"),
                new[] { r.RecordId },
                new[] { "class", "upstream_register", "measurement_subject", "passed", "total", "divergence_measured", "full_pass_only" }));
        }

        var cls = anyMeasured ? "measured" : anyFullPassOnly ? "no-non-match-measured" : "unmeasured";
        return (cls, usedFallback, trace);
    }

    private static List<string> WhyNoCount(
        List<BoundCount> namedOnly, List<BoundCount> bound,
        List<UnbindableCount> unbound, List<EvidenceRecord> openRecs, List<EvidenceRecord> recs)
    {
        var why = new List<string>();
        if (unbound.Count > 0) why.Add("per-surface-count-in-the-record-is-not-bound-to-a-surface-by-any-field");
        if (namedOnly.Any(x => x.Count.AttributionValue == Attribution.Disclaimed)) why.Add("count-disclaimed-by-source");
        if (namedOnly.Any(x => x.Count.AttributionValue == Attribution.ModelOrCandidate)) why.Add("count-is-a-model-or-candidate-score");
        if (namedOnly.Any(x => x.Count.AttributionValue == Attribution.AliasInherited)) why.Add("count-belongs-to-a-sibling-surface");
        if (namedOnly.Any(x => x.Count.AttributionValue == Attribution.NamedNotMeasured)) why.Add("named-in-a-group-record-not-itself-measured");
        if (bound.Any(x => x.Count.MeasurementSubject == Subject.HandbookRecomputation)) why.Add("count-is-a-handbook-recomputation-over-stored-bits-not-a-live-excel-run");
        if (bound.Any(x => !x.Count.HasNumerator)) why.Add("count-published-in-prose-not-extracted");
        if (bound.Count > 0 && bound.All(x => !x.Count.UsableForState)) why.Add("only-superseded-or-withdrawn-counts-on-record");
        if (openRecs.Count > 0) why.Add("open-row-with-no-count");
        if (why.Count == 0 && recs.Count > 0) why.Add("qualitative-verdict-only");
        return why;
    }
}
