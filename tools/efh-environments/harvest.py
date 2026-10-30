"""Harvest Excel answers keyed by function and exact argument bits, and join them across Excel builds.

Reads (never writes) OxFunc/smart-fuzzer/cache and runs (the originals archived in OxFunc-Evidence
cache/ and runs/), OxFunc/docs/function-lane/evidence, and the W109 banks archived in
OxFunc-Evidence/work/w109 (gzip blobs). Does not read OxFunc/smart-fuzzer/work.

Accepted as Excel readings:
  oracle-cache-v1 records (environment in the record);
  *_excel_comparison* records with formula_text and excel_outcome, only from cell-reference runs
    (formula-literal runs are skipped: Excel reads a typed literal to 15 significant digits, so the
    input is not the bits the record names);
  answers files {"function", "witnesses": [{args, expected_bits}]} not named as graph, prediction,
    model, candidate, local, race, replay, freeze or judgement output; environment from the file's
    capture_provenance, a capture_provenance.json or run rollup beside it, a build<NNNNN> file name
    (W109), or DnaCalc's folder label (migration/evidence/oxfunc-smart-fuzzer.json).

Usage: python -I harvest.py <OxFunc root> <DnaCalc labels json> <out.json>
Output: per function and environment the distinct inputs read, first/last date and source files;
every input read under two builds with its results (overlap_detail); same-environment conflicts.
"""
import json, os, re, sys, collections, hashlib, struct

OX = sys.argv[1]            # C:/Work/DnaCalc/OxFunc
LABELS = sys.argv[2]        # DnaCalc migration/evidence/oxfunc-smart-fuzzer.json
OUT = sys.argv[3]

roots = [("sf", os.path.join(OX, "smart-fuzzer", "cache"), "cache"),
         ("sf", os.path.join(OX, "smart-fuzzer", "runs"), "runs"),
         ("doc", os.path.join(OX, "docs", "function-lane", "evidence"), "docs/function-lane/evidence"),
         ("w109", os.path.join(os.path.dirname(OX), "OxFunc-Evidence", "work", "w109"), "work/w109")]
import gzip
def opentext(p):
    if p.endswith(".gz"):
        return gzip.open(p, "rt", encoding="utf-8-sig", errors="replace")
    return open(p, encoding="utf-8-sig", errors="replace")
BUILDNAME = re.compile(r"build[-_]?(\d{5})", re.I)

labels = {}
for l in json.load(open(LABELS, encoding="utf-8"))["labels"]:
    if l["label"] != "unknown" and l.get("excel_build"):
        labels[l["entry"]] = l["excel_build"]

BAD_NAME = re.compile(r"graph|pred|model|candidate|local|race|replay|oxfunc|freeze|judg", re.I)

def norm_result(v):
    if v is None:
        return None
    if isinstance(v, dict):
        if "digest_payload" in v:
            return norm_result(v["digest_payload"])
        if v.get("kind") == "number" and "bits_hex" in v:
            return "number:" + v["bits_hex"].lower()
        return "json:" + json.dumps(v, sort_keys=True)
    s = str(v)
    if s.startswith("0x"):
        return "number:" + s.lower()
    if s.startswith("number:0x"):
        return s.lower()
    return s

def norm_arg(a):
    if isinstance(a, str):
        return a.lower() if a.startswith("0x") else a
    if isinstance(a, dict):
        if "bits_hex" in a:
            return a["bits_hex"].lower()
        if a.get("kind") == "text":
            return "text:" + str(a.get("value"))
        if a.get("kind") == "logical" or a.get("kind") == "boolean":
            return "logical:" + str(a.get("value"))
        return "json:" + json.dumps(a, sort_keys=True)
    if isinstance(a, bool):
        return "logical:" + str(a)
    if isinstance(a, (int, float)):
        return "0x%016x" % struct.unpack("<Q", struct.pack("<d", float(a)))[0]
    return "json:" + json.dumps(a, sort_keys=True)

NUM = re.compile(r"^[+-]?(\d+(\.\d*)?|\.\d+)([eE][+-]?\d+)?$")
def formula_key(fn, ft):
    m = re.match(r"^=\s*([A-Za-z0-9_.]+)\((.*)\)\s*$", ft)
    if not m:
        return None
    parts = [p.strip() for p in m.group(2).split(",")] if m.group(2).strip() else []
    bits = []
    for p in parts:
        if not NUM.match(p):
            return None
        bits.append("0x%016x" % struct.unpack("<Q", struct.pack("<d", float(p)))[0])
    return (fn or m.group(1)).upper() + "|" + "|".join(bits)

def env_from(env):
    if not isinstance(env, dict):
        return None
    b = env.get("excel_build")
    if not b:
        return None
    return {
        "excel_version": env.get("excel_version"),
        "excel_build": str(b),
        "excel_bitness": env.get("excel_bitness"),
        "os": env.get("excel_operating_system") or env.get("operating_system"),
        "cpu": env.get("cpu_id"),
        "workbook_compatibility": env.get("workbook_compatibility"),
        "plumbing": env.get("excel_input_plumbing"),
    }

def env_id(e):
    return "|".join(str(e.get(k)) for k in ("excel_version", "excel_build", "excel_bitness", "os", "cpu"))

folder_env_cache = {}
def folder_env(d, stop):
    """capture_provenance.json beside, or rollup/manifest excel_environment up to stop."""
    if d in folder_env_cache:
        return folder_env_cache[d]
    res = None
    cur = d
    while True:
        for name in ("capture_provenance.json", "rollup.json", "manifest.json", "run_manifest.json"):
            p = os.path.join(cur, name)
            if os.path.isfile(p):
                try:
                    j = json.load(opentext(p))
                except Exception:
                    continue
                cand = None
                if isinstance(j, dict):
                    cand = env_from(j.get("environment")) or env_from(j.get("excel_environment"))
                    if cand is None and isinstance(j.get("capture_provenance"), dict):
                        cand = env_from(j["capture_provenance"].get("environment"))
                if cand:
                    res = (cand, os.path.relpath(p, OX).replace("\\", "/") + ":" + name)
                    break
        if res or os.path.normcase(cur) == os.path.normcase(stop):
            break
        cur = os.path.dirname(cur)
    folder_env_cache[d] = res
    return res

entries = {}   # hash -> (eidx, res, srcidx) first-seen, or dict when seen again
envs = {}
eids = []
eid_index = {}
srcs = []
src_index = {}
coverage = collections.defaultdict(collections.Counter)
intern = sys.intern
def _h(key):
    return int.from_bytes(hashlib.blake2b(key.encode("utf-8"), digest_size=8).digest(), "little")
prov = {}
DATE = re.compile(r"(20\d\d)(\d\d)(\d\d)")
def folder_of(src):
    parts = src.split("/")
    if parts[0] == "docs":
        return "/".join(parts[:5]) + "/"
    return "/".join(parts[:2]) + "/"
def add(key, env, res, src, kind, date=None):
    if key is None or res is None:
        return
    eid = env_id(env) if env else "unknown"
    if date is None:
        m = DATE.search(src)
        if m:
            date = m.group(1) + "-" + m.group(2) + "-" + m.group(3)
    fn0 = key.split("|")[0]
    pv = prov.get((fn0, eid))
    if pv is None:
        pv = prov[(fn0, eid)] = {"folders": collections.Counter(), "files": set(), "first": None, "last": None, "kinds": set()}
    fo = folder_of(src)
    pv["folders"][fo] += 1
    if len(pv["files"]) < 50:
        pv["files"].add(src)
    pv["kinds"].add(kind)
    if date:
        if pv["first"] is None or date < pv["first"]: pv["first"] = date
        if pv["last"] is None or date > pv["last"]: pv["last"] = date
    if env:
        envs[eid] = env
    ei = eid_index.get(eid)
    if ei is None:
        ei = eid_index[eid] = len(eids); eids.append(eid)
    s = src + "|" + kind
    si = src_index.get(s)
    if si is None:
        si = src_index[s] = len(srcs); srcs.append(s)
    res = intern(res)
    h = _h(key)
    cur = entries.get(h)
    fn = key.split("|")[0]
    if cur is None:
        entries[h] = (ei, res, si)
        coverage[fn][eid] += 1
        return
    if isinstance(cur, tuple):
        cur = {"key": key, "obs": {eids[cur[0]]: {cur[1]: [srcs[cur[2]]]}}}
        entries[h] = cur
    obs = cur["obs"]
    if eid not in obs:
        coverage[fn][eid] += 1
    r = obs.setdefault(eid, {})
    lst = r.setdefault(res, [])
    if len(lst) < 2:
        lst.append(s)

def label_build(rel):
    # rel like cache/xxx/... or runs/xxx/...
    parts = rel.split("/")
    for k in (parts[0] + "/" + parts[1] + "/", parts[0] + "/" + parts[1]):
        if k in labels:
            return labels[k]
    return None

nfiles = 0
for kind_root, root, prefix in roots:
    for dp, dn, fn in os.walk(root):
        for f in fn:
            p = os.path.join(dp, f)
            if kind_root == "w109":
                if not f.endswith(".gz"):
                    continue
                f = f[:-3]
            if not (f.endswith(".json") or f.endswith(".jsonl")):
                continue
            rel = (prefix + "/" + os.path.relpath(p, root).replace("\\", "/"))
            if rel.endswith(".gz"):
                rel = rel[:-3]
            nfiles += 1
            if nfiles % 2000 == 0:
                print(nfiles, len(entries), file=sys.stderr, flush=True)
            try:
                size = os.path.getsize(p)
            except OSError:
                continue
            is_answers = "answers" in f.lower() and not BAD_NAME.search(f)
            if f.endswith(".jsonl") or size > 50_000_000:
                # line mode: oracle-cache and comparison records
                fenv = None
                try:
                    with opentext(p) as fh:
                        for line in fh:
                            if "oracle-cache-v1" in line:
                                try:
                                    r = json.loads(line)
                                except Exception:
                                    continue
                                env = env_from(r.get("environment"))
                                args = r.get("args") or []
                                if args and isinstance(args[0], list):
                                    args = args[0]
                                key = r.get("function_name", "").upper() + "|" + "|".join(norm_arg(a) for a in args)
                                add(key, env, norm_result(r.get("outcome")), rel, "oracle-cache-v1", (r.get("recorded_utc") or "")[:10] or None)
                            elif "excel_outcome" in line and "formula_text" in line and "excel_comparison" in line:
                                try:
                                    r = json.loads(line)
                                except Exception:
                                    continue
                                if fenv is None:
                                    fe = folder_env(dp, root)
                                    fenv = fe[0] if fe else False
                                    if not fe:
                                        b = label_build(rel)
                                        fenv = {"excel_version": "16.0", "excel_build": b} if b else False
                                if not fenv or fenv.get("plumbing") not in ("cell_value2", "cell_value2_matrix", "cell_value2_bulk"):
                                    continue
                                key = formula_key(r.get("function_name"), r.get("formula_text", ""))
                                add(key, fenv or None, norm_result(r.get("excel_outcome")), rel, "excel-comparison", None)
                except Exception as ex:
                    print("ERR", rel, ex, file=sys.stderr)
                continue
            if not is_answers:
                continue
            try:
                j = json.load(opentext(p))
            except Exception:
                continue
            items = j if isinstance(j, list) else [j]
            for it in items:
                if not isinstance(it, dict) or "witnesses" not in it:
                    continue
                env = None
                if isinstance(it.get("capture_provenance"), dict):
                    env = env_from(it["capture_provenance"].get("environment"))
                if env is None:
                    fe = folder_env(dp, root)
                    env = fe[0] if fe else None
                if env is None and kind_root == "w109":
                    m = BUILDNAME.search(f)
                    env = {"excel_version": "16.0", "excel_build": m.group(1)} if m else None
                if env is None:
                    b = label_build(rel) if kind_root == "sf" else None
                    env = {"excel_version": "16.0", "excel_build": b} if b else None
                cdate = None
                if isinstance(it.get("capture_provenance"), dict):
                    cdate = (it["capture_provenance"].get("captured_utc") or "")[:10] or None
                fname = str(it.get("function") or it.get("function_name") or "").upper()
                for w in it.get("witnesses") or []:
                    if not isinstance(w, dict) or "expected_bits" not in w:
                        continue
                    key = fname + "|" + "|".join(norm_arg(a) for a in (w.get("args") or []))
                    add(key, env, norm_result(w.get("expected_bits")), rel, "answers", cdate)

print("files", nfiles, "keys", len(entries), file=sys.stderr)

# analysis
by_build_pair = collections.defaultdict(lambda: collections.Counter())
conflicts_cross = []
conflicts_same = []
overlap_detail = []
multi = 0
for h, cur in entries.items():
    if isinstance(cur, tuple):
        continue
    multi += 1
    key, em = cur["key"], cur["obs"]
    fn = key.split("|")[0]
    known = {eid: rs for eid, rs in em.items() if eid != "unknown"}
    for eid, rs in known.items():
        if len(rs) > 1:
            conflicts_same.append((key, eid, rs))
    builds = {}
    for eid, rs in known.items():
        b = envs[eid]["excel_build"]
        builds.setdefault(b, set()).update(rs.keys())
    if len(builds) > 1:
        bl = sorted(builds)
        allres = set().union(*builds.values())
        pair = " vs ".join(bl)
        by_build_pair[fn][pair + (" DIFFER" if len(allres) > 1 else " same")] += 1
        overlap_detail.append({"key": key, "builds": {b: sorted(v) for b, v in builds.items()}, "by_env": {e: {r: q for r, q in rs.items()} for e, rs in known.items()}})
        if len(allres) > 1:
            conflicts_cross.append((key, known))
print("multi", multi, file=sys.stderr)

out = {
    "files": nfiles,
    "keys": len(entries),
    "envs": envs,
    "coverage": {fn: dict(c) for fn, c in sorted(coverage.items())},
    "overlap": {fn: dict(c) for fn, c in sorted(by_build_pair.items())},
    "cross_build_conflicts": [{"key": k, "by_env": {e: {r: s for r, s in rs.items()} for e, rs in em.items()}} for k, em in conflicts_cross],
    "overlap_detail": overlap_detail,
    "prov": [{"fn": k[0], "env": k[1], "first": v["first"], "last": v["last"], "kinds": sorted(v["kinds"]), "folders": dict(v["folders"]), "files": sorted(v["files"])} for k, v in sorted(prov.items())],
    "same_env_conflicts_count": len(conflicts_same),
    "same_env_conflicts": [{"key": k, "env": e, "results": {r: s for r, s in rs.items()}} for k, e, rs in conflicts_same[:400]],
}
json.dump(out, open(OUT, "w", encoding="utf-8"), indent=1)
print("cross", len(conflicts_cross), "same", len(conflicts_same), file=sys.stderr)
