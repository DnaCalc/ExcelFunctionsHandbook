"""A small index of the Rust source of calc-func: functions, their spans and what they call.

Not a Rust parser. It blanks comments and string literals, then finds `fn` items, `impl` owners
and `use` imports by text. Call edges resolve to functions by name, path and import, so a miss
drops an edge; it never invents code. Every fragment keeps its file and line span.
"""
import re
from pathlib import Path

PLUMBING = {"catalogue/kit", "kernel", "coerce", "call", "lib", "catalogue", "help", "value"}


def mask(text):
    """The text with comments, strings and char literals replaced by spaces, offsets kept."""
    out = list(text)
    i, n = 0, len(text)

    def blank(a, b):
        for j in range(a, b):
            if out[j] != "\n":
                out[j] = " "

    while i < n:
        c = text[i]
        if text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            blank(i, j)
            i = j
        elif text.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j):
                    depth += 1
                    j += 2
                elif text.startswith("*/", j):
                    depth -= 1
                    j += 2
                else:
                    j += 1
            blank(i, j)
            i = j
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            blank(i + 1, j)
            i = j + 1
        elif c == "r" and re.match(r'r#*"', text[i:i + 8]) and not (i and (text[i - 1].isalnum() or text[i - 1] == "_")):
            hashes = len(re.match(r'r(#*)"', text[i:i + 8]).group(1))
            closing = '"' + "#" * hashes
            j = text.find(closing, i + 2 + hashes)
            j = n if j < 0 else j
            blank(i + 2 + hashes, j)
            i = j + len(closing)
        elif c == "b" and text.startswith('b"', i) and not (i and (text[i - 1].isalnum() or text[i - 1] == "_")):
            i += 1
        elif c == "'":
            m = re.match(r"'(\\u\{[0-9a-fA-F]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'])'", text[i:i + 12])
            if m:
                blank(i + 1, i + len(m.group(0)) - 1)
                i += len(m.group(0))
            else:
                i += 1
        else:
            i += 1
    return "".join(out)


def matching(code, start, opening="{", closing="}"):
    """The offset just past the bracket that closes the one at `start`."""
    depth = 0
    for i in range(start, len(code)):
        if code[i] == opening:
            depth += 1
        elif code[i] == closing:
            depth -= 1
            if depth == 0:
                return i + 1
    return len(code)


class Item:
    __slots__ = ("key", "module", "file", "name", "owner", "start", "end", "start_line", "end_line", "text", "code")

    def __init__(self, **values):
        for k, v in values.items():
            setattr(self, k, v)


class Index:
    def __init__(self, src, prefix="crates/calc-func/src/"):
        self.prefix = prefix
        self.files = {}
        self.items = {}
        self.by_name = {}
        self.imports = {}
        self.modules = {}
        for path in sorted(Path(src).rglob("*.rs")):
            relative = path.relative_to(src).as_posix()
            self._file(relative, path.read_text(encoding="utf8"))

    def _file(self, relative, text):
        module = relative[:-3]
        if module.endswith("/mod"):
            module = module[:-4]
        code = mask(text)
        self.files[relative] = (text, code)
        self.modules.setdefault(module.rsplit("/", 1)[-1], []).append(module)
        tests = []
        for m in re.finditer(r"#\[cfg\(test\)\]\s*(?:pub(?:\([a-z]+\))?\s+)?mod\s+\w+\s*\{", code):
            tests.append((m.start(), matching(code, m.end() - 1)))
        impls = []
        for m in re.finditer(r"^[ \t]*(?:unsafe\s+)?impl\b([^{;]*)\{", code, re.M):
            head = m.group(1)
            head = head.split(" for ")[-1]
            name = re.findall(r"([A-Za-z_]\w*)", re.sub(r"<[^{]*>", " ", head))
            impls.append((m.end() - 1, matching(code, m.end() - 1), name[0] if name else ""))
        self.imports[module] = self._imports(code)
        position = 0
        for m in re.finditer(r"\bfn\s+([A-Za-z_]\w*)", code):
            if m.start() < position or any(a <= m.start() < b for a, b in tests):
                continue
            bracket = self._body(code, m.end())
            if bracket is None:
                continue
            end = matching(code, bracket)
            line_start = text.rfind("\n", 0, m.start()) + 1
            start = self._leading(text, line_start)
            owner = next((name for a, b, name in impls if a < m.start() < b), "")
            key = f"{module}::{owner + '::' if owner else ''}{m.group(1)}"
            if key in self.items:
                key = f"{key}@{text.count(chr(10), 0, line_start) + 1}"
            item = Item(key=key, module=module, file=relative, name=m.group(1), owner=owner, start=start, end=end,
                        start_line=text.count("\n", 0, start) + 1, end_line=text.count("\n", 0, end) + 1,
                        text=text[start:end], code=code[start:end])
            self.items[key] = item
            self.by_name.setdefault(m.group(1), []).append(item)
            position = end

    @staticmethod
    def _body(code, at):
        depth = 0
        for i in range(at, len(code)):
            c = code[i]
            if c in "([<" and c != "<":
                depth += 1
            elif c in ")]":
                depth -= 1
            elif c == "{" and depth == 0:
                return i
            elif c == ";" and depth == 0:
                return None
        return None

    @staticmethod
    def _leading(text, start):
        """Back over attribute and doc-comment lines directly above an item."""
        while start > 0:
            previous = text.rfind("\n", 0, start - 1) + 1
            line = text[previous:start - 1].strip()
            if line.startswith(("///", "#[", "//!")) and not line.startswith("//!"):
                start = previous
            else:
                break
        return start

    @staticmethod
    def _imports(code):
        found = {}

        def walk(prefix, tree):
            parts, depth, current = [], 0, ""
            for c in tree:
                if c == "{":
                    depth += 1
                elif c == "}":
                    depth -= 1
                if c == "," and depth == 0:
                    parts.append(current)
                    current = ""
                else:
                    current += c
            parts.append(current)
            for part in parts:
                part = part.strip()
                if not part:
                    continue
                if "{" in part:
                    head, rest = part.split("{", 1)
                    walk(prefix + [s for s in head.strip().rstrip(":").split("::") if s], rest.rsplit("}", 1)[0])
                else:
                    alias = None
                    if " as " in part:
                        part, alias = [x.strip() for x in part.split(" as ", 1)]
                    segments = prefix + [s for s in part.split("::") if s]
                    if segments[-1] == "self":
                        segments = segments[:-1]
                    found[alias or segments[-1]] = segments

        for m in re.finditer(r"^\s*(?:pub(?:\([a-z]+\))?\s+)?use\s+([^;]+);", code, re.M):
            walk([], re.sub(r"\s+", " ", m.group(1)))
        return found

    def resolve_module(self, segment, module):
        """The module a path segment names, seen from `module`."""
        imported = self.imports.get(module, {}).get(segment)
        if imported:
            segment = imported[-1]
        if segment in ("self", "super", "crate"):
            return None
        names = self.modules.get(segment)
        if not names:
            return None
        for preferred in ("ported/", "catalogue/", ""):
            for name in names:
                if name.startswith(preferred):
                    return name
        return names[0]

    def calls(self, code, module):
        """The items a piece of masked code refers to."""
        found = []
        seen = set()

        def add(item):
            if item is not None and item.key not in seen:
                seen.add(item.key)
                found.append(item)

        for m in re.finditer(r"((?:[A-Za-z_]\w*::)*)([a-z_][a-z0-9_]*)\b(\s*(?:::<[^>]*>)?\s*\()?", code):
            path, name, call = m.groups()
            candidates = self.by_name.get(name)
            if not candidates:
                continue
            if m.start() and code[m.start() - 1] == ".":
                local = [i for i in candidates if i.module == module]
                if len(local) == 1 and call:
                    add(local[0])
                continue
            segments = [s for s in path.split("::") if s]
            if segments:
                last = segments[-1]
                owners = [i for i in candidates if i.owner == last]
                if owners:
                    in_module = [i for i in owners if i.module == module]
                    add((in_module or owners)[0])
                    continue
                target = self.resolve_module(last, module)
                add(next((i for i in candidates if i.module == target and not i.owner), None))
                continue
            local = next((i for i in candidates if i.module == module and not i.owner), None)
            if local:
                add(local)
                continue
            imported = self.imports.get(module, {}).get(name)
            if imported and len(imported) >= 2:
                target = self.resolve_module(imported[-2], module)
                add(next((i for i in candidates if i.module == target and not i.owner), None))
            elif call:
                continue
        return found

    def closure(self, roots):
        """Every item reachable from `roots`, stopping at shared plumbing."""
        reached, plumbing = {}, {}
        stack = list(roots)
        while stack:
            item = stack.pop()
            if item.key in reached:
                continue
            reached[item.key] = item
            for callee in self.calls(item.code, item.module):
                if callee.module in PLUMBING:
                    plumbing[callee.key] = callee
                else:
                    stack.append(callee)
        return reached, plumbing
