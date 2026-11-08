#!/usr/bin/env python3
"""Regenerates data/calcfunc/ and the catalogue fields of data/functions/ from a pinned DnaCalc commit.

    python3 tools/efh-calcfunc/regenerate.py --dnacalc PATH [--pin SHA] [--corpus-run DIR]

PATH is a clean clone of https://github.com/DnaCalc/DnaCalc at the pin; the pin lives in
data/calcfunc/pin.json. --corpus-run names the output of

    cargo xtask corpus --oracle --no-capture --cache CACHE --out DIR

at that same commit, and refreshes data/calcfunc/corpus-tally.json; without it the committed tally
is used and must name the pin. Nothing here is edited by hand: state, signatures, arity, admission
and the code excerpts all come from the commit.
"""
import argparse, csv, hashlib, io, json, re, subprocess, sys, tempfile, textwrap
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
sys.path.insert(0, str(HERE))
import rustindex  # noqa: E402
import tally as corpus_tally  # noqa: E402

GITHUB = "https://github.com/DnaCalc/DnaCalc"
DATA = REPO / "data"
OUT = DATA / "calcfunc"
SERIES = json.loads((HERE / "non-catalogue.json").read_text(encoding="utf8"))
STATE_LABELS = {
    "matches": "Matches every recorded Excel answer",
    "close": "Close: last-bit deviations remain",
    "differs": "Known larger differences",
    "refuses": "Refuses or not implemented",
    "deferred": "Deferred: its owner has not built it yet",
    "host": "Needs a host",
    "absent": "Not in Excel",
    "unmeasured": "Implemented, no recorded answers attributed to it",
}
OPERATORS = {
    "OP_ADD": ("+", "infix"), "OP_SUBTRACT": ("-", "infix"), "OP_MULTIPLY": ("*", "infix"),
    "OP_DIVIDE": ("/", "infix"), "OP_POWER": ("^", "infix"), "OP_NEGATE": ("-", "prefix"),
    "OP_UNARY_PLUS": ("+", "prefix"), "OP_PERCENT": ("%", "postfix"), "OP_EQUAL": ("=", "infix"),
    "OP_NOT_EQUAL": ("<>", "infix"), "OP_LESS_THAN": ("<", "infix"), "OP_LESS_EQUAL": ("<=", "infix"),
    "OP_GREATER_THAN": (">", "infix"), "OP_GREATER_EQUAL": (">=", "infix"), "OP_CONCAT": ("&", "infix"),
}


def git(root, *args):
    return subprocess.check_output(["git", "-C", str(root), *args], text=True).strip()


def show(root, pin, path):
    return subprocess.check_output(["git", "-C", str(root), "show", f"{pin}:{path}"]).decode("utf8")


class Dumper:
    """Builds the dump tool against the checkout: the catalogue's own signature help, and calls."""

    def __init__(self, root):
        self.scratch = tempfile.TemporaryDirectory(prefix="efh-calcfunc-")
        scratch = Path(self.scratch.name)
        (scratch / "src").mkdir()
        (scratch / "src/main.rs").write_text((HERE / "dump/src/main.rs").read_text())
        (scratch / "Cargo.toml").write_text((HERE / "dump/Cargo.toml.in").read_text().replace("@DNACALC@", str(root)))
        (scratch / "Cargo.lock").write_text((Path(root) / "Cargo.lock").read_text())
        (scratch / "rust-toolchain.toml").write_text((Path(root) / "rust-toolchain.toml").read_text())
        subprocess.run(["cargo", "build", "--release", "--quiet"], cwd=scratch, check=True)
        self.binary = str(scratch / "target/release/efh-calcfunc-dump")

    def catalogue(self):
        return json.loads(subprocess.check_output([self.binary]))

    def calls(self, lines):
        out = subprocess.run([self.binary, "eval"], input="\n".join(lines) + "\n", text=True, capture_output=True, check=True)
        return out.stdout.split()


def family_regions(text, code):
    regions = []
    for m in re.finditer(r"\bFAMILY\s*:\s*(?:super::)?Family\s*=\s*(?:super::)?Family\s*\{", code):
        regions.append((m.start(), rustindex.matching(code, m.end() - 1)))
    for m in re.finditer(r"\bported_family!\s*\{", code):
        regions.append((m.start(), rustindex.matching(code, m.end() - 1)))
    return regions


def opener(code, position):
    """The bracket that encloses `position`, searching backwards."""
    depth = 0
    for i in range(position - 1, -1, -1):
        c = code[i]
        if c in ")}]":
            depth += 1
        elif c in "({[":
            if depth == 0:
                return i
            depth -= 1
    return -1


def entry_item(index, target):
    """The declaration of one catalogue entry and the kernels it names."""
    spelling, notation = target["spelling"], target["notation"]
    for relative, (text, code) in index.files.items():
        if not relative.startswith("catalogue/"):
            continue
        module = relative[:-3]
        for start, end in family_regions(text, code):
            region = text[start:end]
            if notation == "call":
                row = re.search(r'^[ \t]*\d+\s+\w+\s+"' + re.escape(spelling) + r'"\s*\(\d+,\s*\d+\)', region, re.M)
                if row:
                    at = start + row.start()
                    bar = code.index("|", at)
                    open_brace = code.index("{", code.index("|", bar + 1))
                    stop = rustindex.matching(code, open_brace)
                    first = at
                    return row_item(index, relative, module, spelling, text, code, first, stop), []
                for m in re.finditer(r'"' + re.escape(spelling) + r'"', region):
                    item = literal_item(index, relative, module, spelling, text, code, start + m.start())
                    if item:
                        return item
            else:
                kind = notation.capitalize()
                for m in re.finditer(r'"' + re.escape(spelling) + r'"\s*,\s*Notation::' + kind, region):
                    item = literal_item(index, relative, module, spelling + " " + notation, text, code, start + m.start())
                    if item:
                        return item
    raise SystemExit(f"no declaration found for {spelling} ({notation})")


def row_item(index, relative, module, name, text, code, start, stop):
    line = text.rfind("\n", 0, start) + 1
    item = rustindex.Item(key=f"{module}::entry::{name}", module=module, file=relative, name=name, owner="",
                          start=line, end=stop, start_line=text.count("\n", 0, line) + 1,
                          end_line=text.count("\n", 0, stop) + 1, text=text[line:stop], code=code[line:stop])
    return item


def literal_item(index, relative, module, name, text, code, position):
    bracket = opener(code, position)
    if bracket < 0:
        return None
    if code[bracket] == "{":
        stop = rustindex.matching(code, bracket)
        head = text.rfind("\n", 0, bracket) + 1
        block = code[head:stop]
        kernels = re.findall(r"Kernel::(?:Eager|Lazy)\((\w+)\)", block) + re.findall(r"(?:run|start):\s*(\w+)", block)
    elif code[bracket] == "(":
        stop = rustindex.matching(code, bracket, "(", ")")
        head = text.rfind("\n", 0, bracket) + 1
        block = code[head:stop]
        arguments = re.findall(r"([a-z_][a-z0-9_]*)\s*,?\s*\)\s*$", block.rstrip())
        kernels = arguments[:1]
    else:
        return None
    item = row_item(index, relative, module, name, text, code, bracket, stop)
    roots = [i for k in kernels for i in index.by_name.get(k, []) if i.module == module and not i.owner]
    return item, roots


def extract_code(index, dump, formula_index, formula_roots):
    """Each entry's declaration, its own kernels and the kernels it shares."""
    reached = {}
    for target in dump:
        found = entry_item(index, target)
        item, roots = found
        items, plumbing = index.closure([item] + roots)
        name = target["spelling"] if target["notation"] == "call" else f'{target["spelling"]} {target["notation"]}'
        reached[name] = (item, items, plumbing)
    for name, keys in formula_roots.items():
        roots = [formula_index.items[k] for k in keys]
        items, plumbing = formula_index.closure(roots)
        reached[name] = (roots[0], items, plumbing)
    users = Counter()
    for _, items, _ in reached.values():
        for key in items:
            users[key] += 1
    fragments, entries = {}, {}

    def keep(item, prefix):
        key = item.key
        if key not in fragments:
            fragments[key] = dict(file=prefix + item.file, start_line=item.start_line, end_line=item.end_line,
                                  name=(f"{item.owner}::{item.name}" if item.owner else item.name),
                                  module=item.module, users=users.get(key, 0), text=textwrap.dedent(item.text).rstrip() + "\n")
        return key

    for name, (item, items, plumbing) in sorted(reached.items()):
        prefix = "crates/calc-fml/src/" if name in formula_roots else "crates/calc-func/src/"
        own, shared = [], []
        for key, value in sorted(items.items(), key=lambda kv: (kv[1].file, kv[1].start_line)):
            (own if users[key] == 1 else shared).append(keep(value, prefix))
        order = [item.key] + [k for k in own if k != item.key]
        entries[name] = dict(own=order, shared=shared, plumbing=[keep(v, "crates/calc-func/src/") for _, v in sorted(plumbing.items())])
    return dict(fragments=fragments, entries=entries)


def read_bank(root, pin):
    rows = {}
    for row in csv.DictReader(io.StringIO(show(root, pin, "crates/calc-func/witness-status.tsv")), delimiter="\t"):
        if row["bank"] == "full":
            rows[row["function"]] = row
    return rows


DEFERRAL_TEXT = {
    "external_service": ("Deferred: needs an outside service", "Deferred by policy: the answer needs an outside service that DNA Calc does not run. The catalogue declares this entry deferred (owner {owner})."),
    "python": ("Deferred: needs an outside service", "Deferred by policy: the answer needs Python run by an outside service. The catalogue declares this entry deferred (owner {owner})."),
    "lists_in_cells": ("Deferred: lists in cells", "Deferred: the catalogue declares this a lists-in-cells function (Insider Beta, workbook compatibility version 3), owned by {owner}. DNA Calc does not run it until lists in cells land."),
}


def declared_group(row, spec):
    """The group of a catalogue entry that holds no kernel or reaches its answer through a host,
    read from the entry's own declaration; None for any other entry. The hand-held text in
    non-catalogue.json is used only when it names the same kind."""
    kind, implementation = row["kind"], row["implementation"]

    def pick(group_kind, summary, reason):
        same = spec.get("kind") == group_kind
        return {"kind": group_kind, "summary": spec.get("summary") if same and spec.get("summary") else summary,
                "reason": spec.get("reason") if same and spec.get("reason") else reason}
    if row["availability"] == "absent_on_windows":
        return pick("not_in_excel", "Not in Excel: recorded Windows Excel does not have it",
                    "The catalogue declares this entry absent on Windows: Excel 16.0 build 20527 gives #NAME? for it, the answer a missing name gets, and the kernel returns the same.")
    if implementation["state"] == "deferred":
        summary, reason = DEFERRAL_TEXT[implementation["class"]]
        return pick("deferred", summary, reason.format(owner=implementation["owner"]))
    if kind["type"] == "host_route":
        route = kind["route"]
        if route == "rtd":
            return pick("host", "Needs a real-time data server",
                        "The formula layer evaluates RTD over the host's real-time connector (read_topic in calc-fml); the answer comes from a COM server the host starts. The catalogue entry is a host route and declares no kernel (owner " + implementation.get("owner", "none") + ").")
        return pick("host", "Needs the host to fetch",
                    "The catalogue entry is a host route (" + route + "): the answer is what the host's fetch provider returns for the URL, and the entry gives what Excel gives when the fetch fails.")
    if implementation["state"] == "not_held":
        return pick("host", "Needs a host that can load code",
                    "The catalogue declares this entry not held (owner " + implementation["owner"] + "): the host that loads code does the work. DNA Calc recognises the name and calculates no answer.")
    return None


def state_of(name, group, bank, corpus, conflicts):
    """The plain conclusion for one entry, from its measured counts."""
    b = bank or {}
    c = corpus or {}
    rows = b.get("rows", 0)
    read = rows - b.get("unread", 0)
    cells = c.get("cells", 0) - c.get("pending", 0)
    total = read + cells
    last = b.get("last_bit", 0) + c.get("last_bit", 0)
    larger = b.get("larger_numeric", 0) + c.get("larger_numeric", 0)
    other = b.get("other_mismatch", 0) + c.get("other_mismatch", 0)
    refused = b.get("refused", 0) + c.get("refused", 0)
    worst = max(b.get("worst_ulp", 0) if b.get("larger_numeric", 0) == 0 else 0, c.get("worst_ulp", 0))
    sources = " and ".join(x for x in (f"{read:,} in the answer bank" if read else "", f"{cells:,} workbook cells" if cells else "") if x)
    if group["kind"] in ("deferred", "host"):
        return group["kind"], group["summary"], group["reason"]
    if group["kind"] == "not_in_excel":
        return "absent", group["summary"], group["reason"]
    if group["kind"] == "absent":
        return "refuses", group["summary"], group["reason"]
    if total == 0:
        return "unmeasured", "Implemented; no recorded answers are attributed to this entry", group.get("reason") or ""
    if refused == total:
        return "refuses", f"Refuses all {total:,} recorded calls", group.get("reason") or "The catalogue row declines the call as not modelled."
    if larger or other or refused:
        parts = []
        if larger:
            parts.append(f"{larger:,} differ by more than 16 ULP")
        if last:
            parts.append(f"{last:,} differ by at most 16 ULP")
        if other:
            parts.append(f"{other:,} differ in kind or value")
        if refused:
            parts.append(f"{refused:,} are refused")
        wrong = larger + last + other + refused
        detail = f"Of {total:,} recorded answers ({sources}), {wrong:,} do not match: {', '.join(parts)}."
        if conflicts:
            detail += f" Excel's own recorded readings disagree with each other on {conflicts:,} of these inputs."
        if refused == wrong:
            return "differs", f"Refuses {refused:,} of {total:,} recorded calls; the rest match", detail
        return "differs", f"Known larger differences on {wrong:,} of {total:,} recorded answers", detail
    if last:
        return "close", f"Close: {last:,} of {total:,} recorded answers differ in the last bits, worst {worst} ULP", \
            f"Everything else matches ({sources}). Last-bit means a finite number within 16 ULP of Excel's."
    return "matches", f"Matches all {total:,} recorded answers", f"Recorded: {sources}."


def ulps(a, b):
    def ordered(bits):
        n = int(bits, 16)
        return (~n & ((1 << 64) - 1)) if n >> 63 else n | (1 << 63)
    return abs(ordered(a) - ordered(b))


def host_cpu():
    for line in Path("/proc/cpuinfo").read_text().splitlines():
        if line.startswith("model name"):
            return line.split(":", 1)[1].strip()
    return "an unnamed CPU"


def environment_label(env):
    cpu = " ".join(x for x in (env["cpu_vendor"], env["cpu_microarchitecture"]) if x != "not-recorded") or "CPU not recorded"
    bits = "" if env["excel_bitness"] == "not-recorded" else f", {env['excel_bitness']}"
    return f"{env['excel_version']} build {env['excel_build']}", f"{cpu} ({env['platform'].capitalize()}{bits})"


def environment_variants(dumper, state, code, fragments, github, pin):
    """One record per Excel environment with recorded differences, and what our Rust answers there."""
    result = {}
    for path in sorted((REPO / "content/evidence/environments").glob("FUNC.*.json")):
        record = json.loads(path.read_text(encoding="utf8"))
        if not record["differences"]:
            continue
        fid, name = record["function_id"], record["surface_name"]
        environments = {e["env_id"]: e for e in record["environments"]}
        rows = {}
        for difference in record["differences"]:
            inputs = [i["bits"] for i in difference["inputs"]]
            for observation in difference["observations"]:
                rows.setdefault(observation["env_id"], []).append((inputs, observation, difference))
        calls = sorted({" ".join([name] + inputs) for items in rows.values() for inputs, _, _ in items})
        answers = dict(zip(calls, dumper.calls(calls)))
        entry = state[fid]
        first = code["entries"][entry["code"]]["own"][1 if len(code["entries"][entry["code"]]["own"]) > 1 else 0]
        host = host_cpu()
        variants = []
        ranges = {env_id: environment_label(environments[env_id]) for env_id in rows}
        for env_id, items in sorted(rows.items()):
            build, cpu = ranges[env_id]
            others = sorted(ranges[e][0] + ", " + ranges[e][1] for e in rows if e != env_id)
            matched = sum(answers[" ".join([name] + inputs)] == o["result"]["bits"] for inputs, o, _ in items)
            distance = max(ulps(o["result"]["bits"], other["result"]["bits"]) for _, o, d in items for other in d["observations"] if other is not o)
            evidence = {}
            for _, o, d in items:
                source = o["evidence"]
                evidence[source["path"]] = {k: source.get(k) for k in ("repository", "commit", "path")} | {"locator": ",".join(sorted({x["difference_id"] for _, _, x in items})), "note": "not rechecked" if d["recheck"]["status"] == "not-rechecked" else d["recheck"]["status"]}
            reproduced = matched == len(items)
            vendor = environments[env_id]["cpu_vendor"]
            same_vendor = vendor.lower() in host.lower()
            variants.append({
                "kind": "environment",
                "name": f"{name} on Excel {build}, {cpu}",
                "key": {"excel_build_range": build, "cpu": cpu, "env_id": env_id},
                "difference": f"On {len(items)} recorded inputs this environment's answer is {distance} ULP from the answer recorded in {'; '.join(others)}. CPU, Excel build and operating system all differ between the two machines, so the cause is not isolated.",
                "state": ("matches" if reproduced else "differs") if same_vendor else "not-run",
                "state_summary": (f"Run on {host}: gives this answer on {matched} of {len(items)} of these inputs." if same_vendor else
                                  f"Not run on this CPU. On x86_64 the kernel executes the CPU's own x87 instructions, so its answer follows the machine; on {host} it gave this answer on {matched} of {len(items)}."),
                "evaluated_on": host,
                "code": github(first),
                "evidence": list(evidence.values()),
                "error_bound": None,
                "onecalc_udf": None,
            })
        result[fid] = variants
    for fid, variants in SERIES.get("variants", {}).items():
        for variant in variants:
            entry = state[fid]
            own = code["entries"][entry["code"]]["own"]
            result.setdefault(fid, []).append({**variant, "kind": "environment", "code": github(own[0]), "error_bound": None, "onecalc_udf": None})
    return result


def split_pair(name):
    parts = [p.strip(" *") for p in re.split(r"\s*[,،、，;]\s*|\s+و\s*|\s+(?:and|en|und|et|y|e|i|и|ve|và|ja|och|og|in|si|și|és)\s+", name) if p.strip(" *")]
    return parts if len(parts) == 2 else None


def localize(functions):
    """Fills what the harvest holds but the old projection left out: split byte-function pairs,
    and the shared list of the IS family, which the source does not assign name by name."""
    raw = json.loads((DATA / "localization/microsoft-support-function-index.json").read_text(encoding="utf8"))["rows"]
    by_guid = {}
    for row in raw:
        by_guid.setdefault(row["function_article_guid"], []).append(row)
    for f in functions.values():
        info = f.get("localization") or {}
        rows = by_guid.get(info.get("function_article_guid"), [])
        if info.get("status") == "combined_row" and not f["localized_names"]:
            english = next((split_pair(r["localized_name"]) for r in rows if r["locale_tag"] == "en-US"), None)
            if not english or f["surface_name"] not in english:
                continue
            at = english.index(f["surface_name"])
            names = []
            for row in rows:
                pair = split_pair(row["localized_name"])
                if pair:
                    names.append({"locale": row["locale_tag"], "name": pair[at]})
            f["localized_names"] = names
            info["resolved_name_count"] = len(names)
            info["status"] = "split_combined_row"
        if info.get("status") == "unresolved_shared_guid":
            family = {}
            for row in rows:
                family.setdefault(row["locale_tag"], []).append(row["localized_name"].rstrip(" *"))
            f["localized_family_names"] = [{"locale": k, "names": v} for k, v in sorted(family.items())]


def signature_of(target, previous):
    described = {p["name"]: p.get("description") for p in ((previous or {}).get("parameters") or [])}
    return {
        "display": target["display"],
        "trailing_repeats": any(p["repeats"] for p in target["parameters"]),
        "parameters": [{"name": p["name"], "optional": p["optional"], "repeats": p["repeats"],
                        "description": described.get(p["name"])} for p in target["parameters"]],
    }


def parse_signature(display):
    inner = display[display.index("(") + 1:display.rindex(")")]
    names = [p.strip() for p in inner.split(",") if p.strip()]
    repeats = bool(names) and names[-1] in ("...", "…")
    if repeats:
        names = names[:-1]
    parameters = []
    for index, name in enumerate(names):
        optional = name.startswith("[")
        parameters.append({"name": name.strip("[] "), "optional": optional,
                           "repeats": repeats and index == len(names) - 1, "description": None})
    return {"display": display, "trailing_repeats": repeats, "parameters": parameters}


def blank_entry(name, spec):
    fid = f"FUNC.{name}"
    return {
        "function_id": fid, "surface_name": name, "published_row_id": fid, "entry_kind": "built_in_function",
        "category": spec["category"], "admission": {"label": "", "reason": None},
        "metadata_status": "added_from_catalogue_or_public_source", "xlcall": None,
        "arity": {"min": None, "max": None}, "classification": {}, "axes": None, "axis_provenance": {},
        "signature": None, "signature_placeholder": False,
        "descriptions": {"short": spec["short"], "short_source": "dna-calc-own", "long": None},
        "docs": {"microsoft_url": spec.get("source"), "english_description": None} if spec.get("source") else None,
        "localized_names": [], "localization": None, "artifacts": None, "version_marker": None,
        "special_interface_kind": None, "admission_interface_kind": None, "registry_backed": False,
        "vintage": {},
    }


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dnacalc", required=True)
    parser.add_argument("--pin")
    parser.add_argument("--corpus-run")
    args = parser.parse_args()
    root = Path(args.dnacalc).resolve()
    pin_path = OUT / "pin.json"
    pin = args.pin or json.loads(pin_path.read_text())["commit"]
    if git(root, "rev-parse", "HEAD") != pin or git(root, "status", "--porcelain", "--", "crates", "docs"):
        raise SystemExit(f"{root} must be a clean checkout of the pin {pin}")

    dumper = Dumper(root)
    everything = dumper.catalogue()
    # Entries that hold a kernel (the add-in's excepted) carry code; the rest are declarations.
    dump = [t for t in everything if t["implementation"]["state"] == "held" and t["kind"]["type"] in ("worksheet", "operator", "host_route")]
    declared = [t for t in everything if t not in dump]
    by_call = {t["spelling"]: t for t in dump if t["notation"] == "call"}
    by_operator = {(t["spelling"], t["notation"]): t for t in dump if t["notation"] != "call"}
    # Declarations by the name a workbook uses: an add-in entry by its export name.
    by_declared = {(t["kind"]["export"] if t["kind"]["type"] == "addin" else t["spelling"]): t
                   for t in declared if t["notation"] == "call" and t["kind"]["type"] != "binding"}
    index = rustindex.Index(root / "crates/calc-func/src")
    formula_index = rustindex.Index(root / "crates/calc-fml/src", "crates/calc-fml/src/")
    formula_roots = {name: spec["code"] for name, spec in SERIES["entries"].items() if spec.get("code")}
    code = extract_code(index, dump, formula_index, formula_roots)

    problems = json.loads(show(root, pin, "docs/generated/problem-functions.json"))
    bank = read_bank(root, pin)
    tally_path = OUT / "corpus-tally.json"
    if args.corpus_run:
        counted = corpus_tally.tally(args.corpus_run)
        counted["commit"] = pin
        write_json(tally_path, counted)
    counted = json.loads(tally_path.read_text())
    if counted["commit"] != pin:
        raise SystemExit("the corpus tally names another commit; run with --corpus-run")
    failing = {r["function"]: r for r in problems["functions"]}
    corpus_rows = counted["functions"]
    for name, row in failing.items():
        c = corpus_rows.get(name, {})
        if (row["corpus"]["larger_numeric"], row["corpus"]["last_bit"], row["corpus"]["other_mismatch"]) != \
                (c.get("larger_numeric", 0), c.get("last_bit", 0), c.get("other_mismatch", 0)):
            raise SystemExit(f"corpus tally disagrees with the problem list on {name}")
    measured_bank = {}
    for name, row in bank.items():
        detail = failing.get(name, {"bank": {}})["bank"]
        merged = {k: detail.get(k, 0) for k in ("larger_numeric", "last_bit", "other_mismatch", "refused", "unread")}
        rows, matched = int(row["rows"]), int(row["matched"])
        if rows == matched and any(merged.values()):
            # The status table is the count of record. A problem list generated before the rows
            # were re-recorded can still list failures for a function whose rows now all match.
            print(f"note: problem-functions.json lists bank failures for {name} but witness-status.tsv matches all {rows} rows; using the status table", file=sys.stderr)
            merged = dict.fromkeys(merged, 0)
        short = rows - matched - sum(merged.values())
        if short > 0 and any(merged.values()) or short > 0 and rows != matched:
            # The problem list was generated before the rows were re-recorded and does not explain
            # every mismatch the status table counts (or lists none). The status table is the count
            # of record; the unexplained remainder is reported as a difference in kind or value.
            print(f"note: witness-status.tsv counts {short} more mismatches for {name} than problem-functions.json lists; counted as other_mismatch", file=sys.stderr)
            merged["other_mismatch"] += short
        if rows - matched != sum(merged.values()):
            raise SystemExit(f"bank counts do not reconcile for {name}")
        ulp = 0 if row["worst_ulp"] == "-" else int(row["worst_ulp"])
        measured_bank[name] = dict(rows=rows, matched=matched, worst_ulp=ulp, excel_builds=row["excel_builds"].split(","), **merged)
    conflicts = Counter(r["function"] for r in csv.DictReader(
        io.StringIO(show(root, pin, "crates/calc-func/witness-differences.tsv")), delimiter="\t"))

    functions = {}
    for path in sorted((DATA / "functions").glob("FUNC.*.json")):
        f = json.loads(path.read_text(encoding="utf8"))
        functions[f["function_id"]] = f
    for name, spec in SERIES["added"].items():
        functions.setdefault(f"FUNC.{name}", blank_entry(name, spec))
    localize(functions)
    descriptions_path = REPO / "content/descriptions/dna-calc-own.json"
    descriptions = json.loads(descriptions_path.read_text(encoding="utf8"))
    for name, spec in SERIES["added"].items():
        fid = f"FUNC.{name}"
        descriptions["descriptions"].setdefault(fid, spec["short"])
        reference = REPO / f"content/reference/{fid}.json"
        if not reference.exists():
            try_it = {"formula": spec["try_it"]}
            if spec.get("kind"):
                try_it.update(preview_status="worksheet-error", note="The OneCalc preview has no answer for this function: see the state above.")
            write_json(reference, {"schema": "efh.reference/v1", "function_id": fid, "try_it": try_it,
                                   "versions": [], "implementations": [], "variants": []})
    descriptions["descriptions"] = dict(sorted(descriptions["descriptions"].items()))
    write_json(descriptions_path, descriptions)

    state = {}
    counts = Counter()
    for fid, f in sorted(functions.items()):
        name = f["surface_name"]
        spec = {**SERIES["entries"].get(name, {}), **SERIES["added"].get(name, {})}
        target = None
        if f["entry_kind"] == "built_in_operator":
            key = OPERATORS.get(name)
            target = by_operator.get(key) if key else None
            corpus = None
            if key and key[1] == "infix":
                merged = Counter()
                for label in (f"Infix {key[0]}", key[0]):
                    merged.update(corpus_rows.get(label, {}))
                worst = max((corpus_rows.get(l, {}).get("worst_ulp", 0) for l in (f"Infix {key[0]}", key[0])), default=0)
                corpus = dict(merged, worst_ulp=worst) if merged else None
            else:
                # Prefix, postfix and reference operators: uses read from the parsed formulas.
                corpus = counted.get("operators", {}).get(name)
            group = {"kind": "implemented"}
            code_key = f"{key[0]} {key[1]}" if key else None
            if not target:
                group = {"kind": "formula-layer", "reason": spec.get("reason") or "Written in the formula language itself and handled by the formula layer, not by a catalogue row."}
                code_key = name if name in code["entries"] else None
            measured = None
        else:
            target = by_call.get(name) or by_declared.get(name)
            corpus = corpus_rows.get(name)
            measured = measured_bank.get(name)
            code_key = name if name in code["entries"] else None
            # A catalogue row at the pin always wins, so a function that lands changes state alone.
            # Its own declaration (deferred, not held, host route) decides those states.
            group = declared_group(target, spec) if target else None
            if group:
                pass
            elif target:
                group = {"kind": "implemented", "reason": spec.get("reason")}
            elif spec.get("kind"):
                group = {"kind": spec["kind"], "summary": spec.get("summary"), "reason": spec.get("reason")}
            else:
                raise SystemExit(f"{name}: neither in the catalogue nor listed in non-catalogue.json")
        label, summary, detail = state_of(name, group, measured, corpus, conflicts.get(name, 0))
        counts[label] += 1
        if spec.get("note"):
            detail = f"{detail} {spec['note']}".strip()
        state[fid] = dict(state=label, state_label=STATE_LABELS[label], summary=summary, detail=detail,
                          bank=measured, corpus=corpus, in_catalogue=bool(target), code=code_key)
        # Signature, arity and admission come from the commit, never from an older registry.
        if target:
            f["signature"] = signature_of(target, f.get("signature"))
            f["arity"] = {"min": target["min"], "max": target["max"]}
        elif spec.get("signature"):
            f["signature"] = parse_signature(spec["signature"])
            required = sum(1 for p in f["signature"]["parameters"] if not p["optional"])
            f["arity"] = {"min": required, "max": spec.get("max_arity", len(f["signature"]["parameters"]))}
        if spec.get("operator_signature"):
            f["signature"] = {"display": spec["operator_signature"], "trailing_repeats": False, "parameters": (f.get("signature") or {}).get("parameters") or []}
        f["signature_placeholder"] = False
        f["registry_backed"] = bool(target)
        f["admission"] = {
            "label": {"matches": "implemented", "close": "implemented", "differs": "implemented", "unmeasured": "implemented",
                      "refuses": "not_modelled" if target else "not_implemented"}.get(label, group["kind"]),
            "reason": group.get("reason") if label in ("refuses", "deferred", "host", "absent") else None,
        }
        if group["kind"] == "not_in_excel":
            f["admission"]["label"] = "absent"
        if group["kind"] == "formula-layer":
            f["admission"]["label"] = "formula_layer"
        f.pop("oxfunc_tree_clean", None)
        vintage = f.get("vintage") or {}
        vintage.pop("oxfunc_tree_clean", None)
        source = f"calcfunc-catalogue:{pin[:12]}"
        for field in ("arity", "signature", "signature_placeholder", "admission", "registry_backed"):
            vintage[field] = source
        f["vintage"] = dict(sorted(vintage.items()))
        write_json(DATA / "functions" / f"{fid}.json", f)

    missing = [fid for fid, f in functions.items() if not (f.get("signature") or {}).get("display")]
    if missing:
        raise SystemExit(f"entries without a signature: {missing}")

    index_doc = json.loads((DATA / "index.json").read_text(encoding="utf8"))
    rows = {r["function_id"]: r for r in index_doc["rows"]}
    for fid, f in functions.items():
        rows[fid] = {"function_id": fid, "surface_name": f["surface_name"], "entry_kind": f["entry_kind"],
                     "category": f["category"], "admission": f["admission"]["label"]}
    index_doc["rows"] = [rows[k] for k in sorted(rows)]
    c = index_doc["counts"]
    c.update(total_entries=len(rows), added_entries=len(SERIES["added"]),
             functions=sum(f["entry_kind"] == "built_in_function" for f in functions.values()),
             registry_backed=sum(bool(f["registry_backed"]) for f in functions.values()),
             export_only=sum(not f["registry_backed"] for f in functions.values()),
             deferred=0, placeholder_signatures=0)
    provenance = index_doc["provenance"]
    provenance.pop("oxfunc_tree_clean", None)
    provenance["calcfunc_commit"] = pin
    index_doc["vintage_states"] = {k: {x: y for x, y in v.items()} for k, v in index_doc["vintage_states"].items()}
    write_json(DATA / "index.json", index_doc)

    fragments = code.pop("fragments")
    github = lambda key: f"{GITHUB}/blob/{pin}/{fragments[key]['file']}#L{fragments[key]['start_line']}-L{fragments[key]['end_line']}"
    write_json(OUT / "variants.json", environment_variants(dumper, state, code, fragments, github, pin))
    report_commit = git(root, "log", "-1", "--format=%H", pin, "--", "docs/generated/problem-functions.json")
    unchanged = git(root, "diff", "--stat", report_commit, pin, "--", "crates/calc-func/src") == ""
    write_json(OUT / "code.json", dict(commit=pin, repository=GITHUB, entries=code["entries"]))
    write_json(OUT / "fragments.json", fragments)
    write_json(OUT / "state.json", dict(commit=pin, counts=dict(sorted(counts.items())), functions=state))
    write_json(pin_path, dict(
        repository=GITHUB, commit=pin, commit_date=git(root, "show", "-s", "--format=%cs", pin),
        measured_application_revision=problems["application"]["revision"],
        report_commit=report_commit, calc_func_unchanged_since_report=unchanged,
        answer_bank=problems["bank"], corpus=problems["corpus"], rule=problems["rule"],
        sources={p: hashlib.sha256(show(root, pin, p).encode("utf8")).hexdigest()
                 for p in ("docs/generated/problem-functions.json", "docs/generated/numeric-fingerprints.json",
                           "crates/calc-func/witness-status.tsv", "crates/calc-func/witness-differences.tsv",
                           "crates/calc-fml/src/eval/builtins.rs")},
    ))
    print(json.dumps(dict(counts=dict(counts), entries=len(rows), catalogue=len(dump), declared=len(declared),
                          fragments=len(fragments), code_bytes=sum(len(v["text"]) for v in fragments.values())), indent=1))


if __name__ == "__main__":
    main()
