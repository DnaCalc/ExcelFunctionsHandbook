using Efh.Rubric;
using Xunit;

namespace Efh.Rubric.Tests;

/// <summary>
/// The 24 overclaim tests of FOUNDATION 3.8, with the six corrections of
/// ATTRIBUTION-CORRECTIONS 7 applied (OT-4, OT-6, OT-13, OT-15 inverted, OT-16, T4(d)).
///
/// Two kinds of assertion appear here and they are not equal in strength:
///   * FORBIDDEN-STRING assertions run unconditionally. They are meaningful whatever the evidence
///     layer contains, because they say what may never be rendered.
///   * REQUIRED-RENDERING assertions depend on an evidence record existing. content/evidence/records
///     is written by a separate concurrent process, so where the record is not yet on disk the test
///     asserts the absence-correct state instead and says so in its message. Reduced-mode tests are
///     enumerated by <see cref="PreconditionReportTests"/> so none of them passes silently.
/// </summary>
public class OverclaimTests
{
    private static readonly string[] CompletenessWords =
        { "verified", "validated", "complete", "bit-exact", "bit-for-bit", "fully tested", "proven correct" };

    // ---------------------------------------------------------------- OT-1 GCD

    [Fact]
    public void OT01_GCD_renders_a_structural_shortfall_and_no_numeric_claim()
    {
        var a = HandbookFixture.Get("GCD");
        var s = a.Structural.Label;

        // G-8: a structural label may never use the vocabulary of bit equality.
        Must.NotContain(s, "bit", "G-8 forbids 'bit' in any structural-axis label.");
        Must.NotContain(s, "exact", "G-8 forbids 'exact' in any structural-axis label.");
        Must.NotContain(s, "verified", "'verified' is a rejected completeness word.");

        Assert.Equal("N8", a.Numeric.State);
        Assert.Equal("no numeric comparison record in the six sources listed under Sources", a.Numeric.Label);

        if (a.Structural.State == "S6")
        {
            Must.Contain(s, "did not match Excel", "S6 must state the shortfall, not a match.");
            Assert.False(s.StartsWith("matched", StringComparison.Ordinal));
        }
        else
        {
            Assert.Equal("S8", a.Structural.State);
        }
    }

    // ---------------------------------------------------------------- OT-2 SLOPE

    [Fact]
    public void OT02_SLOPE_never_borrows_the_group_73_and_never_claims_to_be_tested()
    {
        var a = HandbookFixture.Get("SLOPE");
        var page = HandbookFixture.Page("SLOPE");

        Must.NotContain(page, "73", "record 13's group total of 73 may never reach SLOPE's page.");
        Must.NotContain(page, "unit-tested", "SLOPE's module contains zero tests.");
        Must.NotContain(page, "test coverage", "no per-function test count exists in OxFunc, for anyone.");

        // SLOPE's module holds zero tests, so the depth tier is D1 whatever the evidence says.
        Assert.Equal("D1", a.Depth);
        Must.Contain(a.DepthLabel, "no #[test] inside it", "D1 must say the module holds no test.");

        // The 4/4 lives in a two-subject record that marks BOTH subjects 'header', so no field binds
        // the figure to SLOPE rather than INTERCEPT. It may not become a counted warrant.
        if (a.Flags.HasCountedRecord)
            Assert.True(a.Flags.HasPerSurfaceCount,
                "a counted warrant on SLOPE must rest on a per-surface count, never on a group total.");
        else
            Assert.Equal("W3", a.Warrant);
    }

    // ---------------------------------------------------------------- OT-3 GAMMA

    [Fact]
    public void OT03_GAMMA_is_W4_on_a_measured_zero_and_never_says_matched()
    {
        var a = HandbookFixture.Get("GAMMA");
        var page = HandbookFixture.Page("GAMMA");

        Must.NotContain(a.Numeric.Label, "matched Excel", "GAMMA matched nothing.");
        Must.NotContain(page, "verified", "'verified' is a rejected completeness word.");

        if (!HandbookFixture.HasEvidence("GAMMA"))
        {
            Assert.Equal("N8", a.Numeric.State);
            return;
        }

        Assert.Equal("N1", a.Numeric.State);
        // Warrant is not quality: a W4 on a 0-of-79 measurement is the point of the ladder.
        if (a.Warrant is "W4" or "W5")
            Must.Contain(a.WarrantLabel, "of which 0 matched",
                "a counted warrant label must carry the outcome, not only the denominator.");
    }

    // ---------------------------------------------------------------- OT-4 NORMDIST (RETARGETED, AC 7)

    [Fact]
    public void OT04_NORMDIST_is_not_given_record_34s_eight_of_ten_and_does_not_claim_two_builds()
    {
        var a = HandbookFixture.Get("NORMDIST");

        // AC R8-R11: record 34's ten witnesses are NORM.DIST, NORM.INV, NORMSDIST, NORMSINV,
        // NORM.S.DIST, NORM.S.INV, ERF(1), ERFC(1), GAUSS(1), PHI(0). NORMDIST is not among them.
        // AC R13: the 8/10 was measured on one build; "both builds" must not be required.
        Assert.NotEqual("N6", a.Numeric.State);
        Must.NotContain(a.Numeric.Label, "8 of 10", "the 8-of-10 is not NORMDIST's measurement.");
        Must.NotContain(a.Numeric.Label, "two Excel builds", "the numeric 8-of-10 was taken on one build.");

        // The worst axis renders first, whatever the two states are.
        var worst = a.AxisOrderWorstFirst[0];
        Assert.Contains(worst, new[] { "numeric-bits", "structural-admission" });
        if (a.Numeric.State == "N8" && a.Structural.State == "S5")
            Assert.Equal("numeric-bits", worst);
    }

    // ---------------------------------------------------------------- OT-5 PMT

    [Fact]
    public void OT05_PMT_publishes_the_in_sample_count_with_its_flags_and_never_as_a_clean_pass()
    {
        var a = HandbookFixture.Get("PMT");
        var page = HandbookFixture.Page("PMT");

        Must.NotContain(page, "165/234", "not a PMT pass rate.");
        Must.NotContain(page, "3,840/3,840", "may never render unqualified.");
        Must.NotContain(a.Numeric.Label, "matched Excel", "PMT holds an open measured divergence.");

        if (!HandbookFixture.HasEvidence("PMT")) return;

        Assert.Equal("N1", a.Numeric.State);

        // SD-5: the in-sample count publishes WITH its flags, never bare.
        if (a.Warrant is "W4" or "W5")
        {
            Assert.True(a.Flags.CorpusWasRepairTarget,
                "PMT's 2304/2304 combsweep is scored on the corpus that exposed the defect; the flag must be set.");
            Must.Contain(a.WarrantLabel, "the same corpus that exposed the defect",
                "SD-5: PMT's in-sample count publishes with its flags.");
            Assert.Contains(a.Limitations,
                l => l.Contains("target of the repair it scores", StringComparison.Ordinal));
        }
    }

    // ---------------------------------------------------------------- OT-6 COUPDAYSNC (RETARGETED, AC 7)

    [Fact]
    public void OT06_the_458_guard_sits_on_COUPDAYSNC_and_COUPDAYS_keeps_a_group_reason()
    {
        var nc = HandbookFixture.Get("COUPDAYSNC");
        var d = HandbookFixture.Get("COUPDAYS");

        // AC: COUPDAYS has no 458 figure at all, so the guard belonged on COUPDAYSNC.
        Must.NotContain(HandbookFixture.Page("COUPDAYS"), "458",
            "COUPDAYS has no such figure; rendering one would invent it.");

        // Where COUPDAYSNC does carry the 458, it is an Excel-vs-Excel divergence count and may
        // never render as a pass rate.
        var ncPage = HandbookFixture.Page("COUPDAYSNC");
        if (ncPage.Contains("458", StringComparison.Ordinal))
            Must.NotContain(nc.Numeric.Label, "matched Excel on every one of 458",
                "COUPDAYSNC's 458 measures a divergence between two Excel publications, not a pass rate.");

        Assert.NotEqual("W4", d.Warrant);
        Assert.NotEqual("W5", d.Warrant);
        if (HandbookFixture.HasEvidence("COUPDAYS"))
            Assert.Equal("W3", d.Warrant);
    }

    // ---------------------------------------------------------------- OT-7 DCOUNT

    [Fact]
    public void OT07_DCOUNT_never_borrows_the_sixteen_of_sixteen()
    {
        var a = HandbookFixture.Get("DCOUNT");
        var page = HandbookFixture.Page("DCOUNT");

        Must.NotContain(page, "16/16", "the 16-lane total is not DCOUNT's evidence.");
        Must.NotContain(a.Numeric.Label, "matched Excel", "DCOUNT's evidence is unit tests only.");
        Must.NotContain(a.Structural.Label, "matched Excel on every one",
            "'pass their lib tests only' is not an Excel comparison.");

        Assert.NotEqual("W4", a.Warrant);
        Assert.NotEqual("W5", a.Warrant);
        if (HandbookFixture.HasEvidence("DCOUNT"))
            Assert.True(a.Structural.State is "S7" or "S8",
                $"DCOUNT's structural state was {a.Structural.State}; a disclaimed record may not produce S5/S6.");
    }

    // ---------------------------------------------------------------- OT-8 ERF.PRECISE

    [Fact]
    public void OT08_ERF_PRECISE_never_publishes_a_research_model_score_as_a_pass_rate()
    {
        var a = HandbookFixture.Get("ERF.PRECISE");

        Assert.NotEqual("W4", a.Warrant);
        Assert.NotEqual("W5", a.Warrant);
        Assert.True(a.Numeric.State is not ("N5" or "N6"),
            "a model-or-candidate-score may never produce a matched-Excel numeric state.");
        Must.NotContain(a.Numeric.Label, "663", "663/1218 is a research model's score, not OxFunc's.");

        if (HandbookFixture.HasEvidence("ERF.PRECISE") && a.WhyNoCount is not null)
            Assert.Contains(a.WhyNoCountAllApplicable, w => w.Contains("model-or-candidate", StringComparison.Ordinal));
    }

    // ---------------------------------------------------------------- OT-9 IMCOS

    [Fact]
    public void OT09_IMCOS_publishes_the_pigeonhole_and_never_the_word_tested()
    {
        var a = HandbookFixture.Get("IMCOS");
        var page = HandbookFixture.Page("IMCOS");

        Must.NotContain(page, "unit-tested", "no per-function test count exists.");
        Must.NotContain(page, "test coverage", "no per-function test count exists.");

        Assert.Equal("W2", a.Warrant);
        Must.Contain(a.WarrantLabel, "whether any of them exercises this function is not recorded",
            "W2 must deny the per-function reading in its own label.");
        Assert.True(a.Flags.InPigeonholeSet,
            "IMCOS sits in complex_family.rs, which has fewer #[test] functions than ids mapped to it.");
    }

    // ---------------------------------------------------------------- OT-10 STDEV

    [Fact]
    public void OT10_STDEV_is_W1_D1_and_claims_nothing()
    {
        var a = HandbookFixture.Get("STDEV");
        var page = HandbookFixture.Page("STDEV");

        Must.NotContain(page, "unit-tested", "STDEV's module holds zero tests.");
        Must.NotContain(page, "verified", "'verified' is a rejected completeness word.");

        Assert.Equal("D1", a.Depth);
        if (!HandbookFixture.HasEvidence("STDEV")) Assert.Equal("W1", a.Warrant);
    }

    // ---------------------------------------------------------------- OT-11 LAMBDA

    [Fact]
    public void OT11_LAMBDA_is_W0_D0_and_is_not_called_not_implemented()
    {
        var a = HandbookFixture.Get("LAMBDA");
        var page = HandbookFixture.Page("LAMBDA");

        Must.NotContain(page, "not implemented",
            "LAMBDA is export-only-formula-layer; W0 means no module was found where the scan looked.");

        Assert.Equal("W0", a.Warrant);
        Assert.Equal("D0", a.Depth);
        Must.Contain(a.WarrantLabel, "in the places searched",
            "W0 must scope its absence to where the scan looked.");
    }

    // ---------------------------------------------------------------- OT-12 HARMEAN

    [Fact]
    public void OT12_HARMEAN_renders_both_absences_precisely()
    {
        var a = HandbookFixture.Get("HARMEAN");

        Assert.Equal("N8", a.Numeric.State);
        Assert.Equal("S8", a.Structural.State);
        Must.Contain(a.Numeric.Label, "in the six sources listed under Sources",
            "the N8 clause must be narrow: it is an absence in named sources, not an absence in the world.");
        Assert.Contains(a.Limitations,
            l => l.Contains("running copy of Excel", StringComparison.Ordinal));
    }

    // ---------------------------------------------------------------- OT-13 POWER (RETARGETED, AC 7)

    [Fact]
    public void OT13_POWER_does_not_claim_four_hundred_cleanly_held_out_rows()
    {
        var a = HandbookFixture.Get("POWER");
        var page = HandbookFixture.Page("POWER");

        // AC R25: neither half of the 715 is cleanly held out; the 400-row sweep is where the second
        // correction was found. The old test required "715 counted rows of which 400 were held out".
        Must.NotContain(page, "of which 400 were held out",
            "AC R25: the 400-row sweep is the corpus fix (b) was discovered on, not a held-out gate.");

        if (a.Warrant is "W4" or "W5")
        {
            foreach (var l in new[] { a.WarrantLabel })
            {
                Must.Contain(l, "Excel build", "G-7: every counted label carries a build clause.");
                Must.Contain(l, "arch", "G-7: every counted label carries an arch clause.");
                Must.Contain(l, "CPU", "G-7: every counted label carries a cpu clause.");
            }
        }
    }

    // ---------------------------------------------------------------- OT-14 OP_MULTIPLY

    [Fact]
    public void OT14_OP_MULTIPLY_makes_no_numeric_claim_from_a_structural_count()
    {
        var a = HandbookFixture.Get("OP_MULTIPLY");
        var s = a.Structural.Label;

        Must.NotContain(s, "bit", "G-8.");
        Must.NotContain(s, "exact", "G-8.");
        Must.NotContain(s, "binary64", "a structural count says nothing about number formats.");
        Assert.Equal("N8", a.Numeric.State);

        if (a.Structural.State is "S5" or "S6")
            Must.Contain(s, "says nothing about numeric results",
                "the structural label must deny the numeric reading in its own text.");
    }

    // ---------------------------------------------------------------- OT-15 MATCH (INVERTED, AC 7)

    [Fact]
    public void OT15_MATCH_XMATCH_DELTA_are_the_exact_match_control_group()
    {
        // AC R22-R24 and BUG-FUNC-004: these three use raw IEEE equality and are the deliberately
        // EXCLUDED control arm. FOUNDATION's OT-15 required the opposite and is inverted here.
        foreach (var surface in new[] { "MATCH", "XMATCH", "DELTA" })
        {
            var a = HandbookFixture.Get(surface);
            if (a.InternalEqualityPredicate is null)
            {
                Assert.False(HandbookFixture.HasEvidence(surface) && a.Structural.State == "S5",
                    $"{surface} reached S5 without its internal comparison predicate being recorded; " +
                    "the control-arm fact would be lost.");
                continue;
            }
            Assert.Equal("raw-ieee-equality", a.InternalEqualityPredicate);
            Must.Contain(a.InternalEqualityPredicateLabel!, "raw IEEE equality",
                $"{surface} is the control arm and its page must say so.");
            Must.NotContain(a.InternalEqualityPredicateLabel!, "tolerant",
                "'tolerant' is the wrong word for any of these six surfaces.");
        }

        // The quantise-then-compare predicate belongs to the tolerant arm.
        foreach (var surface in new[] { "SWITCH", "COUNTIF", "SUMIF" })
        {
            var a = HandbookFixture.Get(surface);
            if (a.InternalEqualityPredicate is null) continue;
            Assert.Equal("15-significant-digit-truncation-then-exact-compare", a.InternalEqualityPredicate);
            Must.Contain(a.InternalEqualityPredicateLabel!, "quantise-then-compare",
                "AC: 'tolerance' is the wrong word; the helper truncates and then compares exactly.");
            Must.NotContain(a.InternalEqualityPredicateLabel!, "tolerance",
                "AC: values arbitrarily close but astride a bucket boundary compare unequal.");
        }
    }

    // ---------------------------------------------------------------- OT-16 MMULT (RELAXED, AC 7)

    [Fact]
    public void OT16_MMULT_never_says_no_record_names_this_function()
    {
        var a = HandbookFixture.Get("MMULT");
        var page = HandbookFixture.Page("MMULT");

        // AC: the old expected answer "no record names this function" overshoots. MMULT and MDETERM
        // carry provisional native-COM structural probes.
        Must.NotContain(page, "no record names this function",
            "AC OT-16: relaxed to 'no counted record and no numeric record'.");
        Must.NotContain(page, "no divergence record", "AC OT-16.");

        if (HandbookFixture.HasEvidence("MMULT"))
            Assert.True(a.Structural.State is "S7" or "S6" or "S5",
                $"MMULT's structural state was {a.Structural.State}; a structural record on file may not render S8.");
    }

    // ---------------------------------------------------------------- OT-17 NPER

    [Fact]
    public void OT17_NPER_renders_its_clean_numeric_state_beside_its_streams()
    {
        var a = HandbookFixture.Get("NPER");
        if (a.Numeric.State != "N5") return;

        var namedInAStream = a.OpenDefectStreamRecords.Count > 0
                             || a.DefectStreamFilesNamingThisSurface.Count > 0;
        if (namedInAStream)
            Assert.Contains(a.Limitations,
                l => l.Contains("defect-stream file naming this surface", StringComparison.Ordinal));
    }

    // ---------------------------------------------------------------- OT-18 GAMMADIST

    [Fact]
    public void OT18_GAMMADIST_never_says_it_agreed_with_Excel()
    {
        var a = HandbookFixture.Get("GAMMADIST");
        var page = HandbookFixture.Page("GAMMADIST");

        Must.NotContain(page, "compared with Excel and agreed", "GAMMADIST sits on a residual-register row.");
        Must.NotContain(a.Numeric.Label, "matched Excel", "a residual-register row is an open divergence.");

        if (a.Flags.HasResidualRegisterRow)
            Assert.Equal("N1", a.Numeric.State);
    }

    // ---------------------------------------------------------------- OT-19 MINVERSE

    [Fact]
    public void OT19_MINVERSE_carries_the_landed_kernel_and_not_the_superseded_score()
    {
        var a = HandbookFixture.Get("MINVERSE");
        var page = HandbookFixture.Page("MINVERSE");

        Must.NotContain(page, "still runs Gauss-Jordan", "the Doolittle-LU kernel landed on 2026-07-13.");
        Must.NotContain(page, "80/159", "a superseded score may not render as the current one.");

        if (a.Warrant is "W4" or "W5")
            Assert.True(a.Flags.HasNumericCountedRecord || a.Flags.HasStructuralCountedRecord);
    }

    // ---------------------------------------------------------------- OT-20 PHI (SHARPENED)

    [Fact]
    public void OT20_PHI_claims_no_erf_substrate_and_no_clean_state_while_a_residual_stands()
    {
        var a = HandbookFixture.Get("PHI");
        var page = HandbookFixture.Page("PHI");

        Must.NotContain(page, "erf substrate", "catalog:119 closes PHI on RN53(RN64(x*x)) -> x87 EXP.");
        Must.NotContain(page, "gratio", "PHI is not in the erf/gratio lane.");

        // Where a group residual covers PHI, the numeric axis may not render a clean match.
        if (a.Numeric.State == "N6")
            Assert.False(a.Numeric.Label.StartsWith("matched", StringComparison.Ordinal));
    }

    // ---------------------------------------------------------------- OT-21 POISSON.DIST

    [Fact]
    public void OT21_POISSON_DIST_never_fuses_the_identification_corpus_with_the_held_out_gate()
    {
        var page = HandbookFixture.Page("POISSON.DIST");

        Must.NotContain(page, "34,000", "the 34,000 fuses a 30,000-row identification corpus with a 4,000-row gate.");
        Must.NotContain(page, "34000", "same, unformatted.");

        var a = HandbookFixture.Get("POISSON.DIST");
        if (a.Warrant == "W5")
            Assert.True(a.Flags.HasHeldOutCountedRecord);
    }

    // ---------------------------------------------------------------- OT-22 every N8 page

    [Fact]
    public void OT22_every_N8_page_uses_the_narrow_clause_and_no_site_wide_aggregate()
    {
        var n8 = HandbookFixture.All.Where(a => a.Numeric.State == "N8").ToList();
        Assert.NotEmpty(n8);
        foreach (var a in n8)
        {
            Must.Contain(a.Numeric.Label, "in the six sources listed under Sources",
                $"{a.SurfaceName}: the N8 clause must be narrow.");
            Must.NotContain(a.Numeric.Label, "470", "G-2: site-wide aggregates live on /coverage/ only.");
            Must.NotContain(a.Numeric.Label, "398", "G-2.");
        }
    }

    // ---------------------------------------------------------------- OT-23 every W4/W5 page

    [Fact]
    public void OT23_no_W4_or_W5_page_hides_an_open_measured_divergence()
    {
        var strong = HandbookFixture.All.Where(a => a.Warrant is "W4" or "W5").ToList();
        Assert.NotEmpty(strong);

        foreach (var a in strong)
        {
            // RA-4 may not render without RA-4c: a counted warrant beside an open measured numeric
            // divergence must say so on the same page.
            if (a.Numeric.State == "N1")
                Assert.Equal(N1Text, a.Numeric.Label);

            // Warrant may never be read as quality: the label must carry the outcome.
            Assert.True(a.WarrantLabel.Contains("of which", StringComparison.Ordinal),
                $"{a.SurfaceName}: a counted warrant label must carry its outcome, not only its denominator. Got: {a.WarrantLabel}");
        }
    }

    private const string N1Text =
        "OxFunc and Excel disagree numerically, the disagreement is open, and a shortfall was measured";

    // ---------------------------------------------------------------- OT-24 the pigeonhole set

    [Fact]
    public void OT24_no_page_in_the_pigeonhole_set_claims_to_be_tested()
    {
        var set = HandbookFixture.All.Where(a => a.Flags.InPigeonholeSet).ToList();
        Assert.NotEmpty(set);

        foreach (var a in set)
        {
            var page = Pipeline.RenderedPage(a);
            Must.NotContain(page, "unit-tested", $"{a.SurfaceName} is in the pigeonhole set.");
            Must.NotContain(page, "test coverage", $"{a.SurfaceName} is in the pigeonhole set.");
            Must.NotContain(page, " is tested", $"{a.SurfaceName} is in the pigeonhole set.");
        }
    }

    // ---------------------------------------------------------------- the rejected vocabulary, everywhere

    [Fact]
    public void No_label_anywhere_uses_a_rejected_completeness_word()
    {
        var offenders = new List<string>();
        foreach (var a in HandbookFixture.All)
            foreach (var label in Pipeline.RenderedLabels(a))
                foreach (var word in CompletenessWords)
                    if (label.Contains(word, StringComparison.OrdinalIgnoreCase))
                        offenders.Add($"{a.SurfaceName}: \"{word}\" in \"{label}\"");

        Assert.True(offenders.Count == 0,
            "rejected completeness words rendered:\n" + string.Join("\n", offenders.Take(30)));
    }
}
