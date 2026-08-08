---
schema: efh.function-page/v1
function_id: FUNC.ARABIC
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records:
  - EV-STRUCT-0009
open_problems: []
references:
  - work: "Microsoft Learn — WorksheetFunction.Arabic method (Excel)"
    locator: "https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.arabic"
    role: "documented description, the String argument type and the Double return type; notable for stating no grammar, no range and no error condition"
  - work: "Microsoft Support — ARABIC function"
    locator: "https://support.microsoft.com/en-us/office/arabic-function-9a8da418-c17b-4ef9-a657-9370a30a674f"
    role: "the canonical worksheet article; not retrieved for this pass (fetch returned HTTP 403)"
  - work: "Ifrah, The Universal History of Numbers"
    locator: "the chapters on Roman numeration"
    role: "the historical account of subtractive notation and of why no single canonical grammar exists"
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
family: arabic_fn
role_in_family: >-
  Sole member of its module: the Roman-to-decimal reader, and the only numeral-system function in
  the category whose input notation is non-positional and whose grammar is undocumented.
---

## What it computes

`ARABIC(text)` reads a Roman numeral and returns its value as a number.

Roman numeration is **non-positional**. A digit's contribution does not depend on where it sits in
a place-value column; it depends on the digit itself and on whether a larger digit follows it. The
seven symbols and their values:

| Symbol | `I` | `V` | `X` | `L` | `C` | `D` | `M` |
|---|---|---|---|---|---|---|---|
| Value | 1 | 5 | 10 | 50 | 100 | 500 | 1000 |

The standard evaluation rule is one pass, right to left or left to right, with a single
comparison:

    value(s) = sum over i of   +v(i)   if v(i) >= v(i+1)
                               -v(i)   if v(i) <  v(i+1)

where `v(i)` is the value of the *i*-th symbol and the last symbol always contributes positively.
That rule reads `IX` as `-1 + 10 = 9` and `XI` as `10 + 1 = 11`. It is the rule every
implementation uses, and it is far more permissive than the notation people think of as Roman: it
happily evaluates `IIX` as 8, `VV` as 10, and `IM` as 999, none of which is classical usage.

**This is the mathematically important fact about `ARABIC`**: the map from strings to numbers is
many-to-one and its domain is a matter of choice, not of arithmetic. There is no canonical Roman
grammar — the subtractive forms `IV`, `IX`, `XL`, `XC`, `CD`, `CM` were never universal in Roman
practice, and modern "rules" are a nineteenth-century tidying. So a function that converts Roman to
decimal has to *decide* how permissive to be, and that decision is behaviour, not mathematics.

`ARABIC` is therefore best understood as a parser with an undocumented grammar. What the grammar
accepts is the whole content of this function, and none of it is documented on any page the
Handbook has retrieved.

## Arguments

| Argument | Meaning | Default |
|---|---|---|
| `text` | The Roman numeral to convert. Required. | — |

Microsoft's Learn reference types the argument as **String** and describes it as "the Roman numeral
that you want to convert", and types the return as **Double**. That is the entirety of the
documentation retrieved for this function: no grammar, no case rule, no length limit, no range, no
sign handling, and no error condition.

One argument; the reference engine records an arity of exactly one, a `Custom` coercion-lift
profile and a `Custom` kernel signature — the two `Custom` markers together say that this surface
does not fit any of the standard numeric or text argument shapes and handles its own conversion.

The commonly misunderstood position is the argument's *kind*. `ARABIC` takes text. A reader who
passes a number is not passing a degenerate Roman numeral; they are passing the wrong kind, and the
reference engine's battery shows numeric arguments failing on this surface while an empty argument
succeeds and yields zero. That pairing — numbers rejected, emptiness accepted — is worth knowing
before you write a formula that feeds `ARABIC` from a column that might be blank.

## Result and edge cases

Returns `Number`.

The questions that decide this function's behaviour, and the state of each:

| Question | State |
|---|---|
| Is matching case-sensitive? Is `mcmxcix` accepted? | Undocumented; unchecked |
| Is leading or trailing whitespace tolerated? | Undocumented; unchecked |
| Is a leading minus sign accepted, and does it negate? | Undocumented; unchecked |
| Is the empty string a value or an error? | The reference engine returns zero; Microsoft's Learn page is silent |
| What is the maximum admissible length? | Undocumented; unchecked |
| Are non-classical forms such as `IIII`, `VV`, `IM` accepted? | Undocumented; unchecked. The one-pass rule above accepts all three |
| Is there an upper bound on the value? | Undocumented; unchecked |
| Do arrays lift elementwise? | An upstream defect stream on this module concerns exactly this |

Every row in that table is a real behavioural fact that a formula can depend on, and every one of
them is currently unpublished. This page states the shape of the ignorance rather than filling it
in.

**Empty and blank.** The reference engine's battery returns zero both for the empty string and for
an empty argument. Under the value model an empty cell delivers `Empty`, not `Missing`, and the two
are distinct at the call boundary — see
[The value universe](../model/01-value-universe.md). Whether `ARABIC` observes that distinction is
untested.

**Arrays.** The presence projection names an open upstream defect stream on this module
(`BUG-FUNC-028`, covering text, date, array-lift and coercion gaps), and the evidence record
attached to this page carries a per-surface witness about array-shaped arguments to `ARABIC`
specifically. Array behaviour is the unsettled part of this surface with actual evidence behind the
unsettledness.

## Errors

| Error | Condition | Source |
|---|---|---|
| `#VALUE!` | The argument is not admissible Roman text | Reference engine; **no error condition is documented on Microsoft's Learn page** |
| propagated | An error value in the argument is returned unchanged | Shared coercion rules |

The condition column is deliberately vague because the grammar is unknown. "Not admissible" is
precisely as specific as the published record allows, and sharpening it is what the probe list
below is for.

## Relationships

- **`ROMAN`** — the counterpart in the other direction, and *not* an inverse. `ROMAN` takes an
  optional `form` argument selecting among several degrees of abbreviation, so it is one-to-many;
  `ARABIC` is many-to-one. `ARABIC(ROMAN(n))` should recover `n` for every form and every `n` in
  range, and that round trip is the best available consistency test for the pair. `ROMAN(ARABIC(s))`
  does *not* recover `s` in general, because non-canonical spellings normalise away.
- **`DECIMAL`** — the positional counterpart: text in a stated radix to a number. `ARABIC` is the
  non-positional member of the same job. `DECIMAL` shares this surface's evidence record.
- **`BASE`** — `DECIMAL`'s inverse, and therefore the positional analogue of `ROMAN`.
- **`VALUE`** and **`NUMBERVALUE`** — the general text-to-number readers, which do not understand
  Roman numerals at all.
- **Confused with**: the Arabic *language* or Arabic-Indic *digits*. The function name refers to
  Arabic numerals in the ordinary English sense — the digits 0-9 — and has nothing to do with
  locale or script.

## Notes for implementers

**The one-pass rule is the whole algorithm**, and it is three lines. The interesting work is
entirely in deciding what to reject before running it.

**Validation is a policy, not a correctness requirement.** A permissive implementation runs the
sum and returns a number for anything built from the seven letters. A strict one enforces the
classical constraints: at most three consecutive repeats of `I`, `X`, `C`, `M`; no repeats of `V`,
`L`, `D`; only `I` before `V` or `X`, only `X` before `L` or `C`, only `C` before `D` or `M`; at
most one subtractive pair per magnitude. Both are defensible. They differ on a large set of inputs,
and which one Excel implements is unknown here.

**Case folding.** If the function is case-insensitive, that is an invariant fold, not a locale one
— the seven letters are all ASCII and have no Turkish-i style hazard. If it is case-sensitive, that
is a decision worth documenting. Neither is on record.

**Exactness.** Every reachable value is a small integer, well inside the exactly-representable
range of binary64, so there is no rounding question anywhere in this function. That is unusual for
a math-category surface and is worth stating so that readers do not go looking for one.

**Failure kind.** A parser has two failure modes — "these characters are not in the alphabet" and
"these characters are in the alphabet but not in this order" — and an implementation should decide
whether they produce the same error. If they do, a caller cannot tell a typo from a non-canonical
spelling.

**Round-trip as the test harness.** `ARABIC(ROMAN(n, f))` over all `n` in range and all `f` is a
complete self-consistency suite that requires no oracle, and it is the cheapest way to find
disagreements between a pair of implementations.

## What has not been checked

The evidence attached to this page is **`EV-STRUCT-0009`**, a **structural-verification** record
whose subject list contains `FUNC.ARABIC`. Its own status text is unusually careful and the
Handbook repeats its shape rather than its figures: a structural comparison against Excel **is** on
record for this surface, including a witness row about an array-shaped argument to `ARABIC`
specifically; what does not exist is a **per-surface count**, because the run's figures are a group
total over many surfaces with no split. The record's own words for that state are that "a
structural comparison is on record, no row count was extracted". Its figures are rendered
mechanically beside this page and this prose does not restate them.

So: something has been compared, on the structural axis, and nothing has been counted for this
surface alone. **No numeric-comparison evidence exists, and no Handbook vector suite exists for
`ARABIC`.**

The documented statements above come from Microsoft's Learn `WorksheetFunction.Arabic` reference,
which was retrieved and which documents almost nothing. Microsoft's worksheet article — which is
where any grammar or error statement would live — was **not** retrieved for this pass (HTTP 403).

Probes worth running first, ordered by how much grammar each one settles:

1. **`ARABIC("MCMXCIX")` and `ARABIC("mcmxcix")`** — value plus case rule, in two probes.
2. **`ARABIC("IIII")`, `ARABIC("VV")`, `ARABIC("IM")`, `ARABIC("IIX")`** — the four canonical
   probes of permissiveness. Each is evaluated by the one-pass rule; each is rejected by a strict
   grammar. The pattern of accepts and rejects identifies the validation policy in one shot.
3. **`ARABIC("-IV")` and `ARABIC(" IV ")`** — sign and whitespace, neither documented.
4. **`ARABIC("")`** against an empty cell reference — the `Empty` versus direct-text distinction,
   using a function whose battery already shows zero for both in the reference engine.
5. **A very long admissible string** — a few hundred `M`s — to find the length limit and the
   value ceiling, if either exists.
6. **`ARABIC(ROMAN(n, f))` for `n` across the representable range and every `f`** — the round trip,
   which needs no oracle and would immediately expose any disagreement between the pair.
7. **Array arguments**, given the open `BUG-FUNC-028` stream and the array witness carried in the
   attached evidence record. A `2x1` inline array is the exact shape that witness concerns.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| non-positional | A numeral system in which a symbol's value does not depend on its column |
| subtractive pair | A smaller symbol before a larger one, contributing negatively (`IV`, `IX`, `XL`, ...) |
| one-pass rule | The single-comparison evaluation `+v(i)` unless `v(i) < v(i+1)` |
| permissive grammar | Accepting any string over the seven letters and evaluating it by the one-pass rule |
| strict grammar | Enforcing the classical repeat and subtraction constraints before evaluating |
| group total | An evidence count spanning many surfaces, from which no per-surface figure may be read |

## Sources

- Microsoft Learn, "WorksheetFunction.Arabic method (Excel)" —
  <https://learn.microsoft.com/en-us/office/vba/api/excel.worksheetfunction.arabic> (retrieved: the
  description, the `String` argument type and the `Double` return type; **no grammar, range, case
  rule or error condition is stated there**).
- Microsoft, "ARABIC function" —
  <https://support.microsoft.com/en-us/office/arabic-function-9a8da418-c17b-4ef9-a657-9370a30a674f>
  — the canonical worksheet article. **Not retrieved for this pass** (HTTP 403).
- Ifrah, *The Universal History of Numbers*, chapters on Roman numeration — the historical basis
  for the claim that no canonical grammar exists.
- Handbook evidence record `EV-STRUCT-0009` (subjects include `FUNC.ARABIC`) — a structural
  comparison is on record with a per-surface witness; the count is a group total and carries a
  reader warning against per-surface attribution.
- Handbook projections `data/functions/FUNC.ARABIC.json` (arity, `Custom` coercion-lift and kernel
  signature) and `data/presence/FUNC.ARABIC.json` (implementing module; the `BUG-FUNC-028` defect
  stream).
- Handbook [The value universe](../model/01-value-universe.md) (text as UTF-16 code units; the
  `Empty` versus `Missing` distinction) and
  [Coercion and lifting](../model/02-coercion-and-lifting.md).
