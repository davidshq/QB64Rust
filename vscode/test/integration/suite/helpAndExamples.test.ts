// Help, examples and "QB64: New Program" (change editor-side-panel, tasks 5.2 to 5.4 and 6.1), with the help folder
// runTests.ts makes. The dialogs are stood in for by giving the file, as newProject's tests do.
import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { api, closeAll, open, server, waitFor, workspace, writeFile } from "./helpers";

describe("help, examples and New Program", function () {
    afterEach(closeAll);

    it("the help tab opens _CLAMP beside the code, then _MIN in the same tab, and goes back", async () => {
        const a = await api();
        const editor = await open(writeFile("help/health.bas", "health = _CLAMP(health, 0, 100)\r\n"));
        assert.ok(await a.help.show("_clamp"));
        await waitFor("the help tab", () => a.help.title === "QB64 Help: _CLAMP");
        // The code stays visible, at the left.
        assert.ok(vscode.window.visibleTextEditors.some((e) => e.document === editor.document));
        assert.ok(await a.help.show("_min"));
        assert.strictEqual(a.help.title, "QB64 Help: _MIN");
        const tabs = vscode.window.tabGroups.all.flatMap((g) => g.tabs).filter((t) => t.label.startsWith("QB64 Help"));
        assert.strictEqual(tabs.length, 1, "one help tab");
        a.help.back();
        assert.strictEqual(a.help.title, "QB64 Help: _CLAMP");
        a.help.forward();
        assert.strictEqual(a.help.current, "_min");
    });

    it("Help for This Word: a built-in's page",async () => {
        const a = await api();
        const editor = await open(writeFile("help/word.bas", "health = _CLAMP(health, 0, 100)\r\n"));
        editor.selection = new vscode.Selection(0, 11, 0, 11);
        assert.strictEqual(await a.helpView.helpForWord(), "page");
        await waitFor("the page of _CLAMP", () => a.help.current === "_clamp");
    });

    it("Help for This Word: the program's SUB goes to its definition, and a word without a page to the search", async function () {
        if (!server) {
            this.skip();
        }
        const a = await api();
        await a.server.ready;
        const editor = await open(writeFile("help/ball.bas", "DrawBall 1\r\n' a comment\r\nSUB DrawBall (x)\r\nEND SUB\r\n"));
        editor.selection = new vscode.Selection(0, 2, 0, 2);
        assert.strictEqual(await a.helpView.helpForWord(), "definition");
        await waitFor("the SUB line", () => vscode.window.activeTextEditor?.selection.active.line === 2);
        editor.selection = new vscode.Selection(1, 4, 1, 4);
        assert.strictEqual(await a.helpView.helpForWord(), "search");
        await vscode.commands.executeCommand("workbench.action.closeQuickOpen");
    });

    it("the search finds _CLAMP by a word of its sentence and opens it", async () => {
        const a = await api();
        const chosen = await a.helpView.search("bounds", (names) => names.find((n) => n === "_clamp"));
        assert.strictEqual(chosen, "_clamp");
        await waitFor("the page of _CLAMP", () => a.help.current === "_clamp");
    });

    it("the Examples view: Hello opens as a file of its own, a copy with nothing to save", async () => {
        const a = await api();
        const groups = a.examples.rows.map((r) => r.label);
        assert.deepStrictEqual(groups, ["First steps", "Graphics", "Sound and music", "Small games", "One for every keyword"]);
        await vscode.commands.executeCommand("qb64rust.examples.open", "first-steps/hello");
        const doc = await waitFor("the example's editor", () => vscode.window.activeTextEditor?.document);
        // A real file in the extension's storage: closing it asks nothing, and Run builds it at once.
        assert.ok(!doc.isUntitled && !doc.isDirty);
        assert.strictEqual(path.basename(doc.uri.fsPath), "hello.bas");
        assert.ok(!doc.uri.fsPath.startsWith(a.helpIndex()!.root), "the shipped file is not the one opened");
        assert.strictEqual(doc.languageId, "qb64rust");
        assert.ok(doc.getText().includes('PRINT "Hello, world!"'));
    });

    describe("QB64: New Program", () => {
        const run = async (o: object) => (await api()).newProgram.run(o);

        it("a name given: the one file, open, and nothing else in its folder", async () => {
            const folder = path.join(workspace, "newprog", "games");
            fs.mkdirSync(folder, { recursive: true });
            const r = await run({ file: vscode.Uri.file(path.join(folder, "bounce.bas")) });
            assert.ok(r);
            assert.deepStrictEqual(fs.readdirSync(folder), ["bounce.bas"]);
            assert.strictEqual(fs.readFileSync(r.file, "latin1"), 'PRINT "Hello, world!"\r\n');
            assert.strictEqual(vscode.window.activeTextEditor?.document.uri.fsPath, r.file);
        });

        it("cancel writes nothing", async () => {
            const before = fs.readdirSync(workspace);
            assert.strictEqual(await run({ file: null }), undefined);
            assert.deepStrictEqual(fs.readdirSync(workspace), before);
        });

        it("a name without .bas gets it", async () => {
            const folder = path.join(workspace, "newprog", "plain");
            fs.mkdirSync(folder, { recursive: true });
            const r = await run({ file: vscode.Uri.file(path.join(folder, "bounce")) });
            assert.strictEqual(r && path.basename(r.file), "bounce.bas");
        });

        it("in a folder whose qb64rust.toml says UTF-8, the file is UTF-8; in a plain one, code page 437", async () => {
            const project = path.join(workspace, "newprog", "project");
            fs.mkdirSync(project, { recursive: true });
            fs.writeFileSync(path.join(project, "qb64rust.toml"), '[fixes]\nall = true\n\n[project]\nencoding = "utf-8"\n');
            const text = 'PRINT "╔═╗"\r\n';
            const u = await run({ file: vscode.Uri.file(path.join(project, "box.bas")), text });
            assert.ok(u?.utf8);
            assert.strictEqual(fs.readFileSync(u.file, "utf8"), text);
            assert.strictEqual(vscode.window.activeTextEditor?.document.encoding, "utf8");
            const plain = path.join(workspace, "newprog", "plain");
            fs.mkdirSync(plain, { recursive: true });
            const p = await run({ file: vscode.Uri.file(path.join(plain, "box.bas")), text });
            assert.ok(p && !p.utf8);
            // Code page 437: one byte a character.
            assert.deepStrictEqual([...fs.readFileSync(p.file).subarray(7, 10)], [0xc9, 0xcd, 0xbb]);
            assert.ok(!fs.existsSync(path.join(plain, "qb64rust.toml")));
        });
    });
});
