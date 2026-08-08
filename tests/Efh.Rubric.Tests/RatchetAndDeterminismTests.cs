using System.Text.Json;
using Efh.Rubric;
using Xunit;

namespace Efh.Rubric.Tests;

public class RatchetAndDeterminismTests
{
    private static readonly string BaselinePath =
        Path.Combine(HandbookFixture.Root, "tests", "ratchets.baseline.json");

    [Fact]
    public void The_ratchet_baseline_exists_and_is_split_into_ratcheted_and_reported()
    {
        Assert.True(File.Exists(BaselinePath), "run 'efh rubric emit' first");
        using var doc = JsonDocument.Parse(File.ReadAllText(BaselinePath));
        var root = doc.RootElement;

        Assert.True(root.TryGetProperty("ratcheted", out var ratcheted));
        Assert.True(root.TryGetProperty("reported", out var reported));
        Assert.True(ratcheted.GetArrayLength() > 0);
        Assert.True(reported.GetArrayLength() > 0);

        // A3-S9: no counter that is a fact about the read-only OxFunc repository sits in the
        // ratcheted set. A rise there must never be a red build.
        foreach (var r in reported.EnumerateArray())
        {
            Assert.Equal("review note", r.GetProperty("on_rise").GetString());
            Assert.Equal("a red build", r.GetProperty("never").GetString());
        }
    }

    [Fact]
    public void The_ratcheted_counters_have_not_fallen_below_the_baseline()
    {
        Assert.True(File.Exists(BaselinePath));
        using var doc = JsonDocument.Parse(File.ReadAllText(BaselinePath));
        var c = HandbookFixture.Pipeline.Counters;
        var h = c.HonestyCounters();

        var live = new Dictionary<string, int>(StringComparer.Ordinal)
        {
            ["RC-1"] = h["entries_with_curated_handbook_prose"],
            ["RC-2"] = h["curated_family_pages"],
            ["RC-3"] = h["evidence_records"],
            ["RC-4"] = h["published_vector_suites"],
            ["RC-5"] = h["admitted_handbook_implementations"],
            ["RC-6"] = h["entries_with_a_numeric_bits_counted_record"],
            ["RC-7"] = h["entries_with_a_held_out_counted_record"],
            ["RC-8"] = h["of_those_with_a_per_surface_count"],
            ["RC-9"] = h["entries_with_a_battery_row_set"],
        };

        var offenders = new List<string>();
        foreach (var r in doc.RootElement.GetProperty("ratcheted").EnumerateArray())
        {
            var id = r.GetProperty("id").GetString()!;
            var baseline = r.GetProperty("today").GetInt32();
            if (!live.TryGetValue(id, out var now)) continue;
            if (now < baseline) offenders.Add($"{id} fell from {baseline} to {now}: {r.GetProperty("counter").GetString()}");
        }
        Assert.True(offenders.Count == 0,
            "RATCHET REGRESSION -- a Handbook-controlled counter fell:\n" + string.Join("\n", offenders));
    }

    [Fact]
    public void RC10_upstream_changed_is_zero()
    {
        using var doc = JsonDocument.Parse(File.ReadAllText(BaselinePath));
        var rc10 = doc.RootElement.GetProperty("ratcheted").EnumerateArray()
            .Single(r => r.GetProperty("id").GetString() == "RC-10");
        Assert.Equal(0, rc10.GetProperty("today").GetInt32());
    }

    [Fact]
    public void The_rubric_is_deterministic_same_inputs_give_byte_identical_output()
    {
        var tmp1 = Path.Combine(Path.GetTempPath(), "efh-rubric-det-1-" + Guid.NewGuid().ToString("N"));
        var tmp2 = Path.Combine(Path.GetTempPath(), "efh-rubric-det-2-" + Guid.NewGuid().ToString("N"));
        try
        {
            Directory.CreateDirectory(Path.Combine(tmp1, "site", "api"));
            Directory.CreateDirectory(Path.Combine(tmp2, "site", "api"));

            var p1 = new Pipeline(HandbookFixture.Root);
            var p2 = new Pipeline(HandbookFixture.Root);

            var f1 = Path.Combine(tmp1, "site", "api", "rubric.json");
            var f2 = Path.Combine(tmp2, "site", "api", "rubric.json");
            Emitter.EmitRubric(f1, p1.Engine, p1.Counters, p1.Loader);
            Emitter.EmitRubric(f2, p2.Engine, p2.Counters, p2.Loader);

            Assert.Equal(File.ReadAllBytes(f1), File.ReadAllBytes(f2));
        }
        finally
        {
            if (Directory.Exists(tmp1)) Directory.Delete(tmp1, true);
            if (Directory.Exists(tmp2)) Directory.Delete(tmp2, true);
        }
    }

    [Fact]
    public void The_published_rubric_matches_a_fresh_derivation()
    {
        var published = Path.Combine(HandbookFixture.Root, "site", "api", "rubric.json");
        Assert.True(File.Exists(published), "run 'efh rubric emit' first");

        var tmp = Path.Combine(Path.GetTempPath(), "efh-rubric-fresh-" + Guid.NewGuid().ToString("N"));
        try
        {
            Directory.CreateDirectory(tmp);
            var f = Path.Combine(tmp, "rubric.json");
            var p = new Pipeline(HandbookFixture.Root);
            Emitter.EmitRubric(f, p.Engine, p.Counters, p.Loader);
            Assert.True(File.ReadAllBytes(published).SequenceEqual(File.ReadAllBytes(f)),
                "site/api/rubric.json is stale. The evidence layer is written concurrently; re-run 'efh rubric emit'.");
        }
        finally
        {
            if (Directory.Exists(tmp)) Directory.Delete(tmp, true);
        }
    }

    [Fact]
    public void No_wall_clock_reaches_the_derivation()
    {
        var offenders = new List<string>();
        foreach (var path in Directory.EnumerateFiles(
                     Path.Combine(HandbookFixture.Root, "tools", "efh"), "*.cs", SearchOption.AllDirectories))
        {
            if (path.Contains($"{Path.DirectorySeparatorChar}obj{Path.DirectorySeparatorChar}")) continue;
            var text = File.ReadAllText(path);
            foreach (var token in new[] { "DateTime.Now", "DateTime.UtcNow", "DateTimeOffset.Now", "DateTimeOffset.UtcNow" })
                if (text.Contains(token, StringComparison.Ordinal))
                    offenders.Add($"{Path.GetFileName(path)}: {token}");
        }
        Assert.True(offenders.Count == 0, string.Join("\n", offenders));
    }
}
