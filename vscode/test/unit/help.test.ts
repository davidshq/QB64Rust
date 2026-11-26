import * as assert from "assert";
import { HelpExample, HelpIndex, HelpIndexData, HelpPage, pageNameOf } from "../../src/help/helpIndex";
import { colorLine, helpHtml, renderPage } from "../../src/help/renderPage";

function page(name: string, path: string, title: string, sentence: string, kind: HelpPage["kind"] = "keyword", examples: string[] = []): HelpPage {
    return { name, path, title, sentence, kind, examples };
}

function example(name: string, network = false): HelpExample {
    return { name, title: name, file: `examples/${name}.bas`, output: true, files: false, network };
}

const DATA: HelpIndexData = {
    pages: [
        page("_clamp", "entries/_clamp.md", "_CLAMP", "`_CLAMP(value, low, high)` holds a value between two bounds: below `low` it gives `low`.", "keyword", ["_clamp_1"]),
        page("_min", "entries/_min.md", "_MIN", "`_MIN` gives the lesser of two numbers and `_MAX` the greater."),
        page("left-str", "entries/left-str.md", "LEFT$", "`LEFT$` gives the first characters of a string."),
        page("_openhost", "entries/_openhost.md", "_OPENHOST", "`_OPENHOST` opens a port.", "keyword", ["_openhost_1"]),
        page("chapters/control-flow", "chapters/control-flow.md", "Control flow", "Blocks choose and repeat.", "chapter"),
        page("first-steps", "first-steps.md", "First steps", "This page is for someone who has never written a program.", "guide"),
    ],
    tree: [],
    groups: [],
    keywords: [
        { page: "_clamp", title: "_CLAMP", examples: [example("_clamp_1")] },
        { page: "_openhost", title: "_OPENHOST", examples: [example("_openhost_1", true)] },
    ],
};

const index = new HelpIndex("/help", DATA);

const CLAMP_PAGE = [
    "# _CLAMP",
    "",
    "`_CLAMP(value, low, high)` holds a value between two bounds.",
    "",
    "## Behavior",
    "",
    "Each argument is evaluated once.",
    '<!-- src: language/builtin-functions "_MIN, _MAX and _CLAMP" -->',
    "",
    "## Example",
    "",
    "<!-- begin generated: example:_clamp_1 -->",
    "```qb64",
    "health = _CLAMP(130, 0, 100) ' at most 100",
    'PRINT "Health:"; health',
    "```",
    "",
    "Output:",
    "",
    "```text",
    "Health: 100 ",
    "```",
    "<!-- end generated -->",
    "",
    "## See also",
    "",
    "[`_MIN`](_min.md), [Control flow](../chapters/control-flow.md#loops), [QB64pe](https://qb64phoenix.com/), " +
        "[D-031](../../DIVERGENCES-QB64PE.md), <b>raw</b>.",
    "",
].join("\n");

describe("the help (design D4 and D5 of editor-side-panel)", () => {
    it("the search finds _CLAMP by a word of its sentence", () => {
        assert.deepStrictEqual(
            index.search("bounds").map((p) => p.name),
            ["_clamp"],
        );
        assert.strictEqual(index.search("_clamp")[0].name, "_clamp");
        // A title that holds the query comes before a sentence that does.
        assert.deepStrictEqual(
            index.search("min").map((p) => p.name),
            ["_min"],
        );
        assert.strictEqual(index.search("").length, DATA.pages.length);
        assert.deepStrictEqual(index.search("two bounds value"), [DATA.pages[0]]);
    });

    it("a word's page by its spelling alone", () => {
        assert.strictEqual(pageNameOf("LEFT$"), "left-str");
        assert.strictEqual(pageNameOf("$INCLUDE"), "meta-include");
        assert.strictEqual(pageNameOf("LINE INPUT"), "line-input");
        assert.strictEqual(index.pageOfWord("_Clamp")?.name, "_clamp");
        assert.strictEqual(index.pageOfWord("left$")?.name, "left-str");
        assert.strictEqual(index.pageOfWord("DrawBall"), undefined);
    });

    it("a page's HTML holds its example, its recorded output and the three buttons", () => {
        const html = renderPage(index, DATA.pages[0], CLAMP_PAGE, { trusted: true });
        assert.ok(html.includes('<span class="keyword">_CLAMP</span>'), html);
        assert.ok(html.includes('<span class="comment">&#39; at most 100</span>'), html);
        assert.ok(html.includes('<span class="string">&quot;Health:&quot;</span>'), html);
        assert.ok(html.includes('<pre class="code text"><code>Health: 100 </code></pre>'), html);
        assert.ok(
            html.includes(
                '<div class="example-buttons" data-example="_clamp_1"><button class="primary" data-action="run">Run this example</button>' +
                    '<button data-action="open">Open as a new program</button><button data-action="copy">Copy</button></div>',
            ),
            html,
        );
        // The sources and the generated markers are comments, not shown; raw HTML is text.
        assert.ok(!html.includes("src:") && !html.includes("begin generated"), html);
        assert.ok(html.includes("&lt;b&gt;raw&lt;/b&gt;"), html);
        assert.ok(html.includes('<h2 id="see-also">See also</h2>'), html);
    });

    it("a link to another page becomes a message; a web link stays", () => {
        const html = renderPage(index, DATA.pages[0], CLAMP_PAGE, { trusted: true });
        assert.ok(html.includes('<a href="#" data-page="_min"><code>_MIN</code></a>'), html);
        assert.ok(html.includes('<a href="#" data-page="chapters/control-flow" data-anchor="loops">Control flow</a>'), html);
        assert.ok(html.includes('<a href="https://qb64phoenix.com/" data-web="https://qb64phoenix.com/"'), html);
        // A file of the repository outside the reference is its page on GitHub.
        assert.ok(html.includes('data-web="https://github.com/davidshq/QB64Rust/blob/main/docs/DIVERGENCES-QB64PE.md"'), html);
    });

    it("an untrusted workspace disables Run; a network example has none", () => {
        const html = renderPage(index, DATA.pages[0], CLAMP_PAGE, { trusted: false });
        assert.ok(html.includes('data-action="run" disabled title="Running needs a trusted workspace."'), html);
        const net = renderPage(index, DATA.pages[3], "# _OPENHOST\n\nx.\n\n<!-- begin generated: example:_openhost_1 -->\n```qb64\nPRINT 1\n```\n<!-- end generated -->\n", {
            trusted: true,
        });
        assert.ok(!net.includes('data-action="run"') && net.includes('data-action="copy"'), net);
    });

    it("QB64 is colored by comments, strings, numbers and words", () => {
        const words = new Set(["PRINT", "REM"]);
        assert.strictEqual(
            colorLine('PRINT x1; 3.5; "a" REM no', words),
            '<span class="keyword">PRINT</span> x1; <span class="number">3.5</span>; <span class="string">&quot;a&quot;</span> ' +
                '<span class="comment">REM no</span>',
        );
        assert.strictEqual(colorLine("print &HFF", words), '<span class="keyword">print</span> <span class="number">&amp;HFF</span>');
    });

    it("the tab allows only its own style and script", () => {
        const html = helpHtml("<p>x</p>", { cspSource: "vscode-webview://x", nonce: "N", styleUri: "s.css", scriptUri: "h.js", canBack: false, canForward: true });
        assert.ok(html.includes(`content="default-src 'none'; style-src vscode-webview://x; script-src 'nonce-N'; img-src vscode-webview://x;"`));
        assert.ok(html.includes('data-action="back" title="Back" disabled>'));
        assert.ok(html.includes('data-action="forward" title="Forward">'));
    });
});
