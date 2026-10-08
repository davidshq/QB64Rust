import * as assert from "assert";
import * as vscode from "vscode";
import { api, closeAll, compiler, open, saveAndCheck, setSetting, setText, sleep, waitFor, writeFile, qb64peDiagnostics } from "./helpers";

const plus = "Expected variable/value after '+'";

describe("diagnostics", () => {
    before(function () {
        if (!compiler) {
            this.skip();
        }
    });
    afterEach(closeAll);

    it("error on save, then fixed on save", async () => {
        const editor = await open(writeFile("diag/save.bas", "PRINT 1\r\n"));
        await setText(editor, "PRINT 1\r\nx = 1 +\r\n");
        await saveAndCheck(editor);
        const diags = qb64peDiagnostics(editor.document.uri);
        assert.strictEqual(diags.length, 1);
        assert.strictEqual(diags[0].message, plus);
        assert.strictEqual(diags[0].severity, vscode.DiagnosticSeverity.Error);
        assert.strictEqual(diags[0].range.start.line, 1);

        await setText(editor, "PRINT 1\r\nx = 1 + 2\r\n");
        await saveAndCheck(editor);
        assert.deepStrictEqual(qb64peDiagnostics(editor.document.uri), []);
    });

    it("checkOnSave false: saving does not run the compiler", async () => {
        const a = await api();
        await setSetting("qb64rust", "checkOnSave", false);
        try {
            const editor = await open(writeFile("diag/off.bas", "PRINT 1\r\n"));
            await setText(editor, "x = 1 +\r\n");
            const before = a.diagnostics.lastCheck.get(editor.document.uri.toString());
            await editor.document.save();
            await sleep(1500);
            assert.strictEqual(a.diagnostics.lastCheck.get(editor.document.uri.toString()), before);
            assert.deepStrictEqual(qb64peDiagnostics(editor.document.uri), []);
        } finally {
            await setSetting("qb64rust", "checkOnSave", undefined);
        }
    });

    it("error in an include file is shown on that file", async () => {
        const inc = writeFile("diag/inc/inc.bi", "y = 2\r\ny = 1 +\r\n");
        const main = writeFile("diag/inc/main.bas", "PRINT 0\r\n");
        const editor = await open(main);
        await setText(editor, "PRINT \"main\"\r\n'$INCLUDE:'inc.bi'\r\nPRINT \"after\"\r\n");
        await saveAndCheck(editor);
        assert.deepStrictEqual(qb64peDiagnostics(main), []);
        const diags = qb64peDiagnostics(inc);
        assert.strictEqual(diags.length, 1);
        assert.strictEqual(diags[0].message, plus);
        assert.strictEqual(diags[0].range.start.line, 1);
        assert.strictEqual(diags[0].range.end.character, "y = 1 +".length);
    });

    it("unused-variable warning", async () => {
        const editor = await open(writeFile("diag/warn.bas", "PRINT 0\r\n"));
        await setText(editor, 'DIM unusedvar AS INTEGER\r\nPRINT "done"\r\n');
        await saveAndCheck(editor);
        const diags = qb64peDiagnostics(editor.document.uri);
        assert.strictEqual(diags.length, 1);
        assert.strictEqual(diags[0].severity, vscode.DiagnosticSeverity.Warning);
        assert.strictEqual(diags[0].message, "Unused variable: unusedvar% INTEGER");
        assert.strictEqual(diags[0].range.start.line, 0);
    });

    it("range covers the line from first to last non-blank character", async () => {
        const editor = await open(writeFile("diag/indent.bas", "PRINT 0\r\n"));
        await setText(editor, "IF 1 THEN\r\n    x = 1 +   \r\nEND IF\r\n");
        await saveAndCheck(editor);
        const [d] = qb64peDiagnostics(editor.document.uri);
        assert.deepStrictEqual([d.range.start.line, d.range.start.character, d.range.end.line, d.range.end.character], [1, 4, 1, 11]);
    });

    it("editing removes the document's diagnostics; closing too", async () => {
        const editor = await open(writeFile("diag/edit.bas", "PRINT 0\r\n"));
        await setText(editor, "PRINT 1\r\nx = 1 +\r\n");
        await saveAndCheck(editor);
        assert.strictEqual(qb64peDiagnostics(editor.document.uri).length, 1);
        await editor.edit((b) => b.insert(new vscode.Position(0, 0), " "));
        assert.deepStrictEqual(qb64peDiagnostics(editor.document.uri), []);

        await saveAndCheck(editor);
        assert.strictEqual(qb64peDiagnostics(editor.document.uri).length, 1);
        const uri = editor.document.uri;
        await vscode.commands.executeCommand("workbench.action.closeActiveEditor");
        await waitFor("diagnostics cleared on close", () => qb64peDiagnostics(uri).length === 0, 30000);
    });

    it("rapid saves: only the last result is shown", async () => {
        const a = await api();
        const editor = await open(writeFile("diag/rapid.bas", "PRINT 0\r\n"));
        const key = editor.document.uri.toString();
        const checks: Promise<void>[] = [];
        for (const errorLine of [2, 3, 4]) {
            const lines = ["PRINT 1", "PRINT 2", "PRINT 3", "PRINT 4"];
            lines[errorLine - 1] = "x = 1 +";
            await setText(editor, lines.join("\r\n") + "\r\n");
            await editor.document.save();
            await waitFor("check started", () => a.diagnostics.lastCheck.get(key) !== undefined);
            checks.push(a.diagnostics.lastCheck.get(key)!);
        }
        await Promise.all(checks);
        const diags = qb64peDiagnostics(editor.document.uri);
        assert.deepStrictEqual(diags.map((d) => d.range.start.line), [3]);
    });

    it("command qb64rust.check", async () => {
        const editor = await open(writeFile("diag/command.bas", "x = 1 +\r\n"));
        await vscode.commands.executeCommand("qb64rust.check");
        assert.strictEqual(qb64peDiagnostics(editor.document.uri).length, 1);
    });
});
