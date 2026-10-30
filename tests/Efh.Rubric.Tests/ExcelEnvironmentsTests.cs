using Efh.Rubric;
using Xunit;

namespace Efh.Rubric.Tests;

public sealed class ExcelEnvironmentsTests
{
    [Fact]
    public void Every_F19_record_passes_the_cross_reference_checks()
    {
        var records = ExcelEnvironments.Load(HandbookFixture.Root);
        var errors = ExcelEnvironments.Check(HandbookFixture.Root, records);
        Assert.True(errors.Count == 0, "F19 cross-reference failures:\n" + string.Join("\n", errors));
    }

    [Fact]
    public void The_environment_index_and_page_sections_are_fresh_renders_of_the_F19_records()
    {
        var stale = ExcelEnvironments.Stale(HandbookFixture.Root);
        Assert.True(stale.Count == 0,
            "stale; run 'efh environments render':\n" + string.Join("\n", stale));
    }

    [Fact]
    public void Every_affected_function_with_a_page_carries_the_section_and_no_other_page_does()
    {
        var records = ExcelEnvironments.Load(HandbookFixture.Root)
            .ToDictionary(r => r.FunctionId, StringComparer.Ordinal);
        var failures = new List<string>();
        foreach (var page in Directory.EnumerateFiles(
                     Path.Combine(HandbookFixture.Root, "content", "functions"), "*.md"))
        {
            var id = Path.GetFileNameWithoutExtension(page);
            var has = File.ReadAllText(page).Contains("\n## " + ExcelEnvironments.SectionTitle + "\n");
            var should = records.TryGetValue(id, out var r) && r.HasPageSection;
            if (has != should)
                failures.Add($"{id}: section present {has}, expected {should}");
        }
        Assert.True(failures.Count == 0, string.Join("\n", failures));
    }

    [Fact]
    public void A_record_with_an_observed_difference_renders_it_with_bits_and_both_environments()
    {
        var env = (string id, string build, string cpu) => new ExcelEnvironments.Environment(
            id, "16.0", build, "windows", "Windows (64-bit) NT 10.00", "64-bit", cpu,
            cpu.StartsWith("AMD") ? "AMD" : "Intel", "not-recorded", "2", "not-recorded", "test");
        var ev = new ExcelEnvironments.Evidence("OxFunc", new string('a', 40), "x.json", null, null, null);
        var record = new ExcelEnvironments.Record(
            "content/evidence/environments/FUNC.EXP.json", "FUNC.EXP", "EXP", "Test record.",
            [env("win-x64-16.0.1-amd", "1", "AMD X"), env("win-x64-16.0.1-intel", "1", "Intel Y")],
            [new("win-x64-16.0.1-amd", 1, null, null, [ev]), new("win-x64-16.0.1-intel", 1, null, null, [ev])],
            [new("XC-EXP-001", ["cpu"], ["win-x64-16.0.1-amd", "win-x64-16.0.1-intel"], 1, 1, "m", [ev])],
            [new("XD-EXP-001", ["cpu"], "=EXP(A1)",
                [new("number", "0.5", "0x3fe0000000000000")],
                [
                    new("win-x64-16.0.1-amd", new("number", "1.6487212707001282", "0x3ffa61298e1e069c"), null, ev),
                    new("win-x64-16.0.1-intel", new("number", "1.6487212707001280", "0x3ffa61298e1e069b"), null, ev),
                ],
                "not-rechecked", null, null, "last bit differs")],
            []);
        Assert.Empty(ExcelEnvironments.Check(HandbookFixture.Root, [record])
            .Where(e => !e.Contains("does not match the file name", StringComparison.Ordinal)));
        var section = ExcelEnvironments.RenderSection(record);
        Must.Contain(section, "0x3fe0000000000000", "inputs carry their bits");
        Must.Contain(section, "0x3ffa61298e1e069b", "each result carries its bits");
        Must.Contain(section, "AMD X", "the CPU of each observation is shown");
        Must.Contain(section, "Intel Y", "the CPU of each observation is shown");
        Must.Contain(section, "not-rechecked", "the recheck state is shown");
    }

    [Fact]
    public void Applying_the_section_is_idempotent_and_removable()
    {
        const string page = "---\nfunction_id: FUNC.X\nbody_sections:\n  - What it computes\n  - Page vocabulary\n  - Sources\n---\n\n## What it computes\n\nText.\n\n## Page vocabulary\n\nV.\n\n## Sources\n\nS.\n";
        var ev = new ExcelEnvironments.Evidence("OxFunc", new string('a', 40), "x.json", null, null, null);
        var record = new ExcelEnvironments.Record(
            "content/evidence/environments/FUNC.X.json", "FUNC.X", "X", "Status.", [], [], [], [],
            [new("XS-X-001", ["cpu"], "expected-untested", "Statement.", [ev])]);
        var once = ExcelEnvironments.ApplyToPage(page, record);
        var twice = ExcelEnvironments.ApplyToPage(once, record);
        Assert.Equal(once, twice);
        Assert.Contains("  - " + ExcelEnvironments.SectionTitle + "\n  - Page vocabulary", once);
        Assert.Contains("## " + ExcelEnvironments.SectionTitle, once);
        Assert.Equal(page, ExcelEnvironments.ApplyToPage(once, null));
    }
}
