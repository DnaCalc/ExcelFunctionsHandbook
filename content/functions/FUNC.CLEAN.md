---
schema: efh.function-page/v1
function_id: FUNC.CLEAN
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
family: clean_fn
role_in_family: "The sole member: a filter that deletes a fixed set of nonprinting characters from text."
---

## What it computes

`CLEAN` returns its argument with every character in a fixed set deleted. It is a filter, not a
replacer: removed characters leave nothing behind, so the result is never longer than the input
and carries no substitute spaces.

The set is where the whole content of this function sits, and Microsoft's article is unusually
explicit about its history. `CLEAN` was designed to remove "the first 32 nonprinting characters
in the 7-bit ASCII code (values 0 through 31)". The article then states the limitation in its
own voice: in the Unicode character set there are additional nonprinting characters — it names
values 127, 129, 141, 143, 144 and 157 — and `CLEAN` does not, by itself, remove them.

Read carefully, that documentation describes the function as *codes 0–31 only*, and lists 127,
129, 141, 143, 144, 157 as characters it leaves in place. That is a strong, checkable
statement, and it is exactly the statement the Handbook would most like to see tested, because
the OxFunc reference engine implements a different set: it removes 0–31 **and** 129, 141, 143,
144 and 157, while keeping
127. Whether Excel matches the documentation, matches OxFunc, or matches neither is unresolved
     here — see "What has not been checked".

Filtering is defined over UTF-16 code units (see
[the value universe](../model/01-value-universe.md)), which matters only in that a surrogate
pair can never be hit by a set of values below 158, so astral characters survive `CLEAN`
intact.

## Arguments

`text` — required. Microsoft's article describes it as any worksheet information from which you
want to remove nonprintable characters. It is a text argument, so a number or logical converts
to text first and comes back as its rendering: `CLEAN` of a number is text, not a number.

There is no second argument. The removal set is not configurable, which is why the documented
workaround for the remaining Unicode nonprinting characters is to compose `SUBSTITUTE` calls
around `CLEAN` rather than to parameterise it.

## Result and edge cases

The return kind is `Text`. Notable boundaries:

- **Empty input** yields empty text — there is nothing to filter and nothing to add.
- **All-nonprinting input** yields empty text, not an error. This is the case worth
  remembering, because a cell holding only control characters is indistinguishable after
  `CLEAN` from an empty cell in every downstream test except `ISBLANK`.
- **Whitespace is not touched.** Space (32) is outside the removal range; tab (9) and the
  line-break characters (10, 13) are inside it. `CLEAN` therefore joins lines *without*
  inserting a separator — `"a" & CHAR(10) & "b"` cleans to `ab`, not `a b`. This is the single
  most common surprise, and it is the reason `CLEAN` and `TRIM` are almost always used
  together.
- Arrays, empty cells and omitted arguments follow the shared call model
  ([chapter 03](../model/03-call-pipeline.md)).

## Errors

The article does not enumerate error conditions for `CLEAN`, and none of the ordinary text
inputs suggests one: the function has no domain to violate. The reachable errors are the shared
ones — an error value arriving as the argument propagates
([chapter 02](../model/02-coercion-and-lifting.md)), and a value kind with no text conversion
surfaces `#VALUE!` under the shared coercion rules rather than under any rule of this
function's own.

## Relationships

- **`TRIM`** is the whitespace sibling: it removes leading and trailing spaces and collapses
  runs of interior spaces. `CLEAN` and `TRIM` partition the usual import-cleanup job between
  them, and neither does the other's work.
- **`SUBSTITUTE`** is the escape hatch for the characters `CLEAN` documents itself as leaving
  behind (127, 129, 141, 143, 144, 157), and for the non-breaking space (160), which is not in
  `CLEAN`'s set and is not a space to `TRIM` either. Import-cleanup formulas in the wild are
  usually `TRIM(CLEAN(SUBSTITUTE(x, CHAR(160), " ")))` for exactly this reason.
- **`ASC`** and **`DBCS`** are the other width- and encoding-normalising text functions; they
  change characters rather than delete them.

## Notes for implementers

1. **The removal set is the specification.** Write it as data, not as a range test, so it can
   be diffed against evidence when evidence exists. The two candidate sets differ only at six
   values, and a range test hides that.
2. **Filter, do not replace.** Indices into the input are invalidated by `CLEAN`, so any code
   that cleans a string and then applies a previously computed `FIND` position is wrong.
3. **Operate on code units, not on decoded characters.** Decoding first is harmless for
   correctness here but forces a decision about ill-formed UTF-16, which the value-universe
   chapter records as a reachable state in Excel text.
4. The reference engine's set at commit `473efa3` is: every unit below 32, plus 129, 141, 143,
   144 and 157. It keeps 127 and keeps the zero-width space (8203). Implementation fact about
   OxFunc only.

## What has not been checked

No Handbook vector suite exists for `CLEAN`, and no Excel-comparison evidence record takes
`CLEAN` as a subject. `FUNC.CLEAN` does appear in the group member list of the structural sweep
recorded as `EV-STRUCT-0009`, but that record's subjects are `FUNC.ARABIC` and `FUNC.DECIMAL`,
and the record states in its own reader warning that its figures are properties of the run and
not of any single surface. So: nothing about `CLEAN` has been measured against Excel here.

The probes that would settle this page, in order:

- `CLEAN(CHAR(127))`, and `CLEAN` of each of 129, 141, 143, 144, 157 individually. Six formulas
  decide between the documented set and the reference engine's set.
- `CLEAN(CHAR(160))` — the non-breaking space, to confirm it is untouched.
- `CLEAN` applied to a string containing an astral character, to confirm surrogates survive.
- `CLEAN` of a number and of a logical, to pin the to-text conversion at this call site.
- The same six-formula probe on Excel for the web and on Macintosh, since the documentation
  ties character interpretation to the host elsewhere in the text family.

## Page vocabulary

| Term | Meaning as used here |
|---|---|
| removal set | The fixed set of character codes `CLEAN` deletes |
| nonprinting character | A character with no glyph; here, specifically the codes named in the source |
| filter | Deletion with no substitute character inserted |

## Sources

- Microsoft, "CLEAN function" —
  <https://support.microsoft.com/en-us/office/clean-function-26f3d7c5-475f-4a9c-90e5-4b8ba987ba41>
  (the 0–31 design statement; the named Unicode values 127, 129, 141, 143, 144, 157 that
  `CLEAN` does not remove). Retrieved for this page.
- Handbook `content/model/01-value-universe.md` (UTF-16 code units, ill-formed text as a
  reachable state) and `content/model/02-coercion-and-lifting.md` (to-text conversion, error
  propagation).
- Handbook `content/evidence/records/EV-STRUCT-0009.json` — cited only to state that `CLEAN`
  appears in that run's group member list and is not one of its subjects.
- OxFunc `crates/oxfunc_core/src/functions/clean_fn.rs` at commit `473efa3` — read for the
  reference engine's removal set. Implementation fact only.
