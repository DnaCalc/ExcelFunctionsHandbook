# efh-calcfunc

Regenerates `data/calcfunc/` and the signature, arity and admission fields of `data/functions/` from
one pinned commit of the public repository [DnaCalc/DnaCalc](https://github.com/DnaCalc/DnaCalc).
CalcFunc has no repository of its own: it is the crate `crates/calc-func` there.

```sh
git -C DNACALC checkout PIN
(cd DNACALC && cargo xtask corpus --oracle --no-capture --cache CACHE --out RUN)
python3 tools/efh-calcfunc/regenerate.py --dnacalc DNACALC --pin PIN --corpus-run RUN
cargo run --release --locked --manifest-path tools/efh-site/Cargo.toml -- build
```

`tools/build-site.sh` runs the last two steps with the committed pin and tally. Omit `--pin` and
`--corpus-run` to regenerate at the committed pin. The script refuses a checkout that is not clean
at the pin, and stops if the corpus tally, the bank table and `problem-functions.json` disagree.

| Input at the pin | Gives |
|---|---|
| `crates/calc-func` catalogue, through `dump/` | signature, arity, parameter classes, and each entry's kind, implementation (held, binding, not held, deferred, with owner) and availability: these decide the deferred, host and Not in Excel states (an entry whose availability is `AbsentOnWindows`, such as JIS, is counted on its own as `absent`, apart from the entries that refuse) |
| `crates/calc-fml/src/eval/builtins.rs` | its hash only: the names it lists are built from the catalogue |
| `crates/calc-func/witness-status.tsv` | answer-bank rows, matches and worst ULP per function |
| `docs/generated/problem-functions.json` | mismatch classes per function, and the rule |
| corpus run (`tally.py`) | workbook cells per function, the denominator the report omits; and per operator (`operators`), read from the parsed formula text, so prefix, postfix and reference operators get their own counts. A matched cell counts for every operator it uses; an unmatched cell is charged to the operators only when the formula calls no function |
| `crates/calc-func/witness-differences.tsv` | conflicting Excel readings |
| source text (`rustindex.py`) | quoted entry, its kernels, shared kernels |

`non-catalogue.json` is the one hand-held input: signatures read from named Microsoft pages for
entries the catalogue does not describe, and the policy group of each. A catalogue row at the pin
always overrides it, so a function that lands changes state without an edit.

The eight implementation states are matches, close, differs, refuses, deferred, host, unmeasured and absent ("Not in Excel"). Host-route tests and traces are cited separately from the answer bank and workbook counts; the import fixture supplies separate case totals, while add-in callbacks and real-time protocol observations retain their own scope. The workbook tally combines the plain and `XLFN.` labels for each function. Source extraction follows the shared catalogue's stable family and member IDs to the attached kernels. When `problem-functions.json` lists bank failures for a function whose `witness-status.tsv` row matches every row, the status table wins and the script prints a note.
