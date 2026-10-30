using System.Security.Cryptography;
using System.Text;
using System.Text.Json;
using Xunit;

namespace Efh.Rubric.Tests;

/// <summary>
/// Completeness and association honesty for the full W28 Microsoft Support localization organ.
/// These tests intentionally fail until efh-ingest has regenerated data from the named OxFunc
/// source; they prevent a later projection from silently recreating the old GUID cross-product.
/// </summary>
public class LocalizationCompletenessTests
{
    private const string RawOrgan = "data/localization/function-name-index.json";
    private const string SourceSha256 = "b248fbe16fc977c8287ed99b3996218393543c1855d08c1e8436470b28193231";
    private const string RowsSha256 = "fed477527136c40f9230320ca999e36f6a03685d4f28ccf9c4d07315eb07eb25";

    private static readonly HashSet<string> CombinedRows = new(StringComparer.Ordinal)
    {
        "FUNC.FIND", "FUNC.FINDB", "FUNC.LEFT", "FUNC.LEFTB", "FUNC.LEN", "FUNC.LENB",
        "FUNC.MID", "FUNC.MIDB", "FUNC.REPLACE", "FUNC.REPLACEB", "FUNC.RIGHT", "FUNC.RIGHTB",
        "FUNC.SEARCH", "FUNC.SEARCHB"
    };

    private static readonly HashSet<string> UnresolvedSharedArticles = new(StringComparer.Ordinal)
    {
        "FUNC.ISBLANK", "FUNC.ISERR", "FUNC.ISERROR", "FUNC.ISLOGICAL", "FUNC.ISNA",
        "FUNC.ISNONTEXT", "FUNC.ISNUMBER", "FUNC.ISODD", "FUNC.ISREF", "FUNC.ISTEXT"
    };

    private static readonly HashSet<string> MissingRows = new(StringComparer.Ordinal)
    {
        "FUNC.FORECAST.LINEAR", "FUNC.IMAGINARY"
    };

    private static readonly HashSet<string> EnglishNameMatches = new(StringComparer.Ordinal)
    {
        "FUNC.FORECAST"
    };

    [Fact]
    public void Raw_localization_organ_is_the_complete_unabridged_W28_matrix()
    {
        using var raw = Open(RawOrgan);
        var root = raw.RootElement;
        Assert.Equal("efh.localization.function-name-index/v2", root.GetProperty("schema").GetString());
        var source = root.GetProperty("source");
        Assert.Equal("function-name-index", source.GetProperty("scope").GetString());
        Assert.Equal("OxFunc", source.GetProperty("repository").GetString());
        Assert.EndsWith("W28_FUNCTION_NAME_LOCALIZATION_LIBRARY_SEED.csv",
            source.GetProperty("path").GetString(), StringComparison.Ordinal);
        Assert.Equal(SourceSha256, source.GetProperty("sha256").GetString());
        AssertGuidCommit(Text(source, "commit"));

        using var index = Open("data/index.json");
        var ingestCommit = Text(index.RootElement.GetProperty("provenance"), "oxfunc_commit_at_ingest_full");
        using var debris = Open("content/model/name-debris.json");
        var measuredCommit = Text(debris.RootElement.GetProperty("measured_over"), "measured_at_oxfunc_commit");
        Assert.Equal(ingestCommit, Text(source, "commit"));
        Assert.Equal(ingestCommit, measuredCommit);

        var rows = root.GetProperty("rows").EnumerateArray().ToList();
        Assert.Equal(20_360, rows.Count);
        Assert.Equal(40 * 509, rows.Count);
        Assert.Equal(RowsSha256, Sha256(JsonSerializer.Serialize(root.GetProperty("rows"))));

        var localeGroups = rows.GroupBy(r => Text(r, "locale_tag"), StringComparer.Ordinal).ToList();
        Assert.Equal(40, localeGroups.Count);
        Assert.All(localeGroups, locale => Assert.Equal(509, locale.Count()));

        var articleGroups = rows.GroupBy(r => Text(r, "function_article_guid"), StringComparer.Ordinal).ToList();
        Assert.Equal(500, articleGroups.Count);
        Assert.Equal(499, articleGroups.Count(group => group.Count() == 40));
        Assert.Single(articleGroups, group => group.Count() == 400);
        foreach (var article in articleGroups)
        {
            var byLocale = article.GroupBy(r => Text(r, "locale_tag"), StringComparer.Ordinal).ToList();
            Assert.Equal(40, byLocale.Count);
            Assert.All(byLocale, locale => Assert.Equal(article.Count() / 40, locale.Count()));
        }

        foreach (var row in rows)
        {
            foreach (var field in new[] { "locale_tag", "list_article_guid", "function_article_guid",
                         "localized_name", "localized_function_url", "localized_category" })
                Assert.False(string.IsNullOrWhiteSpace(Text(row, field)), $"blank raw localization field {field}");
            AssertExactGuid(Text(row, "list_article_guid"));
            var functionGuid = Text(row, "function_article_guid");
            AssertExactGuid(functionGuid);
            Assert.True(Uri.TryCreate(Text(row, "localized_function_url"), UriKind.Absolute, out var uri),
                $"invalid localization URL: {Text(row, "localized_function_url")}");
            Assert.Equal(Uri.UriSchemeHttps, uri!.Scheme);
            Assert.Equal("support.microsoft.com", uri.Host);
            Assert.Equal("support.microsoft.com", uri.Authority);
            Assert.True(uri.IsDefaultPort);
            Assert.EndsWith(functionGuid, uri.AbsolutePath, StringComparison.Ordinal);
            Assert.EndsWith(functionGuid, Text(row, "localized_function_url"), StringComparison.Ordinal);
        }
    }

    [Fact]
    public void Function_pages_associate_to_raw_rows_without_cross_products_or_combined_labels()
    {
        var pages = FunctionPages();
        Assert.Equal(541, pages.Count);
        var functions = pages.Where(p => Text(p, "entry_kind") == "built_in_function").ToList();
        var operators = pages.Where(p => Text(p, "entry_kind") == "built_in_operator").ToList();
        Assert.Equal(518, functions.Count);
        Assert.Equal(23, operators.Count);
        Assert.All(operators, p => Assert.Equal(JsonValueKind.Null, p.GetProperty("localization").ValueKind));

        using var raw = Open(RawOrgan);
        var rawByGuid = raw.RootElement.GetProperty("rows").EnumerateArray()
            .GroupBy(row => Text(row, "function_article_guid"), StringComparer.Ordinal)
            .ToDictionary(group => group.Key, group => group.ToList(), StringComparer.Ordinal);
        var handbookFunctionsByGuid = functions
            .GroupBy(page => Text(page.GetProperty("localization"), "function_article_guid"), StringComparer.Ordinal)
            .Where(group => !string.IsNullOrEmpty(group.Key))
            .ToDictionary(group => group.Key, group => group.Count(), StringComparer.Ordinal);

        var byStatus = functions.GroupBy(p => Text(p.GetProperty("localization"), "status"))
            .ToDictionary(g => g.Key, g => g.ToList(), StringComparer.Ordinal);
        Assert.Equal(491, byStatus["article_unique"].Count);
        Assert.Single(byStatus["english_name_match"]);
        Assert.Equal(14, byStatus["combined_row"].Count);
        Assert.Equal(10, byStatus["unresolved_shared_guid"].Count);
        Assert.Equal(2, byStatus["missing_source_row"].Count);

        foreach (var page in functions)
        {
            var id = Text(page, "function_id");
            var association = page.GetProperty("localization");
            Assert.Equal(RawOrgan, Text(association, "raw_organ"));
            var functionGuid = Text(association, "function_article_guid");
            AssertExactGuid(functionGuid);
            var rawRows = rawByGuid.GetValueOrDefault(functionGuid) ?? [];
            Assert.Equal(rawRows.Count, association.GetProperty("article_row_count").GetInt32());
            var names = page.GetProperty("localized_names").EnumerateArray().ToList();
            var status = Text(association, "status");
            if (status is "article_unique" or "english_name_match")
            {
                Assert.Equal(40, names.Count);
                Assert.Equal(40, names.Select(n => Text(n, "locale")).Distinct(StringComparer.Ordinal).Count());
                Assert.Equal(40, association.GetProperty("resolved_name_count").GetInt32());
                var expected = rawRows
                    .Select(row => (Locale: Text(row, "locale_tag"), Name: Text(row, "localized_name")))
                    .OrderBy(row => row.Locale, StringComparer.Ordinal)
                    .ThenBy(row => row.Name, StringComparer.Ordinal)
                    .ToList();
                var actual = names
                    .Select(row => (Locale: Text(row, "locale"), Name: Text(row, "name")))
                    .OrderBy(row => row.Locale, StringComparer.Ordinal)
                    .ThenBy(row => row.Name, StringComparer.Ordinal)
                    .ToList();
                Assert.Equal(expected, actual);

                if (status == "article_unique")
                {
                    Assert.Equal("function", Text(association, "scope"));
                    Assert.Equal(1, handbookFunctionsByGuid[functionGuid]);
                }
                else
                {
                    Assert.Equal("shared_article", Text(association, "scope"));
                    Assert.True(handbookFunctionsByGuid[functionGuid] > 1);
                    var englishRow = Assert.Single(rawRows, row => Text(row, "locale_tag") == "en-US");
                    Assert.Equal(Text(page, "surface_name"), Text(englishRow, "localized_name"));
                }
            }
            else
            {
                Assert.Empty(names);
                Assert.Equal(0, association.GetProperty("resolved_name_count").GetInt32());
            }

            if (CombinedRows.Contains(id))
                Assert.Equal(("shared_published_row", "combined_row"),
                    (Text(association, "scope"), Text(association, "status")));
            if (UnresolvedSharedArticles.Contains(id))
            {
                Assert.Equal(("shared_article", "unresolved_shared_guid"),
                    (Text(association, "scope"), Text(association, "status")));
                Assert.Equal(400, association.GetProperty("article_row_count").GetInt32());
            }
            if (MissingRows.Contains(id))
                Assert.Equal(("function", "missing_source_row"),
                    (Text(association, "scope"), Text(association, "status")));
        }

        Assert.True(EnglishNameMatches.SetEquals(IdSet(byStatus["english_name_match"])));
        Assert.True(CombinedRows.SetEquals(IdSet(byStatus["combined_row"])));
        Assert.True(UnresolvedSharedArticles.SetEquals(IdSet(byStatus["unresolved_shared_guid"])));
        Assert.True(MissingRows.SetEquals(IdSet(byStatus["missing_source_row"])));
    }

    [Fact]
    public void Every_function_has_official_English_help_and_F0_reports_the_association_partition()
    {
        var pages = FunctionPages();
        foreach (var page in pages.Where(p => Text(p, "entry_kind") == "built_in_function"))
        {
            Assert.False(string.IsNullOrWhiteSpace(Text(page.GetProperty("docs"), "microsoft_url")));
            // No Microsoft English description is carried (DnaCalc dc-96l.5, 2026-10-30).
            Assert.Equal(JsonValueKind.Null, page.GetProperty("docs").GetProperty("english_description").ValueKind);
            Assert.False(string.IsNullOrWhiteSpace(Text(page.GetProperty("descriptions"), "short")));
            Assert.Equal("dna-calc-own", Text(page.GetProperty("descriptions"), "short_source"));
        }

        foreach (var id in new[] { "FUNC.COPILOT", "FUNC.JIS" })
        {
            var page = pages.Single(p => Text(p, "function_id") == id);
            var vintage = page.GetProperty("vintage");
            Assert.Equal(Text(vintage, "localization"), Text(vintage, "docs"));
            Assert.Equal(Text(vintage, "docs"), Text(vintage, "descriptions"));
            Assert.Contains("W28_FUNCTION_NAME_LOCALIZATION_LIBRARY_SEED.csv", Text(vintage, "docs"), StringComparison.Ordinal);
        }

        using var index = Open("data/index.json");
        var counts = index.RootElement.GetProperty("counts");
        Assert.Equal(20_360, counts.GetProperty("localization_raw_rows").GetInt32());
        Assert.Equal(40, counts.GetProperty("localization_locales").GetInt32());
        Assert.Equal(509, counts.GetProperty("localization_english_rows").GetInt32());
        var resolved = counts.GetProperty("localization_resolved_name_pages").GetInt32();
        var articleUnique = counts.GetProperty("localization_article_unique_name_pages").GetInt32();
        var englishNameMatch = counts.GetProperty("localization_english_name_match_pages").GetInt32();
        Assert.Equal(492, resolved);
        Assert.Equal(491, articleUnique);
        Assert.Equal(1, englishNameMatch);
        Assert.Equal(resolved, articleUnique + englishNameMatch);
        Assert.Equal(14, counts.GetProperty("localization_shared_published_row_pages").GetInt32());
        Assert.Equal(10, counts.GetProperty("localization_unresolved_shared_article_pages").GetInt32());
        Assert.Equal(2, counts.GetProperty("localization_missing_source_row_pages").GetInt32());
    }

    [Fact]
    public void Name_debris_measurements_reference_the_raw_corpus_and_publish_the_full_BETA_shape()
    {
        using var debris = Open("content/model/name-debris.json");
        var root = debris.RootElement;
        var measured = root.GetProperty("measured_over");
        Assert.Equal("data/localization/function-name-index.json :: rows[].localized_name",
            Text(measured, "corpus"));
        Assert.Equal(20_360, measured.GetProperty("raw_rows").GetInt32());
        Assert.Equal(SourceSha256, Text(measured, "source_sha256"));
        Assert.DoesNotContain("24280", root.GetRawText(), StringComparison.Ordinal);
        var beta = root.GetProperty("trailing_digit_rule").GetProperty("measured");
        Assert.Equal(36, beta.GetProperty("rows_matching_exact_BETA_INVn").GetInt32());
        Assert.Equal(2, beta.GetProperty("rows_matching_spaced_BETA_INVn_variants").GetInt32());
        Assert.Equal(38, beta.GetProperty("rows_matching_the_BETA_INVn_shape").GetInt32());
    }

    private static string Sha256(string value) =>
        Convert.ToHexString(SHA256.HashData(Encoding.UTF8.GetBytes(value))).ToLowerInvariant();

    private static void AssertExactGuid(string value)
    {
        Assert.True(Guid.TryParseExact(value, "D", out var parsed), $"invalid GUID: {value}");
        Assert.Equal(parsed.ToString("D"), value);
    }

    private static void AssertGuidCommit(string value)
    {
        Assert.Equal(40, value.Length);
        Assert.All(value, character =>
            Assert.True(character is >= '0' and <= '9' or >= 'a' and <= 'f',
                $"commit is not lowercase hexadecimal: {value}"));
    }

    private static HashSet<string> IdSet(IEnumerable<JsonElement> pages) =>
        pages.Select(p => Text(p, "function_id")).ToHashSet(StringComparer.Ordinal);

    private static List<JsonElement> FunctionPages() =>
        Directory.EnumerateFiles(Path.Combine(HandbookFixture.Root, "data", "functions"), "*.json")
            .Order(StringComparer.Ordinal)
            .Select(path => JsonDocument.Parse(File.ReadAllText(path)).RootElement.Clone())
            .ToList();

    private static JsonDocument Open(string relativePath) =>
        JsonDocument.Parse(File.ReadAllText(Path.Combine(HandbookFixture.Root,
            relativePath.Replace('/', Path.DirectorySeparatorChar))));

    private static string Text(JsonElement element, string property) =>
        element.GetProperty(property).GetString() ?? string.Empty;
}
