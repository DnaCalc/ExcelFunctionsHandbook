# Version and platform axes

Status: draft (H1) · Sources: OxFunc 937f198

## Why one statement needs three scopes

"ASINH of 1.4e154 returns #NUM!" — is that true? It was observed, bit for bit, on a specific
Excel build, on a specific machine, verified by a specific set of tests. On a different build,
Microsoft may have rewritten the routine. In a workbook saved in a different compatibility
mode, the same build may evaluate the same formula differently. And for some functions, the
last bit of the result depends on which CPU family the machine uses.

A statement about an Excel function's exact behavior is a measurement, and a measurement
without its instrument setup is an anecdote. The Handbook therefore treats every exact-behavior
claim as scoped along three independent axes:

1. the Excel application build (and its update channel),
2. the workbook compatibility mode,
3. the platform and hardware.

A claim that does not name its position on all three is not an exact-behavior claim; it is at
best a summary. This chapter explains each axis and how the Handbook writes the scope down.

## Axis 1: the application build

Excel is a moving target. Builds ship continuously along update channels, and function behavior
changes across them in three distinct ways:

1. **New functions appear.** A function page must say from which version the function exists at
   all; before that, its name in a cell is just an unrecognized name.
2. **Algorithms get rewritten.** The best-known case is the statistical library rewrite shipped
   with Excel 2010: a large family of distribution functions received new algorithms and a
   parallel set of new dotted names (BETA.DIST, CHISQ.INV, and so on), while the old names were
   retained. The rewrite changed the exact numbers these functions return, and Microsoft never
   documented the new algorithms — their behavior is knowable only by observation.
3. **Individual results shift between builds.** Even without an announced rewrite, a numeric
   kernel can change from one build to the next. Only a claim pinned to a build can be checked.

For this reason every empirical observation behind the Handbook records the exact build it was
made against (for example "Excel 16.0 build 20026") and, where relevant, the update channel
that produced that build. Two observations from different builds are different facts, even when
they agree.

## Axis 2: the workbook compatibility mode

The same Excel build does not evaluate every workbook the same way. Some evaluation behavior is
selected per workbook, not per installation:

1. **Workbook compatibility version.** Excel carries a workbook-level compatibility setting
   that can toggle evaluation behavior — most visibly around the dynamic-array formula model.
   Conformance observations therefore record a workbook compatibility version alongside the
   build (the reference environment behind this Handbook pins it explicitly in every test
   record). A claim scoped to one workbook mode says nothing about the other.
2. **The date system.** A workbook is either in the 1900 date system or the 1904 date system,
   and every date-valued function reads and produces serial numbers under the workbook's
   system. The 1900 system additionally preserves Excel's historical leap-year quirk: serial 60
   is the nonexistent date February 29, 1900, and all later dates are offset by it. This is a
   workbook-level environment choice, not a property of any single function.
3. **Names, aliases, and storage forms.** One function identity can surface under several
   written forms. Modern functions are serialized under an `_xlfn.` name prefix when a workbook
   travels through contexts that predate them; for example, the implicit-intersection operator
   written `@` in modern Excel is stored as `_xlfn.SINGLE(...)` in pre-dynamic-array form, and
   modern Excel normalizes it back. These are compatibility representations of a single
   function identity — not separate functions with separate semantics — and the Handbook treats
   them as storage facts, not behavior facts.

The practical consequence: when a result differs between two machines running the same build,
the workbook mode is a prime suspect, and a claim that omits it can produce false alarms in
both directions.

## Axis 3: platform and hardware

Most of Excel's arithmetic is portable. Ordinary double-precision operations (addition,
multiplication, division, square root) are IEEE-754 binary64 operations that produce identical
bits on every mainstream CPU. Claims about functions built only from these operations carry
platform scope trivially — they hold everywhere.

Some of Excel's kernels are not in that class. On x86-64 Windows, Excel evaluates EXP, LN,
LOG10, LOG, and POWER through legacy x87 floating-point instruction sequences inherited from
the old Microsoft C runtime: intermediates live in 80-bit x87 registers, and the result is
rounded to a 64-bit double only at the final store. Two properties follow:

1. **The extended-precision path is observable.** The 80-bit intermediates make these results
   differ, on identifiable inputs, from what any straightforward double-precision
   implementation produces. Matching Excel bit for bit on these functions means reproducing the
   x87 sequence, not just the mathematics.
2. **The last bit can be CPU-family-scoped.** The core x87 instructions involved (F2XM1,
   FYL2X) are implemented in CPU microcode — a vendor-specific approximation, not a
   mathematically defined rounding. On roughly one in two thousand of the hardest inputs, the
   exact result bit tracks the microcode of the machine Excel runs on. Excel on ARM runs a
   software x87 emulation, a different instrument again.

The Handbook therefore distinguishes two strengths of numeric claim. A **value-level claim**
("EXP(710) returns #NUM!") is platform-independent. A **bit-level claim** (the exact 64 bits of
EXP(0.5)) is portable for SSE2-only kernels, but for x87-backed kernels it is scoped to the CPU
family on which it was verified, and the page says so.

## How the Handbook states scope

Every exact-behavior claim on a function page carries three tags:

1. an **Excel build tag** — the build (and channel, where known) the behavior was observed on,
2. a **platform tag** — either portable, or the CPU family scope of a bit-level claim,
3. a **suite version tag** — the version of the Handbook's test suite that verified the claim,
   so a reader can re-run exactly the probes that back it.

Rather than repeating full scope on every line, pages lean on a **current baseline**: a pinned
reference environment (Excel build and channel, locale, workbook compatibility mode, and
platform) against which claims are verified by default. A claim with no explicit tags is a
claim about the current baseline. When the baseline advances to a newer build, previously
verified claims are not silently promoted — they remain attributed to the baseline under which
they were verified until they are re-verified.

One more rule, inherited from the evidence sources: **when documentation and observed behavior
differ, the page says so.** Microsoft's documentation describes ASINH by its defining formula,
which is finite for every representable number; the product returns #NUM! beyond about
1.34e154, because its internal formula overflows first. The Handbook reports the observed
behavior as the behavior, cites the build it was observed on, and explicitly marks the
documentation discrepancy rather than averaging the two into vague language.

## Localized function names

Function names are a display and entry surface, not an identity. The same function is written
SUM in an English locale and SUMME in a German one; Microsoft's own reference keys every
function to a stable identifier that all localized names map onto. The Handbook does the same:
pages are keyed by stable function identity, localized names are listed as data about the entry
surface, and no behavior claim is ever scoped to a name. (Locale can affect actual evaluation
elsewhere — argument separators, text coercion, date formats — but that is evaluation-context
behavior, tracked in its own right; the name itself is pure surface.)

## Page vocabulary

| Label on a function page | Meaning |
|---|---|
| Excel build tag (e.g. "Excel 16.0 build 20026") | The exact application build the claim was verified against |
| Channel tag | The update channel that produced the recorded build |
| Workbook mode tag (e.g. compatibility version, date system) | The workbook-level evaluation mode in force during verification |
| Platform tag: portable | The claim holds bit-for-bit on any IEEE-754 binary64 platform |
| Platform tag: CPU-family-scoped (e.g. "x86-64, x87-verified") | A bit-level claim verified on this CPU family; last-bit behavior elsewhere is unverified |
| Suite version tag | The version of the Handbook test suite whose run backs the claim |
| Current baseline | The pinned build + channel + locale + workbook mode + platform that untagged claims refer to |
| Verified from / introduced in | The earliest version at which the function (or the claimed behavior) is known to exist |
| Documentation discrepancy | Documented and observed behavior differ; the page reports the observed behavior and flags the difference |

## Sources

All paths are in the OxFunc repository at commit 937f198.

| Artifact | Evidentiary role |
|---|---|
| `CHARTER.md` | Declares dual-axis version tracking (application build/channel and workbook compatibility version) as first-class scope, and the rule that documented-vs-observed divergences are recorded and resolved in favor of observed behavior |
| `docs/function-lane/EXCEL_FUNCTION_DEFINITION_DISCUSSION.md` | D-010: workbook compatibility version as a required evaluation axis in probe matrices, to prevent false regressions when version-scoped behavior is intentional |
| `docs/function-lane/DATE_SERIAL_SYSTEM_AND_WORKBOOK_MODE_NOTES.md` | The 1900/1904 date system as a workbook-level mode; the serial-60 leap-year quirk; a worked example of a pinned current baseline (build, channel, locale, workbook lanes) |
| `docs/EXCEL_MATH_DEVIATION_CATALOG.md` | Build-cited empirical deviations between documented mathematics and observed Excel results (ASINH overflow to #NUM!, among others), each entry pinned to a live Excel build |
| `docs/KNOWN_EXACTNESS_DEVIATIONS.md` | Residual-tracking rules, including: behavior proven to be version/channel- or compatibility-scoped is split into versioned evidence rather than treated as tolerance |
| `crates/oxfunc_core/src/excel_numeric/mod.rs` | Module documentation: EXP/LN/LOG paths are bit-exact only via the x87 backend on x86-64; portable fallback is faithful but not bit-exact |
| `crates/oxfunc_core/src/excel_numeric/x87.rs` | Header documentation: the legacy x87 CRT sequences, 80-bit intermediates, microcode-dependent F2XM1/FYL2X, approximate 1-in-2000 CPU-dependent inputs, ARM software x87 emulation |
| `docs/function-lane/FUNCTION_SLICE_OP_IMPLICIT_INTERSECTION_CONTRACT_PRELIM.md` | `_xlfn.SINGLE(...)` and `@` as compatibility/serialization representations of one operator identity, not separate semantics |
| `docs/function-lane/W28_FUNCTION_NAME_LOCALIZATION_LIBRARY_SEED.csv` | Forty-locale function-name table keyed by stable per-function identifiers (e.g. SUM/SUMME under one identity); also carries per-function version markers (e.g. "2010" for the statistical rewrite family) |
| `docs/function-lane/W109_GAMMALN_RESUME.md` | Records that the Excel 2010 statistical rewrite is undocumented at the algorithm level — behavior recoverable only by observation |
