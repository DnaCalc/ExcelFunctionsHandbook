---
schema: efh.function-page/v1
function_id: FUNC.LAMBDA
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

# LAMBDA

## Status: export-only, not deferred

`LAMBDA` has no entry in the reference engine's implementation catalogue. That is a statement about
*where the semantics live*, not about whether the function is supported.

`LAMBDA` is a **binding form**. Its arguments are not values to be computed and handed to a
kernel; the leading arguments are *parameter names*, which have no value at all, and the trailing
argument is an unevaluated *body*. Binding, scoping, and capture are properties of the formula
layer — the parser and evaluator that decide what a name means in a given expression — and not of
the function library, which begins its work only once arguments are values. The Handbook records
`LAMBDA` accordingly: **export-only**, meaning the surface is published and catalogued here while
its semantics are owned upstream of the function lane.

Export-only is not the same state as **deferred**, and the Handbook must not blur them. A deferred
entry is one the reference engine has intentionally declined to implement, and it carries a stated
deferral reason. `LAMBDA` carries no deferral: its admission label in the projected data is
`supported`, it is present in the registry seeds, and it is absent from dispatch because there is
nothing for a value-taking dispatcher to do with it. The battery row for `LAMBDA` records exactly
this — the runner reports that it cannot call the function because it is not in the reference
catalogue, which is a fact about the harness's reach, not a defect finding.

## What it computes

`LAMBDA` constructs a **callable value**: a reusable function defined entirely in formula syntax,
with named parameters and a body expression, and no code behind it.

Evaluating `LAMBDA(...)` does not run the body. It produces a value of the `Callable` kind — a rich
value in the Handbook's value model whose **core projection is the `#CALC!` error**
([chapter 01](../model/01-value-universe.md), "Rich values: the extension layer"). That is why
typing an uncalled lambda into a cell displays `#CALC!`: the cell is showing the legacy-gamut
projection of a value the traditional grid cannot hold. The error is not a failure; it is what a
callable looks like to a surface that has no concept of callables.

A callable becomes useful in exactly two ways:

1. **Immediate invocation** — a call expression applied to the lambda itself, which supplies
   arguments and evaluates the body with the parameters bound to them.
2. **Naming** — defining the lambda as a workbook name, after which the name is callable like any
   built-in function. This is the mechanism by which a workbook grows its own function library.

A third way is what the rest of this shelf is about: passing the callable to a higher-order
function — `MAP`, `REDUCE`, `SCAN`, `BYROW`, `BYCOL`, `MAKEARRAY` — which invokes it many times.

## Arguments

`LAMBDA([parameter1, parameter2, ...], calculation)`

- **`parameterN`** — a name to bind, not a value. Names are positional: the first parameter binds
  the first argument at the call site. The registry records an accepted arity of 1 to 254
  arguments, so the parameter list may be empty and the body is always the last argument.
- **`calculation`** — required, and always the final argument. It is the expression evaluated when
  the callable is invoked, with the parameters in scope.

The projected signature for `LAMBDA` in `data/functions/FUNC.LAMBDA.json` is `null` rather than a
placeholder string — the Handbook suppresses a signature it does not have rather than faking one.
The form above is the documented shape from Microsoft's `LAMBDA` page; that page was not re-fetched
at this revision.

The commonly misunderstood position is the **last** one. Readers write the body first, as they
would in most languages. In `LAMBDA` the body goes at the end, and a lambda whose final argument is
accidentally a parameter name has an unusable body.

## Result and edge cases

Returns a `Callable` rich value.

- **In a cell, uncalled.** Publishes `#CALC!` — the core projection.
- **Invoked with the wrong number of arguments.** An arity failure at the call site; the reference
  engine's callable-invocation path reports an arity mismatch, and how that surfaces as a worksheet
  error is a formula-layer question.
- **Recursion.** A named lambda can call itself. Termination is the author's responsibility and
  there is no documented recursion depth in the sources consulted here.
- **Passed through `IF`.** Survives, because `IF` is a lazy selector that returns its branch
  verbatim ([chapter 03](../model/03-call-pipeline.md)) — one of the few places a callable moves
  through an ordinary function unharmed.
- **Stored in an array.** The reference engine's higher-order helpers deliberately preserve a
  callable carried as an array cell rather than reconstructing it from its `#CALC!` core, so an
  array of lambdas remains invocable. That is an implementation fact about the reference engine and
  a good indication of where a naive implementation loses callables.

## Errors

| Error | Condition |
|---|---|
| `#CALC!` | The core projection of an uncalled callable, published whenever a callable reaches a surface that cannot hold one. |
| `#VALUE!` | Reported by the reference engine's invocation path for callable arity mismatches. |
| `#NAME?` | A named lambda invoked under a name the workbook does not define — a name-resolution failure, not a `LAMBDA` failure. |

Microsoft's `LAMBDA` page is the documented source for the syntax and for the guidance on naming
lambdas; it was not re-fetched at this revision.

## Relationships

- **`LET`** is the other export-only binding form on this shelf, and for the same reason: it binds
  names. `LET` binds values to names within one expression; `LAMBDA` binds parameters for later
  invocation. They compose constantly — a `LAMBDA` body is very often a `LET`.
- **`MAP`, `REDUCE`, `SCAN`, `BYROW`, `BYCOL`, `MAKEARRAY`** consume callables and are the reason
  `LAMBDA` is more than a macro replacement. Each has its own page on this shelf.
- **`ISOMITTED`** tests whether an optional lambda parameter was supplied at the call site.
- **Defined names** in the Name Manager are the delivery mechanism for reusable lambdas; the name
  and the lambda are separate objects and the Handbook catalogues only the latter.
- Readers confuse `LAMBDA` with `LET` (naming values versus naming parameters) and with the older
  practice of writing a VBA `Function`; the latter comparison is useful right up to the point where
  it suggests statements, loops, and side effects, none of which exist here.

## Notes for implementers

1. **The parser owns this, not the library.** An implementation that tries to evaluate `LAMBDA` as
   an ordinary function will have already evaluated the parameter names as expressions and failed
   with `#NAME?` before reaching the body. The unevaluated-argument requirement is why this
   function sits upstream of the function lane.
2. **A callable is a rich value with a core projection, not an error.** Modelling it as `#CALC!`
   throughout loses the callable at the first boundary crossing and makes higher-order functions
   impossible.
3. **Capture semantics must be stated.** What a lambda body sees when it references a cell, a
   defined name, or an enclosing `LET` binding — and *when* it sees it — is the substantive
   question, and nothing in the sources consulted here pins it.
4. **Do not conflate export-only with deferred in metadata.** They produce different reader
   expectations: deferred implies a decision not to implement, export-only implies the semantics
   are implemented elsewhere.

## What has not been checked

There is no Handbook vector suite for `LAMBDA`; `vectors/` publishes nothing at this revision. No
Excel-comparison evidence record names `LAMBDA`. The reference-engine battery could not call it at
all — it is not in the reference catalogue — so there is no reference-engine outcome for it either,
and nobody has checked any of the behaviour described above against Excel for this Handbook.

This is the thinnest evidence position of any entry on this shelf, and the page is written to say
so plainly rather than to imply coverage.

What would settle the open questions:

1. **Capture timing.** Define a named lambda whose body references a cell, change the cell, and
   observe whether an already-formed callable sees the old or the new value. This decides whether
   the binding is by reference or by value and is the foundational unknown.
2. **Scope of enclosing `LET` names.** `LET(a, 1, LAMBDA(x, x+a))(2)` — does the body see `a`?
3. **Recursion depth.** A self-referential named lambda with an increasing counter, run until it
   fails, establishes whether there is a documented or observable limit and which error it
   produces.
4. **Arity behaviour.** Invoking a two-parameter lambda with one, three, and zero arguments —
   pins whether missing parameters bind as `Missing` (which `ISOMITTED` would then detect) or are
   refused.
5. **Array of callables.** `MAP(array_of_lambdas, LAMBDA(f, f(10)))` — tests whether Excel
   preserves callables inside arrays, the behaviour the reference engine deliberately protects.
6. **Boundary crossings.** Returning a lambda from a lambda, and passing one through `IF`,
   `IFERROR`, and a defined name — maps where the callable survives and where it collapses to
   `#CALC!`.

## Page vocabulary

| Term | Meaning |
|---|---|
| export-only | The surface is catalogued here; its semantics are owned upstream of the function lane |
| deferred | A distinct state: the reference engine intentionally declines the function, with a stated reason |
| binding form | A construct whose arguments include names and unevaluated expressions, not values |
| `Callable` | The lambda value kind; its core projection is `#CALC!` |
| core projection | The legacy-gamut value a rich value presents to surfaces that cannot hold it |

## Sources

- Microsoft, LAMBDA function —
  <https://support.microsoft.com/en-us/office/lambda-function-bd212d27-1cd1-4321-a34a-ccbf254b8b67>
  (documented source for syntax and for naming lambdas; not re-fetched at this revision).
- Handbook call-model chapters
  [01 The value universe](../model/01-value-universe.md) — the `Callable` kind and its `#CALC!`
  core projection — and [03 The call pipeline](../model/03-call-pipeline.md) — lazy branch
  selectors passing callables through.
- `data/functions/FUNC.LAMBDA.json` — admission label `supported`, `registry_backed: false`,
  interface kinds `callable_helper_formation` / `helper_formation`, and a suppressed (null)
  signature.
- `data/presence/FUNC.LAMBDA.json` — no implementation module, no dispatch entry, present only in
  the registry seeds.
- `data/battery/FUNC.LAMBDA.json` — every row records that the function is not in the reference
  catalogue and therefore cannot be called by the battery runner.
- OxFunc `crates/oxfunc_core/src/functions/callable_helpers.rs` at commit 473efa3 — the callable
  invocation path and the deliberate preservation of callables carried inside arrays.
