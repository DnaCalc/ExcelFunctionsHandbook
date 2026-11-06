---
schema: efh.function-page/v1
function_id: FUNC.HYPERLINK
depth: curated
curated_at_oxfunc_commit: "473efa3"
evidence_records: []
open_problems: []
references: []
episodes: []
body_sections:
  - What it does
  - Try it
  - Quirks
  - How to implement
  - Version differences
  - Alternatives
  - Sources
family: null
role_in_family: null
---

# HYPERLINK

## What it does

`HYPERLINK(link_location,[friendly_name])` creates a clickable destination with a displayed label. If no label is supplied the link location is displayed. A destination can be a web URL or a desktop-supported file/workbook location.

## Try it

`=HYPERLINK("https://handbook.dnacalc.com/","Function handbook")` displays “Function handbook” as the link label. Creating the formula does not mean that the destination has been fetched or that navigation has succeeded.

## Quirks

Excel for the web supports URL destinations only. A nonexistent or inaccessible location can fail when the user clicks; that is different from calculating the formula. An error-valued friendly name displays its error. Desktop file paths, workbook fragments and web URLs need distinct host navigation rules.

## How to implement

Preserve the location and optional label as a host-visible hyperlink result or side channel. Use the location as label when the second argument is omitted, and propagate a supplied label’s worksheet error. A pure-value evaluator can reproduce a label without implementing navigation; report that capability separately. Do not perform an HTTP request simply to evaluate HYPERLINK. Apply the platform’s normal click/navigation behaviour at the host boundary.

## Version differences

No same-input comparison between named Excel builds is recorded for this function here. The documented behaviour below is not a measurement of every Excel release.

## Alternatives

[WEBSERVICE](FUNC.WEBSERVICE.md) retrieves data rather than making a link; [ENCODEURL](FUNC.ENCODEURL.md) encodes text for URL construction.

## Sources

- [Microsoft: HYPERLINK function](https://support.microsoft.com/en-us/excel/functions/hyperlink-function), opened 6 November 2026; syntax, documented argument rules and errors.
