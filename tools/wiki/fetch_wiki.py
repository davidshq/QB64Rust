#!/usr/bin/env python3
"""Fetch the QB64pe wiki as raw wikitext through its MediaWiki API.

Why: the wiki (https://qb64phoenix.com/qb64wiki/) is the primary documentation source for QB64pe
(study\\13-reference-docs.md). This script stores every page's raw wikitext with its page id, revision id and
timestamp so the text can be converted later (hover help, documentation) and refreshed incrementally.

Licence: the wiki states no licence (`meta=siteinfo&siprop=rightsinfo` is empty). The fetched text is a local
reference only. The cache folder is ignored by git; do not commit or ship wiki text until the QB64pe
maintainers have been asked (decision 2026-10-02, DECISIONS.md).

Usage:
    python tools\\wiki\\fetch_wiki.py                  # main + Template namespaces into tools\\wiki\\cache
    python tools\\wiki\\fetch_wiki.py --out DIR         # elsewhere
    python tools\\wiki\\fetch_wiki.py --namespaces 0    # main namespace only
    python tools\\wiki\\fetch_wiki.py --force           # refetch pages whose revision is unchanged too

Output layout (under --out):
    index.json                     one entry per page: pageid, ns, title, revid, timestamp, file
    pages/<pageid>_<title>.wikitext   raw wikitext (UTF-8); the title part is sanitised for Windows file names

Standard library only. One request at a time, 50 pages per content request (the anonymous API limit), with a
short pause between requests. Rerunning only downloads pages whose latest revision id changed.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

API = "https://qb64phoenix.com/qb64wiki/api.php"
USER_AGENT = "qb64rust-wiki-fetch/0.1 (https://github.com/davidshq; local documentation cache)"
PAGES_PER_REQUEST = 50
PAUSE_SECONDS = 0.5
RETRIES = 4

DEFAULT_OUT = Path(__file__).resolve().parent / "cache"
DEFAULT_NAMESPACES = [0, 10]  # main articles and templates ({{PageSyntax}} etc.)

_BAD_CHARS = re.compile(r'[<>:"/\\|?*\x00-\x1f]')


def api_get(params: dict) -> dict:
    """One GET against the API with retries. Raises on persistent failure or an API error object."""
    params = dict(params, format="json", formatversion="2")
    url = API + "?" + urllib.parse.urlencode(params)
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    last_error: Exception | None = None
    for attempt in range(RETRIES):
        try:
            with urllib.request.urlopen(req, timeout=60) as resp:
                data = json.loads(resp.read().decode("utf-8"))
            if "error" in data:
                raise RuntimeError(f"API error: {data['error']}")
            return data
        except (urllib.error.URLError, TimeoutError, json.JSONDecodeError) as e:
            last_error = e
            time.sleep(2 * (attempt + 1))
    raise RuntimeError(f"request failed after {RETRIES} attempts: {url}\n{last_error}")


def list_pages(namespace: int) -> list[dict]:
    """All pages (pageid, ns, title) in one namespace."""
    pages: list[dict] = []
    params = {
        "action": "query",
        "list": "allpages",
        "apnamespace": str(namespace),
        "aplimit": "500",
    }
    while True:
        data = api_get(params)
        pages.extend(data["query"]["allpages"])
        cont = data.get("continue")
        if not cont:
            return pages
        params = {**params, **cont}
        time.sleep(PAUSE_SECONDS)


def fetch_revisions(pageids: list[int]) -> dict[int, dict]:
    """Latest revision (revid, timestamp, content) for up to PAGES_PER_REQUEST pages, keyed by pageid."""
    data = api_get(
        {
            "action": "query",
            "prop": "revisions",
            "rvprop": "ids|timestamp|content",
            "rvslots": "main",
            "pageids": "|".join(str(p) for p in pageids),
        }
    )
    out: dict[int, dict] = {}
    for page in data["query"]["pages"]:
        if "missing" in page or not page.get("revisions"):
            continue
        rev = page["revisions"][0]
        out[page["pageid"]] = {
            "revid": rev["revid"],
            "timestamp": rev["timestamp"],
            "content": rev["slots"]["main"]["content"],
        }
    return out


def latest_revids(pageids: list[int]) -> dict[int, int]:
    """Latest revision id only (cheap), for deciding what changed."""
    data = api_get(
        {
            "action": "query",
            "prop": "revisions",
            "rvprop": "ids",
            "pageids": "|".join(str(p) for p in pageids),
        }
    )
    return {
        p["pageid"]: p["revisions"][0]["revid"]
        for p in data["query"]["pages"]
        if "missing" not in p and p.get("revisions")
    }


def page_filename(pageid: int, title: str) -> str:
    safe = _BAD_CHARS.sub("_", title).strip(" .")
    return f"{pageid}_{safe[:120]}.wikitext"


def chunks(seq: list, n: int):
    for i in range(0, len(seq), n):
        yield seq[i : i + n]


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument(
        "--out", type=Path, default=DEFAULT_OUT, help=f"output folder (default {DEFAULT_OUT})"
    )
    ap.add_argument(
        "--namespaces",
        type=int,
        nargs="+",
        default=DEFAULT_NAMESPACES,
        help="namespace ids (default: 0 main, 10 Template)",
    )
    ap.add_argument("--force", action="store_true", help="refetch every page")
    args = ap.parse_args(argv)

    out: Path = args.out
    pages_dir = out / "pages"
    pages_dir.mkdir(parents=True, exist_ok=True)
    index_path = out / "index.json"
    index: dict[str, dict] = {}
    if index_path.exists():
        index = {str(e["pageid"]): e for e in json.loads(index_path.read_text("utf-8"))["pages"]}

    listed: list[dict] = []
    for ns in args.namespaces:
        ns_pages = list_pages(ns)
        print(f"namespace {ns}: {len(ns_pages)} pages")
        listed.extend(ns_pages)
        time.sleep(PAUSE_SECONDS)

    # Decide which pages need content.
    to_fetch: list[int] = []
    if args.force:
        to_fetch = [p["pageid"] for p in listed]
    else:
        for batch in chunks(listed, PAGES_PER_REQUEST):
            ids = [p["pageid"] for p in batch]
            current = latest_revids(ids)
            for pid in ids:
                known = index.get(str(pid))
                if (
                    known is None
                    or known.get("revid") != current.get(pid)
                    or not (pages_dir / known["file"]).exists()
                ):
                    to_fetch.append(pid)
            time.sleep(PAUSE_SECONDS)
    print(f"{len(to_fetch)} of {len(listed)} pages to download")

    titles = {p["pageid"]: (p["ns"], p["title"]) for p in listed}
    done = 0
    for batch in chunks(to_fetch, PAGES_PER_REQUEST):
        revs = fetch_revisions(batch)
        for pid, rev in revs.items():
            ns, title = titles[pid]
            fname = page_filename(pid, title)
            (pages_dir / fname).write_text(rev["content"], encoding="utf-8", newline="\n")
            index[str(pid)] = {
                "pageid": pid,
                "ns": ns,
                "title": title,
                "revid": rev["revid"],
                "timestamp": rev["timestamp"],
                "file": fname,
            }
        done += len(batch)
        print(f"  {done}/{len(to_fetch)}", end="\r", flush=True)
        time.sleep(PAUSE_SECONDS)
    if to_fetch:
        print()

    # Drop index entries for pages that no longer exist; keep their files (user may delete the cache).
    live = {str(p["pageid"]) for p in listed}
    removed = [k for k in index if k not in live]
    for k in removed:
        del index[k]
    if removed:
        print(f"{len(removed)} pages no longer on the wiki (index entries dropped)")

    entries = sorted(index.values(), key=lambda e: (e["ns"], e["title"].lower()))
    index_path.write_text(
        json.dumps(
            {
                "source": API,
                "fetched": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
                "licence": "none stated by the wiki; local reference only, do not redistribute",
                "pages": entries,
            },
            indent=1,
            ensure_ascii=False,
        ),
        encoding="utf-8",
    )
    print(f"index: {index_path} ({len(entries)} pages)")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
