import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { closeAll, compiler, open, setText, writeFile } from "./helpers";

async function formatEdits(uri: vscode.Uri): Promise<vscode.TextEdit[]> {
    const edits = await vscode.commands.executeCommand<vscode.TextEdit[]>("vscode.executeFormatDocumentProvider", uri, { tabSize: 4, insertSpaces: true });
    return edits ?? [];
}

async function format(editor: vscode.TextEditor): Promise<vscode.TextEdit[]> {
    const edits = await formatEdits(editor.document.uri);
    const we = new vscode.WorkspaceEdit();
    we.set(editor.document.uri, edits);
    await vscode.workspace.applyEdit(we);
    return edits;
}

function tempFilesIn(dir: string): string[] {
    return fs.readdirSync(dir).filter((n) => n.startsWith(".qb64rust-fmt-"));
}

describe("formatting", () => {
    before(function () {
        if (!compiler) {
            this.skip();
        }
    });
    afterEach(closeAll);

    it("indents and sets keyword case; LF document stays LF; no temp file left", async () => {
        const uri = writeFile("fmt/basic.bas", "for i=1 to 3\nprint   i\nnext\n");
        const editor = await open(uri);
        assert.strictEqual(editor.document.eol, vscode.EndOfLine.LF);
        await format(editor);
        assert.strictEqual(editor.document.getText(), "For i = 1 To 3\n    Print i\nNext\n");
        assert.deepStrictEqual(tempFilesIn(path.dirname(uri.fsPath)), []);
    });

    it("formats unsaved text; the document stays dirty and the file is unchanged", async () => {
        const uri = writeFile("fmt/dirty.bas", "PRINT 1\r\n");
        const editor = await open(uri);
        await setText(editor, "for i=1 to 2\r\nprint i\r\nnext\r\n");
        await format(editor);
        assert.strictEqual(editor.document.getText(), "For i = 1 To 2\r\n    Print i\r\nNext\r\n");
        assert.ok(editor.document.isDirty);
        assert.strictEqual(fs.readFileSync(uri.fsPath, "latin1"), "PRINT 1\r\n");
    });

    it("relative includes resolve from the document's folder", async () => {
        writeFile("fmt/inc/inc.bi", "CONST K = 1\r\n");
        const uri = writeFile("fmt/inc/main.bas", "'$INCLUDE:'inc.bi'\r\nprint k\r\n");
        const editor = await open(uri);
        const edits = await format(editor);
        assert.ok(edits.length > 0);
        // -y takes the name's case from the CONST in inc.bi, which shows the include was read.
        assert.strictEqual(editor.document.getText(), "'$Include:'inc.bi'\r\nPrint K\r\n");
        assert.deepStrictEqual(vscode.languages.getDiagnostics(uri), []);
        assert.deepStrictEqual(tempFilesIn(path.dirname(uri.fsPath)), []);
    });

    it("CP437 characters in strings survive", async () => {
        const bytes = Buffer.concat([Buffer.from('a$ = "', "latin1"), Buffer.from([0xc9, 0xcd, 0xcd, 0xbb]), Buffer.from('"\r\nprint a$\r\n', "latin1")]);
        const editor = await open(writeFile("fmt/box.bas", bytes));
        assert.ok(editor.document.getText().includes("╔══╗"));
        await format(editor);
        assert.strictEqual(editor.document.getText(), 'a$ = "╔══╗"\r\nPrint a$\r\n');
    });

    it("uses the encoding the document was opened with, not the cp437 default", async () => {
        const uri = writeFile("fmt/utf8.bas", Buffer.from('a$ = "é"\r\nprint a$\r\n', "utf8"));
        const doc = await vscode.workspace.openTextDocument(uri, { encoding: "utf8" });
        const editor = await vscode.window.showTextDocument(doc);
        assert.strictEqual(doc.encoding, "utf8");
        await format(editor);
        assert.strictEqual(doc.getText(), 'a$ = "é"\r\nPrint a$\r\n');
    });

    it("syntax error: document unchanged, diagnostics on line 2, no temp file left", async () => {
        const text = "PRINT 1\r\nx = 1 +\r\n";
        const uri = writeFile("fmt/error.bas", text);
        const editor = await open(uri);
        const edits = await format(editor);
        assert.deepStrictEqual(edits, []);
        assert.strictEqual(editor.document.getText(), text);
        const diags = vscode.languages.getDiagnostics(uri);
        assert.deepStrictEqual(diags.map((d) => [d.range.start.line, d.message]), [[1, "Expected variable/value after '+'"]]);
        assert.deepStrictEqual(tempFilesIn(path.dirname(uri.fsPath)), []);
        await vscode.commands.executeCommand("notifications.clearAll");
    });

    it("untitled documents can be formatted", async () => {
        const doc = await vscode.workspace.openTextDocument({ language: "qb64rust", content: "print   1\n" });
        const editor = await vscode.window.showTextDocument(doc);
        await format(editor);
        assert.strictEqual(doc.getText(), "Print 1\n");
        await vscode.commands.executeCommand("workbench.action.revertAndCloseActiveEditor");
    });
});
