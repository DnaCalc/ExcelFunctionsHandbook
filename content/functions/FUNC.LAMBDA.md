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
  - "What it does"
  - "Try it"
  - "Quirks"
  - "How to implement"
  - "Version differences"
  - "Alternatives"
  - "Sources"
family: null
role_in_family: null
---

# LAMBDA

## What it does

`LAMBDA([parameter1, …], calculation)` defines a function in a formula. Parameters come first;
the expression to calculate comes last. Creating the function and calling it are separate steps.

`=LAMBDA(x,x+1)(4)` calls a one-parameter function with 4 and returns 5. Save the definition as a
workbook name to call it by that name elsewhere. A lambda can also be passed to `MAP`, `BYROW`,
`BYCOL`, `REDUCE`, `SCAN` or `MAKEARRAY`.

The calculation is required. Microsoft allows up to 253 parameters. Parameter names follow
Excel's naming rules, with no period in a parameter name.

## Try it

Start with `=LAMBDA(x,x+1)(4)`. Change the body to `x*x`, or try
`=LET(offset,10,LAMBDA(x,x+offset)(4))` to explore an enclosing name.

## Quirks

An uncalled `=LAMBDA(x,x+1)` displays `#CALC!` in a cell. The function must remain callable inside
the evaluator; replacing it with an error as soon as it is created would break higher-order
formulas. Cell display and the function value are different concerns.

Microsoft documents `#VALUE!` for too many parameters or a call with the wrong argument count.
Too much recursion can produce `#NUM!`; no universal recursion depth is specified here.

A parameter is a name to bind, not a cell to look up while defining the lambda. Use a name such
as `item` rather than one that Excel recognizes as a cell address. Do not assume that a function
shown as `#CALC!` has lost its body: a selector can pass a callable on to a later call.

## How to implement

Parse parameter declarations and the body as a binding form. Store a callable containing the
parameter list, body and lexical environment. Do not evaluate the parameter identifiers as
ordinary arguments when constructing that callable.

On invocation, check the argument count, create a child scope, bind supplied arguments in order,
and evaluate the body in that scope. Enclosing `LET` names must remain accessible after the
outer expression has returned the callable. A reference captured by a body also needs its sheet
and workbook context; a string containing its displayed address is insufficient.

Keep callable values intact through selectors and helper functions. Apply the cell's display
rule only when publishing a value to the grid. Guard recursive evaluation with a resource limit;
treat that implementation limit separately from a measured Excel recursion boundary.

## Version differences

Microsoft lists LAMBDA for Microsoft 365 and Excel 2024 on Windows and Mac. This is product
availability, not an observation that specific builds evaluate every lambda alike. No comparison
of recursion boundaries or captured-reference behaviour between named Excel builds is recorded here.

## Alternatives

Use [LET](FUNC.LET.md) to name intermediate values within one expression. Use a named LAMBDA to
reuse a calculation. [MAP](FUNC.MAP.md) applies it to individual array positions; BYROW and BYCOL
pass an entire row or column at a time. VBA functions serve a different purpose when statements
or interaction with the host are required.

## Sources

- [Microsoft: LAMBDA function](https://support.microsoft.com/en-us/excel/functions/lambda-function).
- [The formula call pipeline](../model/03-call-pipeline.md).
