# Holding several implementations of one function

Status: design, November 2026

Excel functions can differ across Excel builds, workbook settings and CPUs. This page says how the
Handbook holds that, and what CalcFunc, our Rust function library, needs in order to follow.

## The model

There is one function and one page. A function has zero or more **variants**, and each variant is
one record in the `variants` list of the function's JSON:

| Field | Meaning |
|---|---|
| `key.excel_build_range` | The Excel builds or workbook setting it applies to |
| `key.cpu` | The CPU class, where the CPU matters |
| `difference` | What is recorded, and against which other variant |
| `state`, `state_summary` | What we conclude about our Rust for this variant |
| `code` | The Rust that implements it, at the pinned commit |
| `evidence` | The recorded answers, or the public page |

A key says only what was recorded. Reading different inputs on different builds shows no difference
and no agreement, so a variant needs the same inputs read in both places. Service updates can
change an answer with no Excel build changing, and the deferred service functions hold no variants
for that reason.

`implementations` holds one record per implementation, naming language, repository, commit, state
and code. `suite` stays null until a Handbook test suite is published.

## What is recorded today

| Function | Axis | Recorded |
|---|---|---|
| `EXP` | CPU, build, OS together | 6 inputs one ULP apart between AMD Zen 2 and Intel Coffee Lake |
| `LN` | CPU, build, OS together | 17 inputs one ULP apart between the same machines |
| `GEOMEAN` | None yet | Root control in one build; no second environment compared |
| `WEEKNUM` | None yet | Two runs in one environment disagree on 479 inputs |
| `LEN`, `MID`, `FIND`, `SEARCH`, `REPLACE` | Workbook Compatibility Version | Versions 1 and 2 count surrogate pairs differently |

The EXP and LN records cannot name their cause, because CPU, build and operating system changed
together. The next useful reading is the same build on both CPUs.

## What CalcFunc needs

1. **A version profile, set by the host.** The default is the latest measured Excel. It holds the
   Excel build, the workbook's Compatibility Version and a CPU class. CalcFunc already takes the
   Compatibility Version from the environment, so this extends a path that exists.
2. **Variant selection in one place.** A catalogue entry keeps one spelling and one identity. Where
   behaviour differs, it holds a table of build ranges and CPU classes against kernels, and the
   profile picks a row. A function with one behaviour has one row and no extra cost.
3. **A software model of each CPU class.** On x86_64 the x87 kernels run the CPU's own instructions,
   so today `EXP` and `LN` follow the machine they run on. A run on an Intel host gave the Intel
   answer on all 23 recorded inputs and the AMD answer on none. A profile that promises an answer
   on any machine must not depend on the host, so the microcode differences need a software form.
4. **Per-variant measurement.** The bank judge and `witness-status.tsv` need the profile as a key
   column, so that each variant has its own rows, matches and worst ULP. The Handbook then takes
   each variant's state from those rows, with no other change.
5. **Recorded answers per environment.** Each variant needs its own readings. Excel builds move, so
   new builds keep being read and compared on the same inputs.

The Handbook needs nothing further: `tools/efh-calcfunc` regenerates these records from the pinned
commit, and a new variant appears on its function page when its data lands.
