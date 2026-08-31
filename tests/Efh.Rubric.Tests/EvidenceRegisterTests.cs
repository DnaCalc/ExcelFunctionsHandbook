using System.Text;
using System.Text.Json;
using Efh.Rubric;
using Xunit;

namespace Efh.Rubric.Tests;

public sealed class EvidenceRegisterTests
{
    [Fact]
    public void The_evidence_register_is_a_complete_fresh_projection_of_every_F5_record()
    {
        var expected = EvidenceRegisterGenerator.Generate(HandbookFixture.Root);
        var registerPath = Path.Combine(
            HandbookFixture.Root, "content", "evidence", "REGISTER.json");
        Assert.True(File.Exists(registerPath), "run 'efh evidence register' first");

        var actual = File.ReadAllBytes(registerPath);
        Assert.True(actual.SequenceEqual(Encoding.UTF8.GetBytes(expected)),
            "content/evidence/REGISTER.json is stale; run 'efh evidence register'");

        using var document = JsonDocument.Parse(actual);
        var root = document.RootElement;
        var sourceCount = Directory.EnumerateFiles(
            Path.Combine(HandbookFixture.Root, "content", "evidence", "records"), "*.json").Count();
        Assert.Equal(sourceCount, root.GetProperty("records_indexed").GetInt32());
        Assert.Equal(sourceCount, root.GetProperty("records").GetArrayLength());
        Assert.All(root.GetProperty("records").EnumerateArray(), record =>
            Assert.False(record.GetProperty("handbook_reverified").GetBoolean()));
    }
}
