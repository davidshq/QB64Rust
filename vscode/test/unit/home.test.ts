import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import { homeHtml } from "../../src/home/homeHtml";
import { HomeState, MAX_RECENT, addRecent, isInside, recentRows, setupRows, shouldOpenHome } from "../../src/home/homeModel";

const manifest = JSON.parse(fs.readFileSync(path.resolve(__dirname, "../../../package.json"), "utf8"));

describe("QB64 Home (spec editor/home; design D11 of editor-side-panel)", () => {
    it("opens by itself only in a QB64 window with no file open, while the setting is on", () => {
        const base = { enabled: true, folders: 0, hasQb64Files: false, openFiles: 0 };
        // A window with no folder.
        assert.strictEqual(shouldOpenHome(base), true);
        // A folder with QB64 files.
        assert.strictEqual(shouldOpenHome({ ...base, folders: 1, hasQb64Files: true }), true);
        // A folder of another project.
        assert.strictEqual(shouldOpenHome({ ...base, folders: 1 }), false);
        // A file is open.
        assert.strictEqual(shouldOpenHome({ ...base, openFiles: 1 }), false);
        // The checkbox was cleared.
        assert.strictEqual(shouldOpenHome({ ...base, enabled: false }), false);
    });

    it("the setting, the command and the activation at startup are in the manifest", () => {
        const setting = manifest.contributes.configuration.properties["qb64rust.showHomeOnStartup"];
        assert.strictEqual(setting.type, "boolean");
        assert.strictEqual(setting.default, true);
        assert.ok((manifest.contributes.commands as { command: string; title: string }[]).some((c) => c.command === "qb64rust.home" && c.title === "Home"));
        assert.ok((manifest.activationEvents as string[]).includes("onStartupFinished"));
    });

    it("a recent program goes first, once, and the list is cut", () => {
        assert.deepStrictEqual(addRecent(["/p/a.bas", "/p/b.bas"], "/p/b.bas", "linux"), ["/p/b.bas", "/p/a.bas"]);
        assert.deepStrictEqual(addRecent(["/p/a.bas"], "/p/A.bas", "linux"), ["/p/A.bas", "/p/a.bas"]);
        // Windows names a file in any case.
        assert.deepStrictEqual(addRecent(["C:\\p\\a.bas"], "c:\\P\\A.BAS", "win32"), ["c:\\P\\A.BAS"]);
        const many = Array.from({ length: 12 }, (_, n) => `/p/${n}.bas`);
        const cut = addRecent(many, "/p/new.bas", "linux");
        assert.strictEqual(cut.length, MAX_RECENT);
        assert.strictEqual(cut[0], "/p/new.bas");
    });

    it("the recent rows leave out a file that is gone, and an example's copy is not a recent program", () => {
        const a = path.join("games", "bounce.bas");
        const rows = recentRows([a, path.join("gone", "old.bas")], (f) => f === a);
        assert.deepStrictEqual(rows, [{ path: a, name: "bounce.bas", folder: "games" }]);
        const storage = path.resolve("storage");
        assert.strictEqual(isInside(path.join(storage, "examples", "first-steps", "hello", "hello.bas"), storage), true);
        assert.strictEqual(isInside(path.resolve("games", "bounce.bas"), storage), false);
    });

    it("the setup's rows: what is there, and a link for what an install would give", () => {
        const all = setupRows({ compiler: true, toolchain: { found: true }, emsdk: { found: true }, platform: "win32" });
        assert.deepStrictEqual(
            all.map((r) => [r.ok, r.label, r.action]),
            [
                [true, "The QB64 compiler", undefined],
                [true, "The C++ compiler", undefined],
                [true, "Programs for the web", undefined],
            ],
        );
        const none = setupRows({ compiler: true, toolchain: { found: false, installable: true }, emsdk: { found: false, installable: true }, platform: "win32" });
        assert.deepStrictEqual(none[1].action, { command: "installToolchain", text: "Install (190 MB)" });
        assert.deepStrictEqual(none[2].action, { command: "installEmsdk", text: "Install (815 MB)" });
        assert.strictEqual(none[2].note, "optional");
        // Linux builds with the system's g++: nothing is downloaded for it.
        const linux = setupRows({ compiler: true, toolchain: { found: false, installable: true }, emsdk: { found: false }, platform: "linux" });
        assert.strictEqual(linux[1].action, undefined);
        assert.ok(linux[1].note.includes("g++"));
        assert.strictEqual(setupRows({ compiler: false, toolchain: { found: false }, emsdk: { found: false }, platform: "win32" })[0].ok, false);
    });

    const state: HomeState = {
        recent: [{ path: "/p/games/<b>.bas", name: "<b>.bas", folder: "games" }],
        setup: setupRows({ compiler: true, toolchain: { found: false, installable: true }, emsdk: { found: true }, platform: "win32" }),
        showOnStartup: true,
    };

    it("the page: four tiles, the recent programs, the setup and the checkbox", () => {
        const html = homeHtml(state, { cspSource: "vscode-webview://x", nonce: "N" });
        for (const command of ["newProgram", "openProgram", "examples", "searchHelp"]) {
            assert.ok(html.includes(`<button class="tile${command === "newProgram" ? " primary" : ""}" data-command="${command}">`), command);
        }
        // A name is escaped, and a click sends the row's number.
        assert.ok(html.includes('<a href="#" role="button" data-command="recent" data-arg="0">&lt;b&gt;.bas</a> <span class="note">games</span>'), html);
        assert.ok(html.includes('data-command="installToolchain">Install (190 MB)</a>'), html);
        assert.ok(html.includes('data-command="page" data-arg="coming-from-qb45">'), html);
        assert.ok(html.includes('<input type="checkbox" data-command="toggle" checked>Show this page when a QB64 window starts'), html);
        const off = homeHtml({ ...state, recent: [], showOnStartup: false }, { cspSource: "vscode-webview://x", nonce: "N" });
        assert.ok(off.includes('<input type="checkbox" data-command="toggle">'), off);
        assert.ok(off.includes("The programs you open are listed here."), off);
    });

    it("the page allows no remote content and only its own script", () => {
        const html = homeHtml(state, { cspSource: "vscode-webview://x", nonce: "N" });
        assert.ok(html.includes(`content="default-src 'none'; style-src vscode-webview://x 'nonce-N'; script-src 'nonce-N';"`));
    });
});
