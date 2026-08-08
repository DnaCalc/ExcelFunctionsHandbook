---
schema: efh.function-page/v1
function_id: FUNC.IMAGE
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
family: image_fn
role_in_family: The sole member of its module, carried by OxFunc as the representative of rich-value publication rather than of scalar return.
---

`IMAGE` is the function that broke the assumption that a formula returns a scalar. It does not
return a picture *file*, a URL, or text: it returns a **rich value** that the host renders in
the cell, and that can be sorted, filtered, referenced and copied like any other cell value.

The reference engine (OxFunc) **admits** it (`admission.label: supported`), holds a live
registry entry with a documented signature and per-parameter descriptions, and implements it in
its own module, `image_fn.rs`. The projection's own note is unusually explicit about why: "The
first widened tranche uses IMAGE as the rich-value publication representative rather than an
ordinary scalar-return function." In other words `IMAGE` is carried as the *exemplar* of a
return shape, not because anyone needs image rendering from a reference engine. Its recorded
axes say the same thing: `rich_value_usage: ProducesRichObject`, `host_interaction_class:
ExternalProvider`, `fec_dependency_profile: ExternalProvider`, `ExternalEventDependent`,
`HostSerialized`, `NonVolatile`.

Admission here means the identity, the signature and the return shape are modelled. It is not a
statement that any behaviour has been demonstrated against Excel.

## What it computes

`IMAGE(source, …)` produces a value whose **core projection** is not the interesting part. In
the two-tier model of [the value universe](../model/01-value-universe.md), a rich value carries
a typed payload plus a core projection that legacy surfaces see; `IMAGE`'s payload is the image
reference and its presentation parameters. What a formula referencing an `IMAGE` cell receives,
and what an add-in reading it through the C API receives, are governed by that projection —
they are not the picture.

The retrieval itself is not the function's semantics. Microsoft's constraints make that clear
and are worth reading as a policy specification rather than as trivia:

- **HTTPS only.** The `source` must use `https`.
- **Supported formats:** BMP, JPG/JPEG, GIF, TIFF, PNG, ICO and WEBP, with WEBP unsupported on
  Web and Android.
- **No authentication.** Images requiring authentication will not render.
- **No redirects.** Redirected URLs are blocked, explicitly for security reasons.

That last pair is the design statement: `IMAGE` will only fetch something a plain anonymous GET
to a fixed HTTPS URL returns. It is deliberately not a general web fetcher, and the contrast
with `WEBSERVICE` — which allows redirects and returns arbitrary text — is instructive about
how Microsoft's posture on formula-initiated network access changed between the two functions.

## Arguments

`IMAGE(source, [alt_text], [sizing], [height], [width])` — arity 1 to 5, matching the reference
engine's recorded arity.

- **`source`** (required) — the HTTPS URL of the image file. Microsoft notes a **255-character
  limit** on the URL as written in the formula, with the documented workaround of referencing a
  cell that contains the longer URL. This is the same inline-versus-referenced asymmetry the
  cube functions carry, and it catches generated workbooks the same way.
- **`alt_text`** (optional) — alternative text describing the image for accessibility. It is a
  real payload field, not a comment: it is what a screen reader announces.
- **`sizing`** (optional) — one of four modes:

  | Value | Behaviour |
  |---|---|
  | 0 | fit the image in the cell, preserving aspect ratio |
  | 1 | fill the cell, ignoring aspect ratio |
  | 2 | keep the original image size, which may exceed the cell |
  | 3 | use the custom `height` and `width` arguments |

- **`height`** (optional) — custom height in pixels; used only when `sizing` is 3.
- **`width`** (optional) — custom width in pixels; used only when `sizing` is 3. If only one of
  the two is supplied, the aspect ratio is preserved.

The argument interaction that trips people up is **`sizing` versus `height`/`width`**, and it
is stricter than it looks. Supplying `height` or `width` with `sizing` 0, 1 or 2 is documented
as a `#VALUE!` error — not an ignored argument. Excel does not usually behave this way; unused
optional arguments are more often tolerated. Here the combination is validated, in both
directions: `sizing` 3 without valid `height` and `width` is likewise `#VALUE!`.

## Result and edge cases

A rich value that the host renders in place. The edges worth naming:

- **It is a cell value, not a floating object.** Unlike a pasted picture, an `IMAGE` result
  moves, sorts and filters with its row. That is the entire reason the function exists.
- **The core projection matters at every boundary.** Copying to another application, reading
  through the C API or automation, and saving to a legacy format each cross a boundary where
  the rich payload may not survive. The value-universe chapter's admission matrix is the frame
  for all of these questions; the Handbook has not measured any of them for `IMAGE`.
- **The image is fetched, not embedded.** The workbook holds a reference. A workbook that
  displays correctly on the author's machine may show nothing on a machine without network
  access to the host, and archival copies are not self-contained.
- **`alt_text` is part of the value.** Changing it changes the value, not merely a display
  attribute.

## Errors

As documented by Microsoft, `#VALUE!` covers a notably wide set of distinct causes:

- the image format is unsupported;
- `source` or `alt_text` is not a string;
- `sizing` is not between 0 and 3;
- `sizing` is 3 but `height` and `width` are missing or invalid;
- `sizing` is 0, 1 or 2 but `width` or `height` was supplied anyway.

Two further codes are documented, and they are the two *extended*-family codes that the
value-universe chapter lists for precisely this situation:

- **`#CONNECT!`** — an internet-connection or server problem.
- **`#BLOCKED!`** — security settings are blocking access.

`IMAGE` is therefore one of the clearest published cases of the extended error family being
reachable from an ordinary worksheet formula, and a useful anchor for anyone trying to
understand what `#CONNECT!` and `#BLOCKED!` are for.

## Relationships

- **`WEBSERVICE`** — the older formula-initiated fetch. Both reach the network from a cell;
  `IMAGE` is narrower by design (HTTPS only, no redirects, no authentication) and returns a
  rich value rather than text.
- **`HYPERLINK`** — returns text plus a presentation hint, and is the classic example of a
  function whose result is core-valued but display-affected. `IMAGE` goes a step further: its
  result is genuinely rich.
- **Linked data types (Stocks, Geography) and `STOCKHISTORY`** — the other modern
  provider-backed surfaces. Linked data types share `IMAGE`'s rich-value machinery;
  `STOCKHISTORY` shares its provider dependence but returns an ordinary spilled array.
- **Pasted pictures and "Place in Cell" images** — the user-interface features that produce the
  same kind of in-cell image without a formula. Readers confuse the two constantly; the
  difference is that only the formula form recalculates.

## Notes for implementers

- The return shape is the specification. An implementation that returns the URL as text
  reproduces nothing of interest; the point of `IMAGE` is the rich value and its core
  projection. This is exactly why OxFunc carries it — as the representative of that return
  shape.
- Argument validation is unusually strict and cross-argument. `sizing` and `height`/`width` are
  validated against each other in both directions, which means validation cannot be done
  per-argument.
- The 255-character inline URL limit is an Excel behaviour, not a URL standard, and must be
  enforced as such.
- The security posture — HTTPS only, no redirect following, no authenticated fetches — is part
  of the function's contract, not an environmental accident. An implementation that follows
  redirects is more permissive than the documented function.
- `NonVolatile` plus `ExternalEventDependent` is an interesting pair: the function is not
  re-evaluated on every calculation, yet its rendered result depends on a remote resource that
  can change or vanish. Cache and refresh policy is host behaviour and is not specified by the
  function.

## What has not been checked

No Handbook vector suite exists for `IMAGE`, and no Excel-comparison evidence is recorded.
Nobody has checked this function against Excel in the Handbook's record. OxFunc has an
implementation module for it, but the Handbook publishes no measurement of how that
implementation compares with Excel, and this page makes no claim about agreement in either
direction.

A conventional value-comparison suite does not fit a function whose result is an image. What
*can* be characterized — and what would be most valuable — is the **argument-validation surface
and the core projection**, neither of which needs a real image to test. The probes, in order:

1. **The `sizing`/`height`/`width` validation matrix.** All four `sizing` values crossed with
   {neither, height only, width only, both}, plus `sizing` omitted. Twenty cells, entirely
   documented-behaviour territory, and the cheapest way to confirm the strictness described
   above.
2. **Out-of-range and non-integer `sizing`.** `-1`, `4`, `2.5`, `TRUE`, and text `"0"`.
3. **The core projection.** `ISTEXT`, `ISNUMBER`, `ISERROR`, `LEN`, `=A1&""` and `=A1+0`
   applied to an `IMAGE` cell; the same cell read through the C API and through automation.
   This is the question the Handbook most wants answered, because it is the one the
   value-universe chapter's admission matrix predicts and nobody has confirmed.
4. **Boundary crossings.** Save to `.xls`, copy to another workbook, copy out to another
   application, and read after reopening without network access.
5. **The 255-character URL boundary.** URLs of exactly 255 and 256 characters, inline and by
   cell reference.
6. **`#CONNECT!` and `#BLOCKED!` reachability.** A URL on an unreachable host, an HTTP (not
   HTTPS) URL, a redirecting URL, and a URL requiring authentication — four different failures
   that the documentation maps onto at least three different codes.
7. **Format coverage.** One file of each documented format, plus one undocumented format
   (SVG is the obvious probe), plus WEBP on Web and Android where it is documented as
   unsupported.
8. **`alt_text` as value.** Two `IMAGE` cells differing only in `alt_text`, compared with `=`,
   to see whether the payload participates in equality.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: available | OxFunc admits the id to its implementation surface; this says nothing about demonstrated behaviour |
| rich value | An extension-layer value with a typed payload plus a core projection |
| core projection | The ordinary-gamut value legacy surfaces see when a rich value crosses a boundary |
| `ProducesRichObject` | The rich-value-usage axis the projection records for this function |
| `#CONNECT!` / `#BLOCKED!` | Extended-family error codes for a failed service connection and for policy-blocked evaluation |

## Sources

- Microsoft, "IMAGE function" —
  <https://support.microsoft.com/en-us/office/image-function-7e112975-5e52-4f2a-b9da-1d913d51f5d5>
  (syntax, all five arguments, the four-value `sizing` table, the supported image formats and
  the WEBP platform exception, the HTTPS requirement, the no-authentication and no-redirect
  rules, the 255-character URL limit and its cell-reference workaround, and every `#VALUE!`,
  `#CONNECT!` and `#BLOCKED!` condition listed above).
- Handbook `data/functions/FUNC.IMAGE.json` — admission label `supported`, arity 1–5, the
  signature with per-parameter descriptions, the classification axes quoted above, and the
  recorded note naming `IMAGE` as the rich-value publication representative.
- Handbook `data/presence/FUNC.IMAGE.json` — the implementing module
  `crates/oxfunc_core/src/functions/image_fn.rs` and its surface-dispatch entry.
- Handbook `content/model/01-value-universe.md` — the two-tier rich-value model, the core
  projection, the boundary admission matrix, and the extended error-code family.
