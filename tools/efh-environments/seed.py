"""Build the F19 seed (content/evidence/environments/*.json) from a harvest.json written by harvest.py.

Usage (from the Handbook root, sibling checkouts of OxFunc and OxFunc-Evidence beside it):
    python -I tools/efh-environments/harvest.py ../OxFunc <DnaCalc>/migration/evidence/oxfunc-smart-fuzzer.json harvest.json
    python -I tools/efh-environments/seed.py harvest.json

Deterministic for a fixed harvest and fixed commits of the three repositories. Writes one file per
function with readings or a sensitivity, and removes nothing else. The seed is a starting point:
after it, F19 records are curated content and later evidence is added to them, not regenerated.
"""
import glob
import hashlib
import json
import os
import subprocess
import sys
from collections import Counter, defaultdict

HB = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
WORK = os.path.dirname(HB)
OXFUNC = os.path.join(WORK, "OxFunc")
EVID = os.path.join(WORK, "OxFunc-Evidence")


def git_head(repo):
    return subprocess.check_output(["git", "-C", repo, "rev-parse", "HEAD"], text=True).strip()


OXFUNC_COMMIT = git_head(OXFUNC)
EVID_COMMIT = git_head(EVID)
HB_COMMIT = git_head(HB)

manifest = {}
for name in ("manifest.tsv", "w109-manifest.tsv"):
    with open(os.path.join(EVID, name), encoding="utf-8") as f:
        next(f)
        for line in f:
            c = line.rstrip("\n").split("\t")
            manifest[c[0]] = c[2]


def sha_of(path):
    if path in manifest:
        return manifest[path]
    if path.startswith("docs/"):
        with open(os.path.join(OXFUNC, path), "rb") as f:
            return hashlib.sha256(f.read()).hexdigest()
    return None


def ev(path, locator=None, note=None, is_dir=False):
    repo = "OxFunc" if path.startswith("docs/") else "OxFunc-Evidence"
    commit = OXFUNC_COMMIT if repo == "OxFunc" else EVID_COMMIT
    return {
        "repository": repo,
        "commit": commit,
        "path": path,
        "locator": locator,
        "sha256": None if is_dir else sha_of(path),
        "note": note,
    }


# ---------------------------------------------------------------- environments

CPU_ZEN2 = "AMD Ryzen 7 4800H with Radeon Graphics"
harvest = json.load(open(sys.argv[1], encoding="utf-8"))
ENVS = harvest["envs"]


def env_record(eid):
    ver, build, bitness, os_, cpu = [None if x == "None" else x for x in eid.split("|")]
    compat = (ENVS.get(eid) or {}).get("workbook_compatibility")
    bit = {"64-bit": "x64", "32-bit": "x86"}.get(bitness, "nr")
    cpu_tag = "amd-zen2" if cpu == CPU_ZEN2 else ("nr" if cpu is None else "other")
    recorded = [k for k, v in (("Excel version and build", build), ("bitness", bitness), ("OS", os_), ("CPU", cpu)) if v]
    missing = [k for k, v in (("bitness", bitness), ("OS", os_), ("CPU", cpu)) if not v]
    label_only = set((ENVS.get(eid) or {}).keys()) <= {"excel_version", "excel_build"}
    basis = (
        ("Build taken from DnaCalc's folder label (migration/evidence/oxfunc-smart-fuzzer.json) or a"
         " build<NNNNN> file name; the capture itself records no environment."
         if label_only else "Recorded by the capture or its run rollup: " + ", ".join(recorded) + ".")
        + (" Not recorded: " + ", ".join(missing) + "." if missing else "")
        + ("" if os_ else " Platform is Windows because every OxFunc capture drove Excel through COM"
           " automation from PowerShell on Windows; the capture does not state the OS.")
        + (" The processor name is that of an AMD Zen 2 part (Ryzen 7 4800H)." if cpu == CPU_ZEN2 else "")
    )
    return {
        "env_id": f"win-{bit}-{ver}.{build}-{cpu_tag}",
        "excel_version": ver or "not-recorded",
        "excel_build": build or "not-recorded",
        "platform": "windows",
        "os": os_ or "not-recorded",
        "excel_bitness": bitness or "not-recorded",
        "cpu": cpu or "not-recorded",
        "cpu_vendor": "AMD" if cpu and cpu.startswith("AMD") else ("not-recorded" if not cpu else "not-recorded"),
        "cpu_microarchitecture": "Zen 2" if cpu == CPU_ZEN2 else "not-recorded",
        "workbook_compatibility": compat or "not-recorded",
        "update_channel": "not-recorded",
        "basis": basis + (f" Workbook Compatibility Version {compat} as recorded." if compat
                          else " Workbook Compatibility Version not recorded.") + " Update channel never recorded.",
    }


# ---------------------------------------------------------------- inputs


names = {}
for p in glob.glob(os.path.join(HB, "data", "functions", "*.json")):
    d = json.load(open(p, encoding="utf-8"))
    names[d["surface_name"].upper()] = (d["function_id"], d["surface_name"])

prov = defaultdict(dict)
for row in harvest["prov"]:
    if row["env"] != "unknown":
        prov[row["fn"]][row["env"]] = row


def dir_of(path):
    return path.rsplit("/", 1)[0] + "/"


def reading_evidence(row):
    files = sorted({f.split("|")[0] for f in row["files"]})
    if len(files) <= 4 and len(row["files"]) < 50:
        return [ev(f) for f in files]
    dirs = Counter()
    for f in files:
        dirs[dir_of(f)] += 1
    out = []
    ranked = sorted(dirs.items(), key=lambda x: (-x[1], x[0]))
    for d, n in ranked[:6]:
        out.append(ev(d, note=f"{n} file(s) in this folder hold readings of this function", is_dir=True))
    if len(ranked) > 6:
        out[-1]["note"] += f"; {len(ranked) - 6} further folder(s) not listed"
    if len(row["files"]) >= 50:
        out[-1]["note"] = (out[-1]["note"] or "") + "; folders drawn from the first 50 files read"
    return out


# comparisons: per function, per env pair
pairs = defaultdict(lambda: defaultdict(lambda: {"n": 0, "differ": 0, "files": defaultdict(set)}))
for o in harvest["overlap_detail"]:
    fn = o["key"].split("|")[0]
    envs = sorted(o["by_env"])
    results = set()
    for e in envs:
        results |= set(o["by_env"][e])
    for i in range(len(envs)):
        for j in range(i + 1, len(envs)):
            a, b = envs[i], envs[j]
            if a.split("|")[1] == b.split("|")[1]:
                continue
            ra, rb = set(o["by_env"][a]), set(o["by_env"][b])
            slot = pairs[fn][(a, b)]
            slot["n"] += 1
            if ra != rb:
                slot["differ"] += 1
            for e, rs in ((a, o["by_env"][a]), (b, o["by_env"][b])):
                for srcs in rs.values():
                    for s in srcs:
                        slot["files"][e].add(s.split("|")[0])

# sensitivities from the Handbook's own F5 records (counts marked cpu_scoped)
sens = defaultdict(list)
for p in sorted(glob.glob(os.path.join(HB, "content", "evidence", "records", "*.json"))):
    d = json.load(open(p, encoding="utf-8"))
    n = sum(1 for c in d.get("counts", []) if isinstance(c, dict) and c.get("cpu_scoped") is True)
    if n:
        for s in d["subjects"]:
            sens[s].append((d["record_id"], n, d["title"]))

W108 = "docs/worksets/W108_EXCEL_NUMERIC_CORE_AND_FINANCIAL_POWER_EXACTNESS.md"
X87_HOST = {"FUNC.EXP", "FUNC.LN", "FUNC.LOG10", "FUNC.LOG", "FUNC.POWER"}
COMBIN_DOC = "docs/function-lane/W109_COMBIN_IDENTIFICATION_20260809.md"

# ---------------------------------------------------------------- records

stem_of = lambda fid: fid.split(".", 1)[1]
out_dir = os.path.join(HB, "content", "evidence", "environments")
os.makedirs(out_dir, exist_ok=True)
fids = {}
for fn in harvest["coverage"]:
    fids[names[fn][0]] = fn
for fid in sens:
    fids.setdefault(fid, None)

written = 0
for fid in sorted(fids):
    fn = fids[fid]
    surface = next(s for (f, s) in names.values() if f == fid)
    stem = stem_of(fid)
    cov = harvest["coverage"].get(fn, {}) if fn else {}
    envs = {}
    readings = []
    for eid in sorted(prov.get(fn, {})) if fn else []:
        er = env_record(eid)
        envs[er["env_id"]] = er
        row = prov[fn][eid]
        readings.append({
            "env_id": er["env_id"],
            "inputs_read": cov.get(eid, 0),
            "first_observed": row["first"],
            "last_observed": row["last"],
            "evidence": reading_evidence(row),
        })
    order = sorted(envs)
    readings.sort(key=lambda r: order.index(r["env_id"]))

    comparisons = []
    for (a, b), slot in sorted(pairs.get(fn, {}).items(), key=lambda x: (env_record(x[0][0])["env_id"], env_record(x[0][1])["env_id"])) if fn else []:
        ea, eb = env_record(a), env_record(b)
        evs = []
        for e in (a, b):
            for f in sorted(slot["files"][e])[:4]:
                evs.append(ev(f))
        comparisons.append({
            "comparison_id": None,
            "axis": ["excel-build"],
            "env_ids": [ea["env_id"], eb["env_id"]],
            "inputs_compared": slot["n"],
            "inputs_differing": slot["differ"],
            "method": "Harvest (tools/efh-environments/harvest.py): Excel answers keyed by function and exact argument"
                      " bits; every input read in both environments, results compared as typed values (numbers by"
                      " their binary64 bits). Formula-literal captures excluded.",
            "evidence": evs,
        })
    if fid == "FUNC.COMBIN":
        e1 = env_record("16.0|20131|None|None|AMD Ryzen 7 4800H with Radeon Graphics")
        e2 = env_record("16.0|20228|64-bit|Windows (64-bit) NT 10.00|None")
        comparisons.append({
            "comparison_id": None,
            "axis": ["excel-build"],
            "env_ids": [e1["env_id"], e2["env_id"]],
            "inputs_compared": 503,
            "inputs_differing": 0,
            "method": "OxFunc's own statement: the July legacy bank (build 20131) and the current dense bank"
                      " (build 20228) share 503 tuples, all with identical result bits. The harvest comparison"
                      " of the same two builds reproduces the count.",
            "evidence": [ev(COMBIN_DOC, locator="lines 60-73")],
        })
    comparisons.sort(key=lambda c: (c["env_ids"], c["method"]))
    for i, c in enumerate(comparisons, 1):
        c["comparison_id"] = f"XC-{stem}-{i:03d}"
        for e in c["env_ids"]:
            if e not in envs:
                envs[e] = env_record(next(k for k in (
                    "16.0|20131|None|None|AMD Ryzen 7 4800H with Radeon Graphics",
                    "16.0|20228|64-bit|Windows (64-bit) NT 10.00|None") if env_record(k)["env_id"] == e))

    sensitivities = []
    for rid, n, title in sens.get(fid, []):
        stmt = (f"Handbook evidence record {rid} marks {n} of its counts for this function as CPU-scoped:"
                " the counted result depends on how the host CPU executes x87 instructions. Every reading"
                " recorded here was taken on an AMD Zen 2 processor or on a host whose CPU was not recorded;"
                " no reading on an Intel CPU, a Mac or Excel for the web exists, so a CPU difference is"
                " neither observed nor ruled out.")
        evs = [{
            "repository": "ExcelFunctionsHandbook",
            "commit": HB_COMMIT,
            "path": f"content/evidence/records/{rid}.json",
            "locator": "counts[].cpu_scoped",
            "sha256": hashlib.sha256(open(os.path.join(HB, "content", "evidence", "records", rid + ".json"), "rb").read()).hexdigest(),
            "note": None,
        }]
        if fid in X87_HOST:
            stmt += (" OxFunc found Excel's last bit here follows the x87 F2XM1/FYL2X microcode on the hardest"
                     " inputs (about 1 in 30). A dump of those x87 primitives on an AMD Zen 2 and an AMD Zen 1"
                     " host (Excel only on the first) differed on none of 3,157 rows (2026-08-23); Intel"
                     " against AMD is untested.")
            stmt += (" The Handbook's model chapter 05 (version axes) states the same mechanism, that on about"
                     " one in two thousand of the hardest inputs the last bit tracks the microcode of the CPU"
                     " Excel runs on; it and the Last Bit material reason about Intel against AMD but record"
                     " no Intel reading.")
            evs.append(ev(W108, locator="lines 236-244"))
            evs.append({
                "repository": "ExcelFunctionsHandbook",
                "commit": HB_COMMIT,
                "path": "content/model/05-version-axes.md",
                "locator": "lines 87-91",
                "sha256": hashlib.sha256(open(os.path.join(HB, "content", "model", "05-version-axes.md"), "rb").read()).hexdigest(),
                "note": "reasoning, not a measurement",
            })
        sensitivities.append({"sensitivity_id": None, "axis": ["cpu"], "status": "expected-untested",
                              "statement": stmt, "evidence": evs})
    for i, s in enumerate(sensitivities, 1):
        s["sensitivity_id"] = f"XS-{stem}-{i:03d}"

    # status
    parts = []
    if readings:
        builds = sorted({envs[r["env_id"]]["excel_build"] for r in readings})
        total = sum(r["inputs_read"] for r in readings)
        parts.append(f"Readings of {surface} with a recorded Excel build: {total:,} input(s) over"
                     f" {len(readings)} recorded environment(s), Excel 16.0 build(s) {', '.join(builds)},"
                     " all on Windows; "
                     + ("where a CPU is recorded it is an AMD Zen 2."
                        if any(envs[r["env_id"]]["cpu"] != "not-recorded" for r in readings)
                        else "none of these captures records the CPU."))
    else:
        parts.append(f"No reading of {surface} with a recorded Excel build was found in the harvested evidence.")
    unk = cov.get("unknown", 0)
    if unk:
        parts.append(f"{unk:,} input(s) were also read in captures that record no build (some may repeat inputs counted above); they are not counted.")
    if comparisons:
        n = sum(c["inputs_compared"] for c in comparisons if "Harvest" in c["method"])
        d = sum(c["inputs_differing"] for c in comparisons)
        parts.append(f"{len(comparisons)} same-input comparison(s) between builds"
                     + (f" over {n:,} input-pair(s)" if n else "")
                     + (": no result differs." if d == 0 else f": {d} differ."))
    else:
        parts.append("No input has been read in two environments, so no difference is known and none is ruled out.")
    if sensitivities:
        parts.append("A CPU dependence is expected and untested (see below).")
    parts.append("No reading from a Mac, Excel for the web, 32-bit Excel or an Intel CPU is recorded.")

    record = {
        "schema": "efh.excel-environments/v1",
        "function_id": fid,
        "surface_name": surface,
        "status_plain": " ".join(parts),
        "environments": [envs[k] for k in sorted(envs)],
        "readings": readings,
        "comparisons": comparisons,
        "differences": [],
        "sensitivities": sensitivities,
    }
    with open(os.path.join(out_dir, fid + ".json"), "w", encoding="utf-8", newline="\n") as f:
        json.dump(record, f, ensure_ascii=False, indent=2)
        f.write("\n")
    written += 1

print(f"wrote {written} records; OxFunc {OXFUNC_COMMIT}, OxFunc-Evidence {EVID_COMMIT}, Handbook {HB_COMMIT}")
