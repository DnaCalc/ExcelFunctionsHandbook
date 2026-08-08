---
schema: efh.function-page/v1
function_id: FUNC.PROPER
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
family: text_search_replace_family
role_in_family: The case transformer with a word-boundary rule; the only member that rewrites letters rather than locating or substituting them.
---

# PROPER

## What it computes

`PROPER(text)` rewrites the case of every letter in `text` according to one rule, applied
character by character:

> A letter is uppercased if it is the first character of `text` or if the character before it
> is **not a letter**. Every other letter is lowercased.

That is Microsoft's documented rule, and reading it literally is the whole story — because
what it says is *not* "capitalize each word". It says "capitalize after any non-letter". The
two descriptions diverge constantly, and the divergences are the reason this function has a
reputation:

| Input | Result under the documented rule |
|---|---|
| `"o'brien"` | `O'Brien` — the apostrophe is not a letter |
| `"2nd place"` | `2Nd Place` — the digit is not a letter |
| `"e-mail"` | `E-Mail` |
| `"McDonald"` | `Mcdonald` — the interior capital is lowercased |
| `"IBM"` | `Ibm` |

None of these is a bug. All five follow from one rule with no word list, no dictionary, and no
exceptions. `PROPER` is a character classifier, not a name formatter, and it should not be
used as one.

The rule is also **unconditional on the rest of the string**: `PROPER` lowercases letters it
did not capitalize, so it destroys existing casing rather than preserving it. This is what
separates it from a "capitalize first letters" transformation.

## Arguments

| Argument | Meaning |
|---|---|
| `text` | The string whose case is rewritten. Required, and the only argument. |

There is no locale argument, no exception list, and no option to preserve existing capitals.
The Handbook's projected signature for this entry is a placeholder — the metadata layer carries
the one-argument arity but not a rendered parameter list — so the argument name above is taken
from Microsoft's documentation rather than from the reference engine's registry.

## Result and edge cases

Returns `Text`.

- Empty `text` returns the empty string.
- Non-letter characters pass through unchanged; only letters are rewritten.
- Numbers and logicals arriving in the argument slot are converted to text first under the
  shared coercion rules, then processed — see
  [Coercion and lifting](../model/02-coercion-and-lifting.md). What the resulting text looks
  like for a number is a to-text question, not a `PROPER` question, and the shared chapter is
  explicit that to-text behaviour is only outlined, not pinned.
- Error values propagate.

The question this page cannot answer is what counts as "a letter" and what "uppercase" means
for scripts where the answer is not obvious: Turkish dotted and dotless i, German ß, Greek
final sigma, and the scripts that have no case at all. Microsoft's page does not address any
of them. The implementing module named in the presence projection carries two open upstream
array-support defect streams (`BUG-FUNC-008`, `BUG-FUNC-016`), so the array shape of this
function is also unsettled.

## Errors

Microsoft's `PROPER` page documents no error conditions of its own. The errors reachable here
are the shared ones: an error value in the argument propagates, and a value that cannot be
converted to text surfaces `#VALUE!` under the shared coercion rules. The Handbook has not
verified either against Excel.

## Relationships

- **`UPPER` and `LOWER`** are the unconditional case transforms. All three lowercase by
  default; `PROPER` differs only in which characters it raises back up.
- **`TEXTSPLIT` / `TEXTBEFORE` / `TEXTAFTER`** are what you actually want if the task is
  "capitalize each word" with a definition of "word" that is not "run of letters" — `PROPER`
  cannot be configured, so word-level work has to be done around it.
- There is no compatibility-replacement pair here: `PROPER` is neither superseded nor a
  retained legacy name.
- Readers confuse `PROPER` with `TRIM`, because both are reached for while cleaning imported
  names. They share no behaviour: `TRIM` deals in spaces, `PROPER` in case.

## Notes for implementers

The whole function is a one-pass scan with one bit of state (was the previous character a
letter?), so the interest is entirely in the character classification.

Three decisions have to be made, and Microsoft's documentation makes none of them:

1. **What "is a letter" means.** ASCII `A-Za-z`, the Unicode `L*` general categories, and the
   host code page's letter table are three different answers, and they differ on accented
   characters, on the Latin Extended blocks, and on modifier letters.
2. **What case mapping is applied.** Simple per-character mapping and full case mapping
   disagree — `ß` uppercases to `SS` under full mapping, which changes the string's length.
3. **Whether the mapping is locale-sensitive.** Turkish `i`/`İ` is the standing example. A
   locale-invariant mapping is the safer default but is a decision, not a neutral choice.

Record whichever you pick as unverified. All three are the sort of thing that silently agrees
with Excel on English data and silently disagrees on everything else.

## What has not been checked

No Handbook vector suite exists for `PROPER`, and no Excel-comparison evidence record names
it. Nobody has checked this function against Excel within the Handbook's record. The rule
stated at the top is Microsoft's documented rule, quoted, not an observation.

Inputs worth probing first:

1. **`PROPER("o'brien 2nd e-mail")`** — one cell that exercises the apostrophe, the digit and
   the hyphen together. If the documented rule holds, this is `O'Brien 2Nd E-Mail`.
2. **`PROPER("IBM")`** — confirms the destructive lowercasing, which is the behaviour users
   most often assume is absent.
3. **Accented and non-ASCII letters**: `PROPER("émile ÜBER")`, to establish whether the letter
   class is ASCII or Unicode.
4. **`PROPER("straße")` and `PROPER("ǅ")`** — simple versus full case mapping, and the
   title-case digraph, which is the one character where "uppercase" has three answers.
5. **Turkish `i` under a Turkish locale**, which is the only probe on this list that requires a
   second machine configuration.
6. **Array input** `PROPER({"ab","cd"})`, given the open `BUG-FUNC-008` and `BUG-FUNC-016`
   streams on array support in this implementing module.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| letter | The character class the capitalization rule keys on; its exact membership is undefined by the documentation |
| full case mapping | Case conversion that may change string length (`ß` → `SS`) |
| destructive lowercasing | `PROPER` lowercases every letter it does not capitalize, discarding existing case |

## Sources

- Microsoft, "PROPER function" —
  <https://support.microsoft.com/en-us/office/proper-function-52a5a283-e8b2-49be-8506-b2887b889f94>
  (the capitalize-after-non-letter rule and the lowercase-everything-else rule).
- Handbook, [Coercion and lifting](../model/02-coercion-and-lifting.md) — to-text conversion of
  non-text arguments, stated there as outline-level rather than pinned.
- Handbook, [The value universe](../model/01-value-universe.md) — worksheet text as UTF-16
  code units.
- Handbook projections `data/functions/FUNC.PROPER.json` (placeholder signature) and
  `data/presence/FUNC.PROPER.json` (implementing module; `BUG-FUNC-008`, `BUG-FUNC-016`).
