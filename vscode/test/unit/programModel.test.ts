import * as assert from "assert";
import { programHtml } from "../../src/panel/programHtml";
import { ProgramDiagnostic, ProgramInputs, ProgramState, firstError, mainActivity, programState, statusBarText } from "../../src/panel/programModel";

const MAIN = "file:///c%3A/games/bounce.bas";
const RULES = "file:///c%3A/games/rules.bm";
const program = { uri: MAIN, name: "bounce.bas" };

function d(uri: string, file: string, line: number, severity: "error" | "warning", message = "expected THEN"): ProgramDiagnostic {
    return { uri, file, line, message, severity };
}

function state(extra: Partial<ProgramInputs>): ProgramState {
    return programState({ program, diagnostics: [], trusted: true, ...extra });
}

/** The last line of the page in every state. */
const FOOT = '\n<p class="foot"><a href="#" role="button" data-command="searchHelp">Search Help</a></p>';

/** The page's body without its script and its last line, the part a state changes. */
function body(s: ProgramState): string {
    const html = programHtml(s, { cspSource: "vscode-webview://x", nonce: "N" });
    const all = html.substring(html.indexOf("<body>") + "<body>".length, html.indexOf("<script")).trim();
    assert.ok(all.endsWith(FOOT), all);
    return all.slice(0, -FOOT.length);
}

const BUTTONS =
    '<p class="name" title="file:///c%3A/games/bounce.bas">bounce.bas</p>\n' +
    '<button class="run primary" data-command="run">&#9654; Run</button>\n' +
    '<div class="row"><button data-command="debug">Debug</button><button data-command="browser">Run in Browser</button>' +
    '<button data-command="build">Build</button></div>';

describe("the Program view's state (design D2 of editor-side-panel)", () => {
    it("no program yet", () => {
        const s = programState({ diagnostics: [], trusted: true });
        assert.deepStrictEqual(s, { kind: "none", trusted: true });
        assert.strictEqual(statusBarText(s), undefined);
        assert.strictEqual(
            body(s),
            '<p class="name">No program is open.</p>\n' +
                '<div class="row"><button class="primary" data-command="newProgram">New Program</button>' +
                '<button data-command="openProgram">Open a Program</button></div>\n' +
                '<p><a href="#" role="button" data-command="getStarted">Get Started with QB64</a></p>',
        );
    });

    it("OK", () => {
        const s = state({});
        assert.deepStrictEqual(s.kind === "program" && s.status, { kind: "ok", warnings: 0 });
        assert.strictEqual(statusBarText(s), "$(check) OK");
        assert.strictEqual(body(s), `${BUTTONS}\n<div class="status ok" role="status"><span class="mark">&#10003;</span> OK</div>`);
    });

    it("OK with warnings", () => {
        const s = state({ diagnostics: [d(MAIN, "bounce.bas", 3, "warning"), d(RULES, "rules.bm", 1, "warning")] });
        assert.deepStrictEqual(s.kind === "program" && s.status, { kind: "ok", warnings: 2 });
        assert.strictEqual(statusBarText(s), "$(check) OK, 2 warnings");
        assert.ok(body(s).endsWith('<span class="mark">&#10003;</span> OK, 2 warnings</div>'));
    });

    it("an error in the main file", () => {
        const s = state({ diagnostics: [d(MAIN, "bounce.bas", 10, "error", "expected THEN <here>")], errorPage: "if" });
        assert.strictEqual(statusBarText(s), "$(error) 1 error: line 10");
        assert.strictEqual(
            body(s),
            `${BUTTONS}\n<div class="status error" role="status"><div class="where">Line 10</div>` +
                '<div class="message">expected THEN &lt;here&gt;</div><div class="links">' +
                '<a href="#" role="button" data-command="goto">Go to line 10</a>' +
                '<a href="#" role="button" data-command="help">Help</a></div></div>',
        );
        // With the page's title, the link names the statement.
        const titled = state({ diagnostics: [d(MAIN, "bounce.bas", 10, "error")], errorPage: "if", errorPageTitle: "IF" });
        assert.ok(body(titled).includes('data-command="help">Help on IF</a>'), body(titled));
    });

    it("a message's backticks are drawn as code", () => {
        const s = state({ diagnostics: [d(MAIN, "bounce.bas", 2, "error", "expected `)` after <x>")] });
        assert.ok(body(s).includes('<div class="message">expected <code>)</code> after &lt;x&gt;</div>'), body(s));
    });

    it("errors in two files: the main file's first", () => {
        const diagnostics = [d(RULES, "rules.bm", 2, "error"), d(MAIN, "bounce.bas", 30, "error"), d(MAIN, "bounce.bas", 40, "error")];
        const s = state({ diagnostics });
        assert.ok(s.kind === "program" && s.status.kind === "error");
        assert.strictEqual(s.status.first.line, 30);
        assert.strictEqual(statusBarText(s), "$(error) 3 errors: line 30");
        assert.ok(body(s).includes('<div class="where">Line 30 <span class="note">(2 more errors)</span></div>'));
        assert.ok(!body(s).includes('data-command="help"'), "no page, no help link");
        // Only the include file's: it is named.
        const only = state({ diagnostics: [d(RULES, "rules.bm", 2, "error")] });
        assert.ok(body(only).includes('<div class="where">rules.bm, line 2</div>'));
    });

    it("the first error is ordered by main file, file name, then line", () => {
        const list = [d(RULES, "rules.bm", 1, "error"), d("file:///a.bi", "a.bi", 9, "error"), d("file:///a.bi", "a.bi", 4, "error")];
        assert.strictEqual(firstError(list, MAIN)?.line, 4);
        assert.strictEqual(firstError([d(MAIN, "bounce.bas", 1, "warning")], MAIN), undefined);
    });

    it("building, the first build and a download, over an error", () => {
        const diagnostics = [d(MAIN, "bounce.bas", 10, "error")];
        const text = (s: ProgramState) => (s.kind === "program" && s.status.kind === "busy" ? s.status.text : undefined);
        assert.strictEqual(text(state({ diagnostics, activity: { kind: "build" } })), "Building…");
        const first = state({ activity: { kind: "firstBuild" } });
        assert.strictEqual(text(first), "Building the QB64 runtime. It can take about a minute, and happens only the first time.");
        // Work that takes a while shows a moving bar; a plain build does not.
        assert.ok(body(first).endsWith('only the first time.<div class="bar"><i></i></div></div>'), body(first));
        assert.ok(!body(state({ activity: { kind: "build" } })).includes('class="bar"'));
        assert.strictEqual(statusBarText(first), "$(sync~spin) QB64");
        const download = state({ activity: { kind: "download", what: "C++ toolchain" } });
        assert.strictEqual(text(download), "Downloading the C++ toolchain. This happens only once.");
        assert.ok(body(download).endsWith('<div class="status busy" role="status">Downloading the C++ toolchain. This happens only once.<div class="bar"><i></i></div></div>'));
        assert.deepStrictEqual(mainActivity({ kind: "build" }, true, "C++ toolchain"), { kind: "download", what: "C++ toolchain" });
        assert.deepStrictEqual(mainActivity({ kind: "build" }, true, undefined), { kind: "firstBuild" });
        assert.deepStrictEqual(mainActivity({ kind: "check" }, false, undefined), { kind: "check" });
    });

    it("an untrusted workspace: the four buttons disabled, with why", () => {
        const s = state({ trusted: false });
        const b = body(s);
        assert.strictEqual((b.match(/ disabled>/g) ?? []).length, 4);
        assert.ok(b.includes('<p class="note">Running needs a trusted workspace. <a href="#" role="button" data-command="trust">Manage Workspace Trust</a></p>'));
    });

    it("the page allows no remote content and only its own script", () => {
        const html = programHtml(state({}), { cspSource: "vscode-webview://x", nonce: "N" });
        assert.ok(html.includes(`content="default-src 'none'; style-src vscode-webview://x 'nonce-N'; script-src 'nonce-N';"`));
        assert.ok(html.includes('<script nonce="N">'));
    });
});
