using Efh.Rubric;
using Xunit;

namespace Efh.Rubric.Tests;

/// <summary>
/// The new tests ATTRIBUTION-CORRECTIONS 7 implies, plus the T4(d) figure guard with the corrected
/// ATANH denominator.
/// </summary>
public class ReconciliationTests
{
    // ---------------------------------------------------------------- the three fabricated fractions

    [Theory]
    [InlineData("NPER", "1293")]
    [InlineData("NPER", "1,293")]
    [InlineData("XNPV", "1705")]
    [InlineData("XNPV", "1,705")]
    public void A_merged_numerator_that_exists_in_no_source_is_never_rendered(string surface, string forbidden)
    {
        // AC 5.3: 1293 = 1286 numeric + 7 typed-error rows; 1705 = 1530 numeric + 175 #NUM! rows.
        // Both merge a numeric-bits count with a structural typed-error count, and both were
        // composed downstream of the harvest. Neither appears in any OxFunc source.
        Must.NotContain(HandbookFixture.Page(surface), forbidden,
            $"{forbidden} merges a numeric count with a typed-error count and appears in no OxFunc source.");
    }

    [Fact]
    public void No_entry_anywhere_renders_either_merged_numerator()
    {
        var offenders = new List<string>();
        foreach (var a in HandbookFixture.All)
        {
            var page = Pipeline.RenderedPage(a);
            foreach (var f in new[] { "1293", "1,293", "1705", "1,705" })
                if (page.Contains(f, StringComparison.Ordinal)) offenders.Add($"{a.SurfaceName}: {f}");
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders));
    }

    [Fact]
    public void ATANH_carries_344_of_350_and_never_338_of_344()
    {
        // AC R1: 338 is a region subtotal; 344 is the source's numerator. The old build guard
        // hard-wired 338/344, which appears in FOUNDATION.md and nowhere else.
        var a = HandbookFixture.Get("ATANH");
        var page = HandbookFixture.Page("ATANH");

        Must.NotContain(page, "338", "AC R1: 338/344 is a composed fraction that exists in no OxFunc source.");

        if (a.Numeric.State is "N5" or "N6")
        {
            Assert.Equal(350, a.Numeric.Total);
            Assert.Equal(344, a.Numeric.Passed);
        }
    }

    // ---------------------------------------------------------------- held-out honesty

    [Fact]
    public void W5_never_places_a_bare_denominator_beside_the_words_held_out()
    {
        // A1-S8: "{n} counted rows, including a held-out corpus" reads as {n} held-out rows.
        var offenders = new List<string>();
        foreach (var a in HandbookFixture.All.Where(a => a.Warrant == "W5"))
        {
            Must.NotContain(a.WarrantLabel, "including a held-out corpus",
                $"{a.SurfaceName}: the phrase reads as if every counted row were held out.");

            var hasSplit = a.WarrantLabel.Contains("of which ", StringComparison.Ordinal)
                           && a.WarrantLabel.Contains("were held out", StringComparison.Ordinal);
            var saysUnsplit = a.WarrantLabel.Contains(
                "of which the source does not split how many were held out", StringComparison.Ordinal);
            if (!hasSplit && !saysUnsplit)
                offenders.Add($"{a.SurfaceName}: {a.WarrantLabel}");
        }
        Assert.True(offenders.Count == 0,
            "W5 labels must read \"{n} counted rows of which {h} were held out\", or say the source does not split:\n"
            + string.Join("\n", offenders));
    }

    [Fact]
    public void YIELDMAT_NPER_XNPV_do_not_fill_the_held_out_slot_the_source_does_not_split()
    {
        // AC R38: all three replay "discovery + held-out" corpora and none splits the count.
        foreach (var surface in new[] { "YIELDMAT", "NPER", "XNPV" })
        {
            var a = HandbookFixture.Get(surface);
            if (a.Warrant != "W5") continue;
            var engine = HandbookFixture.Pipeline.Engine;
            var bound = engine.BoundByFunction[a.FunctionId];
            foreach (var b in bound.Where(b => b.Count.IsHeldOut && b.Count.HeldOutRows is null))
            {
                Assert.Contains("does not split", a.WarrantLabel, StringComparison.Ordinal);
            }
        }
    }

    [Fact]
    public void ACCRINT_never_describes_its_whole_corpus_as_held_out()
    {
        // AC R37: 25,410 of the 145,620 rows are the b39 identification lattice.
        var a = HandbookFixture.Get("ACCRINT");
        var page = HandbookFixture.Page("ACCRINT");

        Must.NotContain(page, "145,620 were held out", "AC R37: 25,410 rows are the identification lattice.");
        Must.NotContain(page, "145620 were held out", "AC R37.");

        if (a.Warrant == "W5" && a.WarrantLabel.Contains("145620", StringComparison.Ordinal))
            Assert.True(a.WarrantLabel.Contains("does not split", StringComparison.Ordinal)
                        || a.WarrantLabel.Contains("of which 120200 were held out", StringComparison.Ordinal),
                "ACCRINT's held-out slot must be split or declared unsplit. Got: " + a.WarrantLabel);
    }

    // ---------------------------------------------------------------- group totals

    [Fact]
    public void The_S6_template_carries_the_joint_clause_exactly_as_S5_does()
    {
        // AC 2.1: as templated in FOUNDATION, 14 pages would render "structural check matched on 46
        // of 47 counted rows" -- a 97.9%-looking per-surface rate for "my one or two cases passed".
        var engine = HandbookFixture.Pipeline.Engine;
        var offenders = new List<string>();
        foreach (var a in HandbookFixture.All.Where(a => a.Structural.State == "S6"))
        {
            var key = a.Structural.PrimaryCountKey;
            if (key is null) continue;
            var b = engine.BoundByFunction[a.FunctionId].First(x => x.Count.Key == key);
            if (b.Count.CountScope != "group") continue;
            if (!a.Structural.Label.Contains("covering " + b.GroupSize + " functions jointly", StringComparison.Ordinal))
                offenders.Add($"{a.SurfaceName}: {a.Structural.Label}");
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders));
    }

    [Fact]
    public void No_group_total_is_ever_rendered_as_a_per_surface_rate()
    {
        var engine = HandbookFixture.Pipeline.Engine;
        var offenders = new List<string>();
        foreach (var a in HandbookFixture.All)
        {
            if (!a.Flags.OnlyCountedEvidenceIsGroupTotal) continue;
            if (a.Warrant is "W4" or "W5")
                if (!a.WarrantLabel.Contains("functions jointly", StringComparison.Ordinal))
                    offenders.Add($"{a.SurfaceName}: group-only counted warrant without the joint clause -> {a.WarrantLabel}");
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders.Take(20)));
    }

    // ---------------------------------------------------------------- GAMMA.DIST pdf/cdf

    [Fact]
    public void GAMMA_DIST_never_renders_the_cdf_figure_as_the_whole_surface()
    {
        // AC 2.1: 337/446 is the cdf mode; the pdf mode was measured at 16.1% on 4,750 rows.
        var a = HandbookFixture.Get("GAMMA.DIST");
        var page = HandbookFixture.Page("GAMMA.DIST");
        if (!page.Contains("337", StringComparison.Ordinal)) return;

        Assert.True(a.Numeric.State != "N5",
            "GAMMA.DIST may not render a clean numeric state while a pdf-mode shortfall stands.");
    }

    // ---------------------------------------------------------------- records that must render together

    [Fact]
    public void Every_record_marked_must_not_render_alone_names_its_companions()
    {
        var known = HandbookFixture.Pipeline.Engine.Records.Select(r => r.RecordId).ToHashSet(StringComparer.Ordinal);
        var offenders = new List<string>();
        foreach (var r in HandbookFixture.Pipeline.Engine.Records.Where(r => r.MustNotRenderAlone))
        {
            if (r.RenderTogetherWith.Count == 0)
                offenders.Add($"{r.RecordId}: must_not_render_alone with no companion");
            foreach (var c in r.RenderTogetherWith)
                if (!known.Contains(c)) offenders.Add($"{r.RecordId}: companion {c} does not exist");
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders));
    }

    [Fact]
    public void Every_entry_holding_a_must_not_render_alone_record_surfaces_its_companions()
    {
        foreach (var a in HandbookFixture.All.Where(a => a.RecordsThatMustNotRenderAlone.Count > 0))
            Assert.NotEmpty(a.CompanionRecords);
    }

    // ---------------------------------------------------------------- SD-3

    [Fact]
    public void The_words_bit_exact_and_bit_for_bit_appear_in_no_handbook_voice_label()
    {
        var offenders = new List<string>();
        foreach (var a in HandbookFixture.All)
            foreach (var label in Pipeline.RenderedLabels(a))
            {
                if (label.Contains("bit-exact", StringComparison.OrdinalIgnoreCase))
                    offenders.Add($"{a.SurfaceName}: bit-exact in \"{label}\"");
                if (label.Contains("bit-for-bit", StringComparison.OrdinalIgnoreCase))
                    offenders.Add($"{a.SurfaceName}: bit-for-bit in \"{label}\"");
                if (label.Contains("bit for bit", StringComparison.OrdinalIgnoreCase))
                    offenders.Add($"{a.SurfaceName}: bit for bit in \"{label}\"");
            }
        Assert.True(offenders.Count == 0,
            "SD-3 forbids these words in Handbook voice until vectors/ publishes:\n" + string.Join("\n", offenders.Take(20)));
    }

    // ---------------------------------------------------------------- SD-1

    [Fact]
    public void The_reconciliation_sentence_travels_with_every_published_share()
    {
        var path = Path.Combine(HandbookFixture.Root, "site", "api", "rubric.json");
        Assert.True(File.Exists(path), "run 'efh rubric emit' first");
        var text = File.ReadAllText(path);
        Assert.Contains(Counters.DenominatorSentence, text, StringComparison.Ordinal);

        var ratchets = Path.Combine(HandbookFixture.Root, "tests", "ratchets.baseline.json");
        Assert.True(File.Exists(ratchets));
        Assert.Contains(Counters.DenominatorSentence, File.ReadAllText(ratchets), StringComparison.Ordinal);
    }
}
