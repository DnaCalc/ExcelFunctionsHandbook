---
schema: efh.function-page/v1
function_id: FUNC.DETECTLANGUAGE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "DETECTLANGUAGE function"
    locator: "https://support.microsoft.com/en-us/excel/functions/detectlanguage-function"
    role: "public source read for this page"
  - work: "TRANSLATE function"
    locator: "https://support.microsoft.com/en-us/excel/functions/translate-function"
    role: "public source read for this page"
episodes: []
body_sections:
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Sources
family: null
role_in_family: null
---

# DETECTLANGUAGE

## What it does

`DETECTLANGUAGE` asks Microsoft Translation Services which language some text is in, and returns a language code such
as `es`. It returns a code, not a language name.

`DETECTLANGUAGE(text)` takes text, or a reference to cells holding text.

```
=DETECTLANGUAGE("Hola mundo!")
```

## Try it

## Quirks

- **One answer, no confidence.** There is no score, so an ambiguous word gets a definite-looking code.
- **Answers may change.** Microsoft calls this a service-backed function, whose languages can change over time.
- **Numbers are not read as text.** Microsoft describes a non-text value as an error, unlike the ordinary conversion
  of numbers to text.
- **Undocumented cases:** empty text, mixed-language text, and whether a multi-cell reference spills.

### When the service is unavailable

Microsoft documents three failure messages:

| Message | Meaning |
|---|---|
| Text Too Long | The cell holds too many characters; no figure is given |
| Error in Value | The input is not text |
| Request Throttled | The daily quota is used up; no figure is given |

The page does not say what appears with no connection. This has not been measured here.

## How to implement

DNA Calc does not run this function yet. It is deferred by policy, because the answer depends on a back end Microsoft
hosts and controls. An implementation needs:

1. **A language classifier.** Another model will disagree on short and mixed text, so it cannot promise Excel's answers.
2. **The code vocabulary** of the service, kept current.
3. **Asynchronous calculation** with caching, and the documented failures.
4. **Measured argument rules:** non-text values, empty text and multi-cell references.

## Version differences

No Excel answers for this function are recorded here, so no comparison between Excel builds exists. Microsoft's page lists Excel for Microsoft 365 on Windows and Mac and Excel Mobile. Service
updates can change answers with no Excel build changing.

## Alternatives

- `TRANSLATE` with `source_language` omitted, when only the translation matters.
- A column of known language codes, when the source of the text is known.

## Sources

- [DETECTLANGUAGE function](https://support.microsoft.com/en-us/excel/functions/detectlanguage-function)
- [TRANSLATE function](https://support.microsoft.com/en-us/excel/functions/translate-function)
