---
schema: efh.function-page/v1
function_id: FUNC.LET
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

# LET

## Status: export-only, not deferred

`LET` has no entry in the reference engine's implementation catalogue, and the reason is the same
as for `LAMBDA`: it is a **binding form**. Its odd-numbered arguments are *names*, which are not
values and must not be evaluated, and its final argument is an expression that must be evaluated
only after those names are in scope. Binding and scoping belong to the formula layer, upstream of
the function library, which starts work only once arguments are values.

The Handbook therefore records `LET` as **export-only**: catalogued and published here, with its
semantics owned upstream. That is a different state from **deferred**, which means the reference
engine intentionally declined to implement something and states a reason. `LET` carries no
deferral — its projected admission label is `supported`, it appears in the registry seeds, and it
is absent from dispatch because a value-taking dispatcher has nothing to do with it. The battery
row records that the runner cannot call it because it is not in the reference catalogue: a fact
about the harness's reach, not a finding about the function.

## What it computes

`LET` names intermediate results inside a single formula and then evaluates a final expression with
those names in scope.

The rule:

1. Arguments arrive as `(name1, value1, name2, value2, …, calculation)`.
2. Each `nameN` is bound to the result of evaluating its `valueN`. Bindings are established in
   order, and a later `valueN` may refer to names bound earlier — so the list is a sequence of
   definitions, not a simultaneous set.
3. The final argument, `calculation`, is evaluated with all the names in scope, and its value is
   the value of the `LET` call.

Two consequences are the whole point of the function:

- **A named subexpression is computed once.** The classic Excel pattern
  `IF(VLOOKUP(...)="", "", VLOOKUP(...))` evaluates the same lookup twice; `LET` gives it a name and
  evaluates it once. This is a readability feature that happens to also be a performance feature.
- **Formulas become legible.** A long expression with three named parts reads as three statements
  instead of one line. `LET` is the closest thing the worksheet has to local variables.

What `LET` is *not*: it is not assignment, and there is no mutation. A name is bound once, to one
value, for the duration of one evaluation. Rebinding the same name in a later pair is a shadowing
question, listed below as unchecked.

## Arguments

`LET(name1, name_value1, [name2, name_value2, ...], calculation)`

- **`nameN`** — a name to bind. It is a name, not a text value: writing `"x"` binds nothing useful.
- **`name_valueN`** — the expression whose result the name denotes.
- **`calculation`** — required, always the final argument, always in an even-numbered total
  position. The registry records an accepted arity of 3 to 253 arguments, so at least one binding
  is required and the calculation is mandatory.

The projected signature in `data/functions/FUNC.LET.json` is `null` — a signature the Handbook does
not have is suppressed rather than faked. The shape above is the documented form from Microsoft's
`LET` page, which was not re-fetched at this revision.

The commonly misunderstood position, again, is the last one: readers write the calculation first,
by analogy with almost every other language's `let`. In Excel the calculation is the tail.

## Result and edge cases

Returns whatever the `calculation` expression returns — any value kind, including arrays and
callables.

- **A name shadowing a workbook-defined name or a function name.** Whether the local binding wins,
  and whether Excel refuses names that collide with built-in function names, is not settled here.
- **A name referenced before it is bound.** Forward references are not expected to work given the
  sequential-binding rule, but the resulting error is not pinned here.
- **A binding whose value is an error.** Whether the error surfaces immediately at binding time or
  only when the name is used depends on evaluation strategy — eager or lazy — and that choice is
  observable whenever a bound-but-unused expression would error.
- **`LET` returning a callable.** `LET(f, LAMBDA(x, x+1), f)` produces a callable, whose core
  projection in a cell is `#CALC!` ([chapter 01](../model/01-value-universe.md)).
- **Arrays.** A bound name may hold an array; the calculation then works with it as an ordinary
  array value.

## Errors

| Error | Condition |
|---|---|
| `#NAME?` | A name used in the calculation that is not bound and is not otherwise defined — a name-resolution failure. |
| `#VALUE!` | Structural failures of the argument list, such as an even total argument count leaving a name without a value. |
| any incoming error | An error produced by a bound expression surfaces through whatever path the evaluation strategy takes. |

None of these is verified here. Microsoft's `LET` page is the documented source for the syntax and
for the constraints on names; it was not re-fetched at this revision.

## Relationships

- **`LAMBDA`** is the other export-only binding form on this shelf. `LET` names values within one
  expression; `LAMBDA` names parameters for later invocation. A `LAMBDA` body is very often a
  `LET`, and the pair is how a workbook grows readable custom functions.
- **Defined names** in the Name Manager are the workbook-scoped counterpart. `LET` bindings are
  formula-scoped and invisible outside the cell; a defined name is visible everywhere and is the
  right tool when the value is shared.
- **`MAP`, `REDUCE`, `SCAN`, `BYROW`, `BYCOL`** frequently appear inside a `LET` calculation, with
  the intermediate arrays given names.
- Readers confuse `LET` with `LAMBDA` (values versus parameters) and with the older habit of
  parking an intermediate result in a helper column. The helper column remains a perfectly good
  answer and is easier to audit; `LET` trades auditability for locality.

## Notes for implementers

1. **Names must not be evaluated as expressions.** This is the entire reason `LET` cannot be an
   ordinary function: by the time a value-taking dispatcher sees the arguments, `name1` has already
   been resolved as a name reference and failed.
2. **Bind sequentially and expose earlier names to later values.** A simultaneous binding model
   gives different answers as soon as one definition uses another.
3. **Decide eager versus lazy binding explicitly.** Evaluating every bound expression up front is
   simpler and makes errors in unused bindings visible; lazy binding avoids them. Both are
   defensible and they are distinguishable by observation, so the choice must be recorded rather
   than inherited from the implementation language.
4. **Do not conflate export-only with deferred in metadata**, for the reasons given at the top of
   this page.

## What has not been checked

There is no Handbook vector suite for `LET`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `LET`. The reference-engine battery could not call it,
because it is not in the reference catalogue, so there is no reference-engine outcome either.
Nobody has checked any of the behaviour described above against Excel for this Handbook.

What would settle the open questions:

1. **Eager or lazy binding.** `LET(a, 1/0, b, 2, b)` — if the result is `2`, binding is lazy; if it
   is `#DIV/0!`, binding is eager. One formula decides it, and much of the rest of this page
   depends on the answer.
2. **Sequential visibility.** `LET(a, 1, b, a+1, b)` — confirms a later value can use an earlier
   name.
3. **Shadowing.** `LET(SUM, 1, SUM)` and `LET(x, 1, x, 2, x)` — whether a binding may shadow a
   built-in name, and whether rebinding the same name is allowed and which value wins.
4. **Forward reference.** `LET(a, b, b, 1, a)` — pins the error for a name used before it is bound.
5. **Name legality.** Names containing digits, periods, non-ASCII characters, and names that look
   like cell addresses (`A1`) — establishes the accepted name grammar, which no source consulted
   here states.
6. **Interaction with implicit intersection and spilling.** A `LET` binding holding an array, used
   in a scalar context inside the calculation — tests whether the `@` operator's behaviour differs
   for a bound name versus a direct expression.

## Page vocabulary

| Term | Meaning |
|---|---|
| export-only | The surface is catalogued here; its semantics are owned upstream of the function lane |
| deferred | A distinct state: the reference engine intentionally declines the function, with a stated reason |
| binding form | A construct whose arguments include names and unevaluated expressions, not values |
| sequential binding | Names bound in order, with later definitions able to use earlier ones |
| formula-scoped name | A binding visible only within the expression that created it |

## Sources

- Microsoft, LET function —
  <https://support.microsoft.com/en-us/office/let-function-34842dd8-b92b-4d3f-b325-b8b8f9908999>
  (documented source for syntax and name constraints; not re-fetched at this revision).
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md) and
  [03 The call pipeline](../model/03-call-pipeline.md).
- `data/functions/FUNC.LET.json` — admission label `supported`, `registry_backed: false`, interface
  kinds `callable_helper_formation` / `helper_formation`, suppressed (null) signature, arity 3–253.
- `data/presence/FUNC.LET.json` — no implementation module, no dispatch entry, present only in the
  registry seeds.
- `data/battery/FUNC.LET.json` — every row records that the function is not in the reference
  catalogue and cannot be called by the battery runner.
