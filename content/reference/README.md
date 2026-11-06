# Function reference records

One handbook-owned `FUNC.<NAME>.json` per catalogue entry. `data/` stays importer-owned.
The generator combines this record with function Markdown and measurement notes into the stable
`/api/functions/FUNC.<NAME>.json`. The [public schema](../../tools/schemas/function-reference.schema.json)
describes that generated record. `llms.txt` explains agent access.

- `try_it`: an English Excel `formula`, optional `inputs` object for referenced cells.
- `sections`: optional Markdown overrides keyed by `what-it-does`, `quirks`, `how-to-implement`,
  `version-differences`, `alternatives`. Otherwise the generator divides the existing essay by subject.
- `versions`: observations by `excel_build` and `platform`, with `behaviour` and `sources`.
  Unknown build/platform is `null`; an observation on one build does not establish a difference.
- `implementations`: `language`, `flavour`, `source`, `suite`. List only after a published
  Handbook suite passes; implementation guidance in prose can describe an algorithm before that.
- `variants`: `name` (for example a future `EXP.PERFECT`), `difference`, `error_bound`,
  `onecalc_udf`. The example name is not an existing implementation or accuracy claim.
  An error bound is null or an object with `metric`, `bound`, `domain`, `basis`.
  A UDF hook is null or an object with `name`, `language`, `source` pointing to OneCalc-loadable code.

Keep missing records as empty arrays and unknown measurements as null. Never invent a build
number, measured error bound, executable UDF, or suite. Preserve build, platform, input route and
numeric bits when adding observations. Sources appear in a small footer on the human page.
