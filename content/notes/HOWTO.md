# How to add a function note

A note records one thing about an Excel function that someone matching Excel, or explaining an
Excel result, needs to know. Notes are harvested in batches from findings that have already landed
(reports, evidence, product commits). Writing a note is never a step of a fix, and no other
repository's work or commit depends on one.

1. Copy an existing `content/notes/FN-*.json` and rename it `FN-<FUNCTION>-<nnn>.json` (the next
   free number for that function; `<FUNCTION>` as in `data/functions/FUNC.<FUNCTION>.json`).
2. Fill it in:
   - `functions`: every `FUNC.*` id it applies to, sorted.
   - `kind`: `quirk`, `excel-differs-from-math` (needs a repro with `correct`, the exact
     mathematically correct value), `bit-matching-detail`, or `documentation-differs` (needs
     `documentation`).
   - `statement`: what Excel does, with its scope. Only what the evidence shows.
   - `status`: `measured` (read off recorded Excel answers) or `inferred` (goes beyond them; put the
     inferred part in `inferred`). `basis` says what the status rests on.
   - `repros`: the call, the exact inputs (`display` must parse to `bits`), Excel's answer,
     `recorded_in` (Excel build, platform, CPU) and `source` (where the reading is kept).
   - `evidence`: repository, full commit, path, locator, for the product commit, research write-up
     and readings.
3. Run `dotnet run --project tools/efh/Efh.Rubric.Cli -- notes render` from the repository root.
   It checks the notes and writes the Notes section of each named function page, `NOTES.md` and
   `site/api/notes.json`. `notes check` does the same without writing.
4. Run `dotnet test tests/Efh.Rubric.Tests` and commit.

Do not write "bit-exact" or "bit-for-bit" (CHARTER 7.8); say what matched, on how many rows, in which
build. The format is `tools/schemas/f20-function-note.schema.json`.
