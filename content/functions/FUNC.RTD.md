---
schema: efh.function-page/v1
function_id: FUNC.RTD
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it computes
  - Arguments
  - Why a reference engine cannot answer this
  - Result and edge cases
  - Errors
  - Relationships
  - Notes for implementers
  - What has not been checked
  - Page vocabulary
  - Sources
family: rtd_fn
role_in_family: >-
  Subscribes a cell to a topic published by an external COM real-time-data server; the value is
  pushed by the server, not computed by Excel.
---

## What it computes

`RTD` does not compute a value. It **names a subscription** and returns whatever the named
server most recently pushed for that topic.

The mechanics, as the function's interface implies: `RTD(prog_id, server, topic1, …)`
identifies a COM automation server by its programmatic identifier, optionally on a named
machine, and hands it an ordered list of topic strings. The server registers the topic, and
thereafter *the server* decides when a new value exists and notifies Excel; Excel then
refreshes the subscribing cells. The direction of control is inverted relative to every other
worksheet function: an ordinary function is pulled by recalculation, and `RTD` is pushed by an
external process.

The canonical use is a market-data feed — one `RTD` call per instrument per field, thousands of
them, updating several times a second without the user touching anything.

Three consequences follow directly, and they are what make `RTD` interesting to a handbook of
function semantics rather than a handbook of Excel features:

1. **`RTD` is not deterministic.** Two evaluations with identical arguments may legitimately
   return different values, because the value is a function of time and of an external
   process's state, not of the arguments.
2. **`RTD` is volatile**, and volatile in the strongest sense: it participates in recalculation
   because something outside the workbook said so.
3. **`RTD` has no mathematical content whatsoever.** There is no kernel to be accurate about,
   no rounding to characterize, no residual plate to draw. Its entire semantics is a protocol.

## Arguments

The projected signature is `RTD(prog_id, server, topic1, [topic2], …)`, with an arity of three
to 255 arguments.

**`prog_id`** — text. The COM programmatic identifier of the real-time-data server (the string
under which the server registered itself on the machine).

**`server`** — text. The machine on which the server runs. Empty text means the local machine;
this is the argument readers most often mis-handle, because "omit it" and "pass an empty
string" are not the same thing, and it is a required position rather than an optional one.

**`topic1`, `topic2`, …** — text. The topic strings passed to the server, in order. Their
meaning is entirely the server's business: Excel neither parses nor validates them. Two servers
can use the same topic string for unrelated data, and there is no schema anywhere in the
system.

All the arguments are text, and all of them are coerced to text before being handed on. That
makes `RTD` calls fully dynamic — the topic can be built from cells — and it also means a typo
is not a syntax error; it is a topic the server has never heard of.

## Why a reference engine cannot answer this

The Handbook's reference engine classifies `RTD` as `ExternalEventDependent`, with an
`ExternalProvider` host-interaction class and an `ExternalProvider` dependency profile. Those
are not decorative labels. The Handbook's uniform behavioural battery records `RTD` as
**not dispatchable** — `cannot-call:nondeterministic-by-declaration:external-event-dependent` —
for every probe row.

That is the honest position and it is worth stating plainly rather than apologetically: a pure
function evaluator cannot evaluate `RTD`, because the answer does not exist inside the
evaluator. Producing one would require a live COM server, a machine on which it is registered,
a subscription, and a moment in time. None of those are properties of the formula.

What *can* be modeled, and what the reference engine does model, is the **shape of the
interaction**: parse the arguments into a request (a program identifier, a server name, an
ordered list of topics), hand that request to a provider interface, and classify the provider's
answer — a value, "no value yet", a denied capability, a failed connection, or a provider-
supplied error. That classification is the part of `RTD` a reference implementation can be
correct about. The value is not.

This is the same boundary that `GETPIVOTDATA` sits on, from the other side: `GETPIVOTDATA`
needs a workbook structure the engine does not own, and `RTD` needs a process the engine cannot
start.

## Result and edge cases

The return kind is whatever the server supplies — typically a number or text, and, in the
transient states, one of the extended error codes.

- **Before the first value arrives**, the documented worksheet experience is a transient
  placeholder. [The value universe](../model/01-value-universe.md) models `#GETTING_DATA` and
  `#BUSY!` as exactly this: extended-family error codes standing in while external data is
  retrieved. Which one appears, and for how long, is host and server behaviour.
- **When the server is unavailable or blocked by policy**, `#N/A`, `#CONNECT!` or `#BLOCKED!`
  are the codes the model reserves for those conditions. The Handbook has not verified which
  `RTD` actually produces under which condition.
- **Values persist across recalculation** until the server pushes a new one; a cell holding an
  `RTD` value is not recomputed from its arguments in the ordinary sense.
- **Saved workbooks** retain the last received value as a cached result, which reopens without
  the server present. That cached value is indistinguishable from a live one by inspection.
- **Threading.** `RTD` is classified `HostSerialized`: it cannot be evaluated on an arbitrary
  calculation thread, because the COM interaction has affinity.

## Errors

Microsoft's documentation for `RTD` describes the function's purpose and arguments; the
Handbook has not confirmed an enumerated error table on that page and does not assert one.

The error surface a reader meets in practice comes from three places, and only the first is
`RTD`'s own:

1. arguments that cannot be coerced to text — `#VALUE!` under the ordinary coercion rules;
2. the transient and connection codes listed above, produced by the host on the server's
   behalf;
3. security policy — real-time-data servers are executable code, and a workbook that arrives
   with `RTD` formulas is a workbook that wants to start a process. Whether it may is a trust
   decision made outside the formula.

## Relationships

- **`GETPIVOTDATA`** is the other function in this Handbook's lookup-and-reference category
  that requires a facility the reference engine does not own. `RTD` needs an external process;
  `GETPIVOTDATA` needs PivotTable structure.
- **`WEBSERVICE` / `FILTERXML`** are the other externally-sourced functions, but they are
  pull-based and synchronous — a request made during evaluation, not a subscription.
- **`NOW` and `TODAY`** are the mundane non-deterministic functions: volatile, but with a
  source of truth (the clock) that any implementation has.
- **User-defined functions marked as volatile** and add-in async functions occupy the adjacent
  ground; the C API's asynchronous-function support is the same problem solved a different way.
- Readers sometimes expect `RTD` to be a data-import function. It is not: it delivers one
  value per call, pushed, with no schema.

## Notes for implementers

- Separate the request from the resolution. Parsing `(prog_id, server, topics…)` into a
  well-formed request is deterministic and testable; obtaining a value is not. The reference
  engine draws exactly this line, with a provider trait on one side and request parsing on the
  other.
- Model the provider's non-value outcomes explicitly — not-yet, capability denied, connection
  failed, provider error — rather than collapsing them into one failure. They surface as
  different worksheet codes and they have different remedies.
- Topic identity matters: the same `(prog_id, server, topics)` tuple from two cells is one
  subscription, and the engine must not create two.
- Do not attempt to make `RTD` deterministic for testing by caching. A cached `RTD` value is a
  correct workbook state and an incorrect test oracle.
- `HostSerialized` thread safety is a hard constraint, not an optimization hint.

## What has not been checked

No Handbook vector suite exists for `RTD`, and no Handbook evidence record is attached to this
page — and, unlike most entries in this Handbook, that is not merely a gap in effort. A vector
suite in the Handbook's sense captures inputs and exact result bits with oracle provenance;
`RTD`'s result is not a function of its inputs, so there is nothing for such a suite to
capture. The battery record for this entry says so directly: every probe row is
`cannot-call:nondeterministic-by-declaration:external-event-dependent`.

What could still be characterized, and has not been:

1. **The transient-code sequence.** With a controlled test server, what a cell shows between
   subscription and first value, and whether it is `#GETTING_DATA`, `#BUSY!`, or something
   else — one of the open classification questions the value-universe chapter names explicitly.
2. **Failure codes by condition**: server not registered, machine unreachable, policy blocked,
   server returning an error.
3. **Argument coercion**: numeric and logical arguments in the topic positions, empty text in
   `server`, and the behaviour at the 255-argument ceiling.
4. **Subscription identity**: whether two cells with identical arguments produce one
   subscription, observable through a counting test server.
5. **Persistence**: what a saved workbook holds and what happens on reopen without the server.

Every one of those requires a test server and a host, not a function evaluator. That is the
honest characterization of this entry: the semantics are a protocol, and the protocol has not
been characterized here.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| subscription | The standing registration of a topic with a server; the thing `RTD` really creates |
| push model | The server decides when a new value exists; evaluation does not pull it |
| `ExternalEventDependent` | Determinism class: the result depends on events outside the evaluator |
| not dispatchable | Battery outcome: the reference engine declines to call the function at all |
| transient placeholder | `#GETTING_DATA` / `#BUSY!` — extended error codes shown while data is pending |

## Sources

- Microsoft, *RTD function* —
  <https://support.microsoft.com/en-us/office/rtd-function-e0cc001a-56f0-470a-9b19-9455dc0eb593>
  (purpose and the `prog_id` / `server` / `topic` argument structure). Not retrieved for this
  page; the behaviour above is stated as documented behaviour and should be re-checked against
  the page.
- Handbook `content/model/01-value-universe.md` (the extended error family, including
  `#GETTING_DATA`, `#BUSY!`, `#CONNECT!` and `#BLOCKED!`, and the open question of how the
  transient placeholders are classified).
- Handbook `content/model/03-call-pipeline.md` (host interaction and the boundary between
  function semantics and engine obligations).
- Handbook `data/battery/FUNC.RTD.json` — the uniform battery record, every row
  `cannot-call:nondeterministic-by-declaration:external-event-dependent`.
- Handbook `data/functions/FUNC.RTD.json` — signature, arity 3–255, and the
  `ExternalEventDependent` / `ExternalProvider` / `HostSerialized` / `VolatileContextual`
  classification axes.
- OxFunc `crates/oxfunc_core/src/functions/rtd_fn.rs` — the reference engine's request/provider
  split: a parsed `RtdRequest` and a provider result classified as value, not-yet, capability
  denied, connection failed, or provider error.
