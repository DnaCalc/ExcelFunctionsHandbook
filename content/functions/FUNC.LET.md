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

# LET

## What it does

`LET(name1, value1, [name2, value2], …, calculation)` names intermediate values inside one
formula, then returns the final calculation. Each later binding and the final calculation can
use earlier names. Microsoft allows up to 126 name/value pairs.

`=LET(price,100,tax,0.15,price*(1+tax))` calculates 115. The names belong to that expression;
they do not create workbook names or fill cells.

## Try it

Try `=LET(x,3,x*x)`, then `=LET(x,3,y,x+2,x*y)`. The second example uses an earlier binding to
calculate the next one. For a reusable calculation, put LET inside a LAMBDA.

## Quirks

The last argument is an expression to return. Ending with a name/value pair leaves no final
calculation. Names must satisfy Excel's naming rules; names that conflict with reference syntax
can be refused. A short name such as `c` can conflict with R1C1 notation.

An inner LET can shadow an outer name. A lambda created in a LET may use its enclosing names;
those names must remain meaningful when the callable is invoked. Worksheet references and
volatile expressions make the timing of evaluation observable, so a mathematical substitution
is not by itself a compatibility rule.

## How to implement

Parse LET as a binding form, preserving the name tokens and the unevaluated expressions. Build
a lexical scope and add bindings in order. A binding's expression resolves names against the
scope that exists before that binding, while subsequent expressions see it. Evaluate the final
expression in the resulting scope and return its value unchanged.

Keep local bindings separate from workbook-defined names and from cell references. A cached
binding must preserve value kind, array shape, reference identity and callable environment.
Measure evaluation of unused and repeated volatile bindings before choosing between eager
binding and evaluation on demand. LET's advertised benefit is avoiding repeated calculation;
that does not settle every unused-binding case.

## Version differences

Microsoft lists LET for Microsoft 365, Excel 2021 and Excel 2024 on Windows and Mac. There is no
build-by-build comparison of evaluation timing recorded here.

## Alternatives

Use [LAMBDA](FUNC.LAMBDA.md) to define a function with arguments for later calls. Workbook-defined
names provide a wider scope. Helper cells expose intermediate results in the worksheet when
that makes a model easier to inspect.

## Sources

- [Microsoft: LET function](https://support.microsoft.com/en-us/excel/functions/let-function).
- [The formula call pipeline](../model/03-call-pipeline.md).
