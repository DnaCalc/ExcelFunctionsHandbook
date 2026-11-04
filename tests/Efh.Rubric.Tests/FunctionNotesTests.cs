using Efh.Rubric;
using Xunit;

namespace Efh.Rubric.Tests;

public sealed class FunctionNotesTests
{
    [Fact]
    public void Every_F20_note_passes_its_checks()
    {
        var notes = FunctionNotes.Load(HandbookFixture.Root);
        var errors = FunctionNotes.Check(HandbookFixture.Root, notes);
        Assert.True(errors.Count == 0, "F20 note failures:\n" + string.Join("\n", errors));
    }

    [Fact]
    public void The_notes_index_api_and_page_sections_are_fresh_renders_of_the_notes()
    {
        var stale = FunctionNotes.Stale(HandbookFixture.Root);
        Assert.True(stale.Count == 0, "stale; run 'efh notes render':\n" + string.Join("\n", stale));
    }

    [Fact]
    public void Every_noted_function_with_a_page_carries_the_section_and_no_other_page_does()
    {
        var noted = FunctionNotes.Load(HandbookFixture.Root).SelectMany(n => n.Functions)
            .ToHashSet(StringComparer.Ordinal);
        var failures = new List<string>();
        foreach (var page in Directory.EnumerateFiles(
                     Path.Combine(HandbookFixture.Root, "content", "functions"), "*.md"))
        {
            var id = Path.GetFileNameWithoutExtension(page);
            var has = File.ReadAllText(page).Contains("\n## " + FunctionNotes.SectionTitle + "\n");
            if (has != noted.Contains(id))
                failures.Add($"{id}: section present {has}, expected {noted.Contains(id)}");
        }
        Assert.True(failures.Count == 0, string.Join("\n", failures));
    }

    private static FunctionNotes.Evidence Ev() =>
        new("DnaCalc", new string('a', 40), "x.rs", null, null);

    private static FunctionNotes.Note Sample(
        string kind = "excel-differs-from-math", string display = "-1.1300000000000001",
        string bits = "0xbff2147ae147ae15", FunctionNotes.Correct? correct = null, string statement = "S.") =>
        new("content/notes/FN-ROUNDUP-001.json", "FN-ROUNDUP-001", ["FUNC.ROUNDUP"], kind, "T", statement,
            "measured", "B.", null, null,
            [new("ROUNDUP(-1.125, 2)", "cell-value2",
                [new("number", new("number", "-1.125", "0xbff2000000000000"))],
                new("number", display, bits), correct, "Excel 16.0 build 20430", Ev())],
            [Ev()], [], "2026-11-04");

    private static readonly FunctionNotes.Correct Exact = new("number", "-1.13", "0xbff2147ae147ae14", "exact");

    [Fact]
    public void A_valid_note_passes_and_renders_bits_and_the_correct_value()
    {
        var note = Sample(correct: Exact);
        Assert.Empty(FunctionNotes.Check(HandbookFixture.Root, [note]));
        var section = FunctionNotes.RenderSection("FUNC.ROUNDUP", [note]);
        Must.Contain(section, "0xbff2000000000000", "inputs carry their bits");
        Must.Contain(section, "0xbff2147ae147ae15", "Excel's answer carries its bits");
        Must.Contain(section, "0xbff2147ae147ae14", "the correct value is shown");
        Must.Contain(section, "Excel 16.0 build 20430", "the environment is shown");
    }

    [Fact]
    public void The_checks_catch_wrong_bits_a_missing_correct_value_and_forbidden_words()
    {
        Assert.Contains(FunctionNotes.Check(HandbookFixture.Root, [Sample(display: "-1.13", correct: Exact)]),
            e => e.Contains("parses to"));
        Assert.Contains(FunctionNotes.Check(HandbookFixture.Root, [Sample()]),
            e => e.Contains("needs a repro with the correct value"));
        Assert.Contains(FunctionNotes.Check(HandbookFixture.Root, [Sample(correct: Exact, statement: "This is bit-exact.")]),
            e => e.Contains("bit-exact"));
        Assert.Contains(FunctionNotes.Check(HandbookFixture.Root, [Sample(kind: "documentation-differs")]),
            e => e.Contains("needs documentation"));
    }

    [Fact]
    public void Applying_the_section_is_idempotent_removable_and_sits_before_the_environment_section()
    {
        const string page = "---\nfunction_id: FUNC.ROUNDUP\nbody_sections:\n  - What it computes\n  - Excel versions, platforms and CPUs\n  - Page vocabulary\n  - Sources\n---\n\n## What it computes\n\nText.\n\n## Excel versions, platforms and CPUs\n\nE.\n\n## Page vocabulary\n\nV.\n\n## Sources\n\nS.\n";
        var note = Sample(correct: Exact);
        var once = FunctionNotes.ApplyToPage(page, "FUNC.ROUNDUP", [note]);
        Assert.Equal(once, FunctionNotes.ApplyToPage(once, "FUNC.ROUNDUP", [note]));
        Assert.Contains("  - Notes\n  - Excel versions, platforms and CPUs", once);
        Assert.True(once.IndexOf("## Notes", StringComparison.Ordinal)
                    < once.IndexOf("## Excel versions", StringComparison.Ordinal));
        Assert.Equal(page, FunctionNotes.ApplyToPage(once, "FUNC.ROUNDUP", []));
    }
}
