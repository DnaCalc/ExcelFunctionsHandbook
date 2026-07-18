# About implementation options

Status: draft (H1) · Sources: Handbook CHARTER

## One function, four answers

Ask four careful engineers to implement GAMMALN and you can get four defensibly different
functions. They will disagree in the last bits — and each can be the right choice, depending on
what you need. The Handbook makes this explicit instead of pretending there is one true
implementation. Every function page uses the same four-flavour framework, while showing only
implementations that actually exist and have passed their stated admission gate:

### Excel compatibility (`excel-bitexact`)

Targets Excel's observed behavior, including quirks, approximations, and historical accidents.
This is the flavour you want for **compatibility and audit**: recomputing a workbook, validating a
migration, or proving that a discrepancy is real. The flavour names the target; its warrant states
what has actually been demonstrated. A finite corpus earns `suite-exact, vN`, not a universal
claim. Stronger bit-exact wording names its declared input domain and complete observation context.

### Natural best

The balanced, state-of-the-art implementation of the *intended* mathematical function — accurate
to within a small, stated bound, fast, and robust across the domain. Often faster than the
maximally-correct flavour and dramatically better than legacy algorithms. This is the
**recommended default for new code** that wants "the function Excel means" rather than "the bits
Excel produces".

### Portable reproducible

Bit-identical results on every CPU, compiler, and platform. Sacrifices a little speed (and
sometimes a little accuracy) for **determinism across environments** — what you want in
distributed systems, consensus-critical computation, and regulated pipelines where two machines
must never disagree.

### Mathematically correct

The correctly rounded (or best-feasible) reference: the true mathematical value of the function,
rounded once to the nearest representable double. This is the **yardstick** — the Handbook's
residual plates measure Excel and every other flavour against it. It is usually the slowest, and
that is fine; its job is to be right.

## How implementations get here

1. An implementation is admitted only after passing its function's **versioned test suite** —
   the downloadable vector suites published on each function page. The badge on the page names
   the suite version it passed.
2. Function pages show **only implementations that exist and have passed**. Absence of a
   flavour or language is stated by omission, not by promises.
3. New implementations — ports to other languages, better algorithms — can be built on request
   or contributed with evidence. A contributed implementation enters the record as a proposed
   claim, is verified against the suite, decided, and credited.
4. The suites are the portability mechanism: because they capture Excel's answers as exact bit
   patterns with provenance, an implementation can be verified anywhere — including platforms
   where Excel does not run.

## Languages

The anchor implementations are in Rust (they are the same kernels that power the DNA Calc
engine's Excel-compatibility work). The Handbook's target languages beyond Rust are Python, C#,
and TypeScript; a small number of exemplar functions carry the full flavour × language spread to
demonstrate the shape. Coverage grows function by function, verified suite-first — never
announced ahead of existence.

## Choosing a flavour

| You need | Take |
|---|---|
| Compatibility with a specific Excel | Excel compatibility (read its warrant and observation scope) |
| The best available answer for new code | Natural best |
| Two machines that can never disagree | Portable reproducible |
| A reference to measure against | Mathematically correct |

If you only need values "close enough for a spreadsheet", any flavour will serve — but then you
will also want to read the function's residual plate, which shows precisely how far apart these
flavours actually are.

## Page vocabulary

| Label | Meaning |
|---|---|
| excel-bitexact | Implementation flavour targeting Excel compatibility; the adjacent warrant states what is demonstrated |
| suite-exact, vN | Matched the named Excel oracle for every vector in suite vN; inputs outside the suite are not claimed |
| characterized bit-exact | Bitwise claim over a declared domain with named mechanism/context evidence |
| natural-best | Balanced state-of-the-art implementation of the intended function |
| portable-reproducible | Bit-identical across CPUs, compilers, and platforms |
| math-correct | Correctly rounded (or best-feasible) reference implementation |
| verification passed, suite vN | Passed every vector of that suite version; not by itself a universal claim |

## Sources

- Handbook `CHARTER.md` section 4 (scope: the four flavours) and section 7 (claim rules).
- The Excel bit-exact anchors originate in the OxFunc project (the DNA Calc function-semantics
  implementation, verified against live Excel oracles); other flavours are Handbook-owned
  engineering.
