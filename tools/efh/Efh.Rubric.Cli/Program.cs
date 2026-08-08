using Efh.Rubric;

var args0 = args.Length > 0 ? args[0] : "";
var args1 = args.Length > 1 ? args[1] : "";

if (args0 != "rubric" || args1 is not ("emit" or "report"))
{
    Console.Error.WriteLine("usage: efh rubric emit   [--root <handbook-root>]");
    Console.Error.WriteLine("       efh rubric report [--root <handbook-root>]");
    return 2;
}

var root = FindRoot(args);
Console.WriteLine($"handbook root: {root}");

var pipeline = new Pipeline(root);
var e = pipeline.Engine;
var c = pipeline.Counters;

Console.WriteLine($"entries       : {e.Assignments.Count}");
Console.WriteLine($"presence rows : {e.Presence.Count}");
Console.WriteLine($"battery rows  : {e.BatteryIds.Count}");
Console.WriteLine($"records read  : {e.Records.Count}");
Console.WriteLine($"counts bound  : {e.BoundByFunction.Values.Sum(v => v.Count)} bindings over {e.BoundByFunction.Count} entries");
Console.WriteLine($"unbindable    : {e.Unbindable.Count} per-surface count rows");
Console.WriteLine();

Console.WriteLine("warrant   " + string.Join("  ", new[] { "W0", "W1", "W2", "W3", "W3R", "W4", "W5" }
    .Select(w => $"{w}={c.Warrant(w)}")));
Console.WriteLine("depth     " + string.Join("  ", new[] { "D0", "D1", "D2", "D3", "D4", "D5" }
    .Select(d => $"{d}={c.Depth(d)}")));
Console.WriteLine("numeric   " + string.Join("  ", new[] { "N1", "N2", "N3", "N4", "N5", "N6", "NR", "N7", "N8" }
    .Select(s => $"{s}={c.NumericState(s)}")));
Console.WriteLine("structural" + "  " + string.Join("  ", new[] { "S5", "S6", "SR", "S7", "S8" }
    .Select(s => $"{s}={c.StructuralState(s)}")));
Console.WriteLine();

foreach (var (k, v) in c.HonestyCounters().OrderBy(x => x.Key, StringComparer.Ordinal))
    Console.WriteLine($"  {k,-70} {v,6}");

Console.WriteLine();
Console.WriteLine("data hygiene normalisations applied:");
foreach (var g in pipeline.Loader.Hygiene.ByKind())
    Console.WriteLine($"  {g.Key,-70} {g.Count(),6} occurrences in {g.Select(x => x.RecordId).Distinct().Count()} records");

if (args1 == "emit")
{
    pipeline.EmitAll(root);
    Console.WriteLine();
    Console.WriteLine("wrote site/api/rubric.json");
    Console.WriteLine("wrote site/api/rubric-hygiene.json");
    Console.WriteLine("wrote tests/ratchets.baseline.json");
}

return 0;

static string FindRoot(string[] a)
{
    for (var i = 0; i < a.Length - 1; i++)
        if (a[i] == "--root") return a[i + 1];

    var dir = new DirectoryInfo(Directory.GetCurrentDirectory());
    while (dir is not null)
    {
        if (Directory.Exists(Path.Combine(dir.FullName, "data", "functions")))
            return dir.FullName;
        dir = dir.Parent;
    }
    throw new InvalidOperationException("could not locate the Handbook root (no data/functions above cwd)");
}
