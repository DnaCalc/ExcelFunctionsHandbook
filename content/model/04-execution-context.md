# The execution context

Status: draft (H1) · Sources: OxFunc 937f198

## Overview

The previous chapter followed a function call from its arguments to its published result. This
chapter covers everything a function can see *besides* its arguments: the clock behind `NOW`,
the random source behind `RAND`, the calling cell behind `ROW()`, the locale behind `VALUE`,
the external feed behind `RTD`. Collectively this is the **execution context** — and the
central rule of the model is that a function may observe only the context facilities it has
*declared*. Nothing reaches a function ambiently; every host dependency is a named capability,
handed in explicitly, and recorded on the function's page as a chip.

That rule is what makes function behavior reproducible. If you know a function's declared
context dependencies and you pin those inputs — the time, the random sequence, the locale, the
workbook state — you can replay any call and get the same answer. A function that observed
undeclared context would be unauditable; the reference implementation this handbook is
grounded in (OxFunc) treats such observation as a conformance failure by construction, because
the facility simply is not passed in.

## The provider bundle

Concretely, the execution context is a bundle of independent **providers** — one small
interface per facility — assembled by the caller and passed to every invocation
(`FunctionExecutionContextBundle` in the reference implementation). Each provider is optional:
a host that cannot supply a facility leaves it absent, and a function that needs the missing
facility fails in a defined way instead of silently reading a default. The bundle carries:

- **Reference system** (`ReferenceSystemProvider`) — the seam through which references are
  resolved. This is the richest provider; the next section details it.
- **Current time** (`NowProvider`, or a fixed serial number) — the date/time source behind
  `NOW` and `TODAY`, expressed as an Excel serial value. Supplying a fixed serial instead of
  a live clock is how a replay pins time.
- **Randomness** (`RandomProvider`) — the uniform-random source behind `RAND` and its family.
  A seeded or scripted provider makes "random" functions replayable.
- **Locale and format context** (`LocaleFormatContext`) — locale-dependent parsing and
  formatting rules, used by functions such as `VALUE` whose text interpretation depends on
  the locale profile.
- **Host information** (`HostInfoProvider`) — application- and workbook-level facts that are
  not cell values, such as the workbook filename `CELL("filename")` reports.
- **Callable invocation** (`CallableInvoker`) — the facility for invoking a lambda (callable)
  value. Functions that *consume* a callable argument — `MAP`, `REDUCE`, `SCAN`, `BYROW`,
  `BYCOL`, `MAKEARRAY`, `GROUPBY`, `PIVOTBY` — declare which argument position is callable
  (`CallableArgumentSpec::Fixed(n)` or `::Last`), and "needs an invoker" is derived from that
  declaration rather than kept as a second list. Separately, some functions can pass a
  callable *through* unchanged (`IF`, `IFERROR`, `IFNA`, `IFS`, `CHOOSE`, `SWITCH`, `INDEX`
  can return a lambda that was handed to them); that is the declared `InvocablePassthrough`
  fact.
- **Real-time data** (`RtdProvider`) — the external topic feed behind `RTD`: connect to a
  topic, receive its current value, and participate in the topic's update lifecycle.
- **Registered external procedures** (`RegisteredExternalProvider`) — the legacy
  register/call surface (`REGISTER.ID` and relatives) that binds worksheet calls to external
  library procedures.

## The reference system and the calling cell

The reference system provider deserves its own treatment because it is both the most common
dependency and a capability boundary. It answers requests, each of which a host may support
or refuse:

- **dereference** — produce the value(s) a reference points at;
- **enumerate values** — walk a range sparsely (defined cells with coordinates and a declared
  extent), which is how aggregates scan large ranges without materializing every blank;
- **resolve text** — turn text into a reference (the `INDIRECT` operation);
- **facts** — report a reference's identity, kind, and display text without dereferencing;
- **transform** — index into, offset, or trim a reference (`INDEX`, `OFFSET`, and the
  trim-reference operators produce new references this way);
- **compose** — combine two references (range `:`, intersection, union).

The provider also declares **capabilities** up front: whether evaluation-time dereference is
allowed at all, and whether 3-D references, structured (table) references, spill-anchor
references, and external-workbook references are supported. A request outside the declared
capabilities is refused with a specific error rather than answered approximately — the
capability set is part of the reproducibility contract.

Finally, the reference system carries the **caller context** (`CallerContext`): the position
(row, column, and sheet prefix) of the cell whose formula is being evaluated. This is what
`ROW()` and `COLUMN()` with no argument read, and what text-to-reference resolution uses to
anchor relative addresses.

## The declared classes

Beyond the providers it consumes, every function carries four scheduling-relevant
classifications, plus a summary of its context dependency. These are the chips on the
function pages.

**Determinism** (`DeterminismClass`) — is the output a function of the explicit inputs?

- `Deterministic` — same arguments and workbook state, same result. The overwhelming
  majority.
- `PseudoRandom` — output draws on the random provider (`RAND`, `RANDARRAY`).
- `TimeDependent` — output draws on the clock (`NOW`, `TODAY`).
- `ExternalEventDependent` — output depends on an external feed's state (`RTD`).

**Volatility** (`VolatilityClass`) — is the function re-evaluated without its inputs changing?

- `NonVolatile` — recalculated only when a precedent changes.
- `VolatileFull` — participates in every recalculation cycle (`NOW`, `RAND`).
- `VolatileContextual` — re-evaluated under function- or context-specific conditions rather
  than every cycle.

The sources are explicit that the exact boundary between the two volatile forms is
provisional: the definition spec records `volatile_full` versus `volatile_contextual` as
"retained as unresolved terminology pending interactive policy finalization", while the
implementation already ships both variants with working scheduling semantics. Function pages
that display `VolatileContextual` are therefore displaying a provisional classification, and
this handbook says so rather than presenting it as settled.

**Host interaction** (`HostInteractionClass`) — which layer of host state, if any, the
function's meaning depends on: `None`, `WorkbookState` (e.g. `CELL` reading workbook facts),
`ApplicationState`, `EnvironmentState`, or `ExternalProvider` (`RTD`, the cube functions).

**Thread safety** (`ThreadSafetyClass`) — whether the function may be evaluated concurrently:
`SafePure` (no shared mutable host state), `HostSerialized` (safe only when the host
serializes its invocations), `NotThreadSafe`.

**Context dependency profile** (`FecDependencyProfile`, short for formula-evaluation-context
dependency) — a one-value summary of *which* provider families the function touches: `None`,
`RefOnly`, `CallerContext`, `TimeProvider`, `RandomProvider`, `ExternalProvider`,
`LocaleProfile`, or `Composite` (several families). Each function declares this twice: once
for the function proper (the adapter level) and once for the full call surface including
argument preparation. The two can differ — `ABS` itself needs no context (`None`), but its
call surface needs reference resolution to prepare `ABS(A1)` (`RefOnly`). Both are shown on
function pages, because consumers reason about them differently: the surface profile governs
what a call site needs; the adapter profile governs what the function's own semantics need.

## Volatility is not nondeterminism

The two axes are deliberately separate, and conflating them is the single most common
modeling error in recalculation engines (the sources pin this as decision topic D-001):

- **Volatility is an invalidation policy** — a property consumed by the *scheduler*. A
  volatile cell is put back in the recalculation candidate set without any precedent edit.
- **Nondeterminism is an output property** — whether the value can differ between two
  evaluations with identical explicit inputs and workbook state.

The combinations genuinely occur. `NOW` is volatile *and* time-dependent. `RAND` is volatile
*and* pseudo-random. `INDIRECT` is the instructive third case: it is context-*dependent* (its
result depends on workbook state reached through the resolved reference) but not
nondeterministic — same workbook, same text, same result — and its exact classification on
the determinism/host-interaction boundary is a recorded open decision (D-012). And `RTD` is
external-event-dependent, but its invalidation is modeled as a *separate* pathway from
volatility: an external topic update triggers targeted invalidation of the associated cells,
which the sources keep distinct from the volatile tick (documented as provisional lifecycle
mechanics).

The trigger vocabulary the sources use for recalculation causes, all documented as
preliminary: dependency edit (`T-DEP`), volatility tick (`T-VOL`), host state change
(`T-HOST`), external update (`T-EXT`), and version/build drift (`T-VERSION`).

## Why the declarations matter

The classes are not descriptive garnish; each has a consumer that changes behavior based on
it.

**Recalculation engines** use volatility and the trigger classes to decide what to mark dirty,
and determinism to decide what may be cached. A cell containing only `Deterministic` +
`NonVolatile` calls can be skipped when its precedents are unchanged; a `VolatileFull` cell
cannot.

**Optimizers** use the whole set. The reference implementation exposes this as an explicit
hoisting gate: an expression may be lifted out of per-cell evaluation (constant-folded,
computed once) only if every axis allows it under the stated policy. The strictest policy
(`STRICT_CONTEXT_FREE`) admits only deterministic, non-volatile, no-host-interaction,
no-context functions — `PI()` passes, `NOW()` fails. A relaxed policy
(`FIXED_EXECUTION_CONTEXT`) may hoist `NOW()` too, *because* pinning the bundle's providers
makes the call repeatable within the pinned scope. The gate reads determinism, volatility,
host interaction, and both context-dependency profiles together; a function misdeclared on
any one of them would be hoisted incorrectly and produce stale or wrong values.

**Auditors and verifiers** use the declarations as the reproducibility contract. A recorded
evaluation is replayable exactly when every declared context input is captured alongside the
arguments. The declarations tell the auditor *what to capture*: for a `TimeProvider` function,
the serial; for a `RandomProvider` function, the drawn values; for `ExternalProvider`, the
topic snapshots. Conversely, an implementation claiming conformance must not observe
facilities it has not declared — the declaration list is the complete inventory of what can
influence the result.

**Parallel schedulers** read `ThreadSafetyClass` to decide which calls may run concurrently
and which must be serialized onto the host.

## Page vocabulary

Chips a function page may display for the axes in this chapter, with the exact machine names:

| Axis / value | Plain meaning |
|---|---|
| `DeterminismClass::Deterministic` | Same explicit inputs and workbook state, same result |
| `DeterminismClass::PseudoRandom` | Result draws on the random provider |
| `DeterminismClass::TimeDependent` | Result draws on the clock |
| `DeterminismClass::ExternalEventDependent` | Result depends on an external feed |
| `VolatilityClass::NonVolatile` | Recalculated only when a precedent changes |
| `VolatilityClass::VolatileFull` | Recalculated every recalculation cycle |
| `VolatilityClass::VolatileContextual` | Recalculated under specific conditions (provisional classification) |
| `HostInteractionClass::None` | No host-state dependence |
| `HostInteractionClass::WorkbookState` | Depends on workbook-level state beyond cell inputs |
| `HostInteractionClass::ApplicationState` | Depends on application/session state |
| `HostInteractionClass::EnvironmentState` | Depends on platform/environment state |
| `HostInteractionClass::ExternalProvider` | Depends on an external provider |
| `ThreadSafetyClass::SafePure` | Safe to evaluate concurrently |
| `ThreadSafetyClass::HostSerialized` | Safe only under host-serialized invocation |
| `ThreadSafetyClass::NotThreadSafe` | Must not be evaluated concurrently |
| `FecDependencyProfile::None` | No execution-context dependency |
| `FecDependencyProfile::RefOnly` | Needs reference resolution only |
| `FecDependencyProfile::CallerContext` | Needs the calling cell's position/shape |
| `FecDependencyProfile::TimeProvider` | Needs the host time source |
| `FecDependencyProfile::RandomProvider` | Needs the host random source |
| `FecDependencyProfile::ExternalProvider` | Needs an external topic/provider |
| `FecDependencyProfile::LocaleProfile` | Needs locale parsing/format rules |
| `FecDependencyProfile::Composite` | Needs several facility families |
| `CallableArgumentSpec::Fixed(n)` | Argument at position n is a callable (lambda) |
| `CallableArgumentSpec::Last` | The last argument is a callable (lambda) |
| `InvocablePassthrough::Yes` | A callable argument can pass through unchanged to the result |
| `InvocablePassthrough::No` | Callable arguments do not pass through to the result |

## Sources

- `crates/oxfunc_core/src/function_call.rs` — `FunctionExecutionContextBundle` and every
  provider it carries; the `FunctionExecutionContext` trait; callable-argument declarations
  and the derived requires-invoker rule; `InvocablePassthrough`; the hoisting gate
  (`ExpressionHoistPolicy`, `is_hoistable_under`). Implementation source; the
  invocable-passthrough declarations are checked by runtime dispatch tests.
- `crates/oxfunc_core/src/resolver.rs` — `ReferenceSystemProvider` operations,
  `ReferenceSystemCapabilities`, `CallerContext`, capability-denial errors. Implementation
  source.
- `crates/oxfunc_core/src/function.rs` — `DeterminismClass`, `VolatilityClass`,
  `HostInteractionClass`, `ThreadSafetyClass`, `FecDependencyProfile` and their placement on
  `FunctionMeta` (adapter-level and surface-level context profiles). Implementation source.
- `docs/function-lane/EXCEL_FUNCTION_DEFINITION_PRELIM_SPEC.md` — working definitions of
  volatile vs non-deterministic vs host-interactive; the context-facility vocabulary; trigger
  classes; volatility mechanics and the RTD lifecycle (both marked provisional); the
  unresolved `volatile_full` / `volatile_contextual` terminology. Documented; preliminary by
  its own statement.
- `docs/function-lane/EXCEL_FUNCTION_DEFINITION_DISCUSSION.md` — D-001 (volatility vs
  non-determinism), D-002 (host-interaction taxonomy), D-012 (`INDIRECT` classification),
  D-013 (RTD lifecycle), D-015/D-016 (context-profile taxonomy and enforcement). Documented
  open questions.
