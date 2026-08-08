namespace Efh.Rubric;

public sealed record BoundCount(
    EvidenceRecord Record,
    CountRow Count,
    string BindingRule,
    bool IsPerSurface,
    int GroupSize);

public sealed record UnbindableCount(
    string RecordId,
    int CountIndex,
    string AxisValue,
    int SubjectCount,
    IReadOnlyList<string> Subjects,
    string Reason);

/// <summary>
/// Decides which Handbook entry each count belongs to, using only machine-readable fields.
///
/// The <c>counts[]</c> schema carries no field naming the subject a per-surface count belongs to.
/// In a single-subject record the binding is unambiguous. In a multi-subject record it is carried
/// only by the prose inside the display text / <c>source_sentence</c>, and G-9 forbids parsing those.
/// Positional binding (the i-th count to the i-th subject) is a guess, and a wrong guess here puts
/// one surface's shortfall onto another surface's page — the exact class of error this rubric
/// exists to prevent. So an unbindable per-surface count contributes only "a comparison is on
/// record" and never a counted warrant or a matched-Excel state, and every one of them is
/// published in the binding-defect report.
/// </summary>
public static class Binder
{
    public static (Dictionary<string, List<BoundCount>> ByFunction, List<UnbindableCount> Unbindable)
        Bind(IReadOnlyList<EvidenceRecord> records)
    {
        var byFunction = new Dictionary<string, List<BoundCount>>(StringComparer.Ordinal);
        var unbindable = new List<UnbindableCount>();

        void Attach(string fid, BoundCount bc)
        {
            if (!byFunction.TryGetValue(fid, out var list))
                byFunction[fid] = list = new List<BoundCount>();
            list.Add(bc);
        }

        foreach (var r in records)
        {
            var soleHeader = r.SubjectRole
                .Where(kv => kv.Value == "header")
                .Select(kv => kv.Key)
                .ToList();

            foreach (var c in r.Counts)
            {
                // 1. an explicit subject field on the count always wins (forward compatible).
                if (c.ExplicitSubjects.Count > 0)
                {
                    foreach (var s in c.ExplicitSubjects.Where(r.Subjects.Contains))
                        Attach(s, new BoundCount(r, c, Binding.ExplicitField, c.CountScope == "per-surface",
                            c.ExplicitSubjects.Count));
                    continue;
                }

                // 2. group scope: bind to the named members, or to every subject when unnamed.
                if (c.CountScope == "group")
                {
                    var members = c.GroupMembers.Count > 0
                        ? c.GroupMembers.Where(r.Subjects.Contains).ToList()
                        : r.Subjects.ToList();
                    var rule = c.GroupMembers.Count > 0 ? Binding.GroupMembers : Binding.GroupAllSubjects;
                    var k = c.GroupMembers.Count > 0 ? c.GroupMembers.Count : r.Subjects.Count;
                    foreach (var s in members)
                        Attach(s, new BoundCount(r, c, rule, false, k));
                    continue;
                }

                // 3. per-surface in a single-subject record.
                if (r.Subjects.Count == 1)
                {
                    Attach(r.Subjects[0], new BoundCount(r, c, Binding.SingleSubjectRecord, true, 1));
                    continue;
                }

                // 4. per-surface in a multi-subject record with exactly one 'header' subject.
                if (soleHeader.Count == 1)
                {
                    Attach(soleHeader[0], new BoundCount(r, c, Binding.HeaderRole, true, 1));
                    continue;
                }

                // 5. unbindable.
                unbindable.Add(new UnbindableCount(
                    c.RecordId, c.Index, c.AxisValue, r.Subjects.Count, r.Subjects,
                    soleHeader.Count == 0
                        ? "per-surface count in a multi-subject record with no subject_role 'header'"
                        : $"per-surface count in a multi-subject record with {soleHeader.Count} 'header' subjects"));
            }
        }

        foreach (var list in byFunction.Values)
            list.Sort((a, b) => string.CompareOrdinal(a.Count.Key, b.Count.Key));

        return (byFunction, unbindable);
    }

    /// <summary>Functions that hold at least one unbindable per-surface count, via their record's subject list.</summary>
    public static IReadOnlyDictionary<string, List<UnbindableCount>> UnbindableByFunction(
        IReadOnlyList<UnbindableCount> unbindable)
    {
        var map = new Dictionary<string, List<UnbindableCount>>(StringComparer.Ordinal);
        foreach (var u in unbindable)
            foreach (var s in u.Subjects)
            {
                if (!map.TryGetValue(s, out var l)) map[s] = l = new List<UnbindableCount>();
                l.Add(u);
            }
        return map;
    }
}
