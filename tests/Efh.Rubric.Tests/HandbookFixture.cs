using Efh.Rubric;

namespace Efh.Rubric.Tests;

/// <summary>
/// One pipeline run, shared by the whole suite. The suite runs against the organs on disk, so it
/// is re-runnable while the evidence layer is still being written.
/// </summary>
public sealed class HandbookFixture
{
    public static readonly string Root = LocateRoot();
    public static readonly Pipeline Pipeline = new(Root);

    public static IReadOnlyList<Assignment> All => Pipeline.Engine.Assignments;

    public static Assignment? Find(string surface) => Pipeline.Find(surface);

    public static Assignment Get(string surface) =>
        Pipeline.Find(surface) ?? throw new InvalidOperationException(
            $"{surface} is not in data/functions. The rubric's denominator is wrong, not the test.");

    /// <summary>Everything one entry's page would render, joined. Used by the forbidden-string assertions.</summary>
    public static string Page(string surface) => Pipeline.RenderedPage(Get(surface));

    public static IEnumerable<string> LabelsOf(string surface) => Pipeline.RenderedLabels(Get(surface));

    /// <summary>Whether the entry currently holds any evidence record. Records are written by a separate
    /// concurrent process, so a test may legitimately be in reduced mode.</summary>
    public static bool HasEvidence(string surface) => Get(surface).EvidenceRecords.Count > 0;

    private static string LocateRoot()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir is not null)
        {
            if (Directory.Exists(Path.Combine(dir.FullName, "data", "functions"))
                && Directory.Exists(Path.Combine(dir.FullName, "content", "evidence")))
                return dir.FullName;
            dir = dir.Parent;
        }
        throw new InvalidOperationException("could not locate the Handbook root from " + AppContext.BaseDirectory);
    }
}

public static class Must
{
    public static void NotContain(string haystack, string needle, string because)
    {
        if (haystack.Contains(needle, StringComparison.OrdinalIgnoreCase))
            throw new Xunit.Sdk.XunitException(
                $"FORBIDDEN STRING \"{needle}\" is rendered. {because}\n---\n{haystack}\n---");
    }

    public static void Contain(string haystack, string needle, string because)
    {
        if (!haystack.Contains(needle, StringComparison.OrdinalIgnoreCase))
            throw new Xunit.Sdk.XunitException(
                $"REQUIRED STRING \"{needle}\" is not rendered. {because}\n---\n{haystack}\n---");
    }
}
