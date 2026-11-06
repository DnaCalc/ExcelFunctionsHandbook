---
schema: efh.function-page/v1
function_id: FUNC.TRANSLATE
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references:
  - work: "TRANSLATE function"
    locator: "https://support.microsoft.com/en-us/excel/functions/translate-function"
    role: "public source read for this page"
  - work: "DETECTLANGUAGE function"
    locator: "https://support.microsoft.com/en-us/excel/functions/detectlanguage-function"
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

# TRANSLATE

## What it does

`TRANSLATE` sends text to Microsoft Translation Services and returns the translation. Excel only carries the request;
the service decides the answer.

`TRANSLATE(text, [source_language], [target_language])`

- **`text`** is quoted text or a cell reference. It must be text.
- **`source_language`** is a language code such as `"en"`. If omitted, the service detects the language.
- **`target_language`** is a language code such as `"es"`. If omitted, the system language is used.

```
=TRANSLATE("Hello","en","es")
```

## Try it

## Quirks

- **The default target belongs to the machine.** Two people opening one workbook on different systems can get
  different translations from one formula. Name `target_language` in anything that travels.
- **Detection is weakest on short text.** Microsoft recommends naming the source language for short strings, which
  is what cells usually hold.
- **Do not treat the result as a key.** Nothing promises the same text next time, and supported languages can change.
- **Translating there and back does not return the original.**

### When the service is unavailable

Microsoft documents failures as messages about the call:

| Message | Meaning |
|---|---|
| Text Too Long | The cell holds too many characters; no figure is given |
| Error in Value | The input is not text |
| Invalid Language | The code is wrong or unsupported |
| Request Throttled | The daily quota is used up; no figure is given |

The page does not say what appears with no connection, nor how a saved result behaves on recalculation. Neither has
been measured here.

## How to implement

DNA Calc does not run this function yet. It is deferred by policy, because the answer depends on a back end Microsoft
hosts and controls. An implementation needs:

1. **A translation service.** Any other engine gives other words, so it could match Excel's shape but not its answers.
2. **Language handling:** code validation against the service's current list, and the host's system language as the
   default target.
3. **Asynchronous calculation**, with caching so recalculation does not spend quota or change results.
4. **The documented failures**, including throttling and the length limit.

The function needs no arithmetic of its own. Its text handling is limited to coercing the arguments.

## Version differences

No Excel answers for this function are recorded here, so no comparison between Excel builds exists. Microsoft's page lists Excel for Microsoft 365 on Windows and Mac and Excel Mobile. Service
updates can change answers with no Excel build changing, so a build comparison would not separate the two.

## Alternatives

- Translate once and store the result as values, when a report has to stay fixed.
- A lookup table of approved terms, when wording must be controlled.
- `DETECTLANGUAGE` to find the source language first.

## Sources

- [TRANSLATE function](https://support.microsoft.com/en-us/excel/functions/translate-function)
- [DETECTLANGUAGE function](https://support.microsoft.com/en-us/excel/functions/detectlanguage-function)
