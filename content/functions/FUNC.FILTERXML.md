---
schema: efh.function-page/v1
function_id: FUNC.FILTERXML
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
family: web_text_xml_family
role_in_family: The parsing member — takes XML text and an XPath expression and returns the selected nodes, with no network access of its own.
---

`FILTERXML` is an XPath engine wearing a worksheet function's clothes. It takes an XML document
*as text* and an XPath expression *as text*, and returns the node set the expression selects.
It makes no network request of its own; the network is `WEBSERVICE`'s job.

The reference engine (OxFunc) **has an implementation module for it**
(`web_text_xml_family.rs`), it is registry-backed with a real signature and a full set of
classification axes, and it appears in surface dispatch — yet it is recorded as **deferred**,
with the reason **"Deferred with the current W041 family despite bounded local work."** As with
`ENCODEURL`, the deferral is a family-scoping decision rather than a statement that the
function is intractable: the projection classifies it as `Deterministic`, `SafePure`,
`NonVolatile`, `host_interaction_class: None`.

That said, "bounded" is doing more work here than it does for `ENCODEURL`. A faithful
`FILTERXML` requires an XML parser and an XPath processor, and Excel's is the Windows one.

## What it computes

`FILTERXML(xml, xpath)` parses `xml` and evaluates `xpath` against it, returning the selected
result.

The part that decides how the function behaves in a worksheet is what happens to a node set
with more than one node. `FILTERXML` returns **all** matching nodes, not the first one: on a
modern Excel the result spills down a column, and on a pre-dynamic-array Excel the same formula
returns only the top-left value unless entered as an array formula over a range. The same
workbook can therefore appear to behave differently on two Excel versions with no change to the
formula — this is a spill-adaptation difference at the host boundary (see
[the call pipeline](../model/03-call-pipeline.md)), not a difference in the function.

The second decisive fact is that the result is **text**. Nodes selected out of XML come back as
strings, whatever they look like. `//price` returning `12.50` gives you the four-character text
`12.5`-style content, not a number, and `//date` gives you an ISO date string, not a serial.
Arithmetic downstream needs an explicit conversion, and text that does not read as a number is
a named coercion failure rather than a silent zero — see
[coercion and lifting](../model/02-coercion-and-lifting.md).

XPath is a real language, not a path syntax: predicates (`//item[@id='3']`), axes, and the
node/attribute distinction (`//book/@isbn` selects attributes) are all available. That
expressiveness is why `FILTERXML` remains useful even though Power Query supersedes it for
serious ingestion work.

## Arguments

`FILTERXML(xml, xpath)` — arity 2 to 2, per the reference engine's registry entry.

- **`xml`** (required) — Microsoft: "A string in valid XML format". The whole document, as one
  text value. In practice that value comes from a `WEBSERVICE` call or from a cell, and the
  cell route imposes the 32,767-code-unit text cap described in
  [the value universe](../model/01-value-universe.md). That cap is the practical size limit on
  what `FILTERXML` can be given, and it is small for real-world XML.
- **`xpath`** (required) — Microsoft: "A string in standard XPath format".

The argument most often misunderstood is `xml`, and specifically the assumption that it can be
a *file*. It cannot. There is no path form, no URL form, and no streaming form; the document
must already be a string in the worksheet.

The second trap is namespaces. Documents that declare a default namespace need the XPath to
account for it, and the usual `local-name()` workaround is verbose enough that many users
conclude `FILTERXML` is broken when it is behaving correctly.

## Result and edge cases

Text, one value per selected node, spilling on Excel builds that support dynamic arrays.

- **No match.** An XPath that selects nothing. Microsoft's page does not state the result; the
  Handbook does not know whether it is `#VALUE!`, `#N/A`, or an empty result, and will not
  guess. It is the first probe below.
- **Attribute versus element selection.** Both are expressible; whether they are returned
  identically as text is undocumented.
- **Whitespace and entities.** Whether the returned text is the raw node content, the
  whitespace-normalized string value, or the entity-decoded value is not stated anywhere in the
  documentation, and the three differ for real documents.

## Errors

As documented by Microsoft on the page linked below:

- **`#VALUE!`** — `xml` is not valid.
- **`#VALUE!`** — `xml` contains a namespace with a prefix that is not valid.

Both documented conditions concern the *document*; nothing is documented about a malformed
`xpath`, which is the more common author error. That gap is listed as a probe.

The platform restriction is firmly documented and is arguably the most important fact for a
workbook author: `FILTERXML` is **not available in Excel for the web or Excel for Mac**.
Microsoft adds that it "may appear in the function gallery in Excel for Mac, but it relies on
features of the Windows operating system, so it will not return results on Mac". A function
present in the gallery that silently does not work is a distinct and badly signposted state,
and it makes `FILTERXML` a poor choice for any workbook that will be shared across platforms.

## Relationships

- **`WEBSERVICE`** — the usual supplier of the `xml` argument. Microsoft's documented idiom is
  `=FILTERXML(WEBSERVICE("…"&ENCODEURL(C2)), "//path")`, which is the three Web functions used
  as a pipeline: encode, fetch, parse.
- **`ENCODEURL`** — the first stage of that same pipeline.
- **Power Query / Get & Transform** — the modern replacement for this workflow, and the one
  Microsoft steers users toward for anything beyond a single small document. Power Query is not
  a function, so it does not appear in this catalog, but it is the honest answer to "should I
  build on `FILTERXML`?"
- **`TEXTSPLIT`, `TEXTBEFORE`, `TEXTAFTER`** — what people use to hack XML apart when
  `FILTERXML` is unavailable (notably on Mac). Structurally wrong for XML, but portable.
- **`WEBSERVICE` returning JSON** — a frequent confusion. `FILTERXML` parses XML only; there is
  no built-in JSON equivalent in the worksheet function surface.

## Notes for implementers

- The XPath dialect and version are unstated. Windows' MSXML supports XPath 1.0; whether Excel
  exposes anything beyond that is unknown to the Handbook. XPath 1.0 versus 2.0 differ in ways
  that are observable from a worksheet (sequence handling, string functions, node comparison),
  so this is not a detail.
- Namespace handling is the largest single source of behavioural surface, and Microsoft
  documents exactly one line about it — that an invalid prefix yields `#VALUE!`. Everything
  else, including default-namespace behaviour, must be pinned by observation.
- The result shape is host-adapted. The function returns a node sequence; whether that spills,
  scalarizes by implicit intersection, or fills an array-entered range is a publication-stage
  question, not a kernel question.
- Text normalization (whitespace, entity decoding, CDATA) is a per-node decision that a
  compatibility implementation must copy rather than choose.
- A `SafePure` classification for an XML parser is a strong claim about the parser: no external
  entity resolution, no DTD fetching, no network. An implementation that hands the document to
  a permissive parser inherits an entity-expansion attack surface the worksheet author never
  asked for.

## What has not been checked

No Handbook vector suite exists for `FILTERXML`, and no Excel-comparison evidence is recorded.
Nobody has checked this function against Excel in the Handbook's record. OxFunc has an
implementation module for it, but the Handbook publishes no measurement of how that
implementation compares with Excel, and this page makes no claim about agreement in either
direction.

`FILTERXML` is checkable without a network — every vector is a pair of literal strings — so the
absence of a suite is a matter of work not yet done, not of an unavailable oracle. The probes
that would settle the most:

1. **Empty node set.** A well-formed document and an XPath matching nothing. Undocumented, and
   the single most common runtime outcome in real use.
2. **Malformed XPath.** `"//["`, `"not-an-axis::x"`, and an empty `xpath` string. The
   documentation covers bad XML and says nothing about bad XPath.
3. **Multi-node results and shape.** A five-node match on a dynamic-array build and on a
   pre-dynamic-array build, plus the same formula wrapped in `@` implicit intersection, to pin
   the publication behaviour.
4. **Text fidelity.** Nodes containing leading/trailing whitespace, mixed content, a CDATA
   section, `&amp;`, and a numeric character reference — to determine exactly which string a
   node maps to.
5. **Namespaces.** A document with a default namespace, one with a declared prefix, and one
   with an undeclared prefix (the documented `#VALUE!` case), each queried with and without
   `local-name()`.
6. **XPath version markers.** Expressions valid only in XPath 2.0 and later, to establish which
   dialect answers.
7. **Size.** A document at and just over the 32,767-code-unit cell cap, delivered by reference
   and directly from `WEBSERVICE`, since the two paths may enforce the cap differently — the
   value-universe chapter records that formula and interop paths already disagree about
   over-cap text.
8. **Platform.** The same vectors on Excel for Mac and Excel for the web, to record what the
   documented non-availability actually produces.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: deferred | OxFunc intentionally defers the function; the recorded reason is "Deferred with the current W041 family despite bounded local work." |
| registry-backed | OxFunc holds a live registry entry with signature and classification axes for this id |
| node set | The sequence of nodes an XPath expression selects |
| spill | The host-side adaptation that lays a multi-value result across neighbouring cells |
| `SafePure` | The thread-safety class the projection records for this function |

## Sources

- Microsoft, "FILTERXML function" —
  <https://support.microsoft.com/en-us/office/filterxml-function-4df72efc-11ec-4951-86f5-c1374812f5b7>
  (syntax, both argument definitions, the two documented `#VALUE!` conditions, and the
  Mac/web availability statement quoted above).
- Microsoft, "ENCODEURL function" —
  <https://support.microsoft.com/en-us/office/encodeurl-function-07c7fb90-7c60-4bff-8687-fac50fe33d0e>
  (the composed three-function example).
- Handbook `data/functions/FUNC.FILTERXML.json` — admission label and reason, arity 2–2,
  signature `FILTERXML(xml, xpath)`, the classification axes quoted above, and the XLL identity
  `xlfFilterxml` (595).
- Handbook `data/presence/FUNC.FILTERXML.json` — the implementing module
  `crates/oxfunc_core/src/functions/web_text_xml_family.rs` and its surface-dispatch entry.
- Handbook `content/model/01-value-universe.md`, `02-coercion-and-lifting.md` and
  `03-call-pipeline.md` — the text cap and its two enforcement paths, the text-to-number
  conversion rule, and the host-side spill adaptation referenced above.
