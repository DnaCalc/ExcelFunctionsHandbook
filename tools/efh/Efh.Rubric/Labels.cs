namespace Efh.Rubric;

/// <summary>
/// Every rendered label. Three standing rules are enforced by construction here and asserted by
/// tests afterwards:
///   * no label on an axis holding a residual (passed &lt; total) count may begin with "matched";
///   * no structural-axis label contains "bit" or "exact" (G-8);
///   * every counted label carries a build clause, an arch clause and a cpu clause (G-7), plus
///     "covering {k} functions jointly" whenever the count that produced it is group-scoped (G-3a).
/// The words "bit-exact" and "bit-for-bit" appear in no Handbook-voice label (SD-3).
/// </summary>
public static class Labels
{
    public static string BuildClause(CountRow c)
    {
        if (c.Builds.Count > 0)
            return "Excel build " + string.Join(" and ", c.Builds);
        return c.BuildAmbiguityValue switch
        {
            BuildAmbiguity.NotRestated => "Excel build not restated on the scored line",
            BuildAmbiguity.NotStated => "Excel build not stated in this record",
            BuildAmbiguity.SourcesContradict => "Excel build: the sources contradict each other",
            BuildAmbiguity.TwoBuilds => "two Excel builds named, neither carried on the scored line",
            _ => "Excel build not carried in the machine-readable count",
        };
    }

    public static string ArchClause(CountRow c) =>
        c.Arch == "not-stated-in-this-record" ? "arch not stated in this record" : "arch " + c.Arch;

    public static string CpuClause(CountRow c) =>
        c.Cpu == "not-stated-in-this-record" ? "host CPU not restated in this record" : "host CPU " + c.Cpu;

    public static string Env(CountRow c) =>
        $"({BuildClause(c)}, {ArchClause(c)}, {CpuClause(c)})";

    public static string GroupClause(BoundCount b) =>
        b.Count.CountScope == "group" ? $", covering {b.GroupSize} functions jointly" : "";

    public static string CurrencyClause(CountRow c) =>
        c.CurrencyValue == Currency.NotAnnotated
            ? "; the record does not annotate whether this fig" + "ure is the current one"
            : "";

    public static string RepairClause(CountRow c) => c.CorpusWasRepairTarget switch
    {
        "true" => "; measured on the same corpus that exposed the defect",
        "source-does-not-state" => "; the source does not state whether this corpus was the target of the repair it scores",
        _ => "",
    };

    // ------------------------------------------------------------------ warrant

    public const string W0 = "registry name only — no implementation module located in the places searched";
    public const string W1 = "implementation module located; no test inside it";

    public static string W2Label(int n) =>
        $"implementation module located; it contains {n} tests; whether any of them exercises this function is not recorded";

    public static string W3Label(string whyNoCount) =>
        $"an Excel comparison is on record; no row count was extracted into the machine-readable record ({whyNoCount})";

    public static string W3RLabel(BoundCount b)
    {
        var c = b.Count;
        return $"a Handbook recomputation over a stored corpus scored {c.Passed} of {c.Total} rows{GroupClause(b)}; " +
               $"no live Excel was involved and no live-Excel row count is on record for this surface " +
               $"{Env(c)}{CurrencyClause(c)}{RepairClause(c)}";
    }

    /// <summary>
    /// The outcome always travels with the denominator. A warrant label that says "compared to Excel
    /// on 79 counted rows" without saying that 0 of them matched is the shape of overclaim this
    /// ladder exists to prevent, and warrant is not quality: GAMMA is W4 on 0 of 79.
    /// </summary>
    private static string Outcome(CountRow c) =>
        c.IsClean ? $", all {c.Passed} of which matched"
                  : $", of which {c.Passed} matched and {c.Total!.Value - c.Passed!.Value} did not";

    public static string W4Label(BoundCount b)
    {
        var c = b.Count;
        return $"compared to Excel on {c.Total} counted rows{GroupClause(b)}{Outcome(c)}, none held out " +
               $"({c.AxisValue}, predicate {c.ComparisonPredicate}, {BuildClause(c)}, {ArchClause(c)}, {CpuClause(c)})" +
               $"{CurrencyClause(c)}{RepairClause(c)}";
    }

    public static string W5Label(BoundCount b)
    {
        var c = b.Count;
        var held = c.HeldOutRows.HasValue
            ? $"of which {c.HeldOutRows.Value} were held out"
            : "of which the source does not split how many were held out";
        return $"compared to Excel on {c.Total} counted rows{GroupClause(b)}{Outcome(c)}, {held} " +
               $"({c.AxisValue}, predicate {c.ComparisonPredicate}, {BuildClause(c)}, {ArchClause(c)}, {CpuClause(c)})" +
               $"{CurrencyClause(c)}{RepairClause(c)}";
    }

    // ------------------------------------------------------------------ depth

    public const string D0 = "no implementation module located in the places searched";
    public const string D1 = "module located; no #[test] inside it";

    public static string D2Label(int siblings, int tests) =>
        $"family-module tests only — the module serves {siblings} other Handbook surfaces and holds {tests} tests in total; no per-function test count exists";

    public static string D3Label(int siblings, int tests) =>
        $"shared-module tests — the module serves {siblings} other Handbook surfaces and holds {tests} tests in total; no per-function test count exists";

    public static string D4Label(int tests) =>
        $"sole-occupant-module tests — {tests} tests in a module this function does not share";

    public const string D5 = "published Handbook vector suite";

    // ------------------------------------------------------------------ numeric axis

    public const string N1 = "OxFunc and Excel disagree numerically, the disagreement is open, and a shortfall was measured";
    public const string N2 = "named in an open divergence row; no measurement of this surface is published in it";

    public static string N3Label(BoundCount b)
    {
        var c = b.Count;
        return $"reproduces a documented Excel departure from exact mathematics on every one of {c.Total} counted rows" +
               $"{GroupClause(b)} {Env(c)}{CurrencyClause(c)}{RepairClause(c)}";
    }

    public const string N4 = "reproduces a documented Excel departure from exact mathematics; a witness exists, no row count was extracted";

    public static string N5Label(BoundCount b)
    {
        var c = b.Count;
        return $"matched Excel on every one of {c.Total} counted rows{GroupClause(b)} {Env(c)}" +
               $"{CurrencyClause(c)}{RepairClause(c)}";
    }

    public static string N6Label(BoundCount b)
    {
        var c = b.Count;
        var miss = c.Total!.Value - c.Passed!.Value;
        var resid = string.IsNullOrEmpty(c.ResidualAttribution)
            ? ""
            : $"; the residual is attributed to {c.ResidualAttribution}";
        return $"a measured shortfall: {miss} of {c.Total} counted rows did not match Excel{GroupClause(b)}{resid} " +
               $"{Env(c)}{CurrencyClause(c)}{RepairClause(c)}";
    }

    public static string NRLabel(BoundCount b)
    {
        var c = b.Count;
        return $"a Handbook recomputation over a stored corpus scored {c.Passed} of {c.Total} rows{GroupClause(b)}; " +
               $"no live Excel was involved, so this is not a statement that Excel agrees {Env(c)}" +
               $"{CurrencyClause(c)}{RepairClause(c)}";
    }

    public static string N7Label(string whyNoCount) =>
        $"a numeric comparison is on record; no row count was extracted into the machine-readable record ({whyNoCount})";

    public const string N8 = "no numeric comparison record in the six sources listed under Sources";

    // ------------------------------------------------------------------ structural axis

    public static string S5Label(BoundCount b)
    {
        var c = b.Count;
        return $"argument shape, coercion and error placement matched Excel on every one of {c.Total} counted rows" +
               $"{GroupClause(b)}; this says nothing about numeric results {Env(c)}" +
               $"{CurrencyClause(c)}{RepairClause(c)}";
    }

    public static string S6Label(BoundCount b)
    {
        var c = b.Count;
        var miss = c.Total!.Value - c.Passed!.Value;
        var resid = string.IsNullOrEmpty(c.ResidualAttribution)
            ? ""
            : $"; the residual belongs to {c.ResidualAttribution}";
        return $"a measured shortfall on argument shape, coercion and error placement: {miss} of {c.Total} " +
               $"counted rows did not match Excel{GroupClause(b)}{resid}; this says nothing about numeric results " +
               $"{Env(c)}{CurrencyClause(c)}{RepairClause(c)}";
    }

    public static string SRLabel(BoundCount b)
    {
        var c = b.Count;
        return $"a Handbook recomputation over a stored corpus scored {c.Passed} of {c.Total} structural rows" +
               $"{GroupClause(b)}; no live Excel was involved, so this is not a statement that Excel agrees " +
               $"{Env(c)}{CurrencyClause(c)}{RepairClause(c)}";
    }

    public static string S7Label(string whyNoCount) =>
        $"a structural comparison is on record; no row count was extracted into the machine-readable record ({whyNoCount})";

    public const string S8 = "no structural comparison record in the six sources listed under Sources";
}
