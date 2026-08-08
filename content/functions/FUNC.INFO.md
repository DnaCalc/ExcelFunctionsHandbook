---
schema: efh.function-page/v1
function_id: FUNC.INFO
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Page vocabulary
  - Sources
family: info_fn
role_in_family: Sole member of its module; the only worksheet function whose subject is the
  application itself.
---

## What it computes

`INFO(type_num)` returns one fact about the running Excel environment, selected by name.

Its subject is not a value, not a cell and not a workbook — it is the **application and the
machine it is running on**. That makes it the most host-dependent function in the worksheet
surface, and it is declared accordingly: `HostInteractionClass::WorkbookState`,
`FecDependencyProfile::Composite`, `VolatilityClass::VolatileContextual`,
`ThreadSafetyClass::HostSerialized`.

The consequence governs the page: **a function engine with no host cannot answer an `INFO` call
at all.** The Handbook's reference engine declines every one of its battery inputs with
`cannot-call: requires-host-facility: composite`. That is the correct answer, not a coverage
gap. `INFO` can only ever be characterized against a running Excel, on a named platform, at a
named version — and its answers *are* the platform and the version, so its results are
build-scoped by definition rather than by caution.

Microsoft's published `type_text` table, verbatim:

| `Type_text` | Returns |
|---|---|
| `"directory"` | Path of the current directory or folder from Excel startup option |
| `"numfile"` | Number of worksheets in the open workbooks |
| `"origin"` | Absolute cell reference of top-left visible cell (Lotus 1-2-3 compatibility) |
| `"osversion"` | Current operating system version as text |
| `"recalc"` | Current recalculation mode: "Automatic" or "Manual" |
| `"release"` | Version of Microsoft Excel as text |
| `"system"` | Operating environment name: "mac" or "pcdos" |

Two further documented facts sit alongside that table and are as important as the table itself:

1. **Three names were removed.** `"memavail"`, `"memused"` and `"totmem"` are no longer
   supported and return `#N/A`. They reported memory figures in earlier Excel versions. A
   workbook written before the removal still parses and now silently returns errors — one of the
   cleanest examples in Excel of a function whose *domain* shrank across versions.
2. **`INFO` is unavailable in Excel Web App.** The function is a platform-availability boundary,
   not merely a platform-varying answer.

The reference engine's declared parser accepts ten names — the seven live ones plus the three
withdrawn ones, which it routes to the host rather than short-circuiting. So `INFO` is one of
the few functions where the interesting question is not "what does it compute" but "which of
these ten names still answers on the build in front of you".

## Arguments

`type_num` — required, exactly one. The published signature is `INFO(type_num)`, and the name is
a historical misnomer: the argument is **text**, not a number. Microsoft's own table column is
headed `Type_text`.

It is coerced to text and matched case-insensitively after trimming, so `"RECALC"` and
`" recalc "` are the same request. A name outside the recognised set is a failure, not a null
result.

There is no second argument. Unlike `CELL`, `INFO` has no reference position at all — there is
nothing to point it at.

## Result and edge cases

The return kind depends on the name: `Text` for `"directory"`, `"origin"`, `"osversion"`,
`"recalc"`, `"release"` and `"system"`; `Number` for `"numfile"`.

- **`"origin"` is reference-style dependent.** Its documented result differs between A1 and R1C1
  reference styles, and it is scoped to the top-left *visible* cell — so scrolling the window
  changes it. This is the purest volatility on the page: no data changed, no formula changed, the
  answer moved because the user scrolled.
- **`"numfile"` counts worksheets across all open workbooks**, not sheets in the current one.
  Opening an unrelated file changes it.
- **`"recalc"` reports application state** that the user can toggle from the ribbon.
- **`"system"` returns `"mac"` or `"pcdos"`** — the second string is a fossil, and any workbook
  branching on it is branching on 1980s vocabulary.
- **The three withdrawn names return `#N/A`.**
- **Cross-platform and cross-version divergence is the normal case, not the exception.** Every
  answer this function gives is a property of the environment, so two machines are *expected* to
  disagree. A test suite for `INFO` cannot have fixed expected values; it can only have
  invariants (e.g. `"recalc"` is one of two strings) and per-environment recordings.
- **Security posture.** `"directory"`, `"osversion"` and `"release"` expose local environment
  detail into cell values, which then travel with the file. That is worth knowing before putting
  `INFO` into a shared workbook.

## Errors

Microsoft's page documents `#N/A` for the three withdrawn names. The Handbook has read the
`type_text` table and the removal note and does not have a further documented error table to
reproduce.

What the reference engine's declared contract produces, stated as the reference engine's
behaviour and not as Excel's:

- an unrecognised `type_num` name — `#VALUE!`
- an arity failure (zero arguments, or two) — `#VALUE!`
- no host available — the call is refused rather than answered
- a host that recognises the name but cannot answer it — `#VALUE!`

## Relationships

- **`CELL`** is the sibling one level down: same category, same host dependence, same
  cannot-evaluate-without-Excel property, but its subject is a cell rather than the application.
  Between them they are the two functions in this assignment for which the Handbook can hold no
  computed answers at all.
- **`SHEETS()`** with no argument counts sheets in the current workbook; `INFO("numfile")` counts
  them across every open workbook. Confusing the two is easy and the difference only shows up
  when a second file is open.
- **`CELL("filename", …)`** is the usual route to the workbook path, and is generally preferred
  over `INFO("directory")` because it names the file rather than the startup folder.
- **`NOW()` and `TODAY()`** are the other functions whose answers come from the environment
  rather than from arguments, but they are cleanly volatile in a way the dependency graph
  understands; `INFO`'s volatility is contextual and messier.
- **`ISFORMULA`, `SHEET`, `SHEETS`, `FORMULATEXT`** are the other host-backed members of the
  Information category, all of which at least take a reference.

## Notes for implementers

1. **`INFO` must be a declared host query with an explicit no-host failure.** There is no
   correct local answer. Synthesising `"release"` or `"osversion"` from the runtime the engine
   happens to be running on produces a plausible lie.
2. **Keep the withdrawn names in the parser, routed to the host.** Short-circuiting them to
   `#N/A` in the function layer hard-codes a version fact into a version-independent layer, and
   it is the host that knows whether it still supports them.
3. **The argument is text despite being called `type_num`.** Do not add a numeric path.
4. **Any recorded result must carry platform, build and locale.** `INFO`'s outputs are the very
   axes the Handbook's claim rules require claims to be scoped by
   ([claim language and honesty](../model/06-claim-language.md)), so an unscoped `INFO`
   expectation is meaningless by construction.
5. **Do not cache.** `"recalc"`, `"numfile"` and `"origin"` can all change without any workbook
   edit.

## What has not been checked

No Handbook vector suite exists for `INFO`; `vectors/` publishes nothing for this function. No
Excel-comparison evidence record names `INFO` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.**

As with `CELL`, the stronger statement applies: **there are no implementation-side answers to
compare against either.** The reference engine refuses every battery input for want of a host.
For this function the Handbook holds Microsoft's documentation and the declared structure, and
nothing more.

What a host-backed harness would have to establish, and why each item is not a formality:

1. **Which of the ten names answer on a current build.** The removal of `"memavail"`,
   `"memused"` and `"totmem"` is documented; whether any of the surviving seven has since
   changed behaviour is not, and this function has a track record of losing names.
2. **`"system"` on a current Mac and a current Windows build.** The documented answers are
   `"mac"` and `"pcdos"`; confirming that a modern build still says `"pcdos"` is worth doing
   precisely because it would be surprising if it did not, and surprising if it did.
3. **`"release"` across channels.** Its value is the version string, so this probe is
   simultaneously a test of `INFO` and a way to record the observation context that every other
   Handbook claim needs.
4. **`"origin"` under both reference styles and after scrolling**, to pin the documented
   style-dependence and to demonstrate the volatility.
5. **Behaviour on the web platform**, where the function is documented as unavailable —
   unavailable how? A `#NAME?`, a `#VALUE!`, a refusal at entry? That is a real question with
   three plausible answers.
6. **`"numfile"` with one workbook open and with three**, to confirm the cross-workbook scope
   that the one-line documentation states and that almost everyone misreads.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| host query | A call answered by the application and machine rather than by computation |
| `type_text` | The text name selecting which environment fact `INFO` returns |
| withdrawn name | A `type_text` value that once answered and now returns `#N/A` |
| environment-scoped | A result that is a property of the platform, so cannot have a fixed expectation |
| `VolatileContextual` | Declared volatility: the answer can change with no workbook edit |

## Sources

- Microsoft, "INFO function" —
  <https://support.microsoft.com/en-us/office/info-function-725f259a-0e4b-49b3-8b52-58815c69acae>.
  Read for this page: the full `type_text` table verbatim, the note that `"memavail"`,
  `"memused"` and `"totmem"` are no longer supported and return `#N/A`, the statement that
  `INFO` is unavailable in Excel Web App, and the reference-style dependence of `"origin"`.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — host-side adaptation as an
  engine obligation and the caller-aware function class.
- Handbook, [the execution context](../model/04-execution-context.md) — the application and
  workbook state a host query reads.
- Handbook, [version axes](../model/05-version-axes.md) — why a result that *is* the version
  needs the version recorded around it.
- Handbook, [claim language and honesty](../model/06-claim-language.md) — the scoping rules that
  make an unscoped `INFO` expectation meaningless.
- `data/functions/FUNC.INFO.json` — identity (`xlfInfo`, code 244), the published signature
  `INFO(type_num)`, arity 1–1, and the declared host-interaction, dependency, volatility and
  thread-safety axes, as projected at OxFunc `473efa3`.
