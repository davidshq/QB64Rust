"""Extract the QB64pe built-in SUB/FUNCTION table into JSON.

Source: ..\\..\\..\\QB64pe\\source\\subs_functions\\subs_functions.bas (SUB reginternal), a flat list of
`clearid ... id.<field> = <expr> ... regid` blocks. Each block becomes one record. Type expressions are decoded
to names (LONGTYPE - ISPOINTER -> "LONG"); special argument codes (-1 .. -8) are named after the comment at the
top of the source file and their uses in qb64pe.bas.

The script also lists which built-in names qb64pe.bas special-cases by name (`id2.n) = "NAME"` and similar),
because those built-ins need hand-written handling in a new compiler.

Usage: python extract_builtins.py            (writes builtins.json next to this script, prints a summary)
"""

import json
import os
import re
import sys
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
QB64PE = os.path.normpath(os.path.join(HERE, "..", "..", "..", "QB64pe", "source"))
TABLE = os.path.join(QB64PE, "subs_functions", "subs_functions.bas")
COMPILER = os.path.join(QB64PE, "qb64pe.bas")

TYPE_NAMES = {
    "LONGTYPE": "LONG", "ULONGTYPE": "_UNSIGNED LONG", "INTEGERTYPE": "INTEGER", "UINTEGERTYPE": "_UNSIGNED INTEGER",
    "INTEGER64TYPE": "_INTEGER64", "UINTEGER64TYPE": "_UNSIGNED _INTEGER64", "BYTETYPE": "_BYTE",
    "UBYTETYPE": "_UNSIGNED _BYTE", "SINGLETYPE": "SINGLE", "DOUBLETYPE": "DOUBLE", "FLOATTYPE": "_FLOAT",
    "STRINGTYPE": "STRING", "OFFSETTYPE": "_OFFSET", "UOFFSETTYPE": "_UNSIGNED _OFFSET",
}
SPECIAL_CODES = {
    -1: "any-numeric",          # cast to the C overload's type
    -2: "offset+size (largest safe block; CALL INTERRUPT)",
    -3: "offset+size (largest safe block, restricted; graphics GET/PUT)",
    -4: "offset+size (element size; file GET/PUT)",
    -5: "offset",
    -6: "size",
    -7: "_MEM of passed variable",
    -8: "_MEM helper {offset, fullsize, typeval, elementsize, lock} (evaluatetotyp 22406)",
}


def strip_comment(line):
    """Remove a trailing ' comment that is outside string literals."""
    in_str = False
    for i, c in enumerate(line):
        if c == '"':
            in_str = not in_str
        elif c == "'" and not in_str:
            return line[:i], line[i + 1:].strip()
    return line, ""


def split_statements(line):
    """Split on ':' outside string literals."""
    parts, cur, in_str = [], "", False
    for c in line:
        if c == '"':
            in_str = not in_str
        if c == ":" and not in_str:
            parts.append(cur)
            cur = ""
        else:
            cur += c
    parts.append(cur)
    return [p.strip() for p in parts if p.strip()]


def decode_type(expr):
    """Decode one type expression such as 'LONGTYPE - ISPOINTER', '-1', 'UDTTYPE + (1)', 'ISUDT + (1)'."""
    e = re.sub(r"\s+", " ", expr.strip())
    m = re.fullmatch(r"\(?\s*(-?\d+)\s*\)?", e)
    if m:
        v = int(m.group(1))
        return SPECIAL_CODES.get(v, str(v))
    if re.fullmatch(r"(UDTTYPE|ISUDT) \+ \(1\)", e):
        return "_MEM"
    m = re.fullmatch(r"([A-Z0-9]+TYPE)( - ISPOINTER)?", e)
    if m and m.group(1) in TYPE_NAMES:
        return TYPE_NAMES[m.group(1)]
    return "?" + e


def split_concat(expr):
    """Split 'MKL$(a) + MKL$(b)' into ['a', 'b'] respecting parentheses."""
    items, i = [], 0
    s = expr.strip()
    while i < len(s):
        if s.startswith("MKL$(", i):
            depth, j = 1, i + 5
            while depth:
                if s[j] == "(":
                    depth += 1
                elif s[j] == ")":
                    depth -= 1
                j += 1
            items.append(s[i + 5:j - 1])
            i = j
        elif s[i] in " +":
            i += 1
        else:
            raise ValueError("unexpected argument expression: " + expr)
    return items


def parse_string(expr):
    """Evaluate a string literal expression, allowing "a" + "b" concatenation."""
    parts = re.findall(r'"([^"]*)"', expr)
    return "".join(parts)


def parse_table():
    records, cur, anomalies = [], None, []
    with open(TABLE, encoding="latin-1") as f:
        for lineno, raw in enumerate(f, 1):
            code, comment = strip_comment(raw.rstrip("\r\n"))
            for st in split_statements(code):
                if st == "clearid":
                    if cur is not None:
                        anomalies.append({"line": cur["line"], "statement": "clearid",
                                          "note": "block not closed by regid; dropped"})
                    cur = {"line": lineno, "fields": {}, "comments": []}
                elif st == "regid":
                    if cur is None:
                        anomalies.append({"line": lineno, "statement": st, "note": "regid without clearid"})
                    else:
                        records.append(cur)
                    cur = None
                elif cur is None:
                    # Expected framing: the SUB lines and the reginternalsubfunc flag set around the table.
                    if not re.fullmatch(r"(?i)(SUB reginternal|END SUB|reginternalsubfunc\s*=\s*[01])", st):
                        anomalies.append({"line": lineno, "statement": st,
                                          "note": "statement outside a clearid/regid block (ignored)"})
                elif cur is not None:
                    m = re.fullmatch(r"id\.(\w+)\s*=\s*(.*)", st)
                    if m:
                        cur["fields"][m.group(1).lower()] = m.group(2)
                    else:
                        anomalies.append({"line": lineno, "statement": st,
                                          "note": "statement inside a clearid/regid block that is not an id field"})
                        cur["comments"].append("ANOMALY: " + st)
            if cur is not None and comment and not comment.startswith("id."):
                cur["comments"].append(comment)
    return records, anomalies


def build(rec):
    f = rec["fields"]
    out = {"name": parse_string(f["n"]), "line": rec["line"]}
    sf = int(f.get("subfunc", "0"))
    out["kind"] = {1: "function", 2: "sub"}.get(sf, str(sf))
    out["callname"] = parse_string(f.get("callname", '""'))
    if "args" in f:
        out["args"] = int(f["args"])
    if "minargs" in f:
        out["minargs"] = int(f["minargs"])
    if "arg" in f:
        a = f["arg"].strip()
        out["arg_types"] = [] if a == '""' else [decode_type(x) for x in split_concat(a)]
    if "ret" in f:
        out["ret"] = decode_type(f["ret"])
    for key in ("musthave", "mayhave", "specialformat", "hr_syntax", "secondargmustbe", "secondargcantbe"):
        if key in f:
            out[key] = parse_string(f[key])
    if "overloaded" in f:
        out["overloaded"] = f["overloaded"].strip() == "-1"
    if "dependency" in f:
        out["dependency"] = f["dependency"].strip()
    if rec["comments"]:
        out["comments"] = rec["comments"]
    # category: how the compiler treats the entry
    if out["callname"] in ("sub_stub", "func_stub"):
        out["category"] = "stub"
    elif "specialformat" in out:
        out["category"] = "special-format statement"
    elif out.get("overloaded"):
        out["category"] = "overloaded"
    else:
        out["category"] = "plain " + out["kind"]
    return out


def special_cased_names(names):
    """Built-in names that qb64pe.bas compares against by name (hand-written handling)."""
    with open(COMPILER, encoding="latin-1") as f:
        text = f.read()
    upper = {n.upper(): n for n in names}
    found = Counter()
    for m in re.finditer(r'(?:id2?\.n\)?|n\$|firstelement\$|secondelement\$|a2\$)\s*=\s*"([A-Z_$][A-Z0-9_$]*)"',
                         text, re.IGNORECASE):
        key = m.group(1).upper()
        if key in upper:
            found[upper[key]] += 1
    return found


SUPPORT = os.path.normpath(os.path.join(QB64PE, "..", "internal", "support"))
# Included into every program by the old compiler (qb64pe.bas autoIncludeManager, 1720-1790); the colour files
# only with $COLOR:0 or $COLOR:32 (`condition`).
AUTO_INCLUDES = [
    ("include/beforefirstline.bi", None),
    ("include/afterlastline.bm", None),
    ("color/color0.bi", "$COLOR:0"),
    ("color/color32.bi", "$COLOR:32"),
]


def name_of(word):
    """A declared name without its type suffix, upper case."""
    m = re.match(r"[A-Za-z_][A-Za-z0-9_.]*", word.strip())
    return m.group(0).upper() if m else None


def auto_include_names():
    """The names the auto-included BASIC files declare (m2-parser-breadth task 8.2): constants, SUBs, FUNCTIONs,
    TYPEs and shared variables, each with its kind and file. A new compiler that does not include these files yet
    reports a use of such a name "not supported yet" rather than as an error."""
    out = []
    for rel, condition in AUTO_INCLUDES:
        path = os.path.join(SUPPORT, *rel.split("/"))
        with open(path, encoding="latin-1") as f:
            lines = f.read().splitlines()
        seen = set()

        def add(name, kind):
            if name and (name, kind) not in seen:
                seen.add((name, kind))
                rec = {"name": name, "kind": kind, "file": rel}
                if condition:
                    rec["condition"] = condition
                out.append(rec)

        for raw in lines:
            code, _ = strip_comment(raw)
            for stmt in split_statements(code):
                s = stmt.strip()
                u = s.upper()
                if u.startswith("CONST "):
                    depth, cur, items = 0, "", []
                    for c in s[6:]:
                        depth += (c == "(") - (c == ")")
                        if c == "," and depth == 0:
                            items.append(cur)
                            cur = ""
                        else:
                            cur += c
                    items.append(cur)
                    for item in items:
                        add(name_of(item.split("=")[0]), "const")
                elif re.match(r"(DECLARE\s+(LIBRARY\s+)?)?(SUB|FUNCTION)\s", u) and not u.startswith("DECLARE LIBRARY"):
                    word = re.sub(r"^(DECLARE\s+)?(SUB|FUNCTION)\s+", "", s, flags=re.IGNORECASE)
                    add(name_of(word), "sub" if re.search(r"\bSUB\b", u.split()[0] + " " + u.split()[1]) else "function")
                elif u.startswith("TYPE ") and len(u.split()) == 2:
                    add(name_of(s.split()[1]), "type")
                elif re.match(r"(DIM|REDIM)\s+SHARED\s", u):
                    rest = re.sub(r"^(RE)?DIM\s+SHARED\s+", "", s, flags=re.IGNORECASE)
                    if rest.upper().startswith("AS "):
                        rest = " ".join(rest.split()[2:])
                    for item in rest.split(","):
                        add(name_of(item.split("(")[0].split(" AS ")[0]), "variable")
    return out


def main():
    records, anomalies = parse_table()
    entries = [build(r) for r in records]
    names = sorted({e["name"] for e in entries}, key=str.upper)
    special = special_cased_names(names)
    for e in entries:
        if e["name"] in special:
            e["compiler_mentions"] = special[e["name"]]
    result = {
        "source": "QB64pe/source/subs_functions/subs_functions.bas (SUB reginternal)",
        "special_argument_codes": {str(k): v for k, v in SPECIAL_CODES.items()},
        "anomalies": anomalies,
        "entries": entries,
        "auto_include_source": "QB64pe/internal/support/" + ", ".join(r for r, _ in AUTO_INCLUDES),
        "auto_include": auto_include_names(),
    }
    out_path = os.path.join(HERE, "builtins.json")
    with open(out_path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(result, f, indent=1, ensure_ascii=False)
        f.write("\n")

    print("entries:", len(entries), "distinct names (case-insensitive):", len({n.upper() for n in names}))
    print("kinds:", dict(Counter(e["kind"] for e in entries)))
    print("categories:", dict(Counter(e["category"] for e in entries)))
    print("arg types:", dict(Counter(t for e in entries for t in e.get("arg_types", [])).most_common()))
    print("return types:", dict(Counter(e.get("ret", "-") for e in entries if e["kind"] == "function").most_common()))
    print("dependencies:", dict(Counter(e.get("dependency", "-") for e in entries).most_common()))
    unknown = [(e["name"], t) for e in entries for t in e.get("arg_types", []) + [e.get("ret", "")] if t.startswith("?")]
    print("undecoded types:", unknown)
    dup = [n for n, c in Counter(e["name"].upper() for e in entries).items() if c > 1]
    print("names registered more than once:", sorted(dup))
    noret = [e["name"] for e in entries if e["kind"] == "function" and "ret" not in e]
    print("functions without id.ret:", noret)
    print("anomalies:", anomalies)
    print("special-cased by name in qb64pe.bas:", len(special))
    print("auto-include names:", dict(Counter((r["file"], r["kind"]) for r in result["auto_include"])))
    print("wrote", out_path)


if __name__ == "__main__":
    sys.exit(main())
