# GKS-001 — Working and Explicit Presentation Lenses

Date: 2026-07-18  
Source realization: The Handbook of Excel Functions  
Target: Gneiss Public Surface and future presentation conformance profile

## Authorization and Boundary

This handoff was prepared under the Handbook's normal read-only sibling policy. The steward then
explicitly authorized a Gneiss-side implementation pass based on the handoff. The handoff remains
the durable record of what crossed the boundary; Excel-specific vocabulary remains in the
Handbook.

## Pressure Observed

Technically honest pages accumulate qualifications until the primary domain answer becomes hard to
read. Hiding all qualification behind tooltips has the opposite failure: compact labels overclaim,
important limitations become inaccessible on mobile/print, and explanations drift across pages.

The concrete forcing example was a finite-corpus compatibility result. `Bit-exact` was visually
clean but broader than the evidence; repeating the entire corpus/build/platform disclaimer beside
every occurrence made the page noisy.

## Handbook Experiment

The Handbook now declares two presentation lenses over one evaluated answer:

- **Working:** compact domain terminology that is true without expansion;
- **Explicit:** inline expansion of registered meaning, scope, and exclusions.

A reader can switch the page lens or expand one existing term badge. Substantive explanation is
never available only by hover or transient pop-up. Both lenses preserve answer identity, context
hash, result hash, receipt, and staleness.

Implementation artifacts:

- `content/model/08-presentation-lenses.md`
- `content/vocabulary/working-terms.json`
- `site/mockups/function-page.html`
- `site/mockups/homepage.html`

## Candidate Generic Gneiss Contract

1. A presentation lens is not an evaluation context.
2. A compact label must be true without expansion; expanded prose may explain but not rescue it.
3. A material qualified term has a local plain-language rendering and a route to the answer's
   record/receipt.
4. Working and Explicit renderings of one answer preserve its semantic identity and label.
5. Substantive explanation must survive keyboard use, mobile layout, copying, printing, and stable
   linking; tooltip-only disclosure is non-conforming.
6. Domain language and templates remain realization-owned. Gneiss owns only the generic lens and
   answer-envelope invariants.

## Proposed Destination

- Adopt principles 1–6 in `PUBLIC-SURFACE.md` under progressive disclosure.
- Include lens invariants in a candidate experimental profile and its drills.
- Do not change the Core Calculus or Gneiss.Cell for this first adoption.
- Consider a reusable presentation component only after another realization reproduces the need.

## Acceptance Drills

1. Render one answer through Working and Explicit; assert identical answer, context, result, and
   receipt identifiers.
2. Remove a required scope binding; assert that the compact qualified term cannot render.
3. Make the answer stale/contested; assert both lenses display the same semantic state.
4. Walk every material compact term to an inline explanation and the applicable seven doors.
5. Verify Explicit content remains present in mobile and print renderings.

## What Must Not Move Upstream

`suite-exact`, Excel builds, workbook modes, CPU-specific exactness, ULP language, and Handbook
flavour names are `efh.*` domain vocabulary, not Gneiss primitives.
