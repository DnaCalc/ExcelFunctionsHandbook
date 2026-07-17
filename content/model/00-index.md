# The function-call model

Status: draft (H1)

How is an Excel worksheet function called, exactly? These chapters answer that once, uniformly,
so that every function page can describe its particular behavior as values on shared, named
axes instead of re-explaining the machinery. Behavior chips on function pages link here.

| Chapter | Covers |
|---|---|
| [01 — The value universe](01-value-universe.md) | Value kinds, error codes, text realities, boundaries and admission, Missing vs Empty |
| [02 — Coercion and lifting](02-coercion-and-lifting.md) | Type conversion rules, error propagation, blanks, array lifting, direct-vs-scan asymmetry |
| [03 — The call pipeline](03-call-pipeline.md) | Preparation → coercion → kernel → publication; operators as functions; arity and admission |
| [04 — The execution context](04-execution-context.md) | Providers (time, randomness, locale, references, callables…); determinism, volatility, scheduling classes |
| [05 — Version and platform axes](05-version-axes.md) | Why exact-behavior claims carry build, workbook-mode, and platform scope; localized names |
| [06 — Claim language and honesty](06-claim-language.md) | What a claim is, its statuses, scoping rules, and how to challenge one |
| [07 — About implementation options](07-implementation-options.md) | The four flavours, how implementations are admitted, choosing between them |

Grounding: chapters 01–05 are grounded in the OxFunc reference implementation and its
specification corpus (each chapter lists its exact sources); chapters 06–07 are Handbook
doctrine. All chapters are drafts from the phase H1 review; ambiguities found during that
review are filed in `docs/handoffs/`.
