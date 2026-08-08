---
schema: efh.function-page/v1
function_id: FUNC.WEBSERVICE
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
family: null
role_in_family: null
---

`WEBSERVICE` performs an HTTP GET from a formula and puts the response body in a cell. It is
the only worksheet function in this batch whose defining characteristic is a side effect on the
network, and that is why the reference engine (OxFunc) records it as **deferred**, with the
reason **"Deferred external/provider function."** Unlike its two Web-function siblings there is
no implementation module and no live registry entry behind the name — the projection is
catalog-only. A reference implementation could reproduce the URL validation, but not the
answer, because the answer belongs to someone else's server.

## What it computes

`WEBSERVICE(url)` issues a **GET** request for `url` and returns the response body as text.

Four properties follow from the documentation and matter more than the one-line description:

1. **GET only.** Microsoft's own error list constrains `url` to "the 2048 characters that are
   allowed for a GET request". There is no POST form, no header control, no body, no
   authentication argument. Anything an API needs beyond a URL cannot be expressed.
2. **HTTP(S) only.** Microsoft documents `#VALUE!` "for protocols that aren't supported, such
   as `ftp://` or `file://`". `WEBSERVICE` is not a general fetcher.
3. **The response is text, and unstructured.** No parsing, no content-type handling. What comes
   back is a string; making sense of it is `FILTERXML`'s job, or manual text surgery.
4. **The cell text cap is a hard ceiling on the response.** Microsoft documents `#VALUE!` when
   the arguments "result in a string that is not valid or that contains more than the allowable
   cell limit of 32767 characters". A response larger than one cell can hold is an error, not a
   truncation. Most real API responses exceed it.

Together these make `WEBSERVICE` a narrow instrument: small, public, GET-addressable,
text-returning endpoints, on Windows Excel, with the URL fully specified up front.

## Arguments

`WEBSERVICE(url)`

- **`url`** (required) — Microsoft: "The URL of the web service to be called".

The argument's admissible values are bounded by three documented limits at once: at most 2,048
characters (the GET limit), a supported protocol, and a result that fits in a cell. The
misunderstanding worth naming is that `url` is not a template — there is no separate parameter
list, so query parameters are built by concatenation, and any value interpolated into the URL
must be percent-encoded first. Microsoft says so directly: "For the best results, the URL for
WEBSERVICE should be encoded prior to inclusion in the formula", pointing at `ENCODEURL`.

The reference engine's projection records no arity for this id, because there is no live
registry entry — arity here comes from the documented signature only.

## Result and edge cases

Text, or an error. Microsoft's page documents no other return kind.

The interesting edges are the ones that follow from a formula reaching the network, and the
documentation addresses almost none of them:

- **When does it fire?** A formula that makes an HTTP request has to decide when to re-issue
  it. Whether `WEBSERVICE` is volatile, what triggers a re-fetch, and whether the result
  survives save/reopen without a recalculation are not stated on the function's page. This is
  the most consequential undocumented behaviour on this page, because it determines whether
  opening a workbook silently contacts a third-party server.
- **Trust and blocking.** External-data prompts, Protected View, and Trust Center policy all
  sit between the formula and the request. The value-universe chapter records `#BLOCKED!` and
  `#CONNECT!` as extended error codes for exactly this territory; whether `WEBSERVICE` produces
  either of them is not documented on its page and the Handbook does not know.
- **Non-200 responses.** Whether a 404 body is returned as text or collapses to `#VALUE!` is
  not stated.
- **Encoding.** How response bytes become UTF-16 text — declared charset, HTTP header, or
  assumption — is not stated.

## Errors

Microsoft documents `#VALUE!` for all four of the following:

| Condition | As documented |
|---|---|
| the arguments cannot return the data | "If arguments are unable to return the data" |
| result too large or invalid | more than "the allowable cell limit of 32767 characters" |
| URL too long | more than "the 2048 characters that are allowed for a GET request" |
| unsupported protocol | "such as `ftp://` or `file://`" |

Everything collapses onto one error code. From `#VALUE!` alone a workbook cannot distinguish a
typo, an oversized response, a dead host, and a blocked protocol — which makes `WEBSERVICE`
unusually hard to debug from the grid, and makes any diagnostic wrapper around it a guess.

Availability is documented as: Excel for Microsoft 365, Excel 2024, 2021, 2019 and 2016, with
Mac excluded — "The WEBSERVICE function may appear in the Excel for Mac function gallery, but
it relies on Windows operating system features, so it will not return results on Mac."

## Relationships

- **`ENCODEURL`** — the documented preprocessor for any value interpolated into `url`.
- **`FILTERXML`** — the documented postprocessor for an XML response. The three functions are
  designed as a pipeline and Microsoft's examples always show them that way.
- **Power Query / Get & Transform** — the supported modern path for pulling data from the web
  into a workbook, with headers, authentication, paging, refresh control, and no 32,767-character
  ceiling. For anything beyond a toy endpoint this is the honest recommendation.
- **`HYPERLINK`** — confused with `WEBSERVICE` by name. `HYPERLINK` never fetches anything; it
  produces a clickable link.
- **`STOCKHISTORY`** and the linked data types — the other route by which modern Excel reaches
  external data, but through a Microsoft-operated provider rather than an arbitrary URL.

## Notes for implementers

- This function is a security boundary, not just an I/O call. A worksheet formula that issues
  an outbound request can be used to exfiltrate cell contents in a query string, and the
  trigger is opening a file. Any implementation must decide deliberately about consent,
  caching, and whether requests fire on load; "match Excel" is not a sufficient specification
  because Excel's own answer here is governed by host policy rather than function semantics.
- Recalculation semantics are the specification. A pure-function model does not fit: the same
  argument may legitimately produce different results at different times, so the function's
  determinism class is a host contract rather than a kernel property.
- The 2,048-character limit is a documented Excel behaviour, not an HTTP rule, and must be
  enforced as such.
- Response decoding needs an explicit, stated policy. Guessing a charset produces mojibake that
  is indistinguishable, from the grid, from a server bug.

## What has not been checked

No Handbook vector suite exists for `WEBSERVICE`, and no Excel-comparison evidence is recorded.
Nobody has checked this function against Excel in the Handbook's record. Every behavioural
statement above is either Microsoft's documented behaviour, cited as such, or explicitly named
as unknown.

A vector suite in the ordinary sense is not achievable for `WEBSERVICE`: its result is not a
function of its argument. What *is* achievable, and what the Handbook would need, is a
characterization run against a controlled local endpoint that can be made to return exactly
chosen bytes and status codes. With such an endpoint, the probes worth running:

1. **Recalculation and trigger behaviour.** Instrument the endpoint and record when requests
   actually arrive: on entry, on `F9`, on full rebuild, on file open, on save. This is the
   single most important unknown, and it is a security-relevant one.
2. **The size boundary.** Responses of exactly 32,767 and 32,768 code units — error or
   truncation, and does an astral character straddling the cap behave as the value-universe
   chapter's surrogate-splitting observation would suggest?
3. **The URL-length boundary.** URLs of exactly 2,048 and 2,049 characters.
4. **Status codes.** 200, 204, 301 with a redirect, 404, and 500, each with a body — recording
   which yield text and which yield `#VALUE!`.
5. **Content types and charsets.** UTF-8 with and without a BOM, UTF-16, ISO-8859-1, and a
   declared-but-wrong charset.
6. **Protocol rejection.** `http`, `https`, `ftp`, `file`, and a scheme-less string, confirming
   which are refused and at what stage.
7. **Policy interaction.** The same formula in a workbook opened from a trusted location, from
   the internet zone, and in Protected View, to find out whether `#BLOCKED!` or `#CONNECT!` are
   reachable from this function at all.
8. **Platform.** Excel for Mac and Excel for the web, to record what the documented
   non-availability produces.

## Page vocabulary

| Term | Meaning on this page |
|---|---|
| reference engine: deferred | OxFunc intentionally defers the function; the recorded reason is "Deferred external/provider function." |
| catalog-only | The projection carries identity and documentation metadata but no implementation module or live registry entry |
| GET | The only HTTP method the function can express |
| response body | The text returned by the server; not parsed by this function |
| `#BLOCKED!` / `#CONNECT!` | Extended-family error codes for policy-blocked evaluation and failed service connections |

## Sources

- Microsoft, "WEBSERVICE function" —
  <https://support.microsoft.com/en-us/office/webservice-function-0546a35a-ecc6-4739-aed7-c0b7ce1562c4>
  (syntax, the `url` argument, all four documented `#VALUE!` conditions with their exact
  wording, the encode-first recommendation, the version list, and the Mac availability
  statement).
- Microsoft, "ENCODEURL function" —
  <https://support.microsoft.com/en-us/office/encodeurl-function-07c7fb90-7c60-4bff-8687-fac50fe33d0e>
  (the composed `WEBSERVICE`/`FILTERXML` examples).
- Handbook `data/functions/FUNC.WEBSERVICE.json` — admission label and reason, category, the
  XLL identity `xlfWebservice` (596), and the catalog-only metadata status.
- Handbook `data/presence/FUNC.WEBSERVICE.json` — no implementation module, no dispatch entry.
- Handbook `content/model/01-value-universe.md` — the extended error-code registry including
  `#BLOCKED!` and `#CONNECT!`, and the text cap with its observed surrogate-splitting
  behaviour.
