"""Reads RuleTester cases out of a JS / TS rule test file without running it: each object literal with a `code`
and an `output` that are plain strings (literals, `+` of literals, templates without `${}`, `unIndent`,
`noFormat` or `String.raw` tagged) is a case; one whose options name a `parser` is flagged."""

import re

PUNCT_BEFORE_REGEX = set("(,=:[!&|?{};+-*%<>~^")


def tokens(src):
    """(kind, value) tokens: str, tpl (raw template text, `${` inside marked), id, punct; comments dropped."""
    i, n, out = 0, len(src), []
    while i < n:
        c = src[i]
        if c.isspace():
            i += 1
        elif src.startswith("//", i):
            j = src.find("\n", i)
            i = n if j < 0 else j
        elif src.startswith("/*", i):
            j = src.find("*/", i + 2)
            i = n if j < 0 else j + 2
        elif c in "\"'":
            j = i + 1
            while j < n and src[j] != c:
                j += 2 if src[j] == "\\" else 1
            out.append(("str", src[i + 1:j]))
            i = j + 1
        elif c == "`":
            j, depth = i + 1, 0
            while j < n:
                if src[j] == "\\":
                    j += 2
                    continue
                if src.startswith("${", j):
                    depth += 1
                    j += 2
                    continue
                if depth and src[j] == "}":
                    depth -= 1
                elif not depth and src[j] == "`":
                    break
                j += 1
            out.append(("tpl", src[i + 1:j]))
            i = j + 1
        elif c == "/" and (not out or out[-1][0] == "punct" and out[-1][1] in PUNCT_BEFORE_REGEX
                           or out[-1] == ("id", "return")):
            j, cls = i + 1, False
            while j < n and src[j] != "\n":
                if src[j] == "\\":
                    j += 2
                    continue
                if src[j] == "[":
                    cls = True
                elif src[j] == "]":
                    cls = False
                elif src[j] == "/" and not cls:
                    break
                j += 1
            m = re.match(r"[a-z]*", src[j + 1:])
            out.append(("regex", src[i:j + 1]))
            i = j + 1 + len(m.group(0))
        elif c.isalnum() or c in "_$":
            m = re.match(r"[\w$]+", src[i:])
            out.append(("id", m.group(0)))
            i += len(m.group(0))
        else:
            out.append(("punct", c))
            i += 1
    return out


ESC = {"n": "\n", "t": "\t", "r": "\r", "b": "\b", "f": "\f", "v": "\v", "0": "\0"}


def cook(s):
    """A JS string literal's or a template's text, its escapes decoded."""
    def esc(m):
        e = m.group(1)
        if e[0] == "u":
            return chr(int(e[2:-1] if e[1] == "{" else e[1:], 16))
        if e[0] == "x":
            return chr(int(e[1:], 16))
        if e[0] == "\n":
            return ""
        return ESC.get(e, e)
    return re.sub(r"\\(u\{[0-9a-fA-F]+\}|u[0-9a-fA-F]{4}|x[0-9a-fA-F]{2}|\n|.)", esc, s, flags=re.S)


def unindent(text):
    """ESLint's `unIndent`: the first newline and the last blank line dropped, the least indentation removed."""
    lines = re.sub(r"\n\s*$", "", re.sub(r"^\n", "", text)).split("\n")
    ind = [len(re.match(r" *", ln).group(0)) for ln in lines if ln.strip()]
    m = min(ind) if ind else 0
    return "\n".join(ln[m:] for ln in lines)


def string_expr(toks, k):
    """A plain string expression at token k: (its value, the token after it), or (None, k)."""
    parts = []
    while True:
        tag = None
        if k + 1 < len(toks) and toks[k][0] == "id" and toks[k + 1][0] == "tpl":
            tag, k = toks[k][1], k + 1
        elif (k + 3 < len(toks) and toks[k] == ("id", "String") and toks[k + 1] == ("punct", ".")
              and toks[k + 2] == ("id", "raw") and toks[k + 3][0] == "tpl"):
            tag, k = "raw", k + 3
        if k >= len(toks):
            return None, k
        kind, val = toks[k]
        if kind == "str":
            parts.append(cook(val))
        elif kind == "tpl" and "${" not in val:
            if tag == "raw":
                parts.append(val)
            elif tag in (None, "noFormat", "dedent") or tag == "unIndent":
                text = cook(val)
                parts.append(unindent(text) if tag in ("unIndent", "dedent") else text)
            else:
                return None, k
        else:
            return None, k
        k += 1
        if k < len(toks) and toks[k] == ("punct", "+"):
            k += 1
            continue
        if k < len(toks) and toks[k] == ("punct", "."):
            return None, k  # a method on it: not plain
        return "".join(parts), k


def cases(src):
    """[{code, output, parser, jsx, line}] of a test file's source."""
    toks = tokens(src)
    out, stack = [], []  # stack: (bracket, the object's fields or None)
    k = 0
    while k < len(toks):
        kind, val = toks[k]
        if kind == "punct" and val in "{[(":
            stack.append((val, {"parser": False, "jsx": False, "k": k} if val == "{" else None))
        elif kind == "punct" and val in "}])":
            if stack:
                b, obj = stack.pop()
                if b == "{" and obj is not None:
                    if isinstance(obj.get("code"), str) and isinstance(obj.get("output"), str) \
                            and obj["code"] != obj["output"]:
                        out.append({"code": obj["code"], "output": obj["output"], "parser": obj["parser"],
                                    "jsx": obj["jsx"], "tok": obj["k"]})
                    parent = next((o for bb, o in reversed(stack) if bb == "{"), None)
                    if parent is not None:
                        parent["parser"] |= obj["parser"]
                        parent["jsx"] |= obj["jsx"]
        elif (kind in ("id", "str") and k + 1 < len(toks) and toks[k + 1] == ("punct", ":")
              and stack and stack[-1][0] == "{"):
            obj = stack[-1][1]
            if val in ("code", "output"):
                v, nk = string_expr(toks, k + 2)
                obj[val] = v
                if v is not None:
                    k = nk
                    continue
            elif val == "parser":
                obj["parser"] = True
            elif val == "jsx" and k + 2 < len(toks) and toks[k + 2] == ("id", "true"):
                obj["jsx"] = True
        k += 1
    return out
