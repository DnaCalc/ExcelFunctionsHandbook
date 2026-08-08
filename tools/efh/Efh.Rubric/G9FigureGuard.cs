namespace Efh.Rubric;

/// <summary>
/// Guard G-9. FOUNDATION 2.5: <c>counts[].figure</c> is a verbatim upstream display string.
/// No rubric code path may parse it. The rubric derives from the typed fields
/// (<c>passed</c>, <c>total</c>, <c>axis</c>, <c>attribution</c>, <c>measurement_subject</c>, ...)
/// and from nothing else.
///
/// Two independent enforcement mechanisms:
///   1. This accessor. It is <c>[Obsolete(error: true)]</c>, so any C# that names it fails to
///      compile. It also throws, so any reflection path fails at run time.
///   2. <c>G9SourceGrepTests</c> in tests/, which greps every .cs file under tools/efh/ for
///      the token <c>figure</c> and fails on any hit outside this file.
///
/// The loader (<see cref="Loader"/>) never reads the "figure" JSON property into memory at all,
/// so there is no in-process copy of the string for anything to reach.
/// </summary>
public static class G9FigureGuard
{
    public const string RuleId = "G-9";

    public const string Rule =
        "No code path may read counts[].figure. It is an upstream display string; " +
        "parsing it re-introduces prose-derived numbers into a machine-derived rubric.";

    /// <summary>The name of the forbidden JSON property, held once so the grep test has an anchor.</summary>
    public const string ForbiddenJsonProperty = "figure";

    [Obsolete("G-9: counts[].figure may never be read by the rubric. Use the typed count fields.", error: true)]
    public static string ReadFigure(object anyCount) =>
        throw new InvalidOperationException(
            "G-9 violation: counts[].figure was read at run time. " + Rule);
}
