#!/usr/bin/env python3
"""Derives the pinned `x87-scope.txt` — the set of Handbook function ids whose owning OxFunc
implementation module reaches OxFunc's hardware-x87 backend.

This is a MECHANICAL over-approximation at module granularity. It does not prove that an x87
instruction executed for any particular input; it proves that the module owning the function's
`FunctionMeta` can reach `crate::excel_numeric::x87`. `BATTERY.md` §5 states that limitation in the
same words. Nothing here is hand-curated: every set below is computed from OxFunc source text.

Usage (OxFunc is read-only; nothing is written there):

    efh-battery <handbook> <oxfunc> catalog > catalog.tsv
    python derive_x87_scope.py <oxfunc> catalog.tsv > x87-scope.txt

Steps:
  A. In `crates/oxfunc_core/src/excel_numeric/`, a function is x87-backed if its own body mentions
     `x87::`; the set is closed transitively over calls between functions in that directory.
  B. A kernel module under `crates/oxfunc_core/src/functions/` is SEED-tainted if its text names any
     step-A function through an `excel_numeric::` path (or `excel_numeric::x87` directly).
  C. The seed set is closed transitively over `<module>::` references between kernel modules.
     Modules whose basename starts with `surface_dispatch` are excluded from the graph entirely:
     the dispatcher names every module in the crate, so it is a universal edge and carries no
     routing information.
  D. Each catalog entry's owning module is read EXACTLY from the `use crate::functions::{...}`
     import block of `xll_export_specs.rs`, which binds every `*_META` constant to the module that
     declares it; `FUNCTION_CATALOG`'s literal order gives catalog index -> `*_META`, and
     `catalog.tsv` gives catalog index -> function id.
"""

import json
import os
import re
import sys


def x87_backed_functions(oxfunc_src):
    en = os.path.join(oxfunc_src, "excel_numeric")
    bodies = {}
    for name in sorted(os.listdir(en)):
        if not name.endswith(".rs"):
            continue
        txt = open(os.path.join(en, name), encoding="utf-8").read()
        parts = re.split(
            r"\n(?=\s*(?:#\[[^\]]*\]\s*\n\s*)*(?:pub(?:\([^)]*\))?\s+)?fn\s)", txt
        )
        for part in parts:
            m = re.search(r"\bfn\s+([A-Za-z0-9_]+)", part)
            if m:
                bodies[m.group(1)] = bodies.get(m.group(1), "") + part
    reached = set(n for n, b in bodies.items() if re.search(r"\bx87\s*::", b))
    changed = True
    while changed:
        changed = False
        for n, b in bodies.items():
            if n in reached:
                continue
            for t in sorted(reached):
                if re.search(r"\b" + re.escape(t) + r"\s*\(", b):
                    reached.add(n)
                    changed = True
                    break
    return reached


def kernel_modules(oxfunc_src):
    fn_dir = os.path.join(oxfunc_src, "functions")
    mods = {}
    for name in sorted(os.listdir(fn_dir)):
        if not name.endswith(".rs"):
            continue
        base = name[:-3]
        if base.startswith("surface_dispatch") or base == "mod":
            continue
        mods[base] = open(os.path.join(fn_dir, name), encoding="utf-8").read()
    return mods


def tainted_modules(mods, x87_fns):
    def direct(txt):
        if re.search(r"excel_numeric\s*::\s*x87", txt):
            return True
        for grp in re.findall(r"excel_numeric\s*::\s*\{([^}]*)\}", txt):
            for tok in re.split(r"[,\s:]+", grp):
                if tok in x87_fns:
                    return True
        for n in x87_fns:
            if re.search(r"excel_numeric\s*::\s*" + re.escape(n) + r"\b", txt):
                return True
        return False

    seed = set(b for b, t in mods.items() if direct(t))
    tainted = set(seed)
    changed = True
    while changed:
        changed = False
        for b, t in mods.items():
            if b in tainted:
                continue
            for s in sorted(tainted):
                if re.search(r"\b" + re.escape(s) + r"\s*::", t):
                    tainted.add(b)
                    changed = True
                    break
    return seed, tainted


def catalog_meta_modules(oxfunc_src):
    spec = open(os.path.join(oxfunc_src, "xll_export_specs.rs"), encoding="utf-8").read()
    i = spec.index("{", spec.index("use crate::functions::{"))
    j, depth = i, 0
    while True:
        if spec[j] == "{":
            depth += 1
        elif spec[j] == "}":
            depth -= 1
            if depth == 0:
                break
        j += 1
    meta2mod = {}
    for m in re.finditer(r"([a-z0-9_]+)\s*::\s*(?:\{([^}]*)\}|([A-Z0-9_]+))", spec[i + 1 : j]):
        names = [m.group(3)] if m.group(3) else re.split(r"[,\s]+", m.group(2).strip())
        for n in names:
            n = n.strip()
            if n:
                meta2mod[n] = m.group(1)
    cat = spec[spec.index("const FUNCTION_CATALOG") :]
    cat = cat[cat.index("[") : cat.index("\n];")]
    order = [x.strip().rstrip(",") for x in cat.splitlines()]
    order = [x for x in order if x.endswith("_META")]
    return order, meta2mod


def main():
    oxfunc = sys.argv[1]
    catalog_tsv = sys.argv[2]
    src = os.path.join(oxfunc, "crates", "oxfunc_core", "src")

    x87_fns = x87_backed_functions(src)
    mods = kernel_modules(src)
    seed, tainted = tainted_modules(mods, x87_fns)
    order, meta2mod = catalog_meta_modules(src)

    index_to_id = {}
    for line in open(catalog_tsv, encoding="utf-8"):
        line = line.strip()
        if not line:
            continue
        idx, fid = line.split("\t")
        index_to_id[int(idx)] = fid
    if len(index_to_id) != len(order):
        raise SystemExit(
            "catalog.tsv has %d rows but FUNCTION_CATALOG has %d entries"
            % (len(index_to_id), len(order))
        )

    scoped, seeded = [], []
    for idx, meta in enumerate(order):
        mod = meta2mod.get(meta)
        if mod in tainted:
            scoped.append(index_to_id[idx])
            if mod in seed:
                seeded.append(index_to_id[idx])

    out = sys.stdout
    out.write("# Pinned x87 scope for battery EFH-B1. Generated by derive_x87_scope.py.\n")
    out.write("# Module-granularity reachability over OxFunc source. NOT a proof that an x87\n")
    out.write("# instruction executed for any particular input. See BATTERY.md section 5.\n")
    out.write("# x87-backed excel_numeric functions: %d\n" % len(x87_fns))
    out.write("# seed kernel modules (own text names one): %d\n" % len(seed))
    out.write("# transitively reaching kernel modules: %d\n" % len(tainted))
    out.write("# catalog entries in a seed module: %d\n" % len(seeded))
    out.write("# catalog entries in scope (seed + transitive): %d\n" % len(scoped))
    out.write("# seed modules: %s\n" % " ".join(sorted(seed)))
    out.write("# transitive-only modules: %s\n" % " ".join(sorted(tainted - seed)))
    for fid in sorted(scoped):
        out.write(fid + "\n")

    sys.stderr.write(
        json.dumps(
            {
                "x87_backed_excel_numeric_functions": len(x87_fns),
                "seed_modules": sorted(seed),
                "transitive_only_modules": sorted(tainted - seed),
                "entries_in_seed_module": len(seeded),
                "entries_in_scope": len(scoped),
            },
            indent=1,
        )
        + "\n"
    )


if __name__ == "__main__":
    main()
