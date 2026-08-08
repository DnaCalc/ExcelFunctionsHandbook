---
schema: efh.function-page/v1
function_id: FUNC.DETECTLANGUAGE
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
family: null
role_in_family: null
---

`DETECTLANGUAGE` takes text and returns the language code the service believes it is written
in. It is a classifier exposed as a worksheet function, and Microsoft says so about as plainly
as documentation ever does: **"This function is a service backed function. The supported
languages and results may vary as languages are added or removed."**

That sentence is the most important thing on this page, and it is Microsoft's, not the
Handbook's. It is an explicit disclaimer that the function's output is not stable over time. A
worksheet function that documents its own results as liable to change is a genuinely new kind
of object in this catalog.

The reference engine (OxFunc) records `DETECTLANGUAGE` as **deferred**, with the reason
**"Deferred external/provider function."** There is no implementation module and no live
registry entry — the projection is catalog-only, with no recorded arity, signature or
classification axes. It is one of the sparsest entries in this batch, and that sparseness is
itself accurate: there is nothing to model locally.

## What it computes

Given text, it returns a **language code** — not a language name. Microsoft's example is that
detecting Spanish displays the code `es`. The code vocabulary is the one used by Azure AI
Services Translator, maintained separately from the function page.

Three properties of the classification are worth stating, because they determine whether the
function is fit for a given purpose:

1. **It returns one answer, not a distribution.** There is no confidence score and no
   runner-up. A cell holding one ambiguous word gets a definite-looking code with no signal
   that the classifier was guessing.
2. **It cannot say "I don't know".** The documented failure conditions are about the *call*
   (too long, non-text, throttled), not about the *classification*. What an unclassifiable
   input returns is not documented.
3. **Short text is the hard case.** Microsoft's guidance on the companion `TRANSLATE` function
   — specify the language when known, "especially for shorter texts" — is an acknowledgement
   that detection degrades on short input. Spreadsheet cells are usually short input.

## Arguments

`DETECTLANGUAGE(text)`

- **`text`** (required) — Microsoft: "The text or reference to cells containing text to
  evaluate."

The reference engine records no arity for this id, because there is no live registry entry;
the one-argument shape comes from the documented signature only.

Note the plural in Microsoft's wording — "reference to cells" — which leaves open whether a
multi-cell reference is accepted and, if so, whether the result spills. The Handbook does not
know, and lists it as a probe below rather than assuming.

Availability is documented as Excel for Microsoft 365, Excel for Microsoft 365 for Mac, and
Excel Mobile.

## Result and edge cases

A text language code.

- **Results may change without the workbook changing.** Microsoft's disclaimer, quoted above,
  covers both the supported-language set and the results themselves. A recalculation months
  later may reclassify the same cell.
- **Mixed-language text has no documented behaviour.** A cell containing an English sentence
  and a French one gets one code, and which one is unspecified.
- **Empty and whitespace input have no documented behaviour.**
- **Non-text values are rejected rather than coerced.** Microsoft's error text is explicit:
  "You have a non-text value in your cell. The function only accepts a text argument." That is
  a departure from the ordinary to-text coercion path described in
  [coercion and lifting](../model/02-coercion-and-lifting.md), where numbers and logicals
  convert to text rather than failing. This function is documented as declining that
  conversion — a per-function policy, and one the Handbook has not verified.

## Errors

Microsoft documents three conditions, quoting the messages the user sees rather than naming
worksheet error values:

| Condition | Documented message |
|---|---|
| text too long | "You have too many characters in a cell. Reduce your cell size and try again." |
| non-text value | "You have a non-text value in your cell. The function only accepts a text argument." |
| throttled | "You have exceeded your daily quota of the translation function." |

Two observations the Handbook can make honestly about this list:

- **No thresholds are given.** Neither the character limit nor the daily quota is quantified,
  so neither can be designed around. A workbook that works today can fail tomorrow at a
  boundary nobody can compute.
- **No error *values* are given.** The messages are user-facing text; what a formula can branch
  on — `#VALUE!`, `#BUSY!`, `#CONNECT!`, `#BLOCKED!` — is not stated. The Handbook will not
  guess which condition maps to which code.

## Relationships

- **`TRANSLATE`** — the companion, and effectively the consumer. `DETECTLANGUAGE` is
  `TRANSLATE`'s automatic source-language detection made visible as its own function, which is
  why it is the right tool for auditing what auto-detection concluded. The two share a service,
  a quota, an availability list, and an error vocabulary.
- **`COPILOT`** — the catalog's other service-backed, model-output function. Same
  reproducibility questions, different scope.
- **`INFO`, `CELL`** — the functions readers reach for when they want to know something about
  the *environment*'s language. `DETECTLANGUAGE` says nothing about the workbook or the
  installation; it inspects the text.
- **`UNICODE`, `CODE`, `LEN`** — the local text-inspection functions. The distinction is worth
  drawing explicitly: those are total functions of their input and computable offline, while
  `DETECTLANGUAGE` sits in the Text category and is neither.

## Notes for implementers

- There is no kernel and no possibility of one. Any implementation is a client for a
  classification service, and the classifier is the specification.
- The documented refusal to coerce non-text arguments, if confirmed, must be declared as a
  per-function deviation rather than inherited from the shared coercion path.
- The service disclaimer means results are not a suitable regression-test target. An
  implementation's test suite must test the call and the error surface, not the answers — the
  same conclusion the Handbook reaches for its own suites below.
- Quota is part of the observable contract even though it is unquantified; an implementation
  without one is more permissive than the documented function.
- The code vocabulary belongs to Azure AI Services Translator and moves independently of Excel.
  Pinning a fixed code list in an implementation will drift.

## What has not been checked

No Handbook vector suite exists for `DETECTLANGUAGE`, and no Excel-comparison evidence is
recorded. Nobody has checked this function against Excel in the Handbook's record. Everything
above is Microsoft's documented behaviour, cited as such, or explicitly flagged as unknown.

A value-comparison suite is impossible in principle, and unusually, Microsoft says so itself:
results "may vary as languages are added or removed". Testing the classifications would be
testing a moving service. What is characterizable is the **call surface**, and every open
question below is about that rather than about classification quality:

1. **Non-text arguments.** `DETECTLANGUAGE(5)`, `DETECTLANGUAGE(TRUE)`, a date, an empty cell,
   and an error value. The documented rejection contradicts the shared to-text coercion
   baseline, and this is the cleanest such disagreement in the batch.
2. **Error-value mapping.** Each of the three documented conditions provoked deliberately,
   recording the actual error value rather than the message text. The documentation gives
   messages; formulas need codes.
3. **Multi-cell input.** A range argument, to resolve the ambiguity in "reference to cells" —
   scalar result, spilled array, implicit intersection, or error.
4. **Empty and degenerate input.** Empty string, a single space, a single letter, a digit
   string, and a punctuation-only string.
5. **The length threshold.** A binary search on input length to locate the too-long boundary,
   and whether it counts UTF-16 code units as the value-universe chapter's text model would
   suggest.
6. **Stability within a session.** The same input evaluated repeatedly, across a recalculation
   and a reopen. Not a test of correctness — a test of whether the function is even a function
   of its argument at a single point in time.
7. **Code form.** Whether returned codes are ever regional (`zh-Hans`, `pt-BR`) or always short
   two-letter codes, which determines whether a workbook can compare the result to a literal.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: deferred | OxFunc intentionally defers the function; the recorded reason is "Deferred external/provider function." |
| catalog-only | The projection carries identity and documentation metadata but no implementation module, live registry entry, arity or signature |
| service backed | Microsoft's own term: the results and supported set may vary over time |
| language code | The short identifier returned, drawn from the Azure AI Services Translator vocabulary |
| throttling | Refusal of service after a daily quota, documented without a stated limit |

## Sources

- Microsoft, "DETECTLANGUAGE function" —
  <https://support.microsoft.com/en-us/office/detectlanguage-function-0748e285-1912-4d24-b735-57d18142fa3b>
  (syntax, the `text` argument, the language-code return with the Spanish example, the
  service-backed disclaimer quoted above, the availability list, and the three documented
  error conditions with their exact message text).
- Microsoft, "TRANSLATE function" —
  <https://support.microsoft.com/en-us/office/translate-function-d34f71c7-2ffe-409a-9a63-5eb5e91aa3dd>
  (the shared service, the auto-detection relationship, and the short-text guidance quoted
  above).
- Microsoft / Azure AI Services Translator, "Supported Languages and Language Codes" — the code
  vocabulary this function returns, referenced from the function page and not reproduced here.
- Handbook `data/functions/FUNC.DETECTLANGUAGE.json` — admission label and reason, category
  "Text functions", and the catalog-only metadata status with no recorded arity or signature.
- Handbook `data/presence/FUNC.DETECTLANGUAGE.json` — no implementation module, no dispatch
  entry.
- Handbook `content/model/02-coercion-and-lifting.md` — the to-text coercion path this
  function's documentation departs from.
