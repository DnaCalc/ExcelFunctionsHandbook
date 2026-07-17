# The value universe

Status: draft (H1) · Sources: OxFunc 937f198

## Why this chapter exists

Before you can say what a worksheet function does, you have to say what it can be given and
what it can hand back. Excel's value model looks small — numbers, text, a couple of booleans —
but the edges are where implementations diverge: the difference between an empty cell and an
omitted argument, the difference between what a function returns and what a cell finally
shows, the exact set of error values, and what happens to a string at 32,767 characters. This
chapter fixes the vocabulary the rest of the Handbook uses for all of that.

Everything here is stated as a model with named parts, because the per-function pages refer to
these names. Where Microsoft's documentation and observed Excel behavior differ, we say so.

## The core value kinds

A worksheet function deals in a small set of value kinds. The first eight form the core
substrate — the value gamut shared by ordinary formulas, the C API used by add-ins, and COM
automation:

| Kind | What it is |
|---|---|
| Number | An IEEE 754 double-precision (binary64) floating-point number. Dates, times, currency, and percentages are all numbers wearing formats. |
| Text | A sequence of UTF-16 code units. See "Text, exactly" below. |
| Logical | TRUE or FALSE. A distinct kind, not a number — though it converts to 1 or 0 on demand. |
| Error | One of a closed set of worksheet error values, such as `#VALUE!`. Errors are first-class scalar values, not exceptions. |
| Empty | The state of a cell that has no content. A value-shaped stand-in for "nothing is here". |
| Missing | The marker for an argument that was omitted at a call site, as in the second slot of `SUM(1,,2)`. Never stored in a cell. |
| Array | A rectangular, row-major grid of values with at least one row and one column. Element values are themselves values from this universe. |
| Reference | A designator for one or more cell ranges — not the cell values themselves, but the address of where they live. |

Two points that trip up implementers:

1. **Empty and Missing are different kinds.** An empty cell exists in the grid and can be
   referenced; a missing argument exists only at a call boundary. Functions can, and do,
   treat them differently. The full story is in "Missing versus Empty" below.
2. **Errors are values.** `#DIV/0!` flows through arithmetic and function calls like any
   other scalar; nothing is "thrown". Propagation discipline is a coercion topic (chapter 02).

### Reference shapes

A reference value can take several shapes, all under the one Reference kind:

| Shape | Example |
|---|---|
| Single cell | `A1` |
| Area | `A1:B10` |
| Multi-area | `(A1:A2,C1:C2)` — a union of areas |
| Three-dimensional | `Sheet1:Sheet3!A1` — the same range across a sheet span |
| Structured | `Table1[Amount]` — a table-relative designator |
| Spill anchor | `B1#` — the dynamic-array result spilled from an anchor cell |

Most functions never see a reference: the engine resolves references to their values before
the function runs. A minority of functions are declared reference-aware (`OFFSET`, `INDIRECT`,
`ROW`, …) and receive or produce reference values as such.

## Rich values: the extension layer

Modern Excel adds values whose meaning exceeds the core substrate: linked data types (stocks,
geography), in-cell images, and callable lambda values. The model treats these as a two-tier
structure rather than as extra core kinds:

1. Every value has a **core projection** — a value from the core kinds above. For an ordinary
   number or text this projection *is* the whole value.
2. A value may additionally carry a **rich payload**: a typed bag of key/value data with a
   declared fallback, a callable handle, a presentation hint, or error metadata.

The core projection is what legacy surfaces see: when a rich value crosses into a context that
only understands the traditional gamut (an old-style formula, a C API call, VBA), the
projection is what arrives. Examples:

- A **linked data type** carries its key/value payload richly; its core projection is the
  fallback display value (typically text).
- A **lambda value** — the result of evaluating `LAMBDA(x, x+1)` without calling it — is a
  callable rich value whose core projection is the `#CALC!` error. That is exactly why
  entering an uncalled lambda in a cell shows `#CALC!`.
- A **presentation hint** is how the model explains functions that return an ordinary scalar
  but influence formatting: `TODAY()` returns a plain serial number plus a date-format hint;
  `HYPERLINK` returns plain text plus a hyperlink-style hint. The value itself stays core.
- **Error metadata** annotates an error value with which surface it can cross (worksheet
  only, or transferable through the add-in boundary).

Rich payload data is recursive: a payload field can hold a scalar, a nested rich value, or a
rich array (a grid whose elements may themselves be rich). Payload data admits scalars and
emptiness but never a Missing marker or a reference.

## The fourteen error codes

The error kind has a closed, versioned registry of codes. Fourteen are modeled:

| Display | Family | Meaning |
|---|---|---|
| `#NULL!` | legacy | The intersection of ranges that do not intersect (the space operator) |
| `#DIV/0!` | legacy | Division by zero, or by an empty cell |
| `#VALUE!` | legacy | A value of the wrong kind, or a failed conversion |
| `#REF!` | legacy | A reference that is no longer valid (deleted rows, out-of-bounds offset) |
| `#NAME?` | legacy | An unrecognized function or defined name |
| `#NUM!` | legacy | A numeric domain or representability failure (overflow, non-convergence) |
| `#N/A` | legacy | Value not available; the conventional "no match" result of lookups |
| `#GETTING_DATA` | extended | Transient placeholder while external data is being retrieved |
| `#BUSY!` | extended | Transient placeholder while a rich value resolves |
| `#SPILL!` | extended | A dynamic-array result could not spill (something is in the way) |
| `#CALC!` | extended | The calculation engine cannot produce a value (uncalled lambda, empty array) |
| `#FIELD!` | extended | A requested field is absent on a rich or linked-data value |
| `#BLOCKED!` | extended | Evaluation blocked by policy or a disabled feature |
| `#CONNECT!` | extended | A required service connection failed |

The **legacy family** (the first seven) is the set that round-trips through every historical
surface: the C API, VBA's error enumeration, and stored files. The **extended family** exists
on the worksheet but does not map one-to-one onto the legacy enumerations, so whether a given
code survives a boundary crossing is a per-code, per-version fact. Family membership is
version-scoped: which codes exist at all depends on the Excel build and the workbook's
compatibility mode. The source model records the legacy seven firmly; the classification of
the transient placeholders (`#BUSY!`, `#GETTING_DATA`) within the extended family is not yet
pinned by evidence.

One modeling decision worth making explicit: `#NULL!` is an error value, nothing more. The
model reserves a "null-like" category name in case first-class null behavior is ever
evidenced, but that category is admitted at no boundary today. There is no SQL-style null in
the worksheet value universe.

## Text, exactly

Worksheet text is a sequence of **UTF-16 code units**, not Unicode characters and not bytes.
The distinction matters at both documented limits and observed edges:

1. **The cap is 32,767 code units.** Microsoft documents 32,767 as the cell character limit.
   Observed behavior pins it more precisely as a count of UTF-16 code units: characters
   outside the Basic Multilingual Plane (emoji, many CJK extensions) occupy two units — a
   surrogate pair — and count double against the cap.
2. **How the cap is enforced depends on the path — and the two paths disagree.** This is an
   observed distinction, not a documented one:
   - Assigning an over-cap string through the automation interface (`Range.Value2`)
     silently **truncates** at 32,767 code units, with no error raised.
   - Producing an over-cap string in a formula (observed with `REPT`) yields **`#VALUE!`**
     instead — the formula path errors where the interop path truncates.
3. **Truncation can split a surrogate pair.** Because interop truncation counts code units,
   an over-cap string of astral characters was observed to end in a dangling high surrogate —
   half a character. Text handling downstream must survive ill-formed UTF-16; it is a real,
   reachable state, not a theoretical one.

Functions that measure text (`LEN`) count code units, which is consistent with this model: an
emoji has `LEN` 2.

## The boundary model

The single most useful idea in this chapter: **which value kinds are legal depends on where
you are standing.** The model names six boundaries and gives each an admission set.

| Boundary | What it is |
|---|---|
| Cell content | What a cell can durably hold in the traditional grid model |
| Raw function return | What a function body (including an add-in function) may hand back, before any normalization |
| Published formula result | What a formula evaluation finally publishes for its cell |
| Call argument | What arrives in a function's parameter slot |
| Reference domain | The domain of reference-shaped values only |
| Extended domain | The engine-internal carrier through which every evaluable value may pass |

The admission matrix, as pinned in the executable model:

| Kind | Cell content | Raw return | Published result | Call argument | Reference domain | Extended domain |
|---|---|---|---|---|---|---|
| Number | yes | yes | yes | yes | no | yes |
| Text | yes | yes | yes | yes | no | yes |
| Logical | yes | yes | yes | yes | no | yes |
| Error | yes | yes | yes | yes | no | yes |
| Empty | yes | yes | **no** | yes | no | yes |
| Missing | no | **no** | no | yes | no | no |
| Array | no | yes | yes | yes | no | yes |
| Reference | no | yes | yes | yes | yes | yes |
| Rich value | no | yes | yes | yes | no | yes |
| Callable | no | yes | yes | yes | no | yes |
| Presentation-hinted | no | yes | yes | yes | no | yes |
| Error with metadata | no | yes | yes | yes | no | yes |
| Null-like (reserved) | no | no | no | no | no | no |

Three rows carry most of the story:

1. **Empty is admitted everywhere except the published result.** A cell can be empty; a raw
   function return can be empty; an argument can deliver emptiness. But a formula never
   *publishes* empty — by the time a result reaches the sheet, emptiness has been normalized
   away (to numeric zero, in the observed cases). This is why `=A1` with `A1` empty shows
   `0`, not an empty cell.
2. **Missing is admitted only at the call boundary.** It is a call-site phenomenon: no cell
   holds it, no function returns it, no formula publishes it.
3. **Raw return is strictly broader than published result.** The gap between them is a real
   normalization step, pinned by observation: an add-in function can return the C API's "nil"
   (empty) token, and that raw scalar does not survive to the sheet — it normalizes to
   numeric-zero semantics before outer argument binding and publication. Inside an array
   returned by an add-in, empty elements were observed to persist through intermediate
   evaluation, collapsing to zero only when scalarized or published into cells. None of this
   is in Microsoft's documentation; it is empirically pinned by add-in probes recorded in the
   source model.

The boundary a per-function page names when it says "returns an array" or "accepts a
reference" is always one of these six.

## Missing versus Empty

The distinction deserves its own section because both look like "nothing" and behave
differently.

**Missing** arises only from a call site: an argument slot that the formula text left
unfilled, either by omission at the end (`ROUND(1.5)` has no second argument… which is
rejected — see below) or by an empty slot between commas (`SUM(1,,2)`). Whether omission is
legal at all is part of each function's signature: omitting a *required* argument is rejected
when the formula is entered — observed as an entry-time refusal, not a runtime error value.
Omitting an *optional* argument delivers the Missing marker to the function, which then
applies its documented default.

**Empty** arises from the grid: a referenced cell that has no content. `SUM(A1)` with `A1`
never filled delivers emptiness, not a missing argument. Functions commonly treat the two
differently — an aggregate skips empty cells in a scanned range but applies a default (or
zero) for an omitted argument — and the model keeps the two kinds distinct at the call
boundary precisely so per-function pages can state each behavior separately.

The pair `(Missing, Empty)` is also asymmetric in reach, as the admission matrix shows: Empty
exists at four boundaries; Missing at exactly one.

## Page vocabulary

Machine names that per-function pages may display, mapped to plain language:

| Machine name | Meaning |
|---|---|
| `Number` | IEEE 754 double-precision number |
| `Text` | UTF-16 code-unit string, capped at 32,767 units |
| `Logical` | TRUE or FALSE |
| `Error` | One of the fourteen worksheet error codes |
| `Empty` | The no-content state of a cell |
| `Missing` | The omitted-argument marker; call boundary only |
| `Array` | Rectangular grid of values |
| `ReferenceLike` | A cell-range designator (any reference shape) |
| `RichValue` | Extension-layer value with typed key/value payload |
| `Callable` | A lambda value; its core projection is `#CALC!` |
| `Presentation` | A core value carrying a formatting or style hint |
| `ErrorMetadata` | An error value annotated with boundary-crossing metadata |
| `NullLike` | Reserved category; admitted nowhere |
| `CellContent` | Boundary: what a cell durably holds |
| `RawFunctionReturn` | Boundary: what a function body hands back, pre-normalization |
| `PublishedFormulaResult` | Boundary: what a formula publishes to its cell |
| `CallArg` | Boundary: what arrives in a parameter slot |
| `ReferenceDomain` | Boundary: reference-shaped values only |
| `ExtendedDomain` | Boundary: the engine-internal value carrier |
| `A1` / `Area` / `MultiArea` / `ThreeD` / `Structured` / `SpillAnchor` | Reference shapes |

## Sources

- `docs/function-lane/VALUE_UNIVERSE_PRELIM_SPEC.md` — the boundary-scoped value model:
  tag algebra, admission policy, text cap, raw-versus-published rule. Mixed basis: documented
  anchors plus empirically pinned probe results (text truncation and surrogate tail; add-in
  nil-return normalization), each marked as such in the source.
- `docs/function-lane/VALUE_UNIVERSE_RESEARCH_AND_OPEN_QUESTIONS.md` — evidence anchors
  (Microsoft C API and Support documentation) and the open-question register; documented
  basis with provisional interpretations flagged in the source.
- `crates/oxfunc_value_types/src/lib.rs` — the executable model: value kinds, the fourteen
  error codes, reference shapes, rich-value structure, text type with cap and surrogate
  checks, and the boundary admission matrix with tests. This is the pinned form of every
  table in this chapter.
