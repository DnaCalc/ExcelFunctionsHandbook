---
schema: efh.function-page/v1
function_id: FUNC.TRANSLATE
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
family: number_regex_translate_family
role_in_family: The provider-language member — shares its module with local text and number functions but is the only one whose answer comes from a remote service.
---

`TRANSLATE` sends text to Microsoft Translation Services and puts the translation in a cell. It
is a worksheet function whose result is a **model output**: not derived from its arguments by a
rule, not reproducible from a specification, and not guaranteed stable over time.

The reference engine (OxFunc) records it as **deferred**, with a reason more specific than the
rest of this batch: **"Deferred from the current-version completion target even though W036
already pinned a narrow provider-language seam baseline."** Read carefully, that says two
things at once — the function is out of scope for this version's completion target, *and* some
work has already been done on the seam through which provider-language functions are expressed.
The projection reflects that: `metadata_status: function_meta_curated`, a registry-backed
signature with arity 1–3, and `special_interface_kind: provider_language_request` — a
classification that exists for this shape of function. Its axes record
`fec_dependency_profile: ExternalProvider`, `host_interaction_class: ExternalProvider` and
`HostSerialized`.

One recorded axis deserves a flag rather than a restatement. The projection lists
`determinism_class: Deterministic`. For a function whose answer is produced by a remote
translation service that is retrained and updated over time, that classification is at best a
statement about the *call shape* rather than about the answers. The Handbook records the value
and does not endorse it as a description of observable behaviour; nobody has checked whether
`TRANSLATE` returns the same string for the same input across time or across accounts.

## What it computes

`TRANSLATE(text, [source_language], [target_language])` returns `text` rendered in
`target_language`. The mapping is performed by Microsoft Translation Services; Excel is the
transport.

The structurally interesting part is that **both language arguments are optional, and each
omission means something different**:

- Omitting `source_language` invokes **automatic detection** — the service infers the input
  language from the text. Microsoft notes that auto-detection is supported for most languages,
  and recommends specifying the language when it is known, "especially for shorter texts".
  Short strings are exactly where detection is least reliable, and a cell often contains a
  short string.
- Omitting `target_language` falls back to **the system language**. That is a property of the
  machine, not of the workbook. The same file, opened by two colleagues with differently
  localized installs, produces different translations from an identical formula — and neither
  cell shows any sign of it.

That second default is the most important practical fact on this page. Any workbook that will
travel should name `target_language` explicitly.

Language codes are the ordinary short codes (`"en"`, `"es"`); Microsoft maintains the
authoritative list of supported languages and codes separately from the function page.

## Arguments

`TRANSLATE(text, [source_language], [target_language])` — arity 1 to 3, matching the reference
engine's recorded arity.

- **`text`** (required) — the text to translate, quoted or supplied by cell reference.
- **`source_language`** (optional) — the source language code; auto-detected when omitted.
- **`target_language`** (optional) — the target language code; the system language when
  omitted.

Availability is documented as Excel for Microsoft 365, Excel for Microsoft 365 for Mac, and
Excel Mobile — a narrower and *newer* platform set than the Web functions, and notably one that
includes Mac, which `WEBSERVICE` and `FILTERXML` do not.

## Result and edge cases

Text. The edges that matter are the ones a spreadsheet author will actually hit:

- **The result is not stable by construction.** Two calls at different times may differ. Nothing
  in the documentation promises otherwise, and treating a `TRANSLATE` result as a key, an
  identifier, or a value to reconcile against is a mistake.
- **It is not invertible.** `TRANSLATE(TRANSLATE(x, "en", "fr"), "fr", "en")` is not `x`. This
  is obvious when stated and is nonetheless a common workbook construction.
- **Length is bounded, and the bound is not stated.** Microsoft documents an error for text
  that is too long without giving the threshold.
- **Quota is bounded, and the bound is not stated.** Microsoft documents a throttling error for
  exceeding a daily translation quota, again without a number. A workbook full of `TRANSLATE`
  cells can therefore work on Monday and fail on Tuesday, at a boundary nobody can compute in
  advance.
- **Non-text inputs are rejected rather than coerced.** Microsoft lists an error for non-text
  values, which is a departure from the usual to-text coercion path described in
  [coercion and lifting](../model/02-coercion-and-lifting.md). Whether that means numbers and
  logicals genuinely error — rather than being stringified as they are elsewhere in Excel — is
  not something the Handbook has checked, and it is the first probe below.

## Errors

Microsoft's page documents four failure conditions in prose rather than as an error-value
table:

| Condition | As documented |
|---|---|
| text too long | an excessive character count that must be reduced |
| non-text value | non-text values are rejected |
| invalid language | an unsupported or incorrect language code |
| throttling | the daily translation quota has been exceeded |

The Handbook cannot say which worksheet error *values* these map to. That is a real gap: the
documentation names the conditions but not the codes, and the codes are what a formula can
branch on. Given the function's provider dependence, `#VALUE!`, `#BUSY!`, `#CONNECT!` and
`#BLOCKED!` are all plausible for different members of that list, and the Handbook is not going
to guess which.

## Relationships

- **`DETECTLANGUAGE`** — the companion function, and effectively the `source_language`
  auto-detection step exposed on its own. The pair is: detect, then translate. Anyone
  wondering what auto-detection concluded should ask `DETECTLANGUAGE` directly.
- **`WEBSERVICE`** — the way this was done before `TRANSLATE` existed: call a translation API
  by URL and parse the response. Windows-only, unlicensed, and entirely manual. `TRANSLATE`
  replaces that pattern with a first-class function.
- **`COPILOT`** — the other function in the catalog whose result is a model output. The two
  raise the same reproducibility questions and are best read together.
- **`LOWER`, `UPPER`, `PROPER`, `TEXT`** — the text functions readers group it with, and the
  contrast is the point: those are total functions of their arguments, and `TRANSLATE` is not.
  It sits in the Text category and does not behave like anything else there.

## Notes for implementers

- There is no kernel and there cannot be one. Any implementation is a client for a translation
  service, and its results are that service's results.
- The system-language default for `target_language` is a host dependency and must be modelled
  as one. An implementation that hard-codes English produces different answers from Excel for
  every non-English user.
- The `Deterministic` classification in the projection should be treated with care by anyone
  building on it: it cannot mean "same input, same output, forever" for a service-backed
  function. If it means "the call shape is a pure function of the arguments", that is worth
  stating explicitly, because the two readings lead to very different caching strategies.
- Quota and throttling are part of the observable contract even though neither is quantified.
  An implementation with no quota is more permissive than the documented function, and a
  workbook developed against it will fail in Excel.
- The apparent refusal to coerce non-text arguments, if confirmed, is a genuine per-function
  deviation from the shared to-text path and should be declared as such rather than inherited.

## What has not been checked

No Handbook vector suite exists for `TRANSLATE`, and no Excel-comparison evidence is recorded.
Nobody has checked this function against Excel in the Handbook's record. Every behavioural
statement above is Microsoft's documented behaviour, cited as such, or explicitly flagged as
unknown.

A conventional value-comparison suite is impossible: the translations are model outputs and
have no fixed correct answer. What *is* characterizable is the **argument and error surface**,
which is a function of the call rather than of the model, and which is where the unanswered
questions actually are. The probes, in order:

1. **Non-text arguments.** `TRANSLATE(5)`, `TRANSLATE(TRUE)`, `TRANSLATE(A1)` with `A1` empty,
   and `TRANSLATE(#N/A)`. This tests the documented rejection of non-text values against the
   shared coercion baseline, and it is the cleanest disagreement between this function's
   documentation and the rest of the model.
2. **Error-value mapping.** Each of the four documented conditions provoked deliberately —
   over-long text, an invalid code such as `"zz"`, a non-text value, and (harder) quota
   exhaustion — recording the exact error value in each case. The documentation names
   conditions but not codes; this closes that gap.
3. **The length threshold.** A binary search on input length to find where the too-long error
   begins, and whether the threshold counts UTF-16 code units (as the value-universe chapter's
   text model would suggest) or something else.
4. **The `target_language` default.** The same formula on two differently localized Excel
   installations, which is the only way to confirm the system-language fallback.
5. **Language-code forms.** `"en"`, `"EN"`, `"en-GB"`, `"eng"`, and an empty string, to pin how
   permissive code parsing is.
6. **Stability.** The same input translated repeatedly within a session, across a
   recalculation, across a reopen, and on a second machine — to record whether the result is
   stable at all, which is the question the `Deterministic` classification raises.
7. **Round trip.** Translate and translate back, recording the drift. Not a correctness test —
   a demonstration for the page that the function is not invertible.
8. **Empty and whitespace input.** An empty string and a single space, where auto-detection has
   nothing to work with.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: deferred | OxFunc intentionally defers the function; the recorded reason names the W036 provider-language seam baseline |
| `provider_language_request` | The special interface kind the projection records for this function |
| auto-detection | Inferring `source_language` from the text when the argument is omitted |
| system language | The machine's configured language, used as `target_language` when that argument is omitted |
| throttling | Refusal of service after a daily quota, documented without a stated limit |
| model output | A result produced by a remote service rather than by a rule over the arguments |

## Sources

- Microsoft, "TRANSLATE function" —
  <https://support.microsoft.com/en-us/office/translate-function-d34f71c7-2ffe-409a-9a63-5eb5e91aa3dd>
  (syntax, all three arguments, the auto-detection and system-language defaults, the
  recommendation to specify the language for short texts, the availability list, and the four
  documented failure conditions).
- Microsoft's separately maintained "Supported Languages and Language Codes" list, referenced
  from the function page — the authoritative code vocabulary, not reproduced here.
- Handbook `data/functions/FUNC.TRANSLATE.json` — admission label and reason, arity 1–3,
  signature `TRANSLATE(text, [source_language], [target_language])`, `metadata_status:
  function_meta_curated`, `special_interface_kind: provider_language_request`, and the
  classification axes quoted and questioned above.
- Handbook `data/presence/FUNC.TRANSLATE.json` — the implementing module
  `crates/oxfunc_core/src/functions/number_regex_translate_family.rs` and its surface-dispatch
  entry.
- Handbook `content/model/01-value-universe.md` and `02-coercion-and-lifting.md` — the UTF-16
  text model and the to-text coercion path this function's documentation appears to depart from.
