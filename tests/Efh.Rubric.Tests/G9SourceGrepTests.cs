using Xunit;

namespace Efh.Rubric.Tests;

/// <summary>
/// Guard G-9, second mechanism. The first is the [Obsolete(error: true)] accessor in
/// <see cref="G9FigureGuard"/>, which makes any C# naming it fail to compile. This one greps the
/// source, because a future maintainer could reach the string through JsonDocument without ever
/// naming the accessor.
/// </summary>
public class G9SourceGrepTests
{
    private static readonly string ToolsDir = Path.Combine(HandbookFixture.Root, "tools", "efh");
    private static readonly string TestsDir = Path.Combine(HandbookFixture.Root, "tests");

    private static IEnumerable<string> SourceFiles(string dir) =>
        Directory.EnumerateFiles(dir, "*.cs", SearchOption.AllDirectories)
                 .Where(p => !p.Contains($"{Path.DirectorySeparatorChar}obj{Path.DirectorySeparatorChar}")
                          && !p.Contains($"{Path.DirectorySeparatorChar}bin{Path.DirectorySeparatorChar}"));

    [Fact]
    public void No_rubric_source_file_mentions_the_forbidden_json_property()
    {
        var offenders = new List<string>();
        foreach (var path in SourceFiles(ToolsDir))
        {
            var name = Path.GetFileName(path);
            if (name is "G9FigureGuard.cs") continue; // the guard itself, by design

            var lines = File.ReadAllLines(path);
            for (var i = 0; i < lines.Length; i++)
            {
                var line = lines[i];
                if (line.Contains("figure", StringComparison.OrdinalIgnoreCase)
                    || line.Contains("Figure", StringComparison.Ordinal))
                    offenders.Add($"{name}:{i + 1}: {line.Trim()}");
            }
        }

        Assert.True(offenders.Count == 0,
            "G-9: counts[].figure is a verbatim upstream display string and no rubric code path may read it. "
            + "Hits:\n" + string.Join("\n", offenders));
    }

    [Fact]
    public void The_rubric_never_reads_the_property_at_run_time_either()
    {
        // The loader builds CountRow from an explicit field list. If a "figure" field is ever added
        // to CountRow this reflection check fails, whatever the grep says about formatting.
        var props = typeof(CountRow).GetProperties().Select(p => p.Name).ToList();
        Assert.DoesNotContain(props, p => p.Contains("igure", StringComparison.Ordinal));
    }

    [Fact]
    public void The_guard_accessor_is_obsolete_as_an_error()
    {
        var method = typeof(G9FigureGuard).GetMethod(nameof(G9FigureGuard.ReadFigure));
        Assert.NotNull(method);
        var attr = method!.GetCustomAttributes(typeof(ObsoleteAttribute), false)
                          .Cast<ObsoleteAttribute>().SingleOrDefault();
        Assert.NotNull(attr);
        Assert.True(attr!.IsError, "G-9's accessor must be an ERROR, not a warning.");
    }

    [Fact]
    public void The_test_suite_itself_does_not_read_the_property()
    {
        var offenders = new List<string>();
        foreach (var path in SourceFiles(TestsDir))
        {
            var name = Path.GetFileName(path);
            if (name is "G9SourceGrepTests.cs") continue;
            var lines = File.ReadAllLines(path);
            for (var i = 0; i < lines.Length; i++)
                if (lines[i].Contains("ReadFigure", StringComparison.Ordinal))
                    offenders.Add($"{name}:{i + 1}");
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders));
    }
}
