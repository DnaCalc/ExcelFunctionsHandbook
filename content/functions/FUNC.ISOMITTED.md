---
schema: efh.function-page/v1
function_id: FUNC.ISOMITTED
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
family: callable_helpers
role_in_family: The introspection helper — the only member that asks about the call site rather
  than about a value.
---

## What it computes

`ISOMITTED(argument)` returns `TRUE` if the argument slot it was handed was left empty at the
call site, and `FALSE` otherwise.

Microsoft's description: "Checks whether the value in a LAMBDA is missing and returns TRUE or
FALSE." The argument is "The value you want to test, such as a LAMBDA parameter."

This is the only worksheet function whose subject is the value kind **`Missing`** — the marker
that, in Excel's value universe, exists at exactly one boundary and nowhere else. From
[the value universe](../model/01-value-universe.md): `Missing` is "the marker for an argument
that was omitted at a call site"; it is never stored in a cell, never returned by a function,
never published by a formula. It is admitted at the call-argument boundary only.

`ISOMITTED` is how that marker becomes visible. Without it, `Missing` is a kind that exists in
the engine and can never be observed from the sheet.

The mechanism `LAMBDA` supplies is the second half of the story. `LAMBDA` lets a user define a
function with parameters; a caller may leave a parameter slot empty (`(1,)`); the empty slot
delivers `Missing` to the body; `ISOMITTED` reads it. Microsoft's own example:

```
=LAMBDA(x,y, IF(ISOMITTED(y),"Missing second argument",x+y))(1,)
```

That is the whole design pattern — **optional parameters with defaults in user-defined
functions**, which is what `ISOMITTED` exists to make possible. Before it, a `LAMBDA` could not
distinguish "you gave me nothing" from "you gave me zero".

`ISOMITTED` is classified as a **callable runtime helper**
(`admission_interface_kind: callable_runtime_helper`), not an ordinary call. It belongs to the
`LAMBDA` machinery alongside `MAP`, `REDUCE`, `SCAN`, `BYROW`, `BYCOL` and `MAKEARRAY`.

## Arguments

`argument` — required, exactly one. The published signature is `ISOMITTED(value)`.

The argument position is unlike every other in this Handbook, because **what is passed matters
more than what it evaluates to**. `ISOMITTED` is asking about the *provenance* of a slot, so it
must receive the slot's marker, not a resolved value. In practice this means:

- **The intended use is a bare `LAMBDA` parameter name.** `ISOMITTED(y)` inside the body of a
  `LAMBDA(x,y,…)`.
- **An expression is not a slot.** `ISOMITTED(y+0)` cannot be `TRUE` — the addition produced a
  value, and a value is not missing. Wrapping the parameter in anything destroys the thing being
  tested.
- **Outside a `LAMBDA` there is nothing to be missing.** `=ISOMITTED(A1)` on a worksheet is
  asking about a cell reference, which was supplied. The documented subject of this function is a
  `LAMBDA` parameter; using it anywhere else is not what it is for, and the Handbook has not
  established what it does there.

## Result and edge cases

Returns a `Logical`.

- **`Missing` is not `Empty`.** This is the distinction the whole function rests on. A parameter
  bound to a reference to a *blank cell* received something — the `Empty` kind — and is not
  omitted. `ISOMITTED` should be `FALSE` there and `ISBLANK` `TRUE`. The two functions cover the
  two different nothings, and the model keeps them separate precisely so that they can.
- **A parameter bound to `0` or `""` is not omitted.** The whole point.
- **The reference engine returns `FALSE` for every ordinary value**, including an empty cell,
  a number, text, a logical, an error and an array — and `TRUE` only for a genuine `Missing`
  marker. Its published battery, whose twelve inputs are all ordinary values, shows `FALSE`
  throughout, which is consistent but exercises only one side of the function.
- **Errors do not propagate.** `ISOMITTED(#N/A)` classifies rather than forwards, in the same way
  the IS-family predicates do — an error argument was supplied, so the answer is `FALSE`.
- **Trailing omission versus interior omission.** `(1,)` leaves a slot empty at the end;
  `(,2)` leaves one at the start. Both are omissions in the formula-text sense. Whether both
  deliver `Missing` is not established here.
- **Nested `LAMBDA`s.** A parameter forwarded from one `LAMBDA` to another — does the `Missing`
  marker survive the hop, so that the inner `ISOMITTED` sees it? Not established here, and this
  is the question that determines whether optional parameters compose.

## Errors

`ISOMITTED` returns no error of its own for any argument it can be given.

The only failure available to it is **arity**: zero arguments, or two. `ISOMITTED()` is expected
to be refused at formula entry rather than evaluated
([the call pipeline](../model/03-call-pipeline.md)); the reference engine, having no entry-time
surface, reports `#VALUE!` for both.

Microsoft's page documents no error return.

## Relationships

- **`LAMBDA`** is the function `ISOMITTED` exists to serve; outside it, the function has no
  subject.
- **`ISBLANK`** is the near neighbour and the standard confusion. `ISBLANK` tests `Empty` — a
  cell with no content. `ISOMITTED` tests `Missing` — a call-site slot with nothing in it. Two
  kinds, two functions, and the model deliberately refuses to conflate them
  ([the value universe](../model/01-value-universe.md), "Missing versus Empty").
- **`MAP`, `REDUCE`, `SCAN`, `BYROW`, `BYCOL`, `MAKEARRAY`** share `ISOMITTED`'s implementation
  module and its callable-helper classification. They consume `LAMBDA` values; `ISOMITTED`
  inspects a `LAMBDA`'s own binding.
- **`IF`** is the invariable companion: `IF(ISOMITTED(y), <default>, y)` is the idiom, and it is
  worth noting that `IF` is a lazy branch selector
  ([the call pipeline](../model/03-call-pipeline.md)), so the default expression is not evaluated
  when the parameter was supplied.
- **`LET`** often appears alongside, to name the defaulted value once:
  `LET(yy, IF(ISOMITTED(y), 10, y), …)`.
- **Optional arguments in built-in functions** are the analogue this function brings to
  user-defined ones: every built-in with an `[optional]` slot performs internally the test that
  `ISOMITTED` exposes.

## Notes for implementers

1. **`Missing` must survive argument binding.** If a `LAMBDA` invocation normalizes empty slots
   to `Empty` or to `0` before binding parameters, `ISOMITTED` can never return `TRUE`, and the
   failure is silent — every call just takes the non-default branch.
2. **`ISOMITTED` must not have its argument evaluated to a value first.** It needs the binding,
   not the binding's value. This is a lazier argument treatment than any ordinary function's.
3. **`Missing` is admitted at exactly one boundary.** It must never be storable in a cell,
   returnable from a function, or publishable as a result. An engine that lets `Missing` leak is
   creating a value Excel cannot represent.
4. **Decide the nesting behaviour deliberately.** Whether a forwarded parameter carries its
   `Missing` marker into an inner `LAMBDA` determines whether optional parameters compose, and it
   is not documented either way.
5. **The battery cannot reach the `TRUE` case.** Mechanical probing with ordinary values only
   ever exercises `FALSE`. Any test corpus for this function must be constructed by hand, inside
   `LAMBDA` invocations.

## What has not been checked

No Handbook vector suite exists for `ISOMITTED`; `vectors/` publishes nothing for this function.
No Excel-comparison evidence record names `ISOMITTED` as a subject — **nobody has checked this
function's answers against Excel inside the Handbook's record.** The reference engine's own
answers appear on the battery panel under their own label; they are not Excel's answers, and as
noted above those twelve rows are all `FALSE`, so **the function's entire reason for existing —
the `TRUE` case — has no answer on record at all.**

The probes that would settle the open questions:

1. **Microsoft's own documented example**, run: `=LAMBDA(x,y, IF(ISOMITTED(y),"Missing second
   argument",x+y))(1,)`. It should return the message. Nothing in the Handbook currently
   witnesses `ISOMITTED` returning `TRUE`, and this is the one-cell fix.
2. **`Missing` versus `Empty` side by side.** Call the same `LAMBDA` twice — once with the slot
   omitted, once with a reference to a genuinely blank cell — and read `ISOMITTED` and `ISBLANK`
   on both. Four answers, and they are the page's central claim.
3. **Interior omission** — `(,2)` — versus trailing omission `(1,)`.
4. **Nesting.** A `LAMBDA` that forwards a possibly-omitted parameter into another `LAMBDA` that
   tests it. If the marker does not survive, optional parameters do not compose, and that is a
   significant limitation nobody has written down.
5. **`ISOMITTED` outside a `LAMBDA`** — on a worksheet, with a literal, with an empty slot in an
   ordinary function call such as `SUM(1,,2)`. The `SUM` case is interesting because that slot
   *is* documented to deliver `Missing`; whether it can be reached by `ISOMITTED` is unknown.
6. **Version boundary.** The function is documented as available in Excel for Microsoft 365 and
   Excel 2024; its behaviour on the earliest builds that have it has not been separated from its
   behaviour on current ones.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| `Missing` | The omitted-argument marker; live only at the call-argument boundary |
| `Empty` | The no-content state of a cell; a different kind, tested by `ISBLANK` |
| callable runtime helper | The admission class shared with `MAP`, `REDUCE`, `SCAN` and friends |
| slot provenance | Whether an argument position was filled at the call site, independent of value |

## Sources

- Microsoft, "ISOMITTED function" —
  <https://support.microsoft.com/en-us/office/isomitted-function-831d6fbc-0f07-40c4-9c5b-9c73fd1d60c1>.
  Read for this page: the description, the syntax, the argument description, the worked `LAMBDA`
  example verbatim, and the stated availability (Excel for Microsoft 365, Excel 2024, and the
  Mac equivalents).
- Handbook, [the value universe](../model/01-value-universe.md) — the `Missing` kind, its
  single-boundary admission, and the Missing/Empty section this page depends on.
- Handbook, [coercion and lifting](../model/02-coercion-and-lifting.md) — omitted optional
  arguments delivering the `Missing` marker, and `SUM(1,,2)` as the documented example.
- Handbook, [the call pipeline](../model/03-call-pipeline.md) — lazy branch selectors and the
  admission boundary.
- `data/functions/FUNC.ISOMITTED.json` — identity, the published signature `ISOMITTED(value)`,
  arity 1–1, and the `callable_helper_runtime` / `callable_runtime_helper` interface markers, as
  projected at OxFunc `473efa3`.
