// The help the extension ships (OpenSpec change editor-side-panel, design D4 and D5): `help\index.json`, written by
// `tools\reference\reference.py help`. Its pages, its search, the page of a word by its spelling, and where a link
// of a page leads. No VS Code import, so it is unit-tested without it.
import * as fs from "fs";
import * as path from "path";

export interface HelpPage {
    /** `_clamp` for a keyword page, `chapters/control-flow` for a chapter, `first-steps` for a page of the
     * reference itself. */
    name: string;
    /** Under `pages\`: `entries/_clamp.md`. */
    path: string;
    title: string;
    /** The first sentence of the page's summary, as Markdown. */
    sentence: string;
    kind: "keyword" | "chapter" | "guide" | "appendix";
    /** The examples the page embeds: `_clamp_1`, `first-steps/hello`. */
    examples: string[];
}

export interface HelpExample {
    /** `_clamp_1`, `first-steps/hello`. */
    name: string;
    title: string;
    /** Under the help folder: `examples/_clamp_1.bas`. */
    file: string;
    /** Whether a recorded `.output` is beside it. */
    output: boolean;
    /** Whether a `.files` folder is beside it, copied with it to run it. */
    files: boolean;
    /** It opens a port: it gets no Run button (design D7). */
    network: boolean;
}

export interface HelpRow {
    label: string;
    page?: string;
    children?: HelpRow[];
}

export interface HelpIndexData {
    pages: HelpPage[];
    tree: HelpRow[];
    groups: { id: string; title: string; examples: HelpExample[] }[];
    keywords: { page: string; title: string; examples: HelpExample[] }[];
}

/** The same rule as `reference.py`'s `page_name`: `LEFT$` is `left-str`, `$INCLUDE` is `meta-include`,
 * `LINE INPUT` is `line-input`. */
export function pageNameOf(word: string): string {
    let w = word.toLowerCase();
    if (w.startsWith("$")) {
        w = `meta-${w.slice(1)}`;
    }
    if (w.endsWith("$")) {
        w = `${w.slice(0, -1)}-str`;
    }
    return w.replace(/ /g, "-");
}

/** A word of QB64 as "Help for This Word" takes it: `_CLAMP`, `LEFT$`, `$CONSOLE`, `count%`. */
export const WORD = /\$?_?[A-Za-z][A-Za-z0-9_.]*[$%&!#~`]*/;

/** The word of `line` at `column` (or just before it), with its start; undefined between words. */
export function wordAt(line: string, column: number): { word: string; start: number } | undefined {
    const re = new RegExp(WORD.source, "g");
    for (let m = re.exec(line); m; m = re.exec(line)) {
        if (m.index <= column && column <= m.index + m[0].length) {
            return { word: m[0], start: m.index };
        }
    }
    return undefined;
}

/** Markdown of a sentence as plain text: no backticks, emphasis or links. */
export function plainText(markdown: string): string {
    return markdown
        .replace(/\[([^\]]+)\]\([^)]*\)/g, "$1")
        .replace(/`/g, "")
        .replace(/\*\*/g, "");
}

export class HelpIndex {
    private readonly byName = new Map<string, HelpPage>();
    private readonly examples = new Map<string, HelpExample>();

    /** `root` is the help folder (`index.json`, `pages\`, `examples\`). */
    constructor(
        readonly root: string,
        readonly data: HelpIndexData,
    ) {
        for (const p of data.pages) {
            this.byName.set(p.name, p);
        }
        for (const e of [...data.groups.flatMap((g) => g.examples), ...data.keywords.flatMap((k) => k.examples)]) {
            this.examples.set(e.name, e);
        }
    }

    /** The help folder's index, or undefined when there is none (a development copy without `npm run help`). */
    static load(root: string): HelpIndex | undefined {
        try {
            const data = JSON.parse(fs.readFileSync(path.join(root, "index.json"), "utf8")) as HelpIndexData;
            return new HelpIndex(root, data);
        } catch {
            return undefined;
        }
    }

    get pages(): HelpPage[] {
        return this.data.pages;
    }

    page(name: string): HelpPage | undefined {
        return this.byName.get(name);
    }

    example(name: string): HelpExample | undefined {
        return this.examples.get(name);
    }

    /** The Markdown of a page. */
    text(page: HelpPage): string {
        return fs.readFileSync(path.join(this.root, "pages", page.path), "utf8");
    }

    /** The path of an example's program. */
    exampleFile(e: HelpExample): string {
        return path.join(this.root, e.file);
    }

    /** The page of a word by its spelling alone, for "Help for This Word" without the language server: the page
     * named by the reference's rule, else the page whose title is the word. */
    pageOfWord(word: string): HelpPage | undefined {
        const byRule = this.byName.get(pageNameOf(word));
        if (byRule?.kind === "keyword") {
            return byRule;
        }
        const upper = word.toUpperCase();
        return this.data.pages.find((p) => p.kind === "keyword" && p.title.toUpperCase() === upper);
    }

    /**
     * The pages that match `query`, best first (spec editor/help, "Search of the help"): every word of the query is
     * in the title or the first sentence. A title that is the query comes first, then a title that starts with it,
     * then one that holds it, then the rest; within each, A to Z. An empty query gives every page.
     */
    search(query: string): HelpPage[] {
        const words = query.toLowerCase().split(/\s+/).filter(Boolean);
        const q = query.trim().toLowerCase();
        const rank = (p: HelpPage): number => {
            const t = p.title.toLowerCase();
            return t === q ? 0 : t.startsWith(q) ? 1 : t.includes(q) ? 2 : 3;
        };
        const found = this.data.pages.filter((p) => {
            const hay = `${p.title} ${plainText(p.sentence)}`.toLowerCase();
            return words.every((w) => hay.includes(w));
        });
        return found
            .map((p) => ({ p, r: words.length ? rank(p) : 3 }))
            .sort((a, b) => a.r - b.r || a.p.title.replace(/^[_$]/, "").localeCompare(b.p.title.replace(/^[_$]/, "")))
            .map((x) => x.p);
    }

    /**
     * Where a link of `from` leads: another page of the help (with its anchor), a web address, or a file of the
     * repository outside the reference, given as its address on GitHub.
     */
    resolveLink(from: HelpPage, href: string): { page: string; anchor?: string } | { web: string } | undefined {
        if (/^[a-z][a-z0-9+.-]*:/i.test(href)) {
            return /^https?:/i.test(href) ? { web: href } : undefined;
        }
        if (href.startsWith("#")) {
            return { page: from.name, anchor: href.slice(1) };
        }
        const [file, anchor] = href.split("#");
        const target = path.posix.normalize(path.posix.join(path.posix.dirname(from.path), file));
        const page = this.data.pages.find((p) => p.path === target);
        if (page) {
            return { page: page.name, anchor };
        }
        // A file of the repository beside the reference (`../../DIVERGENCES-QB64PE.md` from an entry).
        const repo = path.posix.normalize(path.posix.join("docs/reference", target));
        return repo.startsWith("..") ? undefined : { web: `${REPOSITORY}/blob/main/${repo}${anchor ? `#${anchor}` : ""}` };
    }
}

/** The project's repository, for a link to a file outside the reference. */
export const REPOSITORY = "https://github.com/davidshq/QB64Rust";
