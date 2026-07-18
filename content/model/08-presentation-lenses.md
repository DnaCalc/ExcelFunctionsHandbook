# Working and Explicit presentation lenses

Status: draft (post-H1 design) · Sources: Handbook Charter, Gneiss Public Surface

## The trade-off

A technical reference becomes unreadable if every sentence carries its complete epistemic
qualification. It becomes untrustworthy if compact language hides a materially narrower claim.
The Handbook resolves this with two presentation lenses over one evaluated answer.

## Working

Working is the default reading lens. It uses compact, stable, domain-first terminology such as:

> Excel compatibility · Rust · suite-exact, v3

The compact term must be true on its own. A hidden explanation may clarify its scope; it may not
turn a misleading label into a defensible one. Only load-bearing, independently challengeable
terms receive an explanation affordance.

## Explicit

Explicit expands every registered qualification next to the answer it qualifies:

> This implementation matched Excel bit-for-bit for all 48,112 inputs in suite v3 under the named
> Excel build and platform. Inputs outside that suite are not covered by this claim.

The expansion is inline, persistent, addressable, printable, and accessible. It is never available
only by hover, tooltip, or transient pop-up. A reader may switch the whole page to Explicit or
expand one existing working-term badge in place; no separate forest of information icons is added.

## The Gneiss invariant

Working and Explicit are presentation state, not belief state:

```text
same question + same evaluation context -> same answer + same receipt
same answer + different presentation lens -> different disclosure, never different belief
```

Both lenses therefore preserve answer identity, accepted/defeated/contested state, context hash,
result hash, receipt, and staleness. When evidence changes, both lenses change together.

## What is generated and what is curated

Gneiss supplies the answer envelope: status, scope bindings, basis, receipt, and explanation trace.
The Handbook supplies a versioned domain registry that maps those fields to compact terminology
and plain-language explanation templates. The renderer combines them. Function pages do not copy
and hand-edit the same disclaimer.

The registry entry for a material term declares:

- its stable id and compact label;
- a plain-language meaning template;
- a boundary or non-claim statement;
- fields required before the term may render;
- the doors that must remain reachable.

## The record below both lenses

Explicit is explanation, not the entire audit record. Both lenses lead to the same seven doors:

1. Why
2. Sources
3. As of
4. Rules
5. History
6. Limits
7. Replay

Challenge is an adjacent action: it records rival testimony or a proposed decision; it does not
edit the published answer directly.

## Interaction and accessibility rules

1. The page-level lens control is quiet, keyboard operable, and reflected in a stable URL.
2. A working-term badge may itself expand its local explanation and exposes `aria-expanded`.
3. Explicit content remains in document flow for mobile, copying, printing, and assistive tools.
4. The first material use may expand locally; repeated uses need not duplicate the prose.
5. Status is never carried by color alone.
6. The site generator fails if a registered term's required fields are absent.

## Initial conformance drills

- render one answer through both lenses and compare answer, context, result, and receipt ids;
- remove a required scope binding and confirm the compact term cannot render;
- make the answer stale and contested in fixtures and confirm both lenses agree;
- mechanically walk every material term to its expansion and seven doors;
- print and mobile-render Explicit view without losing substantive explanation.

## Vocabulary source

The initial machine-readable registry is `content/vocabulary/working-terms.json`. It is a Handbook
domain artifact. Generic lessons from its use are handed to Gneiss; Excel-specific terms remain
here.
