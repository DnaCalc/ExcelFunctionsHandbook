---
schema: efh.function-page/v1
function_id: FUNC.VALUETOTEXT
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "Microsoft Support — VALUETOTEXT function"
    locator: "https://support.microsoft.com/en-us/office/valuetotext-function-5fff61a2-301a-4ab2-9ffa-0a5242a08fea"
    role: "documented signature, the two format modes, and the documented #VALUE! condition"
  - work: "OxFunc — FUNCTION_SLICE_VALUETOTEXT_CONTRACT_PRELIM.md"
    locator: "docs/function-lane/FUNCTION_SLICE_VALUETOTEXT_CONTRACT_PRELIM.md"
    role: "upstream outcome model for the two format modes and error propagation"
episodes: []
body_sections:
  - "What it computes"
  - "Arguments"
  - "Result and edge cases"
  - "Errors"
  - "Relationships"
  - "Notes for implementers"
  - "What has not been checked"
  - "Page vocabulary"
  - "Sources"
family: valuetotext_fn
role_in_family: "The scalar renderer: turns any single value into its text form in one of two
  registers."
---

## What it computes

`VALUETOTEXT` renders a value as text. Its reason to exist is that Excel has two different, equally
legitimate answers to "what does this value look like as text", and this function lets you ask for
either one.

**Concise (`format = 0`, the default)** returns the value as it would be *rendered in a cell with
General formatting*. This is the display register: it is what a human reading the sheet sees.
Microsoft's page states this equivalence explicitly, and it is the useful anchor — the output is
defined by reference to General-format rendering, not by an independent formatting rule.

**Strict (`format = 1`)** returns the value in a form suitable for *embedding in a formula*.
Microsoft's page describes it as including escape characters and row delimiters, and generating a
string that can be parsed when entered into the formula bar. Text is quoted, with inner quotes
escaped; Booleans, numbers and errors are left unquoted. This is the round-trip register: the point
is that pasting the result into the formula bar reconstructs the value.

The distinction is exactly the distinction between showing a value and *denoting* it. `"Hello"`
displays as `Hello` and denotes as `"Hello"`. The empty string displays as nothing and denotes as
`""` — and only the second of those two is recoverable.

For anyone building tooling on top of a workbook, that makes strict mode the interesting one: it is
the only ordinary worksheet function that produces a machine-reparseable rendering of an arbitrary
value.

## Arguments

`VALUETOTEXT(value, [format])`

| Argument | Required | Meaning (as documented by Microsoft) |
|---|---|---|
| `value` | yes | The value to return as text. |
| `format` | no | `0` (default) concise; `1` strict. Anything else is documented as `#VALUE!`. |

The `format` argument is a mode selector with exactly two admitted values, and this is one of the
few places in the text family where an out-of-range option is documented as an error rather than
being clamped or ignored. Do not pass `TRUE` expecting it to mean `1` without checking; whether the
logical coerces into the admitted set is not something this page asserts.

The first position takes *any* value, which is the function's whole point. It is not restricted to
scalars in the sense of rejecting other kinds — it is the scalar renderer, and how it responds to
an array argument is discussed below.

## Result and edge cases

The return kind is `Text`. Upstream's contract records the result as always a scalar text value.

- **Errors are rendered, not propagated, in one specific sense — and propagated in another.**
  Microsoft's strict-mode description says errors are left unquoted, which implies they are
  rendered as their error text. Upstream's contract, by contrast, records error values in the first
  argument as *propagating* in both format modes. These two readings can both be true of different
  situations — an error inside an array being rendered, versus an error arriving as the whole
  scalar argument — but the Handbook has not established which applies where. This is a genuine
  open point and it is listed below rather than resolved by assertion.
- **Numbers render as decimal strings** in concise mode, which means the General-format rendering,
  including its width-dependent behaviour in a cell. Whether `VALUETOTEXT`'s rendering depends on
  column width the way a cell's does is not established and is a probe worth running.
- **The empty string is distinguishable in strict mode** (`""`) and not in concise mode.
- **An empty referenced cell** delivers `Empty`, distinct from an omitted argument; see
  [the value universe](../model/01-value-universe.md).
- **Rich values** — linked data types, images, lambdas — have a core projection defined in
  [the value universe](../model/01-value-universe.md). What `VALUETOTEXT` does with a rich value,
  and whether it renders the payload or the projection, is not established here.
- **Array arguments.** [`ARRAYTOTEXT`](FUNC.ARRAYTOTEXT.md) is the function for arrays. Whether
  `VALUETOTEXT` lifts elementwise over an array or delegates to array rendering is a shape question
  the Handbook has not settled; see the probes below.

## Errors

| Error | Basis |
|---|---|
| `#VALUE!` when `format` is anything other than `0` or `1` | documented on Microsoft's page (retrieved for this entry) |
| error propagation from `value` | recorded in upstream OxFunc's contract for both format modes |

The tension between error *propagation* and error *rendering* noted above is unresolved; treat the
second row as upstream's reading rather than as settled Handbook fact.

## Relationships

- [`ARRAYTOTEXT`](FUNC.ARRAYTOTEXT.md) — the array counterpart, introduced alongside this function
  and sharing its `format` argument and its two registers. The pair splits by input shape: one
  value, or a grid.
- `TEXT` — the *formatting* function, and the one readers confuse with this. `TEXT(x, fmt)` applies
  a number-format string you supply; `VALUETOTEXT` applies General formatting or a fixed strict
  grammar. `TEXT` cannot render a logical or an error usefully; `VALUETOTEXT` is defined for every
  value kind.
- [`VALUE`](FUNC.VALUE.md) — despite the name, not the inverse. `VALUE` parses text into a number
  under locale rules; `VALUETOTEXT` renders any value as text. The nearest thing to an inverse of
  strict mode is the formula parser itself.
- `T` — the old narrow relative: returns the text if the argument is text and the empty string
  otherwise. It is a filter, not a renderer.
- `CONCAT` and `&` perform implicit to-text conversion by the general rule in
  [coercion and lifting](../model/02-coercion-and-lifting.md). That conversion is not guaranteed to
  agree with `VALUETOTEXT(x, 0)` for every kind, and where they differ is worth knowing.

## Notes for implementers

- **Do not write your own number renderer.** Concise mode is *defined* as General-format rendering,
  so it inherits every subtlety of that: the switch to scientific notation, the significant-digit
  budget, negative-zero handling. Reusing the engine's General formatter is the only way to stay
  consistent; a fresh `format!("{}", x)` will diverge.
- **Strict mode is a grammar, so write it as one.** Quoting, quote-escaping (doubling), the
  column separator, and the row separator are all part of a syntax that must reparse. The separator
  characters are locale-dependent in Excel's formula language, which means strict mode has a locale
  dependency that concise mode does not obviously share.
- **The two modes must agree on nothing.** Do not implement strict as concise-plus-quotes; the
  number renderings can legitimately differ.
- **Decide the error question explicitly** and test it: error as argument, error inside an array,
  and error as the `format` argument are three different paths.
- **Validate `format` before rendering**, so that `VALUETOTEXT(error_value, 7)` has a defined
  precedence between the two failures.

## What has not been checked

There is no Handbook vector suite for `VALUETOTEXT`, and no Handbook evidence record comparing any
implementation against Excel. Nothing on this page is a measurement. Upstream's own contract notes
that its evidence posture for this function is incomplete — a spec anchor is marked as still to be
attached from public references, and an empirical anchor is listed as required rather than present.
That is unusually candid and worth repeating rather than smoothing over.

The probes that would settle the open points:

1. **The error question.** `VALUETOTEXT(NA())` and `VALUETOTEXT(NA(), 1)` — does the result read
   `#N/A` as *text*, or is the call itself `#N/A`? `ISTEXT` around the result answers it in one
   cell, and it is the single most load-bearing unknown on this page.
2. **Array input.** `VALUETOTEXT({1,2})` — scalar rendering of the whole array, an elementwise
   spill, or an error. Compare with `ARRAYTOTEXT` on the same input.
3. **`format` coercion.** `VALUETOTEXT(1, TRUE)`, `VALUETOTEXT(1, "1")`, `VALUETOTEXT(1, 1.4)` —
   whether the admitted set is `{0,1}` after coercion and truncation, or before.
4. **Strict-mode escaping.** A string containing a double quote, a newline, and a formula-language
   separator character, checked by pasting the result back into the formula bar. That round trip is
   the actual specification of strict mode and is directly testable.
5. **Locale sensitivity of strict mode**, by running probe 4 on a machine whose formula separators
   differ.
6. **General-format dependence.** `VALUETOTEXT(1/3)` in a narrow column and a wide one, to see
   whether the rendering is column-dependent as a cell's display is.
7. **Rich values.** A linked data type and an uncalled `LAMBDA`, to see whether the core projection
   or something richer is rendered.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| concise mode | `format = 0`; the General-format display rendering |
| strict mode | `format = 1`; a rendering intended to reparse in the formula bar |
| register | Which of the two renderings is being asked for |
| core projection | The traditional-gamut value a rich value presents to legacy surfaces |
| round trip | Rendering a value and reparsing the rendering back to the same value |

## Sources

- Microsoft Support, VALUETOTEXT function —
  <https://support.microsoft.com/en-us/office/valuetotext-function-5fff61a2-301a-4ab2-9ffa-0a5242a08fea>
  (retrieved for this page; source of the signature, the two format-mode descriptions, and the
  documented `#VALUE!` for a `format` outside `{0,1}`).
- OxFunc `docs/function-lane/FUNCTION_SLICE_VALUETOTEXT_CONTRACT_PRELIM.md` — the upstream outcome
  model, the scalar-text result rule, error propagation in both modes, and the explicit statement
  that its spec and empirical anchors are not yet attached. Preliminary by its own statement.
- Handbook `content/model/01-value-universe.md` (value kinds, rich values and core projections,
  Empty versus Missing), `02-coercion-and-lifting.md` (to-text conversion, error propagation).
