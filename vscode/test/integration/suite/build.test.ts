import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { closeAll, compiler, dismissingNotifications, open, setSetting, waitFor, writeFile, qb64peDiagnostics } from "./helpers";

const exeOf = (uri: vscode.Uri) => uri.fsPath.replace(/\.bas$/i, process.platform === "win32" ? ".exe" : "");

describe("build and run", function () {
    this.timeout(600000);

    before(function () {
        if (!compiler) {
            this.skip();
        }
    });
    afterEach(closeAll);

    it("Build writes the executable next to the source", async () => {
        const uri = writeFile("build/hello.bas", '$CONSOLE:ONLY\r\nPRINT "Hello"\r\n');
        await open(uri);
        const exe = await vscode.commands.executeCommand<string | undefined>("qb64rust.build");
        assert.strictEqual(exe, exeOf(uri));
        assert.ok(fs.existsSync(exeOf(uri)));
    });

    it("failed build: error in Problems, nothing runs", async () => {
        const uri = writeFile("build/broken.bas", "PRINT 1\r\nx = 1 +\r\n");
        await open(uri);
        const terminalsBefore = vscode.window.terminals.length;
        const ran = await dismissingNotifications(vscode.commands.executeCommand<boolean>("qb64rust.buildAndRun"));
        assert.strictEqual(ran, false);
        assert.ok(!fs.existsSync(exeOf(uri)));
        const diags = qb64peDiagnostics(uri);
        assert.deepStrictEqual(diags.map((d) => [d.range.start.line, d.message]), [[1, "Expected variable/value after '+'"]]);
        assert.strictEqual(vscode.window.terminals.length, terminalsBefore);
    });

    it("Run without an executable offers to build and does nothing when dismissed", async () => {
        const uri = writeFile("build/notbuilt.bas", "PRINT 1\r\n");
        await open(uri);
        const ran = await dismissingNotifications(vscode.commands.executeCommand<boolean>("qb64rust.run"));
        assert.strictEqual(ran, false);
        assert.ok(!fs.existsSync(exeOf(uri)));
    });

    it("Build and Run starts the program in terminal 'QB64' with qb64rust.runArguments", async () => {
        const uri = writeFile("build/args.bas", '$CONSOLE:ONLY\r\nOPEN "args.txt" FOR OUTPUT AS #1\r\nPRINT #1, COMMAND$\r\nCLOSE #1\r\nSYSTEM\r\n');
        const out = path.join(path.dirname(uri.fsPath), "args.txt");
        fs.rmSync(out, { force: true });
        await setSetting("qb64rust", "runArguments", "a b");
        try {
            await open(uri);
            const ran = await vscode.commands.executeCommand<boolean>("qb64rust.buildAndRun");
            assert.strictEqual(ran, true);
            assert.ok(vscode.window.terminals.some((t) => t.name === "QB64"));
            const text = await waitFor("args.txt", () => fs.existsSync(out) && fs.readFileSync(out, "latin1").trim(), 60000);
            assert.strictEqual(text, "a b");
        } finally {
            await setSetting("qb64rust", "runArguments", undefined);
        }
    });

    it("untitled document: asks to save, nothing is built", async () => {
        const doc = await vscode.workspace.openTextDocument({ language: "qb64rust", content: "PRINT 1\n" });
        await vscode.window.showTextDocument(doc);
        const exe = await dismissingNotifications(vscode.commands.executeCommand<string | undefined>("qb64rust.build"));
        assert.strictEqual(exe, undefined);
        await vscode.commands.executeCommand("workbench.action.revertAndCloseActiveEditor");
    });
});
