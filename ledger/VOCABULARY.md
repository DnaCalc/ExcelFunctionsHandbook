# Ledger Vocabulary (`efh.*`) — Draft

Status: `draft` — declarations land in the ledger in phase H3; this document is the human-readable
contract for them. Subjects follow the per-claim-subject rule (one subject per independent claim,
so claim keys never collide across siblings).

## Subjects

| Subject form | Meaning |
|---|---|
| `fn:<function_id>` | A function/operator row, e.g. `fn:FUNC.GAMMALN` |
| `impl:<function_id>:<flavour>:<lang>` | One implementation, e.g. `impl:FUNC.ABS:excel-bitexact:rust` |
| `suite:<function_id>:v<N>` | One test-suite version |
| `claim:compatibility:<function_id>:<observation-context>` | One Excel-compatibility claim family; rival assertions about the same scope deliberately share this subject |
| `page:<function_id>` · `episode:<n>` | Curated content units |

## Predicates (v1, ≤10)

| Predicate | Value | Notes |
|---|---|---|
| `efh.identity` | json: surface name, category, XLL symbol/code | copied from OxFunc registry projection |
| `efh.signature` | json: display + arg specs | null-signature rows stay unclaimed (placeholder honesty) |
| `efh.classification` | json: behavioral axes | tier-A/B stable fields only for hard coupling |
| `efh.admission` | text: supported / deferred(reason) / … | from OxFunc admission policy join |
| `efh.impl` | json: flavour, language, source path+hash | existence of an implementation |
| `efh.impl.verification` | json: suite version, manifest hash, pass counts | impl passes suite |
| `efh.compatibility` | json: target, warrant, observation context, suite version+hash | `suite-exact` is distinct from stronger declared-domain bit-exactness |
| `efh.suite` | json: version, count, sha256, oracle provenance | suite publication fact |
| `efh.page` | json: path, content hash | curated page exists at this state |
| `efh.story` | json: episode number, path, function ids | "The Last Bit" linkage |

## Contexts (v1)

| Context | Policy |
|---|---|
| `public-current` | decided-only admission; the default published belief view with a pinned observation baseline |
| `as-published-<snapshot>` | pinned view for each site publication |

Per-Excel-build/platform contexts arrive with multi-build oracle data (roadmap).

`working` and `explicit` are presentation lenses, not ledger contexts. Both render one answer and
must preserve its context hash, result hash, and receipt.

## Rules

1. Community submissions enter as `proposed` assertions; only steward decisions admit to the
   `current` view. Nothing is auto-admitted.
2. Every assertion carries `src` provenance (e.g. OxFunc snapshot generation + commit, suite
   manifest hash, submission reference).
3. References (books/papers/code) are curated content, not ledger claims, in v1; substrate
   identification statements that carry evidentiary weight are claims.
4. Working-term definitions and explanation templates live in the curated content organ at
   `content/vocabulary/working-terms.json`. Predicate declarations and rendered claims reference
   its version; the prose templates themselves are not duplicated as ledger assertions.
