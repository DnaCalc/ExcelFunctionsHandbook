---
schema: efh.function-page/v1
function_id: FUNC.ENCODEURL
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
role_in_family: The purely local member — percent-encodes text with no network access, and is deferred only by association with the web family it serves.
---

`ENCODEURL` is the odd one out among Excel's three Web functions. `WEBSERVICE` makes a network
request and `FILTERXML` parses what comes back; `ENCODEURL` does neither. It is a string
transformation, computable offline, deterministic, with no host and no service behind it.

Its status in the reference engine reflects exactly that tension. OxFunc **has an
implementation module for it** (`web_text_xml_family.rs`), it is registry-backed with a real
signature and a full set of classification axes, it appears in surface dispatch — and it is
still recorded as **deferred**, with the reason **"Deferred with the current W041 family
despite bounded local work."** That wording is worth reading carefully: the deferral is a
*scoping decision about the family*, not a judgement that the function is hard. "Bounded local
work" is the upstream record acknowledging that this one is tractable. It travels with
`WEBSERVICE` and `FILTERXML` because that is how the version's work was cut, not because it
needs a network.

The classification the projection carries makes the point in machine terms: `TextToText`
kernel, `Deterministic`, `SafePure`, `NonVolatile`, `host_interaction_class: None`. Nothing in
that profile touches the outside world.

## What it computes

`ENCODEURL(text)` returns a URL-encoded copy of `text`: Microsoft's description is that it
replaces certain non-alphanumeric characters with "the percentage symbol (%) and a hexadecimal
number".

The documented example fixes the *strength* of the encoding, which is the fact worth taking
away from this page. Given

```
http://contoso.sharepoint.com/Finance/Profit and Loss Statement.xlsx
```

Microsoft documents the result as

```
http%3A%2F%2Fcontoso.sharepoint.com%2FFinance%2FProfit%20and%20Loss%20Statement.xlsx
```

Two things follow directly from that one example:

1. **It is a component encoder, not a URL sanitizer.** The colon and the slashes of the scheme
   and path are themselves encoded (`%3A`, `%2F`). `ENCODEURL` does not preserve URL structure
   — it destroys it. Feeding a whole URL to `ENCODEURL` and using the result as a URL will not
   work. The function is for encoding *a value you are about to place inside* a URL: a query
   parameter, a path segment, a search term.
2. **Space becomes `%20`, not `+`.** The two conventions differ (`+` belongs to
   `application/x-www-form-urlencoded`), and mixing them is a classic source of silently wrong
   query strings. The documented example settles which one Excel uses.

That is the whole of what the documentation pins. It does not enumerate the character set that
survives unencoded, and it does not state how non-ASCII text is handled — both are listed under
what has not been checked.

## Arguments

`ENCODEURL(text)` — arity 1 to 1, per the reference engine's registry entry.

- **`text`** (required) — Microsoft: "A string to be URL encoded".

The function declares a `TextToText` kernel, so a non-text argument reaches it through ordinary
scalar coercion — numbers and logicals convert to text under the rules in
[coercion and lifting](../model/02-coercion-and-lifting.md), and an error argument propagates
rather than being encoded. Nothing here is specific to `ENCODEURL`; the shared chapter governs
it.

## Result and edge cases

Text. The published result is subject to the ordinary 32,767-code-unit cell limit described in
[the value universe](../model/01-value-universe.md), and percent-encoding *expands* its input —
up to threefold per byte — so a long input can cross the cap after encoding even though it was
comfortably under it before. That interaction is not something the documentation mentions and
is the most likely practical failure mode.

Two edges the documentation leaves open, and which this page will not guess at:

- **Empty input.** Whether an empty string or an empty referenced cell yields an empty string.
- **Non-BMP characters.** Emoji and other astral characters occupy two UTF-16 code units each
  (see the value-universe chapter's text model). Percent-encoding is defined over *bytes*, so
  the function must be performing a UTF-8 (or other) transcoding step that the documentation
  never names.

## Errors

Microsoft's `ENCODEURL` page documents no error conditions. That is not the same as the
function being infallible — the cell text cap alone gives it a reachable failure mode — but the
Handbook has nothing to cite here.

The platform restriction is stated firmly and belongs in this section as much as in any:
Microsoft documents that `ENCODEURL` is **not available in Excel for the web or Excel for Mac**,
and adds that it "may appear in Excel for Mac's function gallery, but it relies on Windows
operating system features, so it will not return results on Mac". A function that appears in
the gallery and then does not work is a distinct, poorly-signposted state, and it is
particularly striking for `ENCODEURL`, since percent-encoding has no obvious dependency on
Windows at all. The dependency is inherited from the family's implementation, not from the
mathematics.

## Relationships

- **`WEBSERVICE`** — the intended consumer. Microsoft's own examples compose them:
  `=WEBSERVICE("…?symbol="&ENCODEURL(C2))`, and the `WEBSERVICE` page recommends encoding the
  URL before use. The composition is the point of the function.
- **`FILTERXML`** — the third member of the trio, applied to what `WEBSERVICE` returns.
  Microsoft's documented three-function idiom is
  `=FILTERXML(WEBSERVICE("…"&ENCODEURL(C2)), "//path")`.
- **`SUBSTITUTE`** — what workbooks used before `ENCODEURL` existed, chained once per character
  class. Anyone maintaining such a formula is looking at a hand-rolled, usually incomplete,
  version of this function.
- **`HYPERLINK`** — frequently confused with it. `HYPERLINK` builds a clickable link;
  `ENCODEURL` produces text. Encoding a whole URL and passing it to `HYPERLINK` produces a link
  that does not resolve, for the component-encoder reason given above.

## Notes for implementers

- The unreserved set is the whole specification, and the documentation does not give it. RFC
  3986 leaves `A–Z a–z 0–9 - . _ ~` unencoded; older `escape`-style encoders also pass `!`,
  `*`, `'`, `(`, `)`. Which of these Excel passes through is unknown to the Handbook and cannot
  be inferred from the single documented example, whose input contains none of them.
- Percent-encoding operates on bytes, so a text-to-bytes step is required and its encoding
  choice is observable. UTF-8 is the modern assumption; a Windows-API-backed implementation
  could plausibly do something else for non-ASCII input.
- Hexadecimal case (`%2F` versus `%2f`) is a free choice for an encoder and a fixed fact for a
  compatibility implementation. The documented example uses uppercase.
- The output length can exceed the input length by up to a factor of nine for astral characters
  (four UTF-8 bytes at three characters each, from two UTF-16 input units). Any implementation
  that pre-sizes a buffer from input length is wrong.

## What has not been checked

No Handbook vector suite exists for `ENCODEURL`, and no Excel-comparison evidence is recorded.
Nobody has checked this function's output against Excel in the Handbook's record. OxFunc has an
implementation module for it, but the Handbook publishes no measurement of how that
implementation compares with Excel, and this page makes no claim about agreement in either
direction.

`ENCODEURL` is, however, the most *checkable* function in this whole batch: it is pure, has one
argument, needs no server, and its entire uncertainty is a character table. A vector suite
would be cheap and would settle nearly everything. The probes, in order:

1. **The full ASCII sweep.** All 128 ASCII code points, one per vector. This single sweep pins
   the unreserved set, the hexadecimal case, and the space convention, and it is the suite that
   should exist first.
2. **Non-ASCII transcoding.** `é` (U+00E9), `€` (U+20AC), a CJK character, and an emoji
   (astral, two UTF-16 units). This determines the byte encoding and whether surrogate pairs
   are handled as one character or two.
3. **Empty and near-empty input.** Empty string literal, reference to an empty cell, and a
   single space — distinguishing the Empty and Missing outcomes described in the value-universe
   chapter.
4. **Non-text arguments.** A number, a logical, a date serial, and an error value, to confirm
   the coercion path into the `TextToText` kernel.
5. **The length cap.** An input whose encoded form crosses 32,767 code units while the input
   does not, to find out whether the result truncates or errors.
6. **Platform.** The same vectors on Excel for Mac and Excel for the web, to record what
   "appears in the gallery but does not return results" actually produces.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: deferred | OxFunc intentionally defers the function; the recorded reason is "Deferred with the current W041 family despite bounded local work." |
| registry-backed | OxFunc holds a live registry entry with signature and classification axes for this id |
| component encoder | Encodes reserved URL characters too; produces a value to embed in a URL, not a usable URL |
| unreserved set | The characters an encoder leaves untouched |
| `TextToText` | The kernel signature class the projection records for this function |

## Sources

- Microsoft, "ENCODEURL function" —
  <https://support.microsoft.com/en-us/office/encodeurl-function-07c7fb90-7c60-4bff-8687-fac50fe33d0e>
  (syntax, the `text` argument, the worked encoding example quoted above, the composed
  `WEBSERVICE` and `FILTERXML` examples, and the Mac/web availability statement).
- Microsoft, "WEBSERVICE function" —
  <https://support.microsoft.com/en-us/office/webservice-function-0546a35a-ecc6-4739-aed7-c0b7ce1562c4>
  (the recommendation to encode URLs before inclusion in the formula).
- Handbook `data/functions/FUNC.ENCODEURL.json` — admission label and reason, arity 1–1,
  signature `ENCODEURL(text)`, the classification axes quoted above, and the XLL identity
  `xlfEncodeurl` (597).
- Handbook `data/presence/FUNC.ENCODEURL.json` — the implementing module
  `crates/oxfunc_core/src/functions/web_text_xml_family.rs` and its surface-dispatch entry.
- Handbook `content/model/01-value-universe.md` and `content/model/02-coercion-and-lifting.md`
  — the UTF-16 text model with its 32,767-code-unit cap, and the to-text coercion rules.
- IETF RFC 3986, section 2.3 (unreserved characters) — named above as the standard against
  which the unknown character set would be compared, not as a statement of Excel's behaviour.
