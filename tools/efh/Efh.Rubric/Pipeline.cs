namespace Efh.Rubric;

/// <summary>One entry point, used identically by the CLI and by every test. Re-runnable: it reads
/// whatever is on disk now and derives from that, so it can be run again as the evidence layer
/// finishes being written.</summary>
public sealed class Pipeline
{
    public Loader Loader { get; }
    public RubricEngine Engine { get; }
    public Counters Counters { get; }

    public Pipeline(string handbookRoot)
    {
        Loader = new Loader(handbookRoot);
        var functions = Loader.LoadFunctions();
        var presence = Loader.LoadPresence();
        var records = Loader.LoadEvidence();
        var battery = Loader.LoadBatteryIds();
        Engine = new RubricEngine(functions, presence, records, battery);
        Counters = new Counters(Engine, Loader);
    }

    public Assignment this[string surfaceOrFunctionId] =>
        Engine.Assignments.FirstOrDefault(a => a.FunctionId == surfaceOrFunctionId)
        ?? Engine.Assignments.FirstOrDefault(a => a.SurfaceName == surfaceOrFunctionId)
        ?? Engine.Assignments.First(a => a.FunctionId == "FUNC." + surfaceOrFunctionId);

    public Assignment? Find(string surfaceOrFunctionId) =>
        Engine.Assignments.FirstOrDefault(a => a.FunctionId == surfaceOrFunctionId)
        ?? Engine.Assignments.FirstOrDefault(a => a.SurfaceName == surfaceOrFunctionId)
        ?? Engine.Assignments.FirstOrDefault(a => a.FunctionId == "FUNC." + surfaceOrFunctionId);

    /// <summary>All text a page would render for one entry: every label, in both lenses.</summary>
    public static IEnumerable<string> RenderedLabels(Assignment a)
    {
        yield return a.WarrantLabel;
        yield return a.DepthLabel;
        yield return a.Numeric.Label;
        yield return a.Structural.Label;
        if (a.InternalEqualityPredicateLabel is not null) yield return a.InternalEqualityPredicateLabel;
    }

    public static string RenderedPage(Assignment a) =>
        string.Join("\n", new[]
        {
            a.SurfaceName,
            a.Warrant + ": " + a.WarrantLabel,
            a.Depth + ": " + a.DepthLabel,
            a.Numeric.State + ": " + a.Numeric.Label,
            a.Structural.State + ": " + a.Structural.Label,
            a.InternalEqualityPredicateLabel ?? "",
            a.WhyNoCount ?? "",
        }.Concat(a.Limitations)
         .Concat(a.OpenDefectStreamRecords)
         .Concat(a.DefectStreamFilesNamingThisSurface));

    public void EmitAll(string handbookRoot)
    {
        var root = handbookRoot.Replace('\\', '/').TrimEnd('/');
        Emitter.EmitRubric(Path.Combine(root, "site", "api", "rubric.json"), Engine, Counters, Loader);
        Emitter.EmitHygiene(Path.Combine(root, "site", "api", "rubric-hygiene.json"), Loader, Engine);
        Emitter.EmitRatchets(Path.Combine(root, "tests", "ratchets.baseline.json"), Engine, Counters, Loader);
    }
}
