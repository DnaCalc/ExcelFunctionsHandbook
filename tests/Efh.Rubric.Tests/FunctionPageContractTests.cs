using System.Globalization;
using System.Text.Json;
using System.Text.RegularExpressions;
using Xunit;
using YamlDotNet.Core;
using YamlDotNet.RepresentationModel;

namespace Efh.Rubric.Tests;

public sealed class FunctionPageContractTests
{
    private static readonly string[] RequiredClosingSections = ["Page vocabulary", "Sources"];
    private static readonly Regex MarkdownLinkPattern = new(
        @"(?<!!)\[[^\]\r\n]*\]\((?<destination><[^>\r\n]+>|[^)\s]+)(?:\s+[^)]*)?\)",
        RegexOptions.Compiled | RegexOptions.CultureInvariant);

    [Fact]
    public void Every_function_page_satisfies_the_F6_front_matter_and_body_contract()
    {
        var schemaPath = Path.Combine(
            HandbookFixture.Root, "tools", "schemas", "f6-function-page.schema.json");
        using var schemaDocument = JsonDocument.Parse(File.ReadAllText(schemaPath));
        var schema = schemaDocument.RootElement;

        var pages = Directory.EnumerateFiles(
                Path.Combine(HandbookFixture.Root, "content", "functions"), "*.md")
            .OrderBy(path => path, StringComparer.Ordinal)
            .ToArray();
        Assert.NotEmpty(pages);

        var errors = new List<string>();
        foreach (var page in pages)
            ValidatePage(page, schema, errors);

        Assert.True(errors.Count == 0,
            "F6 function-page contract failures:\n" + string.Join("\n", errors));
    }

    [Fact]
    public void Every_relative_Markdown_link_in_an_F6_body_resolves()
    {
        var pages = Directory.EnumerateFiles(
                Path.Combine(HandbookFixture.Root, "content", "functions"), "*.md")
            .OrderBy(path => path, StringComparer.Ordinal)
            .ToArray();
        Assert.NotEmpty(pages);

        var errors = new List<string>();
        foreach (var page in pages)
        {
            var relativePath = Path.GetRelativePath(HandbookFixture.Root, page).Replace('\\', '/');
            var lines = File.ReadAllText(page).ReplaceLineEndings("\n").Split('\n');
            var closingFence = Array.FindIndex(lines, 1, line => line == "---");
            if (closingFence < 0)
            {
                errors.Add($"{relativePath}: cannot locate the F6 body");
                continue;
            }

            foreach (var destination in ReadMarkdownLinkDestinations(lines.Skip(closingFence + 1)))
            {
                var target = destination;
                if (target.StartsWith('<') && target.EndsWith('>'))
                    target = target[1..^1];
                if (target.StartsWith('#')
                    || target.StartsWith("//", StringComparison.Ordinal)
                    || Uri.TryCreate(target, UriKind.Absolute, out _))
                    continue;

                var separator = target.IndexOfAny(['#', '?']);
                var pathPart = separator >= 0 ? target[..separator] : target;
                if (pathPart.Length == 0)
                    continue;

                try
                {
                    var decodedPath = Uri.UnescapeDataString(pathPart)
                        .Replace('/', Path.DirectorySeparatorChar);
                    var resolvedPath = Path.GetFullPath(
                        Path.Combine(Path.GetDirectoryName(page)!, decodedPath));
                    if (!File.Exists(resolvedPath) && !Directory.Exists(resolvedPath))
                        errors.Add($"{relativePath}: {destination} does not resolve");
                }
                catch (Exception exception) when (
                    exception is ArgumentException or NotSupportedException or UriFormatException)
                {
                    errors.Add($"{relativePath}: {destination} is not a valid relative link");
                }
            }
        }

        Assert.True(errors.Count == 0,
            "Unresolved relative Markdown links in F6 bodies:\n" + string.Join("\n", errors));
    }

    [Fact]
    public void F5_and_F6_patterns_accept_the_canonical_evidence_record_ids()
    {
        var f5Path = Path.Combine(
            HandbookFixture.Root, "tools", "schemas", "f5-evidence-record.schema.json");
        var f6Path = Path.Combine(
            HandbookFixture.Root, "tools", "schemas", "f6-function-page.schema.json");

        using var f5Document = JsonDocument.Parse(File.ReadAllText(f5Path));
        using var f6Document = JsonDocument.Parse(File.ReadAllText(f6Path));
        var f5Pattern = f5Document.RootElement.GetProperty("properties")
            .GetProperty("record_id").GetProperty("pattern").GetString()!;
        var f6Pattern = f6Document.RootElement.GetProperty("properties")
            .GetProperty("evidence_records").GetProperty("items")
            .GetProperty("pattern").GetString()!;

        Assert.Equal(f5Pattern, f6Pattern);

        var failures = new List<string>();
        var recordDirectory = Path.Combine(
            HandbookFixture.Root, "content", "evidence", "records");
        foreach (var path in Directory.EnumerateFiles(recordDirectory, "*.json")
                     .OrderBy(path => path, StringComparer.Ordinal))
        {
            using var record = JsonDocument.Parse(File.ReadAllText(path));
            var fileId = Path.GetFileNameWithoutExtension(path);
            var recordId = record.RootElement.GetProperty("record_id").GetString();
            if (!StringComparer.Ordinal.Equals(fileId, recordId))
                failures.Add($"{Path.GetFileName(path)}: record_id is {recordId ?? "null"}");
            if (recordId is null || !Regex.IsMatch(recordId, f5Pattern, RegexOptions.CultureInvariant))
                failures.Add($"{Path.GetFileName(path)}: {recordId ?? "null"} does not match {f5Pattern}");
        }

        Assert.True(failures.Count == 0,
            "Canonical evidence-record IDs do not satisfy F5/F6:\n" + string.Join("\n", failures));
    }

    private static void ValidatePage(string path, JsonElement schema, List<string> errors)
    {
        var relativePath = Path.GetRelativePath(HandbookFixture.Root, path).Replace('\\', '/');
        var lines = File.ReadAllText(path).ReplaceLineEndings("\n").Split('\n');
        if (lines.Length == 0 || lines[0] != "---")
        {
            errors.Add($"{relativePath}: missing opening front-matter fence");
            return;
        }

        var closingFence = Array.FindIndex(lines, 1, line => line == "---");
        if (closingFence < 0)
        {
            errors.Add($"{relativePath}: missing closing front-matter fence");
            return;
        }

        var yaml = string.Join('\n', lines.Skip(1).Take(closingFence - 1));
        var stream = new YamlStream();
        try
        {
            stream.Load(new StringReader(yaml));
        }
        catch (YamlException exception)
        {
            errors.Add($"{relativePath}: invalid YAML: {exception.Message}");
            return;
        }

        if (stream.Documents.Count != 1 || stream.Documents[0].RootNode is not YamlMappingNode mapping)
        {
            errors.Add($"{relativePath}: front matter must be one YAML mapping");
            return;
        }

        ValidateNode(mapping, schema, relativePath, "$", errors);

        var functionId = ScalarValue(mapping, "function_id");
        var expectedId = Path.GetFileNameWithoutExtension(path);
        if (!StringComparer.Ordinal.Equals(expectedId, functionId))
            errors.Add($"{relativePath}: function_id {functionId ?? "null"} does not match filename");
        if (!File.Exists(Path.Combine(
                HandbookFixture.Root, "data", "functions", expectedId + ".json")))
            errors.Add($"{relativePath}: no matching data/functions/{expectedId}.json");

        var hasOverride = TryGet(mapping, "description_override", out _);
        var hasOverrideReason = TryGet(mapping, "description_override_reason", out _);
        if (hasOverride != hasOverrideReason)
            errors.Add($"{relativePath}: description_override and its reason must occur together");

        if (TryGet(mapping, "evidence_records", out var evidenceNode)
            && evidenceNode is YamlSequenceNode evidenceRecords)
        {
            foreach (var item in evidenceRecords.Children.OfType<YamlScalarNode>())
            {
                var recordId = item.Value ?? "";
                if (!File.Exists(Path.Combine(
                        HandbookFixture.Root, "content", "evidence", "records", recordId + ".json")))
                    errors.Add($"{relativePath}: evidence record {recordId} does not exist");
            }
        }

        var actualSections = ReadH2Sections(lines.Skip(closingFence + 1));
        var declaredSections = SequenceValues(mapping, "body_sections");
        if (!declaredSections.SequenceEqual(actualSections, StringComparer.Ordinal))
        {
            errors.Add($"{relativePath}: body_sections [{string.Join(" | ", declaredSections)}] "
                       + $"!= authored H2s [{string.Join(" | ", actualSections)}]");
        }

        if (actualSections.Count < RequiredClosingSections.Length
            || !actualSections.TakeLast(RequiredClosingSections.Length)
                .SequenceEqual(RequiredClosingSections, StringComparer.Ordinal))
        {
            errors.Add($"{relativePath}: final H2s must be Page vocabulary, Sources");
        }
    }

    private static void ValidateNode(
        YamlNode node,
        JsonElement schema,
        string relativePath,
        string location,
        List<string> errors)
    {
        if (schema.TryGetProperty("type", out var type)
            && !MatchesAnyType(node, type))
        {
            errors.Add($"{relativePath}: {location} has the wrong YAML type");
            return;
        }

        if (node is YamlScalarNode scalar && !IsYamlNull(scalar))
        {
            if (schema.TryGetProperty("const", out var constant)
                && constant.ValueKind == JsonValueKind.String
                && scalar.Value != constant.GetString())
                errors.Add($"{relativePath}: {location} must equal {constant.GetString()}");

            if (schema.TryGetProperty("enum", out var values)
                && values.ValueKind == JsonValueKind.Array
                && !values.EnumerateArray().Any(value =>
                    value.ValueKind == JsonValueKind.String && value.GetString() == scalar.Value))
                errors.Add($"{relativePath}: {location} has value {scalar.Value} outside its enum");

            if (schema.TryGetProperty("pattern", out var pattern)
                && !Regex.IsMatch(
                    scalar.Value ?? "", pattern.GetString()!, RegexOptions.CultureInvariant))
                errors.Add($"{relativePath}: {location} value {scalar.Value} does not match {pattern.GetString()}");
        }

        if (node is YamlMappingNode mapping)
            ValidateMapping(mapping, schema, relativePath, location, errors);
        else if (node is YamlSequenceNode sequence)
            ValidateSequence(sequence, schema, relativePath, location, errors);
    }

    private static void ValidateMapping(
        YamlMappingNode mapping,
        JsonElement schema,
        string relativePath,
        string location,
        List<string> errors)
    {
        var entries = new List<(string Key, YamlNode Value)>();
        foreach (var entry in mapping.Children)
        {
            if (entry.Key is not YamlScalarNode { Value: not null } key)
            {
                errors.Add($"{relativePath}: {location} has a non-scalar key");
                continue;
            }
            entries.Add((key.Value!, entry.Value));
        }

        var properties = schema.TryGetProperty("properties", out var propertyElement)
            ? propertyElement
            : default;
        var declared = properties.ValueKind == JsonValueKind.Object
            ? properties.EnumerateObject().Select(property => property.Name)
                .ToHashSet(StringComparer.Ordinal)
            : [];

        if (schema.TryGetProperty("required", out var required))
        {
            var present = entries.Select(entry => entry.Key).ToHashSet(StringComparer.Ordinal);
            foreach (var item in required.EnumerateArray())
            {
                var key = item.GetString()!;
                if (!present.Contains(key))
                    errors.Add($"{relativePath}: {location} is missing required key {key}");
            }
        }

        if (schema.TryGetProperty("additionalProperties", out var additional)
            && additional.ValueKind == JsonValueKind.False)
        {
            foreach (var entry in entries.Where(entry => !declared.Contains(entry.Key)))
                errors.Add($"{relativePath}: {location} has undeclared key {entry.Key}");
        }

        if (schema.TryGetProperty("propertyOrder", out var propertyOrder))
        {
            var ranks = propertyOrder.EnumerateArray()
                .Select((item, index) => (item.GetString()!, index))
                .ToDictionary(item => item.Item1, item => item.index, StringComparer.Ordinal);
            var priorRank = -1;
            foreach (var entry in entries.Where(entry => ranks.ContainsKey(entry.Key)))
            {
                var rank = ranks[entry.Key];
                if (rank < priorRank)
                    errors.Add($"{relativePath}: {location} key {entry.Key} is out of contract order");
                priorRank = Math.Max(priorRank, rank);
            }
        }

        if (properties.ValueKind != JsonValueKind.Object)
            return;
        foreach (var entry in entries)
        {
            if (properties.TryGetProperty(entry.Key, out var childSchema))
                ValidateNode(entry.Value, childSchema, relativePath, location + "." + entry.Key, errors);
        }
    }

    private static void ValidateSequence(
        YamlSequenceNode sequence,
        JsonElement schema,
        string relativePath,
        string location,
        List<string> errors)
    {
        if (!schema.TryGetProperty("items", out var itemSchema))
            return;
        for (var index = 0; index < sequence.Children.Count; index++)
            ValidateNode(
                sequence.Children[index], itemSchema, relativePath, $"{location}[{index}]", errors);
    }

    private static bool MatchesAnyType(YamlNode node, JsonElement type) =>
        type.ValueKind switch
        {
            JsonValueKind.String => MatchesType(node, type.GetString()!),
            JsonValueKind.Array => type.EnumerateArray()
                .Any(item => MatchesType(node, item.GetString()!)),
            _ => true,
        };

    private static bool MatchesType(YamlNode node, string type) => type switch
    {
        "object" => node is YamlMappingNode,
        "array" => node is YamlSequenceNode,
        "string" => node is YamlScalarNode scalar && !IsYamlNull(scalar),
        "null" => node is YamlScalarNode scalar && IsYamlNull(scalar),
        "integer" => node is YamlScalarNode scalar
                     && scalar.Style is ScalarStyle.Plain or ScalarStyle.Any
                     && int.TryParse(
                         scalar.Value, NumberStyles.Integer, CultureInfo.InvariantCulture, out _),
        _ => true,
    };

    private static bool IsYamlNull(YamlScalarNode scalar) =>
        scalar.Style is ScalarStyle.Plain or ScalarStyle.Any
        && scalar.Value is null or "" or "~" or "null" or "Null" or "NULL";

    private static string? ScalarValue(YamlMappingNode mapping, string key) =>
        TryGet(mapping, key, out var node) && node is YamlScalarNode scalar
            ? scalar.Value
            : null;

    private static IReadOnlyList<string> SequenceValues(YamlMappingNode mapping, string key)
    {
        if (!TryGet(mapping, key, out var node) || node is not YamlSequenceNode sequence)
            return [];
        return sequence.Children.OfType<YamlScalarNode>()
            .Select(item => item.Value ?? "")
            .ToArray();
    }

    private static bool TryGet(YamlMappingNode mapping, string key, out YamlNode value)
    {
        foreach (var entry in mapping.Children)
        {
            if (entry.Key is YamlScalarNode scalar && scalar.Value == key)
            {
                value = entry.Value;
                return true;
            }
        }
        value = null!;
        return false;
    }

    private static IReadOnlyList<string> ReadH2Sections(IEnumerable<string> lines)
    {
        var sections = new List<string>();
        char? fence = null;
        foreach (var line in lines)
        {
            var trimmed = line.TrimStart();
            if (trimmed.StartsWith("```", StringComparison.Ordinal)
                || trimmed.StartsWith("~~~", StringComparison.Ordinal))
            {
                var marker = trimmed[0];
                if (fence is null)
                    fence = marker;
                else if (fence == marker)
                    fence = null;
                continue;
            }
            if (fence is not null)
                continue;

            var heading = Regex.Match(line, @"^##[ \t]+(.+?)[ \t]*$");
            if (heading.Success)
                sections.Add(heading.Groups[1].Value);
        }
        return sections;
    }

    private static IEnumerable<string> ReadMarkdownLinkDestinations(IEnumerable<string> lines)
    {
        char? fence = null;
        foreach (var line in lines)
        {
            var trimmed = line.TrimStart();
            if (trimmed.StartsWith("```", StringComparison.Ordinal)
                || trimmed.StartsWith("~~~", StringComparison.Ordinal))
            {
                var marker = trimmed[0];
                if (fence is null)
                    fence = marker;
                else if (fence == marker)
                    fence = null;
                continue;
            }
            if (fence is not null)
                continue;

            var prose = Regex.Replace(line, @"`+[^`]*`+", "");
            foreach (Match match in MarkdownLinkPattern.Matches(prose))
                yield return match.Groups["destination"].Value;
        }
    }
}
