"""Per-function tally of one `cargo xtask corpus --oracle --no-capture` run.

Follows the labelling of DnaCalc-Research racer/corpus_cases.py (formula_label, classify) so its
failure counts reconcile with docs/generated/problem-functions.json, and adds what that report
leaves out: the matched cells, so each function has a denominator.
"""
import json, re, struct, math, sys
from collections import Counter
from pathlib import Path

STRIP = re.compile(r'^(?:_XLFN\.|_XLWS\.|_XLETA\.|_XLUDF\.)+')
CALL = re.compile(r'([A-Za-z_][A-Za-z0-9_.]*)\s*\(')


def workbook_directory(corpus, workbook, version):
    label = re.sub(r'[^A-Za-z0-9_.-]', '_', workbook['name'])
    if version >= 2:
        return Path(corpus) / 'workbooks' / (label[:96] + '-' + workbook['sha256'])
    return Path(corpus) / 'workbooks' / label


def formula_label(formula, formulas, cell):
    seen = {cell}
    while True:
        body = formula.strip().removeprefix('=')
        reference = re.fullmatch(r"(?:(?:'((?:[^']|'')+)'|([A-Za-z_][A-Za-z0-9_.]*))!)?\$?([A-Za-z]{1,3})\$?([1-9][0-9]*)", body)
        if reference is None:
            break
        quoted, sheet, column, row = reference.groups()
        sheet = quoted.replace("''", "'") if quoted is not None else sheet or cell[0]
        index = 0
        for letter in column.upper():
            index = index * 26 + ord(letter) - ord('A') + 1
        target = (sheet, int(row), index)
        if target in seen:
            return 'Cell reference (cycle)'
        target_formula = formulas.get(target, {}).get('formula')
        if not isinstance(target_formula, str) or not target_formula.strip():
            return 'Cell reference (unresolved)'
        seen.add(target)
        cell, formula = target, target_formula
    tokens = re.sub(r'"(?:[^"]|"")*"', '', body)
    names = re.findall(r'([A-Za-z][A-Za-z0-9_.]*)\s*\(', tokens.upper())
    if names:
        return STRIP.sub('', names[1] if len(names) > 1 and names[0] == 'INDEX' else names[0])
    operator = re.search(r'<>|<=|>=|[<>=&+*/^%\-]', tokens)
    return 'Infix ' + operator.group() if operator else 'Formula without function'


def ordered(bits):
    n = int(bits, 16)
    return (~n & ((1 << 64) - 1)) if n >> 63 else n | (1 << 63)


def number(bits):
    return struct.unpack('>d', bytes.fromhex(bits))[0]


def tally(corpus):
    corpus = Path(corpus)
    summary = json.loads((corpus / 'summary.json').read_text())
    version = summary.get('version', 1)
    key = lambda r: (r['sheet'], r['row'], r['column'])
    functions = {}
    nested = Counter()
    pending = 0
    for workbook in summary['workbooks']:
        directory = workbook_directory(corpus, workbook, version)
        captures = sorted(directory.glob('*.capture.json'))
        if captures:
            runs = [(c, directory / c.name.removesuffix('.capture.json')) for c in captures]
        elif workbook['capture'].startswith('committed capture '):
            runs = [(Path(workbook['capture'].removeprefix('committed capture ')), directory)]
        else:
            pending += workbook['counts']['formula_cells']
            continue
        for capture_path, run in runs:
            capture = json.loads(Path(capture_path).read_text())
            triage = json.loads((run / 'triage.json').read_text())
            evaluations = json.loads((run / 'evaluations.json').read_text())
            expected = {key(r): r for r in capture['observations']}
            formulas = {key(r): r for r in capture['formulas']}
            actual = {key(r): r for r in evaluations['evaluations']}
            for verdict in triage['cells']:
                k = key(verdict)
                formula = formulas.get(k, {}).get('formula', '')
                label = formula_label(formula, formulas, k)
                state = verdict['verdict']
                if verdict.get('class') == 'not-modelled':
                    label = STRIP.sub('', verdict['key'])
                row = functions.setdefault(label, Counter())
                row['cells'] += 1
                if state == 'matched' and ('external_links' in verdict or 'circular_rule' in verdict):
                    # Taken from another workbook's link cache or a held circular value:
                    # the summary counts these outside its matched total, and so does this.
                    row['pending'] += 1
                elif state == 'matched':
                    row['matched'] += 1
                    tokens = re.sub(r'"(?:[^"]|"")*"', '', formula).upper()
                    for name in set(STRIP.sub('', n) for n in CALL.findall(tokens)):
                        if name != label:
                            nested[name] += 1
                elif state in ('external_link', 'circular_rule', 'awaiting_capture'):
                    row['pending'] += 1
                elif state == 'mismatched':
                    e, a = expected.get(k), actual.get(k)
                    x = e.get('excel_raw') if e else None
                    y = a.get('returned', {}).get('value') if a else None
                    if verdict.get('class') != 'capture' and isinstance(x, dict) and isinstance(y, dict) and set(x) == set(y) == {'number'} \
                            and math.isfinite(number(x['number'])) and math.isfinite(number(y['number'])):
                        distance = abs(ordered(x['number']) - ordered(y['number']))
                        if distance <= 16:
                            row['last_bit'] += 1
                            row['worst_ulp'] = max(row['worst_ulp'], distance)
                        else:
                            row['larger_numeric'] += 1
                    else:
                        row['other_mismatch'] += 1
                else:
                    row['refused'] += 1
    return {'functions': {k: dict(v) for k, v in sorted(functions.items())},
            'nested_matched': dict(sorted(nested.items())),
            'wholly_pending_cells': pending,
            'totals': summary['totals']['counts']}


if __name__ == '__main__':
    json.dump(tally(sys.argv[1]), open(sys.argv[2], 'w'), indent=1, sort_keys=True)
