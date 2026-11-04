using System.Globalization;
using System.Text;
using System.Text.Json;

namespace Efh.Rubric;

/// <summary>
/// F20: function notes (content/notes/FN-*.json). Each note is a quirk, a place where Excel's answer
/// differs from the mathematical one, a detail needed to match Excel's bits, or a place where the
/// documentation differs from behaviour, with reproducing calls and evidence. Renders a "Notes"
/// section on every named function page, the index content/notes/NOTES.md and site/api/notes.json.
/// Deterministic: the same notes always render the same bytes.
/// </summary>
public static class FunctionNotes
{
    public const string SectionTitle = "Notes";
    public const string NotesRelativeDirectory = "content/notes";
    public const string IndexRelativePath = "content/notes/NOTES.md";
    public const string ApiRelativePath = "site/api/notes.json";
    private const string BeginMarker = "<!-- efh:notes begin";
    private const string EndMarker = "<!-- efh:notes end -->";

    /// <summary>Words the charter keeps out of Handbook voice (CHARTER 7.8) and the owner's ban.</summary>
    private static readonly string[] ForbiddenPhrases = ["bit-exact", "bit-for-bit", "bit for bit", "production ready", "production-ready"];

    public sealed record Value(string Kind, string Display, string? Bits);
    public sealed record Correct(string Kind, string Display, string? Bits, string How);
    public sealed record Input(string Name, Value Value);
    public sealed record Evidence(string Repository, string Commit, string Path, string? Locator, string? Note);
    public sealed record Documentation(string Claim, string Source);

    public sealed record Repro(
        string Call, string InputRoute, IReadOnlyList<Input> Inputs, Value Excel, Correct? Correct,
        string RecordedIn, Evidence Source);

    public sealed record Note(
        string Path, string NoteId, IReadOnlyList<string> Functions, string Kind, string Title,
        string Statement, string Status, string Basis, string? Inferred, Documentation? Documentation,
        IReadOnlyList<Repro> Repros, IReadOnlyList<Evidence> Evidence, IReadOnlyList<string> Open,
        string RecordedOn);

    public static string KindLabel(string kind) => kind switch
    {
        "quirk" => "Quirk",
        "excel-differs-from-math" => "Excel differs from the mathematical answer",
        "bit-matching-detail" => "Detail for matching Excel's bits",
        "documentation-differs" => "Documentation differs from behaviour",
        _ => kind,
    };

    // ------------------------------------------------------------------ loading

    public static IReadOnlyList<Note> Load(string root)
    {
        var dir = System.IO.Path.Combine(root, "content", "notes");
        if (!Directory.Exists(dir))
            return [];
        return Directory.EnumerateFiles(dir, "*.json")
            .OrderBy(p => System.IO.Path.GetFileName(p), StringComparer.Ordinal)
            .Select(p => Parse(root, p))
            .ToArray();
    }

    private static Note Parse(string root, string path)
    {
        using var doc = JsonDocument.Parse(File.ReadAllText(path));
        var r = doc.RootElement;
        var rel = System.IO.Path.GetRelativePath(root, path).Replace('\\', '/');
        var docEl = r.GetProperty("documentation");
        return new Note(
            rel,
            Str(r, "note_id"),
            StrList(r, "functions"),
            Str(r, "kind"),
            Str(r, "title"),
            Str(r, "statement"),
            Str(r, "status"),
            Str(r, "basis"),
            NStr(r, "inferred"),
            docEl.ValueKind == JsonValueKind.Object ? new Documentation(Str(docEl, "claim"), Str(docEl, "source")) : null,
            r.GetProperty("repros").EnumerateArray().Select(e =>
            {
                var c = e.GetProperty("correct");
                return new Repro(
                    Str(e, "call"), Str(e, "input_route"),
                    e.GetProperty("inputs").EnumerateArray()
                        .Select(i => new Input(Str(i, "name"), ParseValue(i.GetProperty("value")))).ToArray(),
                    ParseValue(e.GetProperty("excel")),
                    c.ValueKind == JsonValueKind.Object
                        ? new Correct(Str(c, "kind"), Str(c, "display"), NStr(c, "bits"), Str(c, "how"))
                        : null,
                    Str(e, "recorded_in"),
                    ParseEvidence(e.GetProperty("source")));
            }).ToArray(),
            r.GetProperty("evidence").EnumerateArray().Select(ParseEvidence).ToArray(),
            StrList(r, "open"),
            Str(r, "recorded_on"));
    }

    private static Value ParseValue(JsonElement e) => new(Str(e, "kind"), Str(e, "display"), NStr(e, "bits"));

    private static Evidence ParseEvidence(JsonElement e) => new(
        Str(e, "repository"), Str(e, "commit"), Str(e, "path"), NStr(e, "locator"), NStr(e, "note"));

    private static string Str(JsonElement e, string name) =>
        e.GetProperty(name).GetString() ?? throw new InvalidDataException($"{name} is null");

    private static string? NStr(JsonElement e, string name) =>
        e.TryGetProperty(name, out var v) && v.ValueKind == JsonValueKind.String ? v.GetString() : null;

    private static IReadOnlyList<string> StrList(JsonElement e, string name) =>
        e.GetProperty(name).EnumerateArray().Select(x => x.GetString()!).ToArray();

    // ------------------------------------------------------------------ checks

    /// <summary>Checks the schema cannot express. Returns one message per problem.</summary>
    public static IReadOnlyList<string> Check(string root, IReadOnlyList<Note> notes)
    {
        var errors = new List<string>();
        foreach (var n in notes)
        {
            var p = n.Path;
            if (System.IO.Path.GetFileNameWithoutExtension(p) != n.NoteId)
                errors.Add($"{p}: note_id {n.NoteId} does not match the file name");
            if (n.Functions.Count == 0)
                errors.Add($"{p}: names no function");
            foreach (var f in n.Functions)
                if (!File.Exists(System.IO.Path.Combine(root, "data", "functions", f + ".json")))
                    errors.Add($"{p}: {f} is not in data/functions");
            if (!n.Functions.SequenceEqual(n.Functions.OrderBy(x => x, StringComparer.Ordinal)))
                errors.Add($"{p}: functions[] is not sorted");
            if (n.Functions.Distinct(StringComparer.Ordinal).Count() != n.Functions.Count)
                errors.Add($"{p}: duplicate function");
            var stem = n.NoteId.Length > 7 ? n.NoteId[3..^4] : "";
            if (!n.Functions.Any(f => f[(f.IndexOf('.') + 1)..] == stem))
                errors.Add($"{p}: note_id stem {stem} is not one of the listed functions");
            if (n.Status == "inferred" && string.IsNullOrWhiteSpace(n.Inferred))
                errors.Add($"{p}: status inferred needs the inferred text");
            if (n.Status == "measured" && n.Repros.Count == 0)
                errors.Add($"{p}: a measured note needs at least one reproducing call");
            if (n.Kind == "documentation-differs" && n.Documentation is null)
                errors.Add($"{p}: documentation-differs needs documentation");
            if (n.Kind == "excel-differs-from-math" && !n.Repros.Any(r => r.Correct is not null))
                errors.Add($"{p}: excel-differs-from-math needs a repro with the correct value");
            for (var i = 0; i < n.Repros.Count; i++)
            {
                var r = n.Repros[i];
                var where = $"{p}: repros[{i}]";
                foreach (var inp in r.Inputs)
                    CheckValue(where + " input " + inp.Name, inp.Value.Kind, inp.Value.Display, inp.Value.Bits, errors);
                CheckValue(where + " excel", r.Excel.Kind, r.Excel.Display, r.Excel.Bits, errors);
                if (r.Correct is { } c)
                {
                    CheckValue(where + " correct", c.Kind, c.Display, c.Bits, errors);
                    if (c.Kind == r.Excel.Kind && c.Display == r.Excel.Display && c.Bits == r.Excel.Bits)
                        errors.Add($"{where}: the correct value equals Excel's answer");
                }
                if (r.InputRoute == "cell-value2" && r.Inputs.Count == 0)
                    errors.Add($"{where}: cell-value2 route lists no inputs");
            }
            foreach (var text in Texts(n))
                foreach (var w in ForbiddenPhrases)
                    if (text.Contains(w, StringComparison.OrdinalIgnoreCase))
                        errors.Add($"{p}: uses '{w}' (CHARTER 7.8 / owner rule); describe what was measured instead");
        }
        var ids = notes.Select(n => n.NoteId).ToArray();
        if (ids.Distinct(StringComparer.Ordinal).Count() != ids.Length)
            errors.Add("content/notes: duplicate note_id");
        return errors;
    }

    private static IEnumerable<string> Texts(Note n)
    {
        yield return n.Title;
        yield return n.Statement;
        yield return n.Basis;
        if (n.Inferred is not null) yield return n.Inferred;
        if (n.Documentation is not null) yield return n.Documentation.Claim;
        foreach (var o in n.Open) yield return o;
        foreach (var r in n.Repros)
            if (r.Correct is not null) yield return r.Correct.How;
        foreach (var e in n.Evidence.Concat(n.Repros.Select(r => r.Source)))
            if (e.Note is not null) yield return e.Note;
    }

    private static void CheckValue(string where, string kind, string display, string? bits, List<string> errors)
    {
        var needsBits = kind is "number" or "nonfinite";
        if (needsBits != (bits is not null))
        {
            errors.Add($"{where}: {kind} {(needsBits ? "needs" : "must not carry")} bits");
            return;
        }
        if (bits is null)
            return;
        var raw = ulong.Parse(bits[2..], NumberStyles.HexNumber, CultureInfo.InvariantCulture);
        var exponentAllOnes = (raw >> 52 & 0x7ff) == 0x7ff;
        if (kind == "nonfinite" && !exponentAllOnes)
            errors.Add($"{where}: nonfinite bits {bits} are a finite value");
        if (kind == "number")
        {
            if (exponentAllOnes)
                errors.Add($"{where}: number bits {bits} are not finite; use kind nonfinite");
            else if (!double.TryParse(display, NumberStyles.Float, CultureInfo.InvariantCulture, out var d))
                errors.Add($"{where}: display '{display}' is not a decimal number");
            else if ((ulong)BitConverter.DoubleToInt64Bits(d) != raw)
                errors.Add($"{where}: display '{display}' parses to 0x{BitConverter.DoubleToInt64Bits(d):x16}, not {bits}");
        }
    }

    // ------------------------------------------------------------------ rendering helpers

    private static string Cell(string s) => s.Replace("|", "\\|").Replace("\r", " ").Replace("\n", " ");

    private static string Val(string display, string? bits) =>
        bits is null ? $"`{display}`" : $"`{display}` (`{bits}`)";

    private static string Ev(Evidence e, string linkPrefix)
    {
        var loc = e.Locator is null ? "" : $", {e.Locator}";
        var note = e.Note is null ? "" : $" — {e.Note}";
        return e.Repository == "ExcelFunctionsHandbook"
            ? $"[`{e.Path}`]({linkPrefix}{e.Path}){loc}{note}"
            : $"{e.Repository} `{e.Path}`{loc} at `{e.Commit[..10]}`{note}";
    }

    private static string Surface(string functionId) => functionId[(functionId.IndexOf('.') + 1)..];

    /// <summary>One note as Markdown, headed at the given level. linkPrefix leads to the repository root.</summary>
    public static string RenderNote(Note n, string heading, string linkPrefix)
    {
        var sb = new StringBuilder();
        sb.Append(heading).Append(' ').Append(n.NoteId).Append(": ").Append(n.Title).Append('\n');
        sb.Append('\n');
        var applies = n.Functions.Count > 1 ? $" Applies to {string.Join(", ", n.Functions.Select(Surface))}." : "";
        sb.Append($"*{KindLabel(n.Kind)}; {n.Status}; recorded {n.RecordedOn}.*{applies}\n");
        sb.Append('\n');
        sb.Append(ExcelEnvironments.Wrap(n.Statement)).Append('\n');
        sb.Append('\n');
        if (n.Documentation is { } d)
        {
            sb.Append(ExcelEnvironments.Wrap($"Documentation: {d.Claim} ({d.Source})")).Append('\n');
            sb.Append('\n');
        }
        if (n.Repros.Count > 0)
        {
            var anyCorrect = n.Repros.Any(r => r.Correct is not null);
            sb.Append(anyCorrect
                ? "| Call | Inputs (binary64) | Excel | Correct value | Recorded in |\n|---|---|---|---|---|\n"
                : "| Call | Inputs (binary64) | Excel | Recorded in |\n|---|---|---|---|\n");
            foreach (var r in n.Repros)
            {
                var inputs = r.Inputs.Count == 0
                    ? "as typed"
                    : string.Join("; ", r.Inputs.Select(i => $"{i.Name} {Val(i.Value.Display, i.Value.Bits)}"));
                if (r.InputRoute == "cell-value2" && r.Inputs.Count > 0)
                    inputs += " (cells)";
                var correct = r.Correct is null ? "" : $" {Cell(Val(r.Correct.Display, r.Correct.Bits))} |";
                sb.Append($"| `{Cell(r.Call)}` | {Cell(inputs)} | {Cell(Val(r.Excel.Display, r.Excel.Bits))} |")
                  .Append(anyCorrect ? (r.Correct is null ? " — |" : correct) : "")
                  .Append($" {Cell(r.RecordedIn)} |\n");
            }
            sb.Append('\n');
            foreach (var how in n.Repros.Where(r => r.Correct is not null).Select(r => r.Correct!.How).Distinct())
                sb.Append(ExcelEnvironments.Wrap("Correct value: " + how)).Append('\n');
            if (anyCorrect)
                sb.Append('\n');
        }
        sb.Append(ExcelEnvironments.Wrap($"Basis ({n.Status}): {n.Basis}")).Append('\n');
        sb.Append('\n');
        if (n.Inferred is not null)
        {
            sb.Append(ExcelEnvironments.Wrap("Inferred, not measured: " + n.Inferred)).Append('\n');
            sb.Append('\n');
        }
        foreach (var o in n.Open)
            sb.Append("- Open: ").Append(o).Append('\n');
        if (n.Open.Count > 0)
            sb.Append('\n');
        sb.Append("Evidence:\n");
        sb.Append('\n');
        foreach (var e in n.Evidence)
            sb.Append("- ").Append(Ev(e, linkPrefix)).Append('\n');
        foreach (var e in n.Repros.Select(r => r.Source).Distinct().Where(e => !n.Evidence.Contains(e)))
            sb.Append("- ").Append(Ev(e, linkPrefix)).Append(" (readings)\n");
        sb.Append('\n');
        return sb.ToString();
    }

    // ------------------------------------------------------------------ the function-page section

    public static string RenderSection(string functionId, IReadOnlyList<Note> notes)
    {
        var sb = new StringBuilder();
        sb.Append("## ").Append(SectionTitle).Append('\n');
        sb.Append('\n');
        sb.Append(BeginMarker).Append(": generated by `efh notes render` from ")
          .Append(NotesRelativeDirectory).Append("/; edits here are overwritten -->\n");
        sb.Append('\n');
        sb.Append(ExcelEnvironments.Wrap(
            $"Recorded behaviour of `{Surface(functionId)}` that an implementer matching Excel, or a reader explaining an "
            + "unexpected Excel result, needs to know. Each note gives exact binary64 inputs, Excel's answer and where it "
            + "was recorded; \"measured\" means the statement is read off recorded Excel answers, \"inferred\" marks what "
            + "goes beyond them. All notes: [Function notes](../notes/NOTES.md).")).Append('\n');
        sb.Append('\n');
        foreach (var n in notes)
            sb.Append(RenderNote(n, "###", "../../"));
        sb.Append(EndMarker).Append('\n');
        return sb.ToString();
    }

    /// <summary>Returns the page text with the Notes section inserted, replaced or removed, and
    /// body_sections kept in step. The section goes before the generated environment section when the
    /// page has one, otherwise before Page vocabulary.</summary>
    public static string ApplyToPage(string pageText, string functionId, IReadOnlyList<Note> notes)
    {
        var lines = pageText.Split('\n').ToList();
        var start = lines.FindIndex(l => l == "## " + SectionTitle);
        if (start >= 0)
        {
            var end = lines.FindIndex(start, l => l == EndMarker);
            if (end < 0)
                throw new InvalidDataException($"'## {SectionTitle}' without '{EndMarker}'");
            var count = end - start + 1;
            if (end + 1 < lines.Count && lines[end + 1].Length == 0)
                count++;
            lines.RemoveRange(start, count);
        }
        var fmEnd = lines.FindIndex(1, l => l == "---");
        var bs = lines.FindIndex(l => l == "body_sections:");
        if (bs < 0 || bs > fmEnd)
            throw new InvalidDataException("no body_sections in the front matter");
        var existing = lines.FindIndex(bs, l => l == "  - " + SectionTitle || l == "  - \"" + SectionTitle + "\"");
        if (existing >= 0 && existing < fmEnd)
        {
            lines.RemoveAt(existing);
            fmEnd--;
        }
        if (notes.Count > 0)
        {
            string[] anchors = [ExcelEnvironments.SectionTitle, "Page vocabulary"];
            var anchor = anchors.First(a => lines.Contains("## " + a));
            var entry = lines.FindIndex(bs, l => l == "  - " + anchor || l == "  - \"" + anchor + "\"");
            if (entry < 0 || entry > fmEnd)
                throw new InvalidDataException($"body_sections lacks {anchor}");
            var quoted = lines[entry].Contains('"');
            lines.Insert(entry, quoted ? "  - \"" + SectionTitle + "\"" : "  - " + SectionTitle);
            var h2 = lines.FindIndex(l => l == "## " + anchor);
            var block = RenderSection(functionId, notes).TrimEnd('\n').Split('\n');
            lines.InsertRange(h2, block.Append(""));
        }
        return string.Join('\n', lines);
    }

    // ------------------------------------------------------------------ index page and API

    public static string RenderIndex(string root, IReadOnlyList<Note> notes)
    {
        var pages = Directory.Exists(System.IO.Path.Combine(root, "content", "functions"))
            ? Directory.EnumerateFiles(System.IO.Path.Combine(root, "content", "functions"), "*.md")
                .Select(System.IO.Path.GetFileNameWithoutExtension).ToHashSet(StringComparer.Ordinal)
            : new HashSet<string?>(StringComparer.Ordinal);
        string FnLink(string id) => pages.Contains(id)
            ? $"[`{Surface(id)}`](../functions/{id}.md)"
            : $"`{Surface(id)}` (no page yet)";

        var sb = new StringBuilder();
        sb.Append("# Function notes\n\n");
        sb.Append("<!-- generated by `efh notes render` from content/notes/FN-*.json; edits here are overwritten -->\n\n");
        sb.Append("Recorded behaviour of Excel's worksheet functions that an implementation matching Excel needs, or that ")
          .Append("explains a result a user did not expect: quirks, answers that differ from the mathematical value, ")
          .Append("details of the arithmetic needed to reproduce Excel's last bit, and places where Microsoft's ")
          .Append("documentation differs from behaviour. Every note gives exact binary64 inputs and Excel's recorded ")
          .Append("answer, the Excel build and machine of the reading, and the evidence. A note marked measured is read ")
          .Append("off recorded Excel answers over its stated scope; inferred marks what goes beyond them. How to add a ")
          .Append("note: [HOWTO.md](HOWTO.md). Format: ")
          .Append("[`tools/schemas/f20-function-note.schema.json`](../../tools/schemas/f20-function-note.schema.json). ")
          .Append("Machine-readable: [`site/api/notes.json`](../../site/api/notes.json).\n\n");

        var fnCount = notes.SelectMany(n => n.Functions).Distinct().Count();
        sb.Append("## Summary\n\n");
        sb.Append($"- Notes: {notes.Count}, over {fnCount} functions (of the 541 entries = 518 functions + 23 operators).\n");
        foreach (var kind in new[] { "excel-differs-from-math", "bit-matching-detail", "quirk", "documentation-differs" })
            sb.Append($"- {KindLabel(kind)}: {notes.Count(n => n.Kind == kind)}.\n");
        sb.Append($"- Measured: {notes.Count(n => n.Status == "measured")}; inferred: {notes.Count(n => n.Status == "inferred")}.\n\n");

        sb.Append("## Where Excel's answer is wrong\n\n");
        sb.Append("Notes whose reproducing calls give the exact mathematically correct value beside Excel's answer.\n\n");
        var wrong = notes.SelectMany(n => n.Repros.Where(r => r.Correct is not null).Select(r => (n, r))).ToArray();
        if (wrong.Length == 0)
            sb.Append("None recorded.\n\n");
        else
        {
            sb.Append("| Note | Call | Excel | Correct value |\n|---|---|---|---|\n");
            foreach (var (n, r) in wrong)
                sb.Append($"| [{n.NoteId}](#{Anchor(n)}) | `{Cell(r.Call)}` | {Cell(Val(r.Excel.Display, r.Excel.Bits))} | {Cell(Val(r.Correct!.Display, r.Correct.Bits))} |\n");
            sb.Append('\n');
        }

        sb.Append("## By function\n\n");
        sb.Append("| Function | Notes |\n|---|---|\n");
        foreach (var g in notes.SelectMany(n => n.Functions.Select(f => (f, n)))
                     .GroupBy(x => x.f, StringComparer.Ordinal).OrderBy(g => g.Key, StringComparer.Ordinal))
            sb.Append($"| {FnLink(g.Key)} | {string.Join(", ", g.Select(x => $"[{x.n.NoteId}](#{Anchor(x.n)})"))} |\n");
        sb.Append('\n');

        sb.Append("## Notes\n\n");
        foreach (var n in notes)
            sb.Append(RenderNote(n, "###", "../../"));
        return string.Join('\n', sb.ToString().TrimEnd('\n').Split('\n').Select(line =>
            line.Length > 95 && !line.StartsWith('|') && !line.StartsWith('#')
                && !line.StartsWith("- ") && !line.StartsWith("<!--")
                ? ExcelEnvironments.Wrap(line)
                : line)) + "\n";
    }

    /// <summary>GitHub-style heading anchor of a note's H3.</summary>
    public static string Anchor(Note n)
    {
        var text = (n.NoteId + ": " + n.Title).ToLowerInvariant();
        var sb = new StringBuilder();
        foreach (var ch in text)
        {
            if (char.IsLetterOrDigit(ch) || ch == '-' || ch == '_') sb.Append(ch);
            else if (ch == ' ') sb.Append('-');
        }
        return sb.ToString();
    }

    // ------------------------------------------------------------------ write / check

    /// <summary>Every file this generator owns, with the bytes it should hold.</summary>
    public static IReadOnlyDictionary<string, string> Plan(string root)
    {
        var notes = Load(root);
        var errors = Check(root, notes);
        if (errors.Count > 0)
            throw new InvalidDataException("F20 notes fail their checks:\n" + string.Join("\n", errors));
        var plan = new SortedDictionary<string, string>(StringComparer.Ordinal)
        {
            [IndexRelativePath] = RenderIndex(root, notes),
            [ApiRelativePath] = RenderApiFrom(root, notes),
        };
        var pageDir = System.IO.Path.Combine(root, "content", "functions");
        foreach (var page in Directory.EnumerateFiles(pageDir, "*.md").OrderBy(p => p, StringComparer.Ordinal))
        {
            var id = System.IO.Path.GetFileNameWithoutExtension(page);
            var text = File.ReadAllText(page);
            var mine = notes.Where(n => n.Functions.Contains(id, StringComparer.Ordinal)).ToArray();
            if (mine.Length == 0 && !text.Contains("\n## " + SectionTitle + "\n", StringComparison.Ordinal))
                continue;
            plan["content/functions/" + id + ".md"] = ApplyToPage(text, id, mine);
        }
        return plan;
    }

    private static string RenderApiFrom(string root, IReadOnlyList<Note> notes)
    {
        var options = new JsonWriterOptions { Indented = true, Encoder = System.Text.Encodings.Web.JavaScriptEncoder.UnsafeRelaxedJsonEscaping };
        using var ms = new MemoryStream();
        using (var w = new Utf8JsonWriter(ms, options))
        {
            w.WriteStartObject();
            w.WriteString("schema", "efh.function-notes-api/v1");
            w.WriteString("source", NotesRelativeDirectory + "/FN-*.json (efh.function-note/v1)");
            w.WriteNumber("note_count", notes.Count);
            w.WriteStartArray("notes");
            foreach (var n in notes)
            {
                using var doc = JsonDocument.Parse(File.ReadAllText(System.IO.Path.Combine(root, n.Path)));
                doc.RootElement.WriteTo(w);
            }
            w.WriteEndArray();
            w.WriteEndObject();
        }
        return Encoding.UTF8.GetString(ms.ToArray()).Replace("\r\n", "\n") + "\n";
    }

    public static IReadOnlyList<string> Render(string root)
    {
        var changed = new List<string>();
        foreach (var (rel, text) in Plan(root))
        {
            var path = System.IO.Path.Combine(root, rel.Replace('/', System.IO.Path.DirectorySeparatorChar));
            var bytes = new UTF8Encoding(false).GetBytes(text);
            if (File.Exists(path) && File.ReadAllBytes(path).AsSpan().SequenceEqual(bytes))
                continue;
            Directory.CreateDirectory(System.IO.Path.GetDirectoryName(path)!);
            File.WriteAllBytes(path, bytes);
            changed.Add(rel);
        }
        return changed;
    }

    public static IReadOnlyList<string> Stale(string root)
    {
        var stale = new List<string>();
        foreach (var (rel, text) in Plan(root))
        {
            var path = System.IO.Path.Combine(root, rel.Replace('/', System.IO.Path.DirectorySeparatorChar));
            var bytes = new UTF8Encoding(false).GetBytes(text);
            if (!File.Exists(path) || !File.ReadAllBytes(path).AsSpan().SequenceEqual(bytes))
                stale.Add(rel);
        }
        return stale;
    }
}
