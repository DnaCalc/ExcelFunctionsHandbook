---
schema: efh.function-page/v1
function_id: FUNC.REGISTER.ID
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
family: call_register_id_family
role_in_family: The registration half of the XLL seam — binds a DLL entry point and returns the opaque token that CALL consumes.
---

`REGISTER.ID` is the other half of Excel's native-code seam. Where [`CALL`](FUNC.CALL.md)
invokes, `REGISTER.ID` **binds**: it associates a DLL, an entry point and a type signature, and
returns an opaque token standing for that binding. It is the only piece of the XLL registration
machinery that Microsoft documents as usable from an ordinary worksheet.

Its status in the reference engine matches `CALL`'s. It is **admitted** to OxFunc's surface
(`admission.label: supported`), registry-backed, implemented in the shared module
`call_register_id_family.rs`, and present in surface dispatch. Admission means the identity and
the seam are modelled — it is not a statement that behaviour has been demonstrated. The
projection's machine terms are again the informative part: `special_interface_kind:
registered_external_registration`, `admission_interface_kind: registered_external_lookup`,
`ExternalEventDependent`, `fec_dependency_profile: ExternalProvider`, `host_interaction_class:
ApplicationState`, `HostSerialized`. Note one difference from `CALL`: `REGISTER.ID` is recorded
as `NonVolatile` and `ValuesOnlyPreAdapter` — it does not see live references, and it is not
marked volatile.

## What it computes

Two operations behind one name, and the distinction is the whole function:

1. **If the procedure is not yet registered**, `REGISTER.ID` registers it and returns the new
   register id.
2. **If it is already registered**, `REGISTER.ID` returns the existing id.

That is why `type_text` is optional. Microsoft states it directly: "If the function or code
resource is already registered, you can omit this argument." So a two-argument call is a pure
*lookup*, and a three-argument call is *register-or-lookup*. A workbook can therefore call
`REGISTER.ID` repeatedly without accumulating registrations — it is idempotent in effect, which
is what makes it safe to put in a cell that recalculates.

The id itself is opaque and **session-scoped**. It is not a stable identifier: nothing
guarantees the same value across sessions, machines, or Excel builds. A workbook that saves an
id and reuses it later is relying on something no documentation promises.

`REGISTER.ID` is also the *narrower* of the two registration functions. Microsoft draws the
line explicitly: "REGISTER.ID can be used on worksheets (unlike REGISTER), but you cannot
specify a function name and argument names with REGISTER.ID." So the trade is worksheet
availability in exchange for the ability to publish the procedure under a name in Excel's
function list. `REGISTER.ID` binds for `CALL`'s benefit; `REGISTER` binds for the user
interface's benefit.

## Arguments

`REGISTER.ID(module_text, procedure, [type_text])` — arity 2 to 3, matching the reference
engine's recorded arity.

- **`module_text`** (required) — text naming the DLL that contains the function, in Excel for
  Windows. Resolution follows the operating system's library search, so which file this names
  is a property of the machine, not of the workbook.
- **`procedure`** (required) — text naming the function in the DLL. Microsoft adds that "you
  can also use the ordinal value of the function from the EXPORTS statement in the
  module-definition file (.DEF)". Two kinds of value, one argument slot — the same overload
  `CALL` carries, and the same source of confusion.
- **`type_text`** (optional) — text specifying the data type of the return value and of all
  arguments, with the **first letter specifying the return value**. Omissible only when the
  procedure is already registered.

Microsoft also notes that `REGISTER.ID` "has a slightly different syntax for each operating
environment", distinguishing Windows and Macintosh. The Handbook has not characterized the
non-Windows form.

The misunderstood position is `type_text`. It is not a description of the function for Excel's
benefit — it is the marshalling contract, and it is the reason the seam is dangerous. Because
the caller writes it as a string with nothing checking it against the DLL, a wrong `type_text`
is a memory-safety problem rather than a worksheet error. That risk belongs to the seam as a
whole and is set out on the [`CALL`](FUNC.CALL.md) page; it applies from the moment
registration happens, not only at invocation.

## Result and edge cases

A register id — an opaque value whose only documented use is as `CALL`'s first argument. The
Handbook has not characterized its value kind beyond "something a formula can hold and pass
along"; it should be treated as a token, not as a number to compute with.

The interesting edges are all about *when* the binding exists:

- **Session scope.** Registration state belongs to the Excel session. What a saved workbook
  holds is the last computed value, not a live binding.
- **Recalculation.** The function is recorded as `NonVolatile`, so it is not re-evaluated on
  every calculation; whether the id it returns remains valid across a session in which the DLL
  is unloaded is not documented.
- **`UNREGISTER`.** The teardown counterpart exists as a macro-sheet function. What a
  previously returned id does after its registration is torn down is not documented.

## Errors

Microsoft's `REGISTER.ID` page documents no error-value table. As with `CALL`, that absence is
substantive rather than accidental: the interesting failures — a DLL that will not load, an
entry point that is absent, a signature that does not match — are operating-system and
loader-level events, and the documentation does not map them onto worksheet error values.

The Handbook does not know what a missing DLL or a missing entry point returns. That is stated
as an unknown, not filled in by analogy.

## Relationships

- **[`CALL`](FUNC.CALL.md)** — the consumer of the id and the other member of
  `call_register_id_family`. The two functions are one mechanism split across two names, and
  neither page is complete without the other.
- **`REGISTER`** — the macro-sheet-only registration function that can also publish a function
  name and argument names into Excel's user interface. Microsoft contrasts the two explicitly,
  as quoted above. It is not part of this Handbook's worksheet-function catalog.
- **`UNREGISTER`** — the teardown counterpart, likewise macro-sheet-only.
- **XLL add-ins** — every function an XLL publishes goes through the same C API registration
  path. `REGISTER.ID` is that path exposed as a formula, minus the naming.
- **`EUROCONVERT`** — a category-mate under "User defined functions that are installed with
  add-ins", but a computation rather than a seam.

The confusion worth naming: `REGISTER.ID` is not related to the Windows registry, despite the
name and despite the registry caution that appears on the neighbouring `CALL` documentation
page. "Register" here means Excel's in-session table of callable external procedures.

## Notes for implementers

- The register-or-lookup duality is the specification. An implementation that always registers
  will leak bindings across recalculations; one that only looks up cannot bootstrap.
- The id must be opaque and must not be assumed stable. Any implementation that returns, say, a
  hash of the arguments is offering a stability guarantee Excel does not make — and workbooks
  will come to depend on it.
- `type_text`'s code set is C API vocabulary, not worksheet vocabulary. It must be reproduced,
  not redesigned. This is shared with `CALL`; the two functions must agree exactly, since a
  binding made by one is consumed by the other.
- The `ValuesOnlyPreAdapter` classification (unlike `CALL`'s `RefsVisibleInAdapter`) means
  registration takes plain values only, which is consistent with all three arguments being
  strings. See [the call pipeline](../model/03-call-pipeline.md) for what the two profiles
  mean.
- Registration mutates host state. `HostSerialized` is the right classification, and any engine
  running formulas in parallel must respect it.

## What has not been checked

No Handbook vector suite exists for `REGISTER.ID`, and no Excel-comparison evidence is
recorded. Nobody has checked this function against Excel in the Handbook's record. Everything
above is either Microsoft's documented behaviour, cited as such, or a statement about what the
reference engine's projection records.

A vector suite in the usual sense does not apply: the result is a session-scoped token whose
value is not a function of the inputs in any reproducible way. What can be characterized is the
seam's *state machine*, using a purpose-built, memory-safe test DLL. The probes worth running:

1. **Idempotence.** The same three-argument call evaluated twice, and then the two-argument
   lookup form — do all three yield the same id? This is the documented core behaviour and it
   is unverified.
2. **Lookup before registration.** The two-argument form for a procedure never registered in
   this session. Microsoft's phrasing implies it needs `type_text` in that case; what actually
   happens is unstated.
3. **Failure shapes.** A `module_text` naming a non-existent DLL; a `procedure` naming a
   non-existent export; an ordinal out of range. Three failures with no documented error
   values.
4. **Id stability.** The same registration across two Excel sessions and two machines,
   confirming (or refuting) that ids must be treated as session-scoped.
5. **Worksheet availability.** The documented claim that `REGISTER.ID` works on worksheets
   while `REGISTER` does not, tested on both surfaces — this is the function's distinguishing
   feature and worth pinning first if only one probe can be run.
6. **Post-`UNREGISTER` behaviour.** An id retrieved, its registration torn down, then the id
   used in `CALL`.
7. **Modern default posture.** Whether registration is blocked outright under current macro and
   add-in security defaults, and what the cell shows if so.
8. **Value kind of the id.** `ISNUMBER`, `ISTEXT` and `+0` applied to a returned id, to pin its
   place in the value universe.

Deliberately not probed: mismatched `type_text` against a real entry point. That is the
memory-corrupting case and characterizing it is not worth the cost.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: available | OxFunc admits the id to its implementation surface; this says nothing about demonstrated behaviour |
| registration seam | The C API mechanism by which native code becomes callable from a formula |
| register id | An opaque, session-scoped token standing for a bound procedure |
| register-or-lookup | The function's dual behaviour: bind if new, return the existing id if not |
| `type_text` | The caller-supplied marshalling contract; first letter is the return type |
| `ValuesOnlyPreAdapter` | Argument-preparation profile: references are resolved before the function runs |

## Sources

- Microsoft, "REGISTER.ID function" —
  <https://support.microsoft.com/en-us/office/register-id-function-f8f0af0f-fd66-4704-a0f2-87b27b175b50>
  (syntax, all three arguments including the ordinal form of `procedure` and the first-letter
  rule for `type_text`, the "already registered, you can omit this argument" remark, the
  worksheet-versus-`REGISTER` contrast quoted above, and the per-environment syntax note).
- Microsoft, "CALL function" —
  <https://support.microsoft.com/en-us/office/call-function-32d58445-e646-4ffd-8d5e-b45077a5e995>
  (the consuming side of the seam and the caution that governs both pages).
- Handbook `data/functions/FUNC.REGISTER.ID.json` — admission label `supported`, arity 2–3,
  the signature recorded as a placeholder, `special_interface_kind:
  registered_external_registration`, `admission_interface_kind: registered_external_lookup`,
  and the classification axes quoted above.
- Handbook `data/presence/FUNC.REGISTER.ID.json` — the implementing module
  `crates/oxfunc_core/src/functions/call_register_id_family.rs` and its surface-dispatch entry.
- Handbook `content/model/03-call-pipeline.md` — the argument-preparation profiles and the
  admission-versus-runtime boundary referred to above.
