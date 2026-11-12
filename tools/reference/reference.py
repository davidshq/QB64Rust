"""The language reference's tool (tools/reference/README.md; OpenSpec change language-reference; study/49).

    python tools/reference/reference.py inventory              # write docs/reference/inventory.json
    python tools/reference/reference.py evidence <WORD>        # what the repository knows about one word
    python tools/reference/reference.py skeleton <WORD>        # write the word's page with its generated blocks
    python tools/reference/reference.py render [--compiler <qb64rust>]   # refresh generated blocks and files
    python tools/reference/reference.py --check [--compiler <qb64rust>]  # fail when anything is out of date
    python tools/reference/reference.py overlap [--wiki <folder>]        # compare the pages with the wiki cache

The pages are written from the repository's own sources, never from memory (study/49 §2). This tool lists the
words, gathers a word's sources, fills the parts of a page that come from data, and checks the rest.
"""

from __future__ import annotations

import argparse
import concurrent.futures
import functools
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
REF = Path("docs/reference")
ENTRIES = REF / "entries"
EXAMPLES = Path("tests/reference/examples")
AVAILABILITY = Path("tests/reference/availability.json")
INVENTORY = REF / "inventory.json"
WORDS_TOML = Path("tools/reference/words.toml")
BUILTINS = Path("tools/builtins/builtins.json")
KEYWORDS_RS = Path("crates/syntax/src/parser/keywords.rs")
WORDS_RS = Path("crates/format/src/words.rs")
GRAMMAR = Path("vscode/syntaxes/qb64rust.tmLanguage.json")
SPECS = Path("openspec/specs")
# The specs a page may cite besides `language/*`: the others are about tools and tests, not the language.
LANGUAGE_SPECS = ("compiler/pipeline", "compiler/fixes", "compiler/lints")
CALLSITE = Path("tests/callsite")
DIVERGENCES = (Path("docs/DIVERGENCES-QB64PE.md"), Path("docs/DIVERGENCES-QB45.md"))
SOMEDAY = Path("docs/SOMEDAY.md")
VERIFICATION_README = Path("verification/README.md")
# The recorded programs searched for a word, by folder (recursively for the corpus).
PROGRAM_DIRS = (
    Path("verification"),
    Path("verification/qb45"),
    Path("tests/corpus"),
    Path("tests/differential"),
    Path("tests/upstream"),
)
WIKI = Path("tools/wiki/cache/pages")

# The headings of an entry page, in order; the required ones are marked True.
HEADINGS = (
    ("Syntax", True),
    ("Arguments", False),
    ("Result", False),
    ("Behavior", True),
    ("Errors", False),
    ("Differences", False),
    ("Availability", True),
    ("Example", False),
    ("See also", True),
)
# The sections whose every paragraph and list item names its source.
SOURCED = ("Behavior", "Errors")
# The grammar's hand-written lists of the language's own words (vscode/DEVELOPMENT.md, "Grammar").
GRAMMAR_SCOPES = (
    "keyword.control.qb64rust",
    "storage.modifier.qb64rust",
    "storage.type.qb64rust",
    "keyword.operator.word.qb64rust",
)
TOKEN = re.compile(r"\$?_?[A-Za-z][A-Za-z0-9_]*\$?")
CODE_SPAN = re.compile(r"`([^`\n]+)`")
BEGIN = re.compile(r"<!-- begin generated: ([a-z]+(?::[A-Za-z0-9_.-]+)?) -->")
END_MARK = "<!-- end generated -->"
SRC = re.compile(r"<!-- src: (.*?) -->", re.S)
PROBE = re.compile(r"<!-- probe: (.*?) -->")
EXAMPLE_ID = re.compile(r"^[a-z0-9_-]+_[0-9]+$")


def read(path: Path) -> str:
    return (REPO / path).read_text(encoding="utf-8", errors="replace")


def rel(path: Path) -> str:
    """A repository path as the documents write it: backslashes, as in the rest of docs/."""
    return path.as_posix().replace("/", "\\")


# ---------------------------------------------------------------------------------------------------------------
# Words and pages


def page_name(word: str) -> str:
    """The page file of a word, without `.md`: lower case, a leading `$` as `meta-`, a trailing `$` as `-str`, a
    blank as `-` (study/49 §4)."""
    w = word.lower()
    if w.startswith("$"):
        w = "meta-" + w[1:]
    if w.endswith("$"):
        w = w[:-1] + "-str"
    return w.replace(" ", "-")


@dataclass
class Word:
    word: str
    kinds: list[str] = field(default_factory=list)  # function, statement, keyword, metacommand
    sources: list[str] = field(default_factory=list)  # builtins, keywords.rs, words.rs, grammar
    page: str = ""
    batch: int = 0
    area: str = ""

    def as_json(self) -> dict:
        return {
            "word": self.word,
            "kinds": self.kinds,
            "sources": self.sources,
            "page": self.page,
            "batch": self.batch,
            "area": self.area,
        }


def builtin_word(entry: dict) -> str:
    return entry["name"].upper() + (entry.get("musthave") or "")


@functools.cache
def load_builtins() -> dict:
    return json.loads(read(BUILTINS))


def keywords_rs() -> list[str]:
    text = read(KEYWORDS_RS)
    body = text[text.index("const KEYWORDS") :]
    body = body[: body.index("];")]
    return re.findall(r'"([^"]+)"', body)


def words_rs() -> list[str]:
    """The upper-case words of the formatter's spellings: metacommands kept whole (`$INCLUDE`), and the words of a
    spelling of several (`END IF`) one by one. A metacommand's argument (`:ON`, `:'`) is dropped."""
    out = []
    for upper in re.findall(r'\("([^"]+)", "', read(WORDS_RS)):
        if upper.startswith("$"):
            out.append(upper.split(":")[0])
        else:
            out.extend(TOKEN.findall(upper))
    return out


def grammar_words() -> list[str]:
    grammar = json.loads(read(GRAMMAR))
    out: list[str] = []

    def walk(node):
        if isinstance(node, dict):
            if node.get("name") in GRAMMAR_SCOPES and "match" in node:
                m = re.search(r"\(\?:([^)]*)\)", node["match"])
                if m:
                    out.extend(m.group(1).split("|"))
            for v in node.values():
                walk(v)
        elif isinstance(node, list):
            for v in node:
                walk(v)

    walk(grammar)
    return out


@functools.cache
def load_words_toml() -> dict:
    return tomllib.loads(read(WORDS_TOML))


def spec_name(path: Path) -> str:
    """`language/data-read` for `openspec/specs/language/data-read/spec.md`."""
    return path.parent.relative_to(REPO / SPECS).as_posix()


@dataclass
class Requirement:
    spec: str
    heading: str
    text: str  # the whole requirement, scenarios included
    heading_words: set[str]  # words of its heading, in code spans or in capitals
    body_words: set[str]  # words in code spans of the requirement's text before its scenarios
    words: set[str]  # words in code spans anywhere in it


def words_in(code: str, capitals: bool = False) -> set[str]:
    """The upper-case words of a piece of code, a compound of words.toml (`LINE INPUT`) taken as one word. With
    `capitals`, only words written in capitals count: a spec writes BASIC's words so, and a placeholder (`RUN
    line`) in lower case."""
    tokens = [t.upper() if not capitals or t == t.upper() else "" for t in TOKEN.findall(code)]
    compounds = compound_words()
    out = set()
    i = 0
    while i < len(tokens):
        pair = f"{tokens[i]} {tokens[i + 1]}" if i + 1 < len(tokens) else ""
        if pair in compounds:
            out.add(pair)
            i += 2
        else:
            out.add(tokens[i])
            i += 1
    out.discard("")
    return out


@functools.cache
def compound_words() -> frozenset[str]:
    return frozenset(w.upper() for w in load_words_toml().get("compounds", []))


def code_words(text: str) -> set[str]:
    return {w for span in CODE_SPAN.findall(text) for w in words_in(span, capitals=True)}


@functools.cache
def load_requirements() -> tuple[Requirement, ...]:
    out = []
    for path in sorted((REPO / SPECS).glob("*/*/spec.md")):
        name = spec_name(path)
        if not (name.startswith("language/") or name in LANGUAGE_SPECS):
            continue
        parts = re.split(r"^### Requirement: ", path.read_text(encoding="utf-8"), flags=re.M)
        for part in parts[1:]:
            heading, _, rest = part.partition("\n")
            rest = re.split(r"^## ", rest, flags=re.M)[0]
            body = rest.split("#### Scenario")[0]
            heading = heading.strip()
            capitals = {t for t in TOKEN.findall(heading) if t.upper() == t and len(t) > 1}
            out.append(
                Requirement(
                    name, heading, rest.strip(), code_words(heading) | capitals, code_words(body), code_words(rest)
                )
            )
    return tuple(out)


def inventory() -> dict[str, Word]:
    """Every word that needs a page or is covered on one (study/49 §5 item 1)."""
    table = load_words_toml()
    skip = {w.upper() for w in table.get("skip", {})}
    words: dict[str, Word] = {}

    def add(word: str, kind: str, source: str):
        word = word.upper()
        if word in skip:
            return
        w = words.setdefault(word, Word(word))
        if kind and kind not in w.kinds:
            w.kinds.append(kind)
        if source not in w.sources:
            w.sources.append(source)

    data = load_builtins()
    for e in data["entries"]:
        add(builtin_word(e), "statement" if e["kind"] == "sub" else "function", "builtins")
    for f in data["auto_include"]:
        if f["kind"] == "function" and not f["name"].startswith("_IKW_"):
            add(f["name"], "function", "builtins")
    for k in keywords_rs():
        add(k, "keyword", "keywords.rs")
    for k in words_rs():
        add(k, "metacommand" if k.startswith("$") else "", "words.rs")
    for k in grammar_words():
        add(k, "keyword", "grammar")
    for k, kind in table.get("extra", {}).items():
        add(k, kind, "words.toml")

    pages = {k.upper(): v for k, v in table.get("page", {}).items()}
    areas = {k.upper(): v for k, v in table.get("area", {}).items()}
    prefixes = {k.upper(): v for k, v in table.get("prefix", {}).items()}
    spec_batch = table["spec_batch"]
    batch_names = {int(k): v for k, v in table["batches"].items()}
    requirements = load_requirements()
    strings = {builtin_word(e) for e in data["entries"] if string_builtin(e)}
    for w in words.values():
        w.page = pages.get(w.word) or page_name(w.word)
        if not w.kinds:
            w.kinds.append("keyword")
        prefix = max((p for p in prefixes if w.word.startswith(p)), key=len, default=None)
        if w.word in areas:
            w.batch = int(areas[w.word])
        elif prefix:
            w.batch = int(prefixes[prefix])
        else:
            counts: dict[str, list[int]] = {}
            for r in requirements:
                if w.word in r.words:
                    c = counts.setdefault(r.spec, [0, 0])
                    c[0] += 10 if w.word in r.heading_words else 2 if w.word in r.body_words else 0
                    c[1] += 1
            ranked = sorted((-c[0], -c[1], s) for s, c in counts.items() if s in spec_batch)
            w.batch = int(spec_batch[ranked[0][2]]) if ranked else int(table["default_batch"])
            if w.batch == int(table["math_batch"]) and w.word in strings:
                w.batch = int(table["string_batch"])
    # A word covered on another word's page goes with the page's own word.
    owners = {w.page: w.batch for w in words.values() if page_name(w.word) == w.page}
    for w in words.values():
        w.batch = owners.get(w.page, w.batch)
        w.area = batch_names.get(w.batch, "")
    return dict(sorted(words.items()))


def string_builtin(entry: dict) -> bool:
    return entry.get("ret") == "STRING" or (entry.get("arg_types") or [None])[0] == "STRING"


def inventory_json(words: dict[str, Word]) -> str:
    one_source = [
        w.word
        for w in words.values()
        if len(w.sources) == 1
        and w.sources[0] != "builtins"
        and not w.word.startswith("$")
        and w.word not in load_words_toml().get("page", {})
    ]
    doc = {
        "comment": "Written by tools/reference/reference.py inventory; do not edit. A word is covered on its page.",
        "words": [w.as_json() for w in words.values()],
        "one_source": one_source,
    }
    return json.dumps(doc, indent=1) + "\n"


# ---------------------------------------------------------------------------------------------------------------
# Evidence


def program_files() -> list[Path]:
    out = []
    for d in PROGRAM_DIRS:
        base = REPO / d
        if not base.is_dir():
            continue
        pattern = "*.bas" if d.name in ("verification", "qb45") else "**/*.bas"
        out.extend(sorted(base.glob(pattern)))
    return out


def code_tokens(text: str) -> set[str]:
    """The upper-case words of a program's code, comments and string literals left out."""
    out = set()
    for line in text.splitlines():
        line = re.sub(r'"[^"]*"?', " ", line)
        line = re.split(r"'|(?i:^\s*REM\b)", line)[0]
        out.update(words_in(line))
    return out


def first_comment(text: str) -> str:
    for line in text.splitlines()[:4]:
        s = line.strip()
        if s.startswith("'"):
            return s.lstrip("' ").strip()[:160]
    return ""


def callsite_statement(text: str) -> str:
    """The statement under test of a call-site program: its last line of code before `END` or `SYSTEM`."""
    lines = [l.strip() for l in text.splitlines() if l.strip() and not l.strip().startswith("'")]
    while lines and lines[-1].upper() in ("END", "SYSTEM"):
        lines.pop()
    return lines[-1] if lines else ""


@functools.cache
def callsite_index() -> tuple[tuple[Path, str, frozenset[str]], ...]:
    """Each call-site program with its statement under test and that statement's words, read once."""
    out = []
    for path in sorted((REPO / CALLSITE).glob("*.bas")):
        stmt = callsite_statement(path.read_text(encoding="utf-8", errors="replace"))
        out.append((path, stmt, frozenset(words_in(re.sub(r'"[^"]*"', " ", stmt)))))
    return tuple(out)


def callsite_programs(word: str) -> list[tuple[Path, str]]:
    return [(path, stmt) for path, stmt, words in callsite_index() if word in words]


def table_rows(path: Path, prefix: str) -> list[str]:
    return [l for l in read(path).splitlines() if l.startswith(prefix)]


def family_lines() -> dict[str, str]:
    """`v22` -> its line in verification/README.md."""
    out = {}
    for line in read(VERIFICATION_README).splitlines():
        m = re.match(r"- `(v\d+)\*?`", line)
        if m and m.group(1) not in out:
            out[m.group(1)] = line[2:]
    return out


def evidence(word: str, limit: int = 40) -> str:
    """Everything the repository knows about one word, as text for the writer of its page (study/49 §5 item 2)."""
    word = word.upper()
    words = inventory()
    out = []
    w = words.get(word)
    if w is None:
        out.append(f"{word}: not in the inventory (run `inventory`, or see words.toml's [skip])")
    else:
        covered = [x.word for x in words.values() if x.page == w.page]
        out.append(f"# {word}\nkinds: {', '.join(w.kinds)}; sources: {', '.join(w.sources)}")
        out.append(f"page: {rel(ENTRIES / (w.page + '.md'))}, covering {', '.join(covered)}")
        out.append(f"batch {w.batch}: {w.area}")

    data = load_builtins()
    entries = [e for e in data["entries"] if builtin_word(e) == word or e["name"].upper() == word]
    out.append(f"\n## The built-in table ({rel(BUILTINS)}): {len(entries)} entries")
    for e in entries:
        out.append(json.dumps({k: v for k, v in e.items() if k not in ("line", "compiler_mentions")}))
    consts = [c for c in data["auto_include"] if word in c.get("value", "").upper().replace("(", " ").split()]
    if consts:
        out.append(f"auto-included constants whose value names it: {', '.join(c['name'] for c in consts[:20])}")

    reqs = [r for r in load_requirements() if word in r.words]  # a new list: sorted below
    # The most specific first: the word in the heading, then in the text, then by how few other words it names.
    reqs.sort(key=lambda r: (word not in r.heading_words, word not in r.body_words, len(r.words), r.spec))
    out.append(f"\n## Spec requirements that name it in code: {len(reqs)}")
    for r in reqs[:12]:
        out.append(f'\n### {r.spec} "{r.heading}"\n{r.text}')
    for r in reqs[12:]:
        out.append(f'- {r.spec} "{r.heading}"')

    families = family_lines()
    progs = []
    # A metacommand may stand in a comment (`'$INCLUDE: 'x.bi'`), so it is looked for there too.
    meta = re.compile(r"(?im)^\s*(?:'|REM\s+)?\s*" + re.escape(word) + r"(?![A-Z0-9_])") if word.startswith("$") else None
    for p in program_files():
        text = p.read_text(encoding="utf-8", errors="replace")
        if word in code_tokens(text) or (meta and meta.search(text)):
            progs.append((p, first_comment(text)))
    out.append(f"\n## Recorded programs that use it: {len(progs)} (first {limit} listed)")
    seen = set()
    for p, comment in progs[:limit]:
        out.append(f"- {rel(p.relative_to(REPO))}: {comment}")
        fam = re.match(r"(v\d+)", p.name)
        if fam and fam.group(1) in families:
            seen.add(fam.group(1))
    for f in sorted(seen, key=lambda s: int(s[1:])):
        out.append(f"  family {families[f][:300]}")

    sites = callsite_programs(word)
    out.append(f"\n## Call-site programs (one form each): {len(sites)}")
    for p, stmt in sites:
        out.append(f"- {p.stem}: {stmt}")

    for path in DIVERGENCES:
        rows = [r for r in table_rows(path, "| ") if word in code_words(r) and re.match(r"\| [DQ]-\d+", r)]
        out.append(f"\n## {path.name}: {len(rows)} rows")
        out.extend(r[:600] for r in rows)
    rows = [r for r in table_rows(SOMEDAY, "| P") if word in code_words(r)]
    out.append(f"\n## {SOMEDAY.name} rows: {len(rows)}")
    out.extend(r[:400] for r in rows)

    if w is not None:
        examples = sorted((REPO / EXAMPLES).glob(f"{w.page}_*.bas"))
        out.append(f"\n## Examples of the page so far: {', '.join(p.stem for p in examples) or 'none'}")
    return "\n".join(out) + "\n"


# ---------------------------------------------------------------------------------------------------------------
# Generated blocks


def form(entry: dict) -> str:
    """A built-in's form from the table when it has no `hr_syntax`, as the hover writes it
    (crates/lsp/src/hover.rs, `form`)."""
    name = entry["name"].upper() + (entry.get("musthave") or "")
    types = iter(entry.get("arg_types") or [])
    if entry.get("specialformat"):
        args = ""
        for c in entry["specialformat"]:
            args += next(types, "?") if c == "?" else ", " if c == "," else "" if c in "{}" else c
    else:
        args = ", ".join(entry.get("arg_types") or [])
    if entry["kind"] == "function":
        text = f"{name}({args})" if args else name
        return text + (f" AS {entry['ret']}" if entry.get("ret") else "")
    return f"{name} {args}".rstrip()


def syntax_block(word: str, words: dict[str, Word]) -> str | None:
    """The forms of the page's built-ins, statements first; None when no word of the page is a built-in."""
    data = load_builtins()
    page = words[word].page
    covered = [w for w in words.values() if w.page == page]
    lines = []
    for w in covered:
        for kind in ("sub", "function"):
            for e in data["entries"]:
                if builtin_word(e) == w.word and e["kind"] == kind and e.get("category") != "stub":
                    line = e.get("hr_syntax") or form(e)
                    if line not in lines:
                        lines.append(line)
    if not lines:
        return None
    return "```text\n" + "\n".join(lines) + "\n```\n\nFrom the built-in table (`tools\\builtins\\builtins.json`)."


def availability_text(page: str, avail: dict) -> str:
    a = avail.get(page)
    if not a or not a["programs"]:
        return "Not checked: no call-site program, example or probe uses it."
    n, ok = a["programs"], a["ok"]
    if ok == n:
        if n == 1:
            return "Supported. The one program that uses it compiles (a call-site program, an example or a probe)."
        return f"Supported. All {n} programs that use it compile (call-site programs, examples and probes)."
    parts = []
    if a["unsupported"]:
        head = "Not supported yet" if ok == 0 else f"Partly supported: {ok} of {n} programs that use it compile"
        parts.append(head + ". The compiler gives for the others:\n\n" + "\n".join(f"- {m}" for m in a["unsupported"]))
    if a["rejected"]:
        head = "Rejected" if ok == 0 and not a["unsupported"] else "Some forms are rejected"
        parts.append(
            head + ", as in QB64pe. The compiler gives:\n\n" + "\n".join(f"- {m}" for m in a["rejected"])
        )
    return "\n\n".join(parts)


def example_block(ex_id: str) -> str:
    bas = REPO / EXAMPLES / f"{ex_id}.bas"
    if not bas.is_file():
        return f"Missing example program `{rel(EXAMPLES / (ex_id + '.bas'))}`."
    lines = bas.read_text(encoding="utf-8").replace("\r\n", "\n").rstrip("\n").split("\n")
    if lines and lines[0].upper().startswith("$CONSOLE:ONLY"):
        lines = lines[1:]
    out = "```qb64\n" + "\n".join(lines) + "\n```\n"
    # The program's own source files in its folder of data files (an included file), each shown after it.
    files = bas.with_suffix(".files")
    if files.is_dir():
        for f in sorted(files.rglob("*")):
            if f.suffix.lower() in (".bi", ".bm", ".bas"):
                text = f.read_text(encoding="utf-8").replace("\r\n", "\n").rstrip("\n")
                out += f"\nThe file `{f.relative_to(files).as_posix()}`:\n\n```qb64\n{text}\n```\n"
    output = bas.with_suffix(".output")
    if output.is_file():
        text = output.read_bytes().decode("cp437").replace("\r\n", "\n").rstrip("\n")
        out += "\nOutput:\n\n```text\n" + text + "\n```\n"
    else:
        out += "\nNot recorded yet.\n"
    return out


def block_text(name: str, page_word: str, words: dict[str, Word], avail: dict) -> str | None:
    if name == "syntax":
        return syntax_block(page_word, words)
    if name == "availability":
        return availability_text(words[page_word].page, avail)
    if name.startswith("example:"):
        return example_block(name.split(":", 1)[1])
    return None


def refresh(text: str, page_word: str, words: dict[str, Word], avail: dict) -> str:
    """The page with each generated block written again."""

    def one(m: re.Match) -> str:
        name = m.group(1)
        body = block_text(name, page_word, words, avail)
        if body is None:
            body = f"(no generated text for `{name}`)"
        return f"<!-- begin generated: {name} -->\n{body.rstrip()}\n{END_MARK}"

    pattern = re.compile(BEGIN.pattern + r".*?" + re.escape(END_MARK), re.S)
    return pattern.sub(one, text)


def page_word_of(text: str) -> str:
    m = re.match(r"# (.+)\n", text)
    return m.group(1).strip().split(",")[0].strip().upper() if m else ""


def skeleton(word: str) -> Path:
    words = inventory()
    w = words[word.upper()]
    path = REPO / ENTRIES / f"{w.page}.md"
    if path.exists():
        raise SystemExit(f"{rel(path.relative_to(REPO))} exists; skeleton writes only a new page")
    covered = [x.word for x in words.values() if x.page == w.page]
    syntax = (
        "<!-- begin generated: syntax -->\n<!-- end generated -->"
        if syntax_block(w.word, words)
        else "TODO: the forms, written by hand."
    )
    has_function = any("function" in x.kinds for x in words.values() if x.page == w.page)
    parts = [
        f"# {', '.join(covered)}\n\nTODO: one or two sentences.\n",
        f"## Syntax\n\n{syntax}\n",
        "## Arguments\n\nTODO\n",
    ]
    if has_function:
        parts.append("## Result\n\nTODO\n")
    parts += [
        "## Behavior\n\nTODO <!-- src: TODO -->\n",
        "## Errors\n\nTODO <!-- src: TODO -->\n",
        "## Differences\n\nTODO\n",
        "## Availability\n\n<!-- begin generated: availability -->\n<!-- end generated -->\n",
        f"## Example\n\n<!-- begin generated: example:{w.page}_1 -->\n<!-- end generated -->\n",
        "## See also\n\nTODO\n",
    ]
    path.parent.mkdir(parents=True, exist_ok=True)
    text = "\n".join(parts)
    path.write_text(refresh(text, w.word, words, load_availability()), encoding="utf-8", newline="\n")
    return path


# ---------------------------------------------------------------------------------------------------------------
# Availability


def load_availability() -> dict:
    path = REPO / AVAILABILITY
    return json.loads(path.read_text(encoding="utf-8")) if path.is_file() else {}


def page_files() -> list[Path]:
    return sorted((REPO / ENTRIES).glob("*.md"))


def probes_of(page: Path, words: dict[str, Word]) -> list[tuple[str, str, Path | None]]:
    """(label, program text, its folder of data files) for each program that shows whether the page's words
    compile: the call-site programs of its words, its examples and its `probe:` lines."""
    text = page.read_text(encoding="utf-8")
    covered = [w.word for w in words.values() if w.page == page.stem]
    out = []
    seen = set()
    for word in covered:
        for path, _ in callsite_programs(word):
            if path not in seen:
                seen.add(path)
                files = path.with_suffix(".files")
                out.append((f"callsite:{path.stem}", path.read_text(encoding="utf-8"), files if files.is_dir() else None))
    for path in sorted((REPO / EXAMPLES).glob(f"{page.stem}_*.bas")):
        files = path.with_suffix(".files")
        out.append((f"example:{path.stem}", path.read_text(encoding="utf-8"), files if files.is_dir() else None))
    for i, line in enumerate(PROBE.findall(text), 1):
        program = "$CONSOLE:ONLY\n" + line.replace(" : ", "\n") + "\nEND\n"
        out.append((f"probe:{i}", program, None))
    return out


def compile_probe(compiler: Path, label: str, program: str, files: Path | None) -> tuple[str, str, list[str]]:
    """(label, ok|unsupported|error, the compiler's error messages)."""
    with tempfile.TemporaryDirectory(prefix="qbref") as tmp:
        tmp = Path(tmp)
        if files:
            shutil.copytree(files, tmp, dirs_exist_ok=True)
        bas = tmp / "p.bas"
        bas.write_text(program, encoding="utf-8")
        r = subprocess.run(
            [str(compiler), "-z", "--no-settings", str(bas), "-o", str(tmp / "p.cpp")],
            capture_output=True,
            text=True,
            errors="replace",
        )
    messages = [m.split("error: ", 1)[1].strip() for m in r.stdout.splitlines() + r.stderr.splitlines() if "error: " in m]
    if r.returncode == 0:
        return label, "ok", []
    if messages and all(m.startswith("not supported yet") for m in messages):
        return label, "unsupported", messages
    return label, "error", messages or [f"exit status {r.returncode}"]


def compute_availability(compiler: Path, words: dict[str, Word]) -> tuple[dict, list[str]]:
    """The availability of every page, and the problems found (a probe with a real error)."""
    jobs = [(page.stem, probe) for page in page_files() for probe in probes_of(page, words)]
    result: dict[str, dict] = {
        page.stem: {"programs": 0, "ok": 0, "unsupported": [], "rejected": []} for page in page_files()
    }
    problems = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=os.cpu_count() or 4) as pool:
        futures = [(stem, pool.submit(compile_probe, compiler, *probe)) for stem, probe in jobs]
        for stem, fut in futures:
            label, verdict, messages = fut.result()
            a = result[stem]
            a["programs"] += 1
            if verdict == "ok":
                a["ok"] += 1
                continue
            if verdict == "error" and not label.startswith("probe:"):
                # A call-site program or an example must compile; only a probe may show a rejection.
                problems.append(f"{stem}: {label} does not compile: {'; '.join(messages)}")
                continue
            kind = "unsupported" if verdict == "unsupported" else "rejected"
            for m in messages:
                if m not in a[kind]:
                    a[kind].append(m)
    for a in result.values():
        a["unsupported"].sort()
        a["rejected"].sort()
    return result, problems


def availability_json(avail: dict) -> str:
    return json.dumps(dict(sorted(avail.items())), indent=1) + "\n"


# ---------------------------------------------------------------------------------------------------------------
# Index and appendices


def index_md(words: dict[str, Word]) -> str:
    pages = {p.stem: p for p in page_files()}
    lines = [
        "# QB64Rust language reference",
        "",
        "<!-- Written by tools/reference/reference.py render; do not edit. -->",
        "",
        "One page per statement, function, keyword and metacommand. Each page says what QB64Rust does, which",
        "errors it gives, and where it differs from QB64pe 4.7.0 and QuickBASIC 4.5. Every rule on a page names",
        "its source, and every example is a test program whose output was recorded (`tools\\reference\\README.md`).",
        "",
        "The examples start with `$CONSOLE:ONLY` so that their output can be recorded; the pages leave that line",
        "out. Run in a window, an example prints the same text there.",
        "",
        f"{len(pages)} pages written; the inventory names {len({w.page for w in words.values()})}.",
        "",
        "## By area",
        "",
    ]
    by_batch: dict[int, list[str]] = {}
    for w in words.values():
        if w.page in pages:
            by_batch.setdefault(w.batch, [])
            if w.page not in by_batch[w.batch]:
                by_batch[w.batch].append(w.page)
    names = {int(k): v for k, v in load_words_toml()["batches"].items()}
    for batch in sorted(by_batch):
        links = ", ".join(f"[{title_of(pages[p])}](entries/{p}.md)" for p in sorted(by_batch[batch]))
        lines.append(f"- **{names.get(batch, batch)}**: {links}")
    lines += ["", "## Appendices", "", "- [The auto-included constants](appendix-constants.md)", ""]
    return "\n".join(lines)


def title_of(page: Path) -> str:
    m = re.match(r"# (.+)\n", page.read_text(encoding="utf-8"))
    return m.group(1).strip() if m else page.stem


# The type a constant's suffix in the auto-included files gives it.
SUFFIX_TYPES = {
    "%%": "_BYTE",
    "~%%": "_UNSIGNED _BYTE",
    "%": "INTEGER",
    "~%": "_UNSIGNED INTEGER",
    "&": "LONG",
    "~&": "_UNSIGNED LONG",
    "&&": "_INTEGER64",
    "~&&": "_UNSIGNED _INTEGER64",
    "%&": "_OFFSET",
    "~%&": "_UNSIGNED _OFFSET",
    "!": "SINGLE",
    "#": "DOUBLE",
    "##": "_FLOAT",
    "$": "STRING",
    "`": "_BIT",
    "~`": "_UNSIGNED _BIT",
}


def constants_md() -> str:
    data = load_builtins()
    files = (
        ("include/beforefirstline.bi", "Always defined", "Defined in every program, before its first line."),
        (
            "color/color0.bi",
            "`$COLOR:0`",
            "Defined after `$COLOR:0`: the 16 colors of the 16-color palette, and `BLINK`.",
        ),
        ("color/color32.bi", "`$COLOR:32`", "Defined after `$COLOR:32`: 32-bit colors, `_RGB32` values."),
    )
    out = [
        "# The auto-included constants",
        "",
        "<!-- Written by tools/reference/reference.py render; do not edit. -->",
        "",
        "The constants QB64pe's auto-included files define, with the text of each value, from the built-in table",
        "(`tools\\builtins\\builtins.json`, `auto_include`).",
        "",
    ]
    for file, title, what in files:
        rows = [c for c in data["auto_include"] if c["file"] == file and c["kind"] == "const"]
        out += [f"## {title}", "", f"{what} {len(rows)} constants.", "", "| Name | Type | Value |", "|---|---|---|"]
        for c in rows:
            value = c["value"].replace("|", "\\|")
            suffix = c.get("suffix", "")
            kind = SUFFIX_TYPES.get(suffix, f"`{suffix}`") if suffix else "by its value"
            out.append(f"| `{c['name']}` | {kind} | `{value}` |")
        out.append("")
    return "\n".join(out)


def generated_files(words: dict[str, Word]) -> dict[Path, str]:
    return {
        INVENTORY: inventory_json(words),
        REF / "README.md": index_md(words),
        REF / "appendix-constants.md": constants_md(),
    }


# ---------------------------------------------------------------------------------------------------------------
# Checks


def sections(text: str) -> list[tuple[str, str]]:
    """(heading, body) of each `## ` section of a page; the text before the first is ("", ...)."""
    parts = re.split(r"^## (.+)\n", text, flags=re.M)
    out = [("", parts[0])]
    for i in range(1, len(parts), 2):
        out.append((parts[i].strip(), parts[i + 1]))
    return out


def blocks(body: str) -> list[str]:
    """The paragraphs and list items of a section, code fences and generated blocks left out."""
    body = re.sub(r"```.*?```", "", body, flags=re.S)
    body = re.sub(BEGIN.pattern + r".*?" + re.escape(END_MARK), "", body, flags=re.S)
    out = []
    for para in re.split(r"\n\s*\n", body):
        para = para.strip()
        if not para:
            continue
        items = re.split(r"\n(?=[-*] |\d+\. )", para)
        out.extend(i.strip() for i in items if i.strip())
    return out


def check_src(item: str, requirements: set[tuple[str, str]], rows: set[str]) -> str | None:
    """None when one source named by a `src:` comment exists, else why not."""
    item = item.strip()
    if re.fullmatch(r"[DQ]-\d{3}", item):
        return None if item in rows else f"no divergence row {item}"
    m = re.fullmatch(r'((?:language|compiler|editor|testing)/[a-z0-9-]+) "(.+)"', item)
    if m:
        if (m.group(1), m.group(2)) in requirements:
            return None
        return f'no requirement "{m.group(2)}" in spec {m.group(1)}'
    m = re.fullmatch(r"(?:read: )?([A-Za-z0-9_.\\/$-]+?)(?::\d+(?:-\d+)?)?", item)
    if m:
        path = Path(m.group(1).replace("\\", "/"))
        for candidate in (path, path.with_name(path.name + ".bas")):
            if (REPO / candidate).exists():
                return None
        return f"no file {m.group(1)}"
    return f"cannot read the source `{item}`"


def check_page(path: Path, words: dict[str, Word], avail: dict, requirements, rows) -> list[str]:
    name = rel(path.relative_to(REPO))
    text = path.read_text(encoding="utf-8")
    problems = []
    if path.stem not in {w.page for w in words.values()}:
        problems.append(f"{name}: no word of the inventory has this page")
        return problems
    word = next(w.word for w in words.values() if w.page == path.stem)
    if refresh(text, word, words, avail) != text:
        problems.append(f"{name}: a generated block or an example is out of date (run render)")
    if "TODO" in text:
        problems.append(f"{name}: has a TODO")
    secs = sections(text)
    if not re.match(r"# .+\n\n\S", secs[0][1]):
        problems.append(f"{name}: no title and summary before the first heading")
    order = [h for h, _ in HEADINGS]
    seen = [h for h, _ in secs[1:]]
    for h in seen:
        if h not in order:
            problems.append(f"{name}: unknown heading `## {h}`")
    known = [h for h in seen if h in order]
    if known != sorted(known, key=order.index) or len(set(known)) != len(known):
        problems.append(f"{name}: headings out of order or repeated")
    for h, required in HEADINGS:
        if required and h not in seen:
            problems.append(f"{name}: missing `## {h}`")
    for h, body in secs[1:]:
        if h in SOURCED:
            for b in blocks(body):
                srcs = SRC.findall(b)
                # A sentence that introduces the list after it ("These are compile errors:") states nothing.
                if not srcs and not b.endswith(":"):
                    problems.append(f"{name}: a block under {h} names no source: {b[:70]!r}")
                for s in srcs:
                    for item in s.split(";"):
                        why = check_src(item, requirements, rows)
                        if why:
                            problems.append(f"{name}: {why}")
    for target in re.findall(r"\]\(([^)\s]+)\)", text):
        if re.match(r"[a-z]+:", target) or target.startswith("#"):
            continue
        if not (path.parent / target.split("#")[0]).exists():
            problems.append(f"{name}: link to {target} leads nowhere")
    for ex in re.findall(r"<!-- begin generated: example:([^ ]+) -->", text):
        if not EXAMPLE_ID.match(ex) or not ex.startswith(path.stem + "_"):
            problems.append(f"{name}: example {ex} is not named <page>_<n>")
        elif not (REPO / EXAMPLES / f"{ex}.bas").is_file():
            problems.append(f"{name}: example {ex} has no program")
    return problems


def check(compiler: Path | None) -> list[str]:
    words = inventory()
    table = load_words_toml()
    problems = []
    for path, want in generated_files(words).items():
        full = REPO / path
        if not full.is_file() or full.read_text(encoding="utf-8") != want:
            problems.append(f"{rel(path)}: out of date (run render)")
    avail = load_availability()
    if compiler:
        fresh, probe_problems = compute_availability(compiler, words)
        problems += probe_problems
        if availability_json(fresh) != availability_json(avail):
            problems.append(f"{rel(AVAILABILITY)}: out of date (run render --compiler)")
    stems = {p.stem for p in page_files()}
    if set(avail) != stems:
        problems.append(f"{rel(AVAILABILITY)}: does not list exactly the pages (run render --compiler)")
    requirements = {(r.spec, r.heading) for r in load_requirements()}
    rows = {m for path in DIVERGENCES for m in re.findall(r"^\| ([DQ]-\d{3}) \|", read(path), re.M)}
    for page in page_files():
        problems += check_page(page, words, avail, requirements, rows)
    done = set(table.get("done", []))
    required = {w.page for w in words.values() if w.batch in done} | set(table.get("pilot", []))
    for page in sorted(required - stems):
        problems.append(f"{rel(ENTRIES / (page + '.md'))}: missing (its batch is done, or it is a pilot page)")
    names = {p.stem for p in (REPO / EXAMPLES).glob("*.bas")}
    for ex in sorted(names):
        page = ex.rsplit("_", 1)[0]
        if page not in stems:
            problems.append(f"{rel(EXAMPLES / (ex + '.bas'))}: no page {page}")
        elif f"example:{ex} " not in (REPO / ENTRIES / f"{page}.md").read_text(encoding="utf-8"):
            problems.append(f"{rel(EXAMPLES / (ex + '.bas'))}: its page does not show it")
    return problems


def render(compiler: Path | None) -> list[str]:
    words = inventory()
    problems = []
    if compiler:
        avail, problems = compute_availability(compiler, words)
        (REPO / AVAILABILITY).parent.mkdir(parents=True, exist_ok=True)
        (REPO / AVAILABILITY).write_text(availability_json(avail), encoding="utf-8", newline="\n")
    avail = load_availability()
    for path, text in generated_files(words).items():
        (REPO / path).parent.mkdir(parents=True, exist_ok=True)
        (REPO / path).write_text(text, encoding="utf-8", newline="\n")
    for page in page_files():
        text = page.read_text(encoding="utf-8")
        word = next((w.word for w in words.values() if w.page == page.stem), None)
        if word:
            new = refresh(text, word, words, avail)
            if new != text:
                page.write_text(new, encoding="utf-8", newline="\n")
    return problems


# ---------------------------------------------------------------------------------------------------------------
# Overlap with the wiki


def wiki_plain(text: str) -> tuple[str, list[str]]:
    """The prose and the code lines of a wiki page, markup removed."""
    code: list[str] = []
    for block in re.findall(r"\{\{CodeStart\}\}(.*?)\{\{CodeEnd\}\}", text, flags=re.S):
        code.extend(strip_wiki(block).splitlines())
    prose = re.sub(r"\{\{CodeStart\}\}.*?\{\{CodeEnd\}\}", " ", text, flags=re.S)
    return strip_wiki(prose), code


def strip_wiki(text: str) -> str:
    text = re.sub(r"\{\{[A-Za-z]+\|([^}|]*)(?:\|[^}]*)?\}\}", r"\1", text)
    text = re.sub(r"\{\{[^}]*\}\}", " ", text)
    text = re.sub(r"\[\[(?:[^|\]]*\|)?([^\]]*)\]\]", r"\1", text)
    text = re.sub(r"<[^>]+>", " ", text)
    return text.replace("'''", "").replace("''", "")


def shingles(text: str, n: int) -> set[tuple[str, ...]]:
    words = re.findall(r"[a-z0-9$_]+", text.lower())
    return {tuple(words[i : i + n]) for i in range(len(words) - n + 1)}


def norm_code(lines: list[str]) -> list[str]:
    out = []
    for line in lines:
        s = " ".join(line.strip().lower().split())
        if s and not s.startswith("'"):
            out.append(s)
    return out


def page_prose(text: str) -> str:
    text = re.sub(BEGIN.pattern + r".*?" + re.escape(END_MARK), " ", text, flags=re.S)
    text = re.sub(r"<!--.*?-->", " ", text, flags=re.S)
    return re.sub(r"```.*?```", " ", text, flags=re.S)


def overlap(wiki: Path, prose_words: int = 10, code_lines: int = 6) -> list[str]:
    if not wiki.is_dir():
        raise SystemExit(f"no wiki cache at {wiki}: fetch it with tools/wiki/fetch_wiki.py (local only)")
    wiki_prose: set[tuple[str, ...]] = set()
    wiki_runs: set[tuple[str, ...]] = set()
    owner: dict[tuple[str, ...], str] = {}
    for page in wiki.glob("*.wikitext"):
        prose, code = wiki_plain(page.read_text(encoding="utf-8", errors="replace"))
        for s in shingles(prose, prose_words):
            wiki_prose.add(s)
            owner.setdefault(s, page.name)
        lines = norm_code(code)
        for i in range(len(lines) - code_lines + 1):
            run = tuple(lines[i : i + code_lines])
            wiki_runs.add(run)
            owner.setdefault(run, page.name)
    problems = []
    for page in page_files():
        name = rel(page.relative_to(REPO))
        hits = shingles(page_prose(page.read_text(encoding="utf-8")), prose_words) & wiki_prose
        for s in sorted(hits)[:3]:
            problems.append(f"{name}: shares {prose_words} words with wiki {owner[s]}: {' '.join(s)}")
    for bas in sorted((REPO / EXAMPLES).glob("*.bas")):
        lines = norm_code(bas.read_text(encoding="utf-8").splitlines())
        for i in range(len(lines) - code_lines + 1):
            run = tuple(lines[i : i + code_lines])
            if run in wiki_runs:
                problems.append(f"{rel(bas.relative_to(REPO))}: shares {code_lines} lines with wiki {owner[run]}")
                break
    return problems


# ---------------------------------------------------------------------------------------------------------------


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("command", nargs="?", choices=("inventory", "evidence", "skeleton", "render", "overlap"))
    ap.add_argument("word", nargs="?")
    ap.add_argument("--check", action="store_true", help="write nothing; exit 1 when anything is out of date")
    ap.add_argument("--compiler", type=Path, help="qb64rust, to compute which forms compile")
    ap.add_argument("--wiki", type=Path, default=REPO / WIKI)
    args = ap.parse_args(argv)
    if args.check:
        problems = check(args.compiler)
        for p in problems:
            print(p)
        print(f"reference: {len(page_files())} pages, {len(problems)} problems")
        return 1 if problems else 0
    if args.command == "inventory":
        words = inventory()
        (REPO / INVENTORY).parent.mkdir(parents=True, exist_ok=True)
        (REPO / INVENTORY).write_text(inventory_json(words), encoding="utf-8", newline="\n")
        print(f"{len(words)} words on {len({w.page for w in words.values()})} pages: {rel(INVENTORY)}")
        return 0
    if args.command in ("evidence", "skeleton") and not args.word:
        ap.error(f"{args.command} needs a word")
    if args.command == "evidence":
        sys.stdout.reconfigure(encoding="utf-8")
        sys.stdout.write(evidence(args.word))
        return 0
    if args.command == "skeleton":
        print(rel(skeleton(args.word).relative_to(REPO)))
        return 0
    if args.command == "render":
        problems = render(args.compiler)
        for p in problems:
            print(p)
        return 1 if problems else 0
    if args.command == "overlap":
        problems = overlap(args.wiki)
        for p in problems:
            print(p)
        print(f"overlap: {len(problems)} problems")
        return 1 if problems else 0
    ap.print_help()
    return 2


if __name__ == "__main__":
    sys.exit(main())
