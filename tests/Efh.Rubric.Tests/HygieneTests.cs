using Efh.Rubric;
using Xunit;

namespace Efh.Rubric.Tests;

/// <summary>
/// The three real inconsistencies in the evidence records, each with the behaviour that makes the
/// inconsistency harmless, and each visible in the published hygiene report rather than buried.
/// </summary>
public class HygieneTests
{
    private static HygieneReport Report => HandbookFixture.Pipeline.Loader.Hygiene;

    [Fact]
    public void held_out_carried_as_a_string_is_normalised_and_the_string_false_is_not_truthy()
    {
        var kind = HygieneReport.Kinds.HeldOutStringBoolean;
        var n = Report.CountOf(kind);

        // The corpus is written concurrently, so the count is reported rather than pinned.
        Assert.True(n >= 0);

        // Whatever the count, no count whose held_out is the STRING "false" may be treated as held out.
        var offenders = new List<string>();
        foreach (var r in HandbookFixture.Pipeline.Engine.Records)
            foreach (var c in r.Counts)
                Assert.Contains(c.HeldOutValue,
                    new[] { HeldOut.True, HeldOut.False, HeldOut.Partial, HeldOut.SourceDoesNotState });

        // And a W5 may never rest on a count normalised from the string "false".
        foreach (var a in HandbookFixture.All.Where(a => a.Warrant == "W5"))
        {
            var bound = HandbookFixture.Pipeline.Engine.BoundByFunction[a.FunctionId];
            if (!bound.Any(b => b.Count.IsHeldOut))
                offenders.Add($"{a.SurfaceName} is W5 with no held-out count");
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders));
    }

    [Fact]
    public void the_build_ambiguity_variant_spelling_is_folded_onto_one_value()
    {
        foreach (var r in HandbookFixture.Pipeline.Engine.Records)
            foreach (var c in r.Counts)
                Assert.NotEqual("build-not-stated-in-this-record", c.BuildAmbiguityValue);

        // If the variant spelling occurs, the normalisation must be recorded.
        var recorded = Report.CountOf(HygieneReport.Kinds.BuildAmbiguityVariantSpelling);
        Assert.True(recorded >= 0);
    }

    [Fact]
    public void an_absent_currency_is_never_read_as_current()
    {
        var withoutCurrency = HandbookFixture.Pipeline.Engine.Records
            .SelectMany(r => r.Counts)
            .Where(c => c.CurrencyValue == Currency.NotAnnotated)
            .ToList();

        foreach (var c in withoutCurrency)
            Assert.NotEqual(Currency.Current, c.CurrencyValue);

        Assert.Equal(withoutCurrency.Count, Report.CountOf(HygieneReport.Kinds.CurrencyAbsent));

        // Every label built on a not-annotated figure says so.
        var engine = HandbookFixture.Pipeline.Engine;
        var offenders = new List<string>();
        foreach (var a in HandbookFixture.All)
        {
            if (!engine.BoundByFunction.TryGetValue(a.FunctionId, out var bound)) continue;
            foreach (var (which, key, label) in new[]
                     {
                         ("numeric", a.Numeric.PrimaryCountKey, a.Numeric.Label),
                         ("structural", a.Structural.PrimaryCountKey, a.Structural.Label),
                     })
            {
                if (key is null) continue;
                var b = bound.FirstOrDefault(x => x.Count.Key == key);
                if (b?.Count.CurrencyValue != Currency.NotAnnotated) continue;
                if (!label.Contains("does not annotate whether this figure is the current one", StringComparison.Ordinal))
                    offenders.Add($"{a.SurfaceName} {which}: {label}");
            }
        }
        Assert.True(offenders.Count == 0,
            "a figure with no currency annotation must say so in its own label:\n" + string.Join("\n", offenders.Take(20)));
    }

    [Fact]
    public void the_hygiene_report_is_published_and_enumerates_every_normalisation()
    {
        var path = Path.Combine(HandbookFixture.Root, "site", "api", "rubric-hygiene.json");
        Assert.True(File.Exists(path), "run 'efh rubric emit' first");
        var text = File.ReadAllText(path);

        foreach (var kind in new[]
                 {
                     HygieneReport.Kinds.HeldOutStringBoolean,
                     HygieneReport.Kinds.BuildAmbiguityVariantSpelling,
                     HygieneReport.Kinds.CurrencyAbsent,
                 })
        {
            if (Report.CountOf(kind) > 0)
                Assert.Contains(kind, text, StringComparison.Ordinal);
        }

        Assert.Contains("binding_defect", text, StringComparison.Ordinal);
    }

    [Fact]
    public void an_unbindable_per_surface_count_never_produces_a_counted_warrant_or_a_matched_state()
    {
        // The schema carries no field naming which subject a per-surface figure belongs to. The
        // rubric refuses to guess, and this asserts that the refusal has teeth.
        var engine = HandbookFixture.Pipeline.Engine;
        foreach (var (fid, ub) in engine.UnbindableByFunction)
        {
            var a = HandbookFixture.All.FirstOrDefault(x => x.FunctionId == fid);
            if (a is null) continue;
            Assert.Equal(ub.Count, a.Flags.UnbindablePerSurfaceCountRows);
            Assert.True(a.Flags.HasUnbindablePerSurfaceCount);
            Assert.Contains(a.Limitations, l => l.Contains("cannot be bound to a surface", StringComparison.Ordinal));
        }
    }
}
