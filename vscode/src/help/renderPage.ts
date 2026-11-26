// A help page as HTML (OpenSpec change editor-side-panel, design D5): the reference page's Markdown drawn by
// markdown-it in the extension host, its links made messages to the help tab, its QB64 code colored, and each
// example given its buttons. Raw HTML in a page is not passed through. No VS Code import, so it is unit-tested
// without it.
import MarkdownIt from "markdown-it";
import { HelpIndex, HelpPage } from "./helpIndex";

export interface RenderOptions {
    /** In a workspace that is not trusted, "Run this example" is disabled (spec editor/help, "An untrusted
     * workspace"). */
    trusted: boolean;
}

export function escapeHtml(text: string): string {
    return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;").replace(/'/g, "&#39;");
}

/** Where an example's buttons go: a paragraph of its own, which markdown-it leaves as it is. */
const MARK = "@@qb64-example:";
const EXAMPLE_BLOCK = /<!-- begin generated: example:([^ ]+) -->([\s\S]*?)<!-- end generated -->/g;

/** The words colored as QB64's own: every word of the keyword pages' titles (`_CLAMP`, `SELECT CASE`). */
export function keywordWords(index: HelpIndex): Set<string> {
    const out = new Set<string>();
    for (const p of index.pages) {
        if (p.kind === "keyword") {
            for (const w of p.title.split(/[\s,]+/)) {
                if (w) {
                    out.add(w.toUpperCase());
                }
            }
        }
    }
    return out;
}

/** One line of QB64 as HTML spans: comments (`'` and `REM`), strings, numbers and the words of `words`. */
export function colorLine(line: string, words: Set<string>): string {
    let out = "";
    let i = 0;
    const span = (cls: string, text: string) => `<span class="${cls}">${escapeHtml(text)}</span>`;
    while (i < line.length) {
        const rest = line.slice(i);
        const c = line[i];
        if (c === "'" || /^REM(\s|$)/i.test(rest)) {
            out += span("comment", rest);
            break;
        }
        if (c === '"') {
            const end = line.indexOf('"', i + 1);
            const s = end < 0 ? rest : line.slice(i, end + 1);
            out += span("string", s);
            i += s.length;
            continue;
        }
        const num = /^(&[HhOoBb][0-9A-Fa-f]+|\d+\.?\d*([eEdD][-+]?\d+)?|\.\d+)[%&!#~`]*/.exec(rest);
        if (num && !/[A-Za-z0-9_]/.test(line[i - 1] ?? "")) {
            out += span("number", num[0]);
            i += num[0].length;
            continue;
        }
        const word = /^\$?_?[A-Za-z][A-Za-z0-9_.]*[$%&!#~`]*/.exec(rest);
        if (word) {
            const w = word[0];
            out += words.has(w.toUpperCase()) || words.has(w.replace(/[%&!#~`]+$/, "").toUpperCase()) ? span("keyword", w) : escapeHtml(w);
            i += w.length;
            continue;
        }
        out += escapeHtml(c);
        i++;
    }
    return out;
}

/** The text of a heading as its anchor, as GitHub makes it: `## See also` is `see-also`. */
export function slug(text: string): string {
    return text
        .toLowerCase()
        .replace(/[^\w\- ]+/g, "")
        .trim()
        .replace(/ /g, "-");
}

function buttons(index: HelpIndex, id: string, o: RenderOptions): string {
    const e = index.example(id);
    if (!e) {
        return "";
    }
    const parts = [];
    // An example that opens a port gets no Run button: it would make Windows ask about the firewall (design D7).
    if (!e.network) {
        const off = o.trusted ? "" : ' disabled title="Running needs a trusted workspace."';
        parts.push(`<button class="primary" data-action="run"${off}>Run this example</button>`);
    }
    parts.push(`<button data-action="open">Open as a new program</button>`, `<button data-action="copy">Copy</button>`);
    return `<div class="example-buttons" data-example="${escapeHtml(id)}">${parts.join("")}</div>`;
}

/** The body of the help page `page`, whose Markdown is `markdown`. */
export function renderPage(index: HelpIndex, page: HelpPage, markdown: string, o: RenderOptions): string {
    const words = keywordWords(index);
    const md = new MarkdownIt({ html: false, linkify: false, typographer: false });
    md.renderer.rules.fence = (tokens, i) => {
        const t = tokens[i];
        const lang = t.info.trim().split(/\s+/)[0];
        const lines = t.content.replace(/\n$/, "").split("\n");
        const code = lang === "qb64" ? lines.map((l) => colorLine(l, words)).join("\n") : escapeHtml(lines.join("\n"));
        return `<pre class="code ${lang === "qb64" ? "qb64" : "text"}"><code>${code}</code></pre>\n`;
    };
    md.renderer.rules.heading_open = (tokens, i, options, _env, self) => {
        const inline = tokens[i + 1];
        tokens[i].attrSet("id", slug(inline?.content ?? ""));
        return self.renderToken(tokens, i, options);
    };
    md.renderer.rules.link_open = (tokens, i, options, _env, self) => {
        const t = tokens[i];
        const where = index.resolveLink(page, t.attrGet("href") ?? "");
        if (where && "page" in where) {
            t.attrSet("href", "#");
            t.attrSet("data-page", where.page);
            if (where.anchor) {
                t.attrSet("data-anchor", where.anchor);
            }
        } else if (where) {
            t.attrSet("href", where.web);
            t.attrSet("data-web", where.web);
            t.attrSet("title", where.web);
        } else {
            t.attrSet("href", "#");
        }
        return self.renderToken(tokens, i, options);
    };
    const text = markdown
        .replace(EXAMPLE_BLOCK, (_m, id: string, body: string) => `${body}\n\n${MARK}${id}@@\n\n`)
        .replace(/<!--[\s\S]*?-->/g, "");
    const html = md.render(text);
    return html.replace(new RegExp(`<p>${MARK}([^@<]+)@@</p>`, "g"), (_m, id: string) => buttons(index, id, o));
}

export interface PanelOptions {
    cspSource: string;
    nonce: string;
    styleUri: string;
    scriptUri: string;
    canBack: boolean;
    canForward: boolean;
}

/** The whole help tab around a page's body: Back, Forward and "Search Help" above it. Only the extension's own
 * style and script load; nothing remote. */
export function helpHtml(body: string, o: PanelOptions): string {
    const csp = `default-src 'none'; style-src ${o.cspSource}; script-src 'nonce-${o.nonce}'; img-src ${o.cspSource};`;
    const back = o.canBack ? "" : " disabled";
    const forward = o.canForward ? "" : " disabled";
    return [
        "<!DOCTYPE html>",
        `<html lang="en"><head><meta charset="UTF-8">`,
        `<meta http-equiv="Content-Security-Policy" content="${csp}">`,
        `<meta name="viewport" content="width=device-width, initial-scale=1.0">`,
        `<link rel="stylesheet" href="${o.styleUri}"></head><body>`,
        `<nav class="toolbar"><button data-action="back" title="Back"${back}>&#8592; Back</button>` +
            `<button data-action="forward" title="Forward"${forward}>Forward &#8594;</button>` +
            `<button data-action="search" title="QB64: Search Help">Search Help</button></nav>`,
        `<main>${body}</main>`,
        `<script nonce="${o.nonce}" src="${o.scriptUri}"></script>`,
        "</body></html>",
    ].join("\n");
}
