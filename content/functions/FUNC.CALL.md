---
schema: efh.function-page/v1
function_id: FUNC.CALL
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - The danger, stated once and plainly
  - What it computes
  - Arguments
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Page vocabulary
  - Sources
family: call_register_id_family
role_in_family: The invocation half of the XLL registration seam — takes a registered procedure and calls it with worksheet arguments.
---

`CALL` is not a function in the sense the rest of this Handbook uses the word. It is a hole in
the function surface: an escape from Excel's evaluator into arbitrary native code in a DLL. It
has no mathematical definition, no domain, and no result kind of its own. What it returns is
whatever the code at the far end returns, marshalled back through a caller-declared type string.

Its status in the reference engine differs from most of this batch. `CALL` is **admitted** to
OxFunc's surface (`admission.label: supported`), is registry-backed with a full set of
classification axes, has an implementation module (`call_register_id_family.rs`), and appears
in surface dispatch. But admission here means *the identity and the seam are modelled*, not
that any behaviour has been demonstrated. The projection also records what kind of thing it is,
in machine terms that are unusually informative: `special_interface_kind:
registered_external_invocation`, `admission_interface_kind: macro_or_host_registered_call`,
`RefsVisibleInAdapter`, `ExternalEventDependent`, `fec_dependency_profile: ExternalProvider`,
`host_interaction_class: ApplicationState`, `HostSerialized`, `VolatileContextual`. Every one
of those is a statement that this call cannot be reasoned about locally.

## The danger, stated once and plainly

Microsoft's own page leads with a caution, and this page will too. `CALL` transfers control to
native code with no type checking beyond a string the *caller* wrote. If `type_text` does not
match the callee's actual signature, arguments are marshalled wrongly and the process may read
or write memory it does not own. Microsoft's wording: the function "is provided for advanced
users only", and using it incorrectly "may cause errors that will require you to restart your
computer".

Three specific properties make this worse than an ordinary foreign-function interface:

1. **The contract is a string in a cell.** `type_text` is data, not a compiled declaration.
   Nothing validates it against the DLL. A one-character error is a memory-safety bug.
2. **The DLL is named by a string too.** `module_text` is resolved by the operating system's
   library search, so which file actually loads depends on the machine, not the workbook.
3. **It is reachable from a document.** A spreadsheet is a document users open casually.
   `CALL` is the mechanism by which a document can execute native code, which is precisely why
   Excel confines it (see below) and why it is disabled or blocked in modern configurations.

The Handbook states this as documented risk and design consequence, not as an observation of a
particular Excel build's current defenses, which it has not measured.

## What it computes

Nothing, in its own right. `CALL` marshals its arguments into a native call, invokes the
procedure, and marshals the return value back into a worksheet value. The "semantics" of a
`CALL` cell are the semantics of the DLL entry point.

The seam has two forms, and knowing which one you are looking at is the first step in reading
any `CALL` formula:

```
CALL(register_id, [argument1], …)                      -- pre-registered
CALL(module_text, procedure, type_text, [argument1], …) -- self-describing
```

The first form takes an id previously produced by `REGISTER` or
[`REGISTER.ID`](FUNC.REGISTER.ID.md) and calls the procedure that id stands for. The second
form carries the whole binding — library, entry point, and type signature — inline, registering
implicitly. The reference engine's recorded arity of 1–255 accommodates both: one argument is
the minimal pre-registered form.

## Arguments

- **`register_id`** — a value returned by a previously executed `REGISTER` or `REGISTER.ID`.
  Not a name, not a handle you can construct: an opaque token produced by this session.
- **`module_text`** — quoted text naming the DLL containing the procedure. Microsoft's page
  qualifies this as the Windows form; the function is documented as having a different shape on
  other environments.
- **`procedure`** — the function's name within the DLL, *or* its ordinal value. Microsoft's
  wording is that the ordinal is used "not [in] text form" — so this argument position accepts
  two different kinds of value with different meanings, which is the position most often
  gotten wrong.
- **`type_text`** — text specifying the return data type and the data types of all arguments,
  with the **first letter specifying the return value**. This is the load-bearing argument and
  the dangerous one. Microsoft's page as fetched does not include the full type-code table;
  the codes are documented in the Excel C API / XLL developer documentation rather than in the
  worksheet-function reference.
- **`argument1, …`** — optional; the values passed through to the procedure.

Note what the reference engine records about argument preparation: `RefsVisibleInAdapter`. The
function sees live references rather than resolved values, which is what allows a DLL to be
handed a range rather than its contents. That places `CALL` in the reference-aware minority
described in [the call pipeline](../model/03-call-pipeline.md).

## Result and edge cases

Whatever the callee returns, converted according to the first letter of `type_text`. The
value-universe chapter's **raw function return** boundary is exactly the boundary this function
sits on: an add-in procedure may hand back things — including the C API's empty/"nil" token —
that a published formula result cannot hold, and a normalization step stands between the two.
See [the value universe](../model/01-value-universe.md), where that raw-versus-published gap is
recorded as empirically pinned by add-in probes.

The most important structural constraint is a confinement one. Microsoft states that `CALL`
"is only callable from Excel macro sheets" — it is not a general worksheet function, despite
appearing in the function catalog. It is also documented as **not available in Excel for the
web**, while being listed for Windows, Mac, iPad and Android.

## Errors

Microsoft's page documents no error-value table for `CALL`. There is a reason for that which is
worth stating rather than glossing: the failure modes of `CALL` are not, mostly, worksheet
errors. A mismatched `type_text` does not produce `#VALUE!` — it produces undefined behaviour
in the process. The error surface of this function is the operating system's, not Excel's.

What the shared model does say: an error value arriving as an argument propagates through
coercion unless a function declares otherwise (see
[coercion and lifting](../model/02-coercion-and-lifting.md)), and `CALL`'s recorded
`error_collapse_profile` is `None`, meaning no special folding. That is an engine-level
statement about the wrapper, not a claim about what a DLL does with what it receives.

## Relationships

- **[`REGISTER.ID`](FUNC.REGISTER.ID.md)** — the other half of the seam, and the only other
  member of `call_register_id_family`. `REGISTER.ID` produces the token; `CALL` consumes it.
  The two pages should be read together.
- **`REGISTER`** — the macro-sheet function that performs registration with full naming (and
  which, unlike `REGISTER.ID`, cannot be used on a worksheet). It does not appear in this
  Handbook's catalog of worksheet functions; it is named here because `CALL`'s first syntax
  form refers to it.
- **`UNREGISTER`** — the teardown counterpart, likewise a macro-sheet function.
- **`EUROCONVERT`** — a category-mate ("User defined functions that are installed with
  add-ins") but nothing like it in kind: `EUROCONVERT` is arithmetic that happens to ship in an
  add-in, whereas `CALL` is the mechanism by which foreign code becomes callable at all.
- **XLL add-in functions generally** — every custom function registered by an XLL arrives
  through this same C API registration machinery. `CALL` is that machinery exposed as a
  formula.

Readers confuse `CALL` with VBA's `Application.Run` and with `Declare` statements. Those are
VBA-level constructs with VBA's own marshalling; `CALL` is the worksheet/macro-sheet route into
the C API, and its type system is the C API's.

## Notes for implementers

- There is nothing here to reimplement as a kernel. An implementation either provides a real
  native-invocation seam — with all the platform, calling-convention and safety consequences —
  or it declines to, and says so. A reference engine that "implements `CALL`" by returning an
  error is making a policy choice and should label it as one.
- `type_text` is a miniature type language, and its exact code set is part of the C API rather
  than the worksheet function surface. Any faithful implementation must reproduce that
  language, not invent an equivalent.
- The macro-sheet confinement is an admission-boundary fact, not a runtime one: the question
  "does this formula run at all here?" precedes every question about what it returns. The
  Handbook's admission-versus-runtime split (see the call-pipeline chapter) is exactly the
  right frame, and `CALL` is one of the clearest cases of a function whose interesting
  behaviour is at admission.
- Because the function is `HostSerialized` and `VolatileContextual` in the projection's terms,
  it cannot participate in multithreaded recalculation the way a pure function can. Any engine
  modelling it must serialize it against host state.

## What has not been checked

No Handbook vector suite exists for `CALL`, and no Excel-comparison evidence is recorded.
Nobody has checked this function against Excel in the Handbook's record. Everything above is
either Microsoft's documented behaviour, cited as such, or a statement about what the reference
engine's projection records.

`CALL` is the function in this batch least suited to a conventional vector suite: its result is
a property of a DLL, not of its inputs, so "the answer" is not well defined. What could be
characterized is the **seam**, using a purpose-built, memory-safe test DLL with known entry
points. With such a DLL, the probes worth running:

1. **Where it is callable from.** The same formula on a worksheet, on an Excel 4.0 macro sheet,
   and from a defined name — to pin the documented macro-sheet confinement precisely, since
   "only callable from macro sheets" and "returns an error on a worksheet" are different
   claims.
2. **Modern default posture.** Whether `CALL` is blocked, prompted, or silently disabled under
   current Trust Center defaults and macro-blocking policy, and which value (if any) the cell
   shows. `#BLOCKED!` exists in the value-universe registry; whether this function can produce
   it is unknown.
3. **Type-string round trips.** Each documented `type_text` code exercised against a matching
   entry point, recording the marshalled value in both directions. This is the only way to pin
   the raw-return normalization the value-universe chapter describes.
4. **Reference arguments.** A range passed to an entry point declared to take a reference,
   confirming the `RefsVisibleInAdapter` classification from the worksheet side.
5. **The two syntax forms.** The same procedure invoked by `register_id` and by inline
   `module_text`/`procedure`/`type_text`, checking that they agree.
6. **Ordinal versus name.** `procedure` given as an ordinal and as a name.
7. **Platform.** Excel for the web (documented as unavailable) and Mac, to record what
   unavailability looks like from the grid.

Deliberately **not** on this list: any probe involving a deliberately mismatched `type_text`.
That is the memory-corrupting case, and characterizing it is not worth the cost.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: available | OxFunc admits the id to its implementation surface; this says nothing about demonstrated behaviour |
| registration seam | The C API mechanism by which native code becomes callable from a formula |
| `register_id` | An opaque session-scoped token standing for a registered procedure |
| `type_text` | The caller-supplied type signature string; first letter is the return type |
| macro sheet | The Excel 4.0 macro sheet, the only surface from which `CALL` is documented to be callable |
| `RefsVisibleInAdapter` | Argument-preparation profile: the function sees live references rather than resolved values |
| raw function return | The value-universe boundary at which an add-in's return value enters Excel, before normalization |

## Sources

- Microsoft, "CALL function" —
  <https://support.microsoft.com/en-us/office/call-function-32d58445-e646-4ffd-8d5e-b45077a5e995>
  (the caution quoted above, both syntax forms, all argument descriptions including the
  first-letter rule for `type_text` and the ordinal form of `procedure`, the macro-sheet
  confinement, and the platform/version list including the exclusion of Excel for the web).
- Microsoft, "REGISTER.ID function" —
  <https://support.microsoft.com/en-us/office/register-id-function-f8f0af0f-fd66-4704-a0f2-87b27b175b50>
  (the registration side of the seam).
- Handbook `data/functions/FUNC.CALL.json` — admission label `supported`, arity 1–255, the
  signature recorded as a placeholder, `special_interface_kind:
  registered_external_invocation`, `admission_interface_kind: macro_or_host_registered_call`,
  and the classification axes quoted above.
- Handbook `data/presence/FUNC.CALL.json` — the implementing module
  `crates/oxfunc_core/src/functions/call_register_id_family.rs` and its surface-dispatch entry.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md` and
  `03-call-pipeline.md` — the raw-versus-published return boundary, the error-propagation
  discipline, the reference-aware argument-preparation profile, and the admission-versus-runtime
  split.
