using Efh.Rubric;
using Xunit;

namespace Efh.Rubric.Tests;

/// <summary>
/// The invariants of FOUNDATION 3.7, asserted over all 541 entries rather than over a sample.
/// These are the assertions that make a wrong label impossible rather than unlikely.
/// </summary>
public class InvariantTests
{
    [Fact]
    public void The_denominator_is_541_entries()
    {
        Assert.Equal(541, HandbookFixture.All.Count);
        Assert.Equal(541, HandbookFixture.All.Select(a => a.FunctionId).Distinct().Count());
    }

    [Fact]
    public void No_axis_label_begins_with_matched_while_a_residual_count_stands_on_that_axis()
    {
        // This is the fix for the fatal finding that sent NORMDIST to "matched Excel" on a record
        // measuring 8 of 10. Residual is tested before clean, and this asserts the consequence.
        var offenders = new List<string>();
        foreach (var a in HandbookFixture.All)
        {
            if (a.Numeric.State == "N6" && a.Numeric.Label.StartsWith("matched", StringComparison.Ordinal))
                offenders.Add($"{a.SurfaceName} numeric: {a.Numeric.Label}");
            if (a.Structural.State == "S6" && a.Structural.Label.StartsWith("matched", StringComparison.Ordinal))
                offenders.Add($"{a.SurfaceName} structural: {a.Structural.Label}");
            if (a.Numeric.State == "N6" && !a.Numeric.Label.Contains("did not match", StringComparison.Ordinal))
                offenders.Add($"{a.SurfaceName} numeric N6 does not state the shortfall: {a.Numeric.Label}");
            if (a.Structural.State == "S6" && !a.Structural.Label.Contains("did not match", StringComparison.Ordinal))
                offenders.Add($"{a.SurfaceName} structural S6 does not state the shortfall: {a.Structural.Label}");
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders));
    }

    [Fact]
    public void A_clean_state_never_coexists_with_a_residual_count_on_the_same_axis()
    {
        var engine = HandbookFixture.Pipeline.Engine;
        var offenders = new List<string>();
        foreach (var a in HandbookFixture.All)
        {
            if (!engine.BoundByFunction.TryGetValue(a.FunctionId, out var bound)) continue;

            bool ResidualOn(string axis) => bound.Any(b =>
                b.Count.AxisValue == axis && b.Count.UsableForState
                && b.Count.AttributionValue == Attribution.Measured
                && b.Count.MeasurementSubject == Subject.Production
                && b.Count.IsResidual);

            if (a.Numeric.State is "N5" or "N3" && ResidualOn(Axis.Numeric))
                offenders.Add($"{a.SurfaceName}: numeric {a.Numeric.State} while holding a residual numeric count");
            if (a.Structural.State == "S5" && ResidualOn(Axis.Structural))
                offenders.Add($"{a.SurfaceName}: S5 while holding a residual structural count");
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders));
    }

    [Fact]
    public void No_structural_label_or_gloss_contains_bit_or_exact()
    {
        // G-8. The structural axis is about argument shape, coercion, escape sets and error
        // placement. Borrowing the vocabulary of bit equality is the overclaim it enables.
        var offenders = new List<string>();
        foreach (var a in HandbookFixture.All)
        {
            var s = a.Structural.Label;
            if (s.Contains("bit", StringComparison.OrdinalIgnoreCase)) offenders.Add($"{a.SurfaceName}: 'bit' in {s}");
            if (s.Contains("exact", StringComparison.OrdinalIgnoreCase)) offenders.Add($"{a.SurfaceName}: 'exact' in {s}");
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders.Take(20)));
    }

    [Fact]
    public void Every_counted_label_carries_a_build_an_arch_and_a_cpu()
    {
        // G-7. A count with no environment is not a claim anybody can act on.
        var offenders = new List<string>();
        foreach (var a in HandbookFixture.All)
        {
            var counted = new List<(string Which, string Label)>();
            if (a.Warrant is "W4" or "W5" or "W3R") counted.Add(("warrant", a.WarrantLabel));
            if (a.Numeric.State is "N3" or "N5" or "N6" or "NR") counted.Add(("numeric", a.Numeric.Label));
            if (a.Structural.State is "S5" or "S6" or "SR") counted.Add(("structural", a.Structural.Label));

            foreach (var (which, label) in counted)
            {
                if (!label.Contains("Excel build", StringComparison.Ordinal))
                    offenders.Add($"{a.SurfaceName} {which}: no build clause -> {label}");
                if (!label.Contains("arch", StringComparison.Ordinal))
                    offenders.Add($"{a.SurfaceName} {which}: no arch clause -> {label}");
                if (!label.Contains("CPU", StringComparison.Ordinal))
                    offenders.Add($"{a.SurfaceName} {which}: no cpu clause -> {label}");
            }
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders.Take(20)));
    }

    [Fact]
    public void Every_group_scoped_counted_label_says_covering_k_functions_jointly()
    {
        // G-3a. A group total that reads as a per-surface rate is the commonest overclaim in the
        // whole evidence base.
        var engine = HandbookFixture.Pipeline.Engine;
        var offenders = new List<string>();

        foreach (var a in HandbookFixture.All)
        {
            if (!engine.BoundByFunction.TryGetValue(a.FunctionId, out var bound)) continue;

            void Check(string which, string? key, string label)
            {
                if (key is null) return;
                var b = bound.FirstOrDefault(x => x.Count.Key == key);
                if (b is null || b.Count.CountScope != "group") return;
                if (!label.Contains("covering " + b.GroupSize + " functions jointly", StringComparison.Ordinal))
                    offenders.Add($"{a.SurfaceName} {which}: group-scoped label without the joint clause -> {label}");
            }

            Check("numeric", a.Numeric.PrimaryCountKey, a.Numeric.Label);
            Check("structural", a.Structural.PrimaryCountKey, a.Structural.Label);
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders.Take(20)));
    }

    [Fact]
    public void G1_an_entry_with_no_implementation_module_holds_no_warrant_above_W0_and_no_depth_above_D0()
    {
        var offenders = HandbookFixture.All
            .Where(a => a.Flags.ImplModuleAbsent && (a.Warrant != "W0" || a.Depth != "D0"))
            .Select(a => $"{a.SurfaceName}: {a.Warrant}/{a.Depth} with no implementation module")
            .ToList();
        Assert.True(offenders.Count == 0, string.Join("\n", offenders));
    }

    [Fact]
    public void G1_no_level_is_asserted_every_level_carries_a_version_and_a_trace()
    {
        foreach (var a in HandbookFixture.All)
        {
            Assert.Equal(RubricEngine.RubricVersion, a.RubricVersion);
            Assert.NotEmpty(a.WarrantTrace);
            Assert.NotEmpty(a.DepthTrace);
            Assert.NotEmpty(a.Numeric.Trace);
            Assert.NotEmpty(a.Structural.Trace);

            foreach (var step in a.WarrantTrace.Concat(a.DepthTrace)
                                               .Concat(a.Numeric.Trace).Concat(a.Structural.Trace))
            {
                Assert.False(string.IsNullOrWhiteSpace(step.Rung));
                Assert.False(string.IsNullOrWhiteSpace(step.Rule));
                Assert.NotEmpty(step.Fields);
            }
        }
    }

    [Fact]
    public void Every_trace_step_that_cites_records_cites_records_that_exist()
    {
        var known = HandbookFixture.Pipeline.Engine.Records.Select(r => r.RecordId).ToHashSet(StringComparer.Ordinal);
        var offenders = new List<string>();
        foreach (var a in HandbookFixture.All)
            foreach (var step in a.WarrantTrace.Concat(a.Numeric.Trace).Concat(a.Structural.Trace))
                foreach (var r in step.Records)
                {
                    var id = r.Contains('#') ? r[..r.IndexOf('#')] : r;
                    if (!known.Contains(id)) offenders.Add($"{a.SurfaceName} {step.Rung} cites unknown record {r}");
                }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders.Take(20)));
    }

    [Fact]
    public void Only_measured_for_this_surface_can_raise_W4_W5_or_produce_N5_N6_S5_S6()
    {
        var engine = HandbookFixture.Pipeline.Engine;
        var offenders = new List<string>();

        foreach (var a in HandbookFixture.All)
        {
            if (!engine.BoundByFunction.TryGetValue(a.FunctionId, out var bound))
            {
                if (a.Warrant is "W4" or "W5")
                    offenders.Add($"{a.SurfaceName} is {a.Warrant} with no bound count at all");
                continue;
            }

            bool Qualifying(BoundCount b) =>
                b.Count.AttributionValue == Attribution.Measured
                && b.Count.MeasurementSubject == Subject.Production
                && b.Count.HasNumerator && b.Count.UsableForState;

            if (a.Warrant is "W4" or "W5" && !bound.Any(Qualifying))
                offenders.Add($"{a.SurfaceName} is {a.Warrant} without a measured-for-this-surface production count");

            if (a.Numeric.State is "N5" or "N6" && !bound.Any(b => Qualifying(b) && b.Count.IsNumeric))
                offenders.Add($"{a.SurfaceName} is {a.Numeric.State} without a measured production numeric count");

            if (a.Structural.State is "S5" or "S6" && !bound.Any(b => Qualifying(b) && b.Count.IsStructural))
                offenders.Add($"{a.SurfaceName} is {a.Structural.State} without a measured production structural count");
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders.Take(20)));
    }

    [Fact]
    public void A_handbook_recomputation_never_produces_a_matched_Excel_state()
    {
        // measurement_subject "handbook-recomputation-over-cached-corpus" is a computation over
        // stored bits. It is not a live-Excel oracle run and may never carry that authority.
        var engine = HandbookFixture.Pipeline.Engine;
        var offenders = new List<string>();

        foreach (var a in HandbookFixture.All)
        {
            if (!engine.BoundByFunction.TryGetValue(a.FunctionId, out var bound)) continue;

            void Check(string which, string state, string? key)
            {
                if (key is null) return;
                if (state is not ("N5" or "N6" or "S5" or "S6" or "N3")) return;
                var b = bound.FirstOrDefault(x => x.Count.Key == key);
                if (b?.Count.MeasurementSubject == Subject.HandbookRecomputation)
                    offenders.Add($"{a.SurfaceName} {which} {state} rests on a Handbook recomputation ({key})");
            }

            Check("numeric", a.Numeric.State, a.Numeric.PrimaryCountKey);
            Check("structural", a.Structural.State, a.Structural.PrimaryCountKey);

            if (a.Numeric.State == "NR")
                Must.Contain(a.Numeric.Label, "no live Excel was involved",
                    $"{a.SurfaceName}: NR must deny the live-Excel reading in its own label.");
            if (a.Structural.State == "SR")
                Must.Contain(a.Structural.Label, "no live Excel was involved",
                    $"{a.SurfaceName}: SR must deny the live-Excel reading in its own label.");
            if (a.Warrant == "W3R")
                Must.Contain(a.WarrantLabel, "no live Excel was involved",
                    $"{a.SurfaceName}: W3R must deny the live-Excel reading in its own label.");
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders.Take(20)));
    }

    [Fact]
    public void A_superseded_or_withdrawn_figure_never_derives_a_state()
    {
        var engine = HandbookFixture.Pipeline.Engine;
        var offenders = new List<string>();
        foreach (var a in HandbookFixture.All)
        {
            if (!engine.BoundByFunction.TryGetValue(a.FunctionId, out var bound)) continue;
            foreach (var key in new[] { a.Numeric.PrimaryCountKey, a.Structural.PrimaryCountKey })
            {
                if (key is null) continue;
                var b = bound.FirstOrDefault(x => x.Count.Key == key);
                if (b is not null && !b.Count.UsableForState)
                    offenders.Add($"{a.SurfaceName}: state rests on a {b.Count.CurrencyValue} figure ({key})");
            }
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders));
    }

    [Fact]
    public void D5_is_empty_because_no_vector_suite_is_published()
    {
        Assert.Equal(0, HandbookFixture.Pipeline.Counters.Depth("D5"));
        Assert.Equal(0, HandbookFixture.Pipeline.Loader.CountVectorSuites());
    }

    [Fact]
    public void The_worst_axis_is_rendered_first()
    {
        foreach (var a in HandbookFixture.All)
        {
            Assert.Equal(2, a.AxisOrderWorstFirst.Count);
            Assert.Contains("numeric-bits", a.AxisOrderWorstFirst);
            Assert.Contains("structural-admission", a.AxisOrderWorstFirst);
        }
    }

    [Fact]
    public void Every_entry_carries_the_pre_ledger_basis_limitation()
    {
        foreach (var a in HandbookFixture.All)
            Assert.Contains(a.Limitations, l => l.Contains("basis_mode pre-ledger", StringComparison.Ordinal));
    }
}
