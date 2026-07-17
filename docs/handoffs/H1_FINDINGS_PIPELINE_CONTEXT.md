# H1 findings: call pipeline and execution context chapters

Scope: ambiguities, contradictions, stale statements, and open questions encountered while
drafting `content/model/03-call-pipeline.md` and `content/model/04-execution-context.md`
against OxFunc 937f198. Internal handoff document, not site content.

1. **Spec/code vocabulary drift: error policy axis.** The definition prelim spec
   (`docs/function-lane/EXCEL_FUNCTION_DEFINITION_PRELIM_SPEC.md` §3.1 item 8) names an
   `error_policy_class` axis with values `strict_propagate | conditional_mask |
   branch_selective | custom`. The implemented axis is `ErrorCollapseProfile { None,
   ReductionFold, SelectorBranch }` plus `ErrorAlgebra` (`crates/oxfunc_core/src/function.rs`,
   `semantic_kernel.rs`) — different names, different partition (the spec has no
   reduction-vs-selector split; the code has no `conditional_mask`). Chapters use the code
   names since those are the checkable declarations. Matters because a reader reconciling
   spec and implementation will find no `error_policy_class` anywhere in code.

2. **Spec axis with no implementation: `compile_eval_class`.** Spec §3.1 item 12 declares
   `compile_eval_class` (`const_foldable_when_closed | runtime_ref_dependent |
   runtime_context_dependent`) as a per-function tag. No such field exists on `FunctionMeta`;
   the implemented mechanism is the opposite direction — hoistability is *derived* from the
   other axes via `ExpressionHoistPolicy` / `is_hoistable_under`
   (`crates/oxfunc_core/src/function_call.rs`). Likely a superseded design; the spec does not
   say so. Chapter 04 describes only the derived gate.

3. **`volatile_full` vs `volatile_contextual` is load-bearing while officially unresolved.**
   Spec §3.2 item 9 marks the two volatile forms "retained as unresolved terminology pending
   interactive policy finalization", yet `VolatilityClass` ships both variants and
   `volatility_allows_hoist` (function_call.rs) gives them distinct hoisting semantics.
   Chapter 04 flags the classification as provisional on pages that display
   `VolatileContextual`. Matters because a consumer scheduling on this chip is building on a
   boundary the sources say may still move.

4. **Operator identity spelling differs across sources.** The definition spec §6 writes
   operator rows as bare `OP_*` (`OP_UNION_REF`); the task framing used "OP.* identities";
   the actual catalog ids are `FUNC.OP_*` (`FUNC.OP_ADD`, `FUNC.OP_TRIM_REF_BOTH`, … —
   `crates/oxfunc_core/src/registry_signature_seed.rs`). Chapters use the real `FUNC.OP_*`
   ids. Site tooling that renders chips should treat `OP_*` in older docs as the same
   identity family.

5. **Duplicate coercion-profile vocabulary in code.** `CoercionLiftProfile` (function.rs)
   and the local `UnaryNumericCoercionLiftProfile { ScalarOnly, ScalarOrArrayElementwise }`
   (`crates/oxfunc_core/src/functions/adapters.rs` lines 9–13) encode overlapping
   unary-numeric vocabulary as two distinct Rust types. Not a behavior contradiction, but a
   second source a future page generator could latch onto by mistake; chapters use only the
   `FunctionMeta`-carried enum.

6. **Layering spec adoption baseline looks stale.** `FUNCTION_ADAPTER_LAYERING_PRELIM_SPEC.md`
   §8.1 lists shared-runner adoption as ABS, ISNUMBER, OP_ADD, SUM, SEQUENCE, INDIRECT (plus
   XMATCH split) — a handful — while function.rs documents 241 catalog `FunctionMeta` entries
   (199 values-only). The spec snapshot predates the catalog build-out. Matters if anyone
   cites §8.1 as the current adoption state.

7. **Argument-gap policy (D-005) is open but the mechanism is settled.** The pipeline
   distinguishes a missing argument from an empty cell as first-class values (adapters.rs
   `coerce_prepared_to_number`: `Missing → MissingArg`, `Empty → EmptyCell`), yet the
   per-family compatibility policy for gaps like `=SUM(A1,,B1)` is explicitly undecided
   (discussion register D-005). Chapter 03 states the mechanism and marks the policy open.
   Function pages will need per-family answers as they are pinned.

8. **Asymmetric `Composite` handling in the hoisting gate.** In function_call.rs, an
   adapter-level `FecDependencyProfile::Composite` is never hoistable (returns `false` under
   every policy, including `FIXED_EXECUTION_CONTEXT`), while a surface-level `Composite` is
   decomposed as "adapter dependency + ref-only". Conservative and probably intentional, but
   undocumented in either spec; a function whose adapter genuinely composes two pinnable
   facilities (e.g. time + locale) can never be hoisted even with everything pinned. Worth a
   source-side comment or a spec note before the axis is presented as fully rationalized.

9. **Discussion register has no closed decision log.** `EXCEL_FUNCTION_DEFINITION_DISCUSSION.md`
   defines a decision-log template (§3) but records no filled entries, while the code
   implements concrete choices for several "open" topics (D-001 separation is implemented;
   D-008 operator inventory exists in the registry; D-018 is explicitly deferred). Which
   decisions are actually final is not recoverable from these documents alone — the code is
   ahead of the decision record. The handbook currently cites topics as "recorded open
   decisions" even where the implementation has clearly picked an answer.

10. **Normative FEC reference is outside the repo.** Both the definition spec (§3.4) and its
    D-015 discussion point to
    `../../../Foundation/reference/conformance/excel-worksheet-engine/model/EXCEL_FORMULA_EVALUATION_CONTEXT_FEC.md`
    for the capability-family definitions — a path outside the OxFunc repo, unavailable to
    this draft. The chapters ground the context-facility vocabulary only in the in-repo enum
    (`FecDependencyProfile`) and the spec's working list. If the external FEC document
    diverges, chapter 04's facility descriptions may need reconciliation.

11. **`NonFinite::Allow` states an unchecked invariant.** Its doc comment says the kernel
    "cannot produce a non-finite result for any valid argument (so this never fires; it
    documents that intent)" — but `Allow` rides on `ExcelRealPolicy::PASS`, the default for
    the overwhelming majority of functions, and nothing at the publication layer enforces the
    claim. If a defaulted kernel ever did overflow, the raw infinity would pass through.
    Chapter 03 phrases `Allow` as a declaration of intent, matching the source; flagging here
    because the distinction between "checked" and "declared" matters for the handbook's claim
    language.

12. **Two-surface admission boundary is only seed-pinned.** The admission-vs-runtime split
    (spec §5.4, D-017) rests on canonical seeds (`SIN()` rejected at entry; `SIN("asd")`,
    `ASIN(2)` runtime errors) with the spec itself noting public references are "too thin to
    close this lane". Chapter 03 presents the boundary as real but the per-family policy as
    open; function pages should not imply admission behavior is known beyond the pinned
    seeds.
