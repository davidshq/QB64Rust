// The QB64 side panel (change editor-side-panel, tasks 4.1, 4.3 and 4.4): the four views, the Program view's Run
// button, and the SUBs and FUNCTIONs of a program with an include file.
import * as assert from "assert";
import * as vscode from "vscode";
import { api, closeAll, compiler, open, server, waitFor, writeFile } from "./helpers";

describe("the QB64 side panel", function () {
    after(closeAll);

    it("contributes the four views, which VS Code knows by id", async () => {
        const ext = vscode.extensions.getExtension("qb64rust.qb64rust")!;
        const views = ext.packageJSON.contributes.views.qb64 as { id: string; name: string }[];
        assert.deepStrictEqual(
            views.map((v) => v.name),
            ["Program", "SUBs & FUNCTIONs", "Examples", "Help"],
        );
        // VS Code adds a `<id>.focus` command for each view it has registered.
        const commands = await vscode.commands.getCommands(true);
        for (const id of ["qb64rust.program", "qb64rust.procedures", "qb64rust.examples", "qb64rust.help"]) {
            assert.ok(commands.includes(`${id}.focus`), `no view ${id}`);
        }
    });

    it("the Program view names the active program and its Run button builds it", async function () {
        if (!compiler) {
            this.skip();
        }
        const a = await api();
        const uri = writeFile("panel/run.bas", "PRINT 1\r\nSYSTEM\r\n");
        await open(uri);
        await waitFor("the Program view's program", () => a.program.state.kind === "program" && a.program.state.name === "run.bas");
        await vscode.commands.executeCommand("qb64rust.program.focus");
        const ran = await a.program.handle("run");
        assert.strictEqual(ran, true, a.builder.lastOutput);
        assert.ok(a.builder.lastCommand?.includes("run.bas"), a.builder.lastCommand);
        assert.deepStrictEqual(a.program.statusBar.shown, { run: "$(play) Run", status: "$(check) OK" });
    });

    it("another kind of file keeps the program, and hides the status bar items", async () => {
        const a = await api();
        await open(writeFile("panel/keep.bas", "PRINT 2\r\n"));
        await waitFor("keep.bas", () => a.program.state.kind === "program" && a.program.state.name === "keep.bas");
        await open(writeFile("panel/notes.md", "# notes\n"));
        await waitFor("the status bar hidden", () => a.program.statusBar.shown === undefined);
        assert.ok(a.program.state.kind === "program" && a.program.state.name === "keep.bas");
    });

    it("lists the SUBs and FUNCTIONs of a program and its include file", async function () {
        if (!server) {
            this.skip();
        }
        const a = await api();
        await a.server.ready;
        writeFile("panel/rules.bm", "FUNCTION Speed! (level)\r\n    Speed! = level * 2\r\nEND FUNCTION\r\n");
        const main = writeFile("panel/bounce.bas", "PRINT Speed!(1)\r\nDrawBall 1\r\nSUB DrawBall (x)\r\nEND SUB\r\n'$INCLUDE: 'rules.bm'\r\n");
        await open(main);
        const rows = await waitFor("the procedures", () => {
            const r = a.procedures.rows;
            return r.length === 3 && r;
        });
        assert.deepStrictEqual(
            rows.map((r) => [r.label, r.file]),
            [
                ["Main program", "bounce.bas"],
                ["DrawBall (x)", "bounce.bas"],
                ["Speed! (level)", "rules.bm"],
            ],
        );
        assert.strictEqual(rows[2].line, 0);
        assert.strictEqual(a.procedures.shown, true);
    });

    it("the SUBs & FUNCTIONs view is not shown for a program of one file", async function () {
        if (!server) {
            this.skip();
        }
        const a = await api();
        await a.server.ready;
        await open(writeFile("panel/alone.bas", "Greet\r\nSUB Greet\r\nEND SUB\r\n"));
        await waitFor("the one-file program's answer", () => a.procedures.rows.length === 2 && a.procedures.rows[1].label === "Greet");
        assert.strictEqual(a.procedures.shown, false);
    });

    it("an error in a closed include file in a folder shows in the Program view", async function () {
        if (!server) {
            this.skip();
        }
        const a = await api();
        await a.server.ready;
        // As Civil War Strategy: a main file that includes files of folders below it, none of them open.
        writeFile("app/rules/army.bm", "SUB Move\r\nEND SUB\r\n");
        writeFile("app/ui/menu.bm", "SUB Menu\r\n    x = (\r\nEND SUB\r\n");
        const main = writeFile("app/game.bas", "Menu\r\nMove\r\n'$INCLUDE: 'rules/army.bm'\r\n'$INCLUDE: 'ui/menu.bm'\r\n");
        await open(main);
        const s = await waitFor("the include file's error", () => {
            const st = a.program.state;
            return st.kind === "program" && st.name === "game.bas" && st.status.kind === "error" && st.status;
        });
        assert.strictEqual(s.first.file, "menu.bm");
        assert.strictEqual(s.first.line, 2);
        assert.deepStrictEqual(a.program.statusBar.shown?.status, "$(error) 1 error: line 2");
    });

    it("QB64 Home: not opened by itself with the setting off; the command opens one tab with a program opened before", async () => {
        const a = await api();
        // runTests.ts turns `qb64rust.showHomeOnStartup` off for the test window.
        assert.strictEqual(a.home.visible, false);
        assert.strictEqual(await a.home.openAtStartup(), false);
        const uri = writeFile("panel/recent.bas", "PRINT 3\r\n");
        await open(uri);
        await waitFor("the recent program", () => a.home.recent[0] === uri.fsPath);
        await vscode.commands.executeCommand("qb64rust.home");
        await waitFor("the page", () => a.home.visible && a.home.state?.recent[0]?.name === "recent.bas");
        await vscode.commands.executeCommand("qb64rust.home");
        const tabs = vscode.window.tabGroups.all.flatMap((g) => g.tabs).filter((t) => t.label === "QB64 Home");
        assert.strictEqual(tabs.length, 1, "one tab");
        assert.strictEqual(a.home.state?.showOnStartup, false);
    });
});
