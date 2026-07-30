#!/usr/bin/env python3
"""Acceptance tests for T7 (FOUNDATION section 6, row T7). Reads only; fails loudly.

    python verify_battery.py <handbook-root>

Every assertion below is a count over the emitted files. Nothing is sampled and nothing is
estimated: a check either passes for all 541 files or the script prints the failing ids.
"""

import json
import os
import re
import struct
import sys

FIXED_LABELS = [
    "zero",
    "negative-one",
    "empty-string",
    "boolean-true",
    "empty-range",
    "error-na",
    "max-double",
    "min-subnormal",
    "inline-array",
    "text-numeral",
    "too-few-args",
    "too-many-args",
]

TOP_KEYS = [
    "schema",
    "function_id",
    "surface_name",
    "battery_id",
    "oxfunc_commit",
    "oxfunc_tree_clean",
    "runner_version",
    "host",
    "rows",
    "label",
]
ROW_KEYS = [
    "label",
    "input_display",
    "outcome_kind",
    "outcome_display",
    "outcome_bits",
    "host_scoped",
]
KINDS = {
    "number",
    "text",
    "boolean",
    "error",
    "array",
    "refused-by-arity",
    "not-dispatchable",
}
LABEL = "OxFunc's own answers. No Excel was involved."


def main():
    root = sys.argv[1] if len(sys.argv) > 1 else "C:/Work/DnaCalc/ExcelFunctionsHandbook"
    bdir = os.path.join(root, "data", "battery")
    fdir = os.path.join(root, "data", "functions")
    scope_path = os.path.join(root, "tools", "efh-battery", "x87-scope.txt")
    scope = set(
        l.strip()
        for l in open(scope_path, encoding="utf-8")
        if l.strip() and not l.startswith("#")
    )

    expected_ids = sorted(f[:-5] for f in os.listdir(fdir) if f.endswith(".json"))
    got_ids = sorted(f[:-5] for f in os.listdir(bdir) if f.endswith(".json"))

    fails = []

    def check(cond, msg):
        if not cond:
            fails.append(msg)

    check(len(got_ids) == 541, "T7(a) file count is %d, expected 541" % len(got_ids))
    check(got_ids == expected_ids, "T7(a) battery ids differ from data/functions ids")

    rows_total = 0
    nonfinite = []
    numbers = 0
    host_rows = 0
    host_entries = set()
    kind_counts = {}
    for fid in got_ids:
        path = os.path.join(bdir, fid + ".json")
        raw = open(path, "rb").read()
        check(not raw.startswith(b"\xef\xbb\xbf"), "%s has a BOM" % fid)
        check(raw.endswith(b"\n"), "%s has no trailing newline" % fid)
        check(b"\r\n" not in raw, "%s has CRLF line endings" % fid)
        doc = json.loads(raw.decode("utf-8"), object_pairs_hook=lambda p: p)
        keys = [k for k, _ in doc]
        check(keys == TOP_KEYS, "%s top-level key order is %s" % (fid, keys))
        d = dict(doc)
        check(d["schema"] == "efh.battery/v1", "%s bad schema" % fid)
        check(d["function_id"] == fid, "%s function_id mismatch" % fid)
        check(d["battery_id"] == "EFH-B1", "%s bad battery_id" % fid)
        check(re.fullmatch(r"[0-9a-f]{40}", d["oxfunc_commit"]) is not None,
              "%s oxfunc_commit is not 40-hex" % fid)
        check(d["oxfunc_tree_clean"] is True, "%s tree not clean" % fid)
        check([k for k, _ in d["host"]] == ["arch", "cpu", "os"], "%s host key order" % fid)
        check(d["label"] == LABEL, "T7(d) %s label constant differs" % fid)

        rows = d["rows"]
        check(len(rows) == 12, "T7(a) %s has %d rows" % (fid, len(rows)))
        check([dict(r)["label"] for r in rows] == FIXED_LABELS,
              "T7(a) %s row labels/order differ" % fid)
        for r in rows:
            rk = [k for k, _ in r]
            check(rk == ROW_KEYS, "%s row key order is %s" % (fid, rk))
            row = dict(r)
            rows_total += 1
            kind_counts[row["outcome_kind"]] = kind_counts.get(row["outcome_kind"], 0) + 1
            check(row["outcome_kind"] in KINDS,
                  "%s unknown outcome_kind %s" % (fid, row["outcome_kind"]))
            if row["outcome_kind"] == "number":
                numbers += 1
                bits = row["outcome_bits"]
                check(isinstance(bits, str) and re.fullmatch(r"0x[0-9a-f]{16}", bits) is not None,
                      "T7(b) %s/%s outcome_bits malformed: %r" % (fid, row["label"], bits))
                disp = row["outcome_display"]
                word = int(bits, 16)
                exponent_all_ones = (word >> 52) & 0x7FF == 0x7FF
                if disp in ("inf", "-inf", "nan"):
                    # OxFunc published a non-finite binary64 as a worksheet number. No decimal
                    # string round-trips a NaN sign/payload, so outcome_bits is the authority and
                    # the display is the IEEE class name. Counted separately below.
                    nonfinite.append((fid, row["label"], disp, bits))
                    check(exponent_all_ones,
                          "T7(b) %s/%s display %r but bits %s are finite"
                          % (fid, row["label"], disp, bits))
                    if disp != "nan":
                        v = float(disp)
                        got = "0x%016x" % struct.unpack("<Q", struct.pack("<d", v))[0]
                        check(got == bits,
                              "T7(b) %s/%s does not round-trip: %s -> %s != %s"
                              % (fid, row["label"], disp, got, bits))
                else:
                    check(re.fullmatch(r"-?\d\.\d{16}e-?\d+", disp) is not None,
                          "T7(b) %s/%s display is not 17 significant digits: %r"
                          % (fid, row["label"], disp))
                    # round-trip: the printed decimal must reproduce the published bits exactly
                    v = float(disp)
                    got = "0x%016x" % struct.unpack("<Q", struct.pack("<d", v))[0]
                    check(got == bits,
                          "T7(b) %s/%s does not round-trip: %s -> %s != %s"
                          % (fid, row["label"], disp, got, bits))
            else:
                check(row["outcome_bits"] is None,
                      "%s/%s carries bits on a non-number" % (fid, row["label"]))
            expected_scoped = (fid in scope) and row["outcome_kind"] == "number"
            check(row["host_scoped"] == expected_scoped,
                  "T7(c) %s/%s host_scoped=%s, pinned list says %s"
                  % (fid, row["label"], row["host_scoped"], expected_scoped))
            if row["host_scoped"]:
                host_rows += 1
                host_entries.add(fid)

    print("files: %d" % len(got_ids))
    print("rows: %d" % rows_total)
    print("number rows: %d (of which non-finite binary64: %d)" % (numbers, len(nonfinite)))
    if nonfinite:
        print("non-finite number rows:")
        for fid, lab, disp, bits in nonfinite:
            print("  %s / %s -> %s %s" % (fid, lab, disp, bits))
    print("host_scoped rows: %d across %d entries" % (host_rows, len(host_entries)))
    print("pinned x87 scope entries: %d" % len(scope))
    print("outcome_kind counts: %s" % json.dumps(dict(sorted(kind_counts.items()))))
    if fails:
        print("\nFAILURES (%d):" % len(fails))
        for f in fails[:50]:
            print("  " + f)
        sys.exit(1)
    print("\nALL CHECKS PASS")


if __name__ == "__main__":
    main()
