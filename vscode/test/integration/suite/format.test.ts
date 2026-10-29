// Format Document and Format Selection by each formatter (change m5b-formatter, design D7; spec editor/formatting):
// `qb64pe -y` (the server stopped, the workspace's `buildWith` `qb64pe`), `qb64rust -y` (the server stopped,
// `buildWith` `qb64rust`) and the language server. The last two need QB64RUST_TEST_QB64RUST.
import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import type { RunRequest } from "../../../src/compiler/runQueue";
import { api, closeAll, compiler, open, qb64peDiagnostics, qb64rustDiagnostics, server, setSetting, setText, waitFor, writeFile } from "./helpers";

const OPTIONS = { tabSize: 4, insertSpaces: true };

async function formatEdits(uri: vscode.Uri): Promise<vscode.TextEdit[]> {
    const edits = await vscode.commands.executeCommand<vscode.TextEdit[]>("vscode.executeFormatDocumentProvider", uri, OPTIONS);
    return edits ?? [];
}

async function apply(editor: vscode.TextEditor, edits: vscode.TextEdit[]): Promise<vscode.TextEdit[]> {
    const we = new vscode.WorkspaceEdit();
    we.set(editor.document.uri, edits);
    await vscode.workspace.applyEdit(we);
    return edits;
}

async function format(editor: vscode.TextEditor): Promise<vscode.TextEdit[]> {
    return apply(editor, await formatEdits(editor.document.uri));
}

/** Format Selection over the whole of line `line`. */
async function formatLine(editor: vscode.TextEditor, line: number): Promise<vscode.TextEdit[]> {
    const range = editor.document.lineAt(line).range;
    const edits = await vscode.commands.executeCommand<vscode.TextEdit[]>("vscode.executeFormatRangeProvider", editor.document.uri, range, OPTIONS);
    return apply(editor, edits ?? []);
}

function tempFilesIn(dir: string): string[] {
    return fs.readdirSync(dir).filter((n) => n.startsWith(".qb64rust-fmt-"));
}

/** Records every compiler run the extension starts until `stop` is called. */
async function recordRuns(): Promise<{ runs: RunRequest[]; stop: () => void }> {
    const queue = (await api()).queue;
    const original = queue.run;
    const runs: RunRequest[] = [];
    queue.run = (r: RunRequest) => {
        runs.push(r);
        return original.call(queue, r);
    };
    return { runs, stop: () => (queue.run = original) };
}

async function stopServer(): Promise<void> {
    await (await api()).server.stop();
}

async function restartServer(): Promise<void> {
    const a = await api();
    a.server.ready = a.server.restart();
    await a.server.ready;
}

describe("formatting with qb64pe -y", () => {
    before(async function () {
        if (!compiler) {
            this.skip();
        }
        await stopServer();
    });
    after(restartServer);
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
        assert.deepStrictEqual(qb64peDiagnostics(uri), []);
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
        const diags = qb64peDiagnostics(uri);
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

    // Spec "Old compiler chosen" and "Lower case keywords": the settings reach qb64pe as -f: settings.
    it("the layout settings: keyword case lower, indent size 2", async () => {
        await setSetting("qb64rust", "format.keywordCase", "lower");
        await setSetting("qb64rust", "format.indentSize", 2);
        try {
            const recorder = await recordRuns();
            const editor = await open(writeFile("fmt/lower.bas", "PRINT 1\r\nIF 1 THEN\r\nPRINT 2\r\nEND IF\r\n"));
            await format(editor);
            recorder.stop();
            assert.strictEqual(editor.document.getText(), "print 1\r\nif 1 then\r\n  print 2\r\nend if\r\n");
            assert.deepStrictEqual(
                recorder.runs.map((r) => [r.kind, r.command]),
                [["format", compiler]],
            );
            assert.ok(recorder.runs[0].args.includes("-f:keywordlowercase=1"));
        } finally {
            await setSetting("qb64rust", "format.keywordCase", undefined);
            await setSetting("qb64rust", "format.indentSize", undefined);
        }
    });

    it("Format Selection changes only the selected line", async () => {
        const editor = await open(writeFile("fmt/range.bas", "for i=1 to 3\r\nprint   i\r\nnext\r\n"));
        await formatLine(editor, 1);
        assert.strictEqual(editor.document.getText(), "for i=1 to 3\r\n    Print i\r\nnext\r\n");
    });
});

describe("formatting with qb64rust -y", () => {
    before(async function () {
        if (!server) {
            console.log("    skipped: QB64RUST_TEST_QB64RUST is not set");
            this.skip();
        }
        await stopServer();
        await setSetting("qb64rust", "buildWith", "qb64rust");
    });
    after(async () => {
        // The test workspace's value (runTests.ts).
        await setSetting("qb64rust", "buildWith", "qb64pe");
        await restartServer();
    });
    afterEach(closeAll);

    it("formats with qb64rust and leaves no temp file", async () => {
        const recorder = await recordRuns();
        const uri = writeFile("fmtr/basic.bas", "for i=1 to 3\r\nprint   i\r\nnext\r\n");
        const editor = await open(uri);
        await format(editor);
        recorder.stop();
        assert.strictEqual(editor.document.getText(), "For i = 1 To 3\r\n    Print i\r\nNext\r\n");
        assert.deepStrictEqual(
            recorder.runs.map((r) => [r.kind, r.command]),
            [["format", server]],
        );
        assert.deepStrictEqual(tempFilesIn(path.dirname(uri.fsPath)), []);
    });

    it("keyword case lower", async () => {
        await setSetting("qb64rust", "format.keywordCase", "lower");
        try {
            const editor = await open(writeFile("fmtr/lower.bas", "PRINT 1\r\n"));
            await format(editor);
            assert.strictEqual(editor.document.getText(), "print 1\r\n");
        } finally {
            await setSetting("qb64rust", "format.keywordCase", undefined);
        }
    });

    it("Format Selection changes only the selected line", async () => {
        const editor = await open(writeFile("fmtr/range.bas", "for i=1 to 3\r\nprint   i\r\nnext\r\n"));
        await formatLine(editor, 1);
        assert.strictEqual(editor.document.getText(), "for i=1 to 3\r\n    Print i\r\nnext\r\n");
    });

    // Spec "Error is not retried".
    it("syntax error: document unchanged, the error on line 2, qb64pe not run", async () => {
        const recorder = await recordRuns();
        const text = "PRINT 1\r\nx = 1 +\r\n";
        const uri = writeFile("fmtr/error.bas", text);
        const editor = await open(uri);
        const edits = await format(editor);
        recorder.stop();
        assert.deepStrictEqual(edits, []);
        assert.strictEqual(editor.document.getText(), text);
        assert.deepStrictEqual(
            qb64rustDiagnostics(uri).map((d) => d.range.start.line),
            [1],
        );
        assert.deepStrictEqual(
            recorder.runs.map((r) => r.command),
            [server],
        );
        assert.deepStrictEqual(tempFilesIn(path.dirname(uri.fsPath)), []);
        await vscode.commands.executeCommand("notifications.clearAll");
    });
});

describe("formatting with the language server", () => {
    before(async function () {
        if (!server) {
            console.log("    skipped: QB64RUST_TEST_QB64RUST is not set");
            this.skip();
        }
        const a = await api();
        await a.server.ready;
        assert.ok(a.server.running, `the server did not start: ${a.server.text}`);
    });
    afterEach(closeAll);

    // Spec "Server running": the edits come from the server; no compiler run, no temp file, no choice asked (the
    // extension's provider is not registered beside the client's).
    it("formats unsaved text with no compiler run and no temp file", async () => {
        const recorder = await recordRuns();
        const uri = writeFile("fmts/basic.bas", "PRINT 1\r\n");
        const editor = await open(uri);
        await setText(editor, "for i=1 to 3\r\nprint   i\r\nnext\r\n");
        await format(editor);
        recorder.stop();
        assert.strictEqual(editor.document.getText(), "For i = 1 To 3\r\n    Print i\r\nNext\r\n");
        assert.ok(editor.document.isDirty);
        assert.deepStrictEqual(recorder.runs, []);
        assert.deepStrictEqual(tempFilesIn(path.dirname(uri.fsPath)), []);
    });

    // Spec "Lower case keywords" and "Setting changed while the server runs".
    it("the layout settings reach the server without a restart", async () => {
        const a = await api();
        const client = a.server.running;
        const editor = await open(writeFile("fmts/settings.bas", "PRINT 1\r\nIF 1 THEN\r\nPRINT 2\r\nEND IF\r\n"));
        try {
            await setSetting("qb64rust", "format.keywordCase", "lower");
            await format(editor);
            assert.strictEqual(editor.document.getText(), "print 1\r\nif 1 then\r\n    print 2\r\nend if\r\n");
            await setSetting("qb64rust", "format.indentSize", 2);
            await format(editor);
            assert.strictEqual(editor.document.getText(), "print 1\r\nif 1 then\r\n  print 2\r\nend if\r\n");
            assert.strictEqual(a.server.running, client, "the server was not restarted");
        } finally {
            await setSetting("qb64rust", "format.keywordCase", undefined);
            await setSetting("qb64rust", "format.indentSize", undefined);
        }
    });

    it("Format Selection changes only the selected line", async () => {
        const editor = await open(writeFile("fmts/range.bas", "for i=1 to 3\r\nprint   i\r\nnext\r\n"));
        await formatLine(editor, 1);
        assert.strictEqual(editor.document.getText(), "for i=1 to 3\r\n    Print i\r\nnext\r\n");
    });

    it("syntax error: document unchanged, the server's error on line 2", async () => {
        const text = "PRINT 1\r\nx = 1 +\r\n";
        const uri = writeFile("fmts/error.bas", text);
        const editor = await open(uri);
        const edits = await format(editor);
        assert.deepStrictEqual(edits, []);
        assert.strictEqual(editor.document.getText(), text);
        await waitFor("the server's error", () => qb64rustDiagnostics(uri).some((d) => d.range.start.line === 1));
        await vscode.commands.executeCommand("notifications.clearAll");
    });

    // Spec "Which formatter": a server that stopped by itself (the client stopped, not the extension) does not run, so
    // the extension's formatter comes back; with the test workspace's `buildWith` `qb64pe` that is `qb64pe -y`. Last in
    // the suite: it restarts the server after. Skipped without QB64RUST_TEST_QB64PE (the linux CI job), as the
    // suite "formatting with qb64pe -y" is.
    it("a server that stopped by itself gives formatting back to the extension", async function () {
        if (!compiler) {
            this.skip();
        }
        const a = await api();
        const client = a.server.running;
        assert.ok(client);
        try {
            await client.stop();
            await waitFor("the extension to see the stop", () => a.server.running === undefined);
            const recorder = await recordRuns();
            const editor = await open(writeFile("fmts/stopped.bas", "for i=1 to 3\r\nprint   i\r\nnext\r\n"));
            await format(editor);
            recorder.stop();
            assert.strictEqual(editor.document.getText(), "For i = 1 To 3\r\n    Print i\r\nNext\r\n");
            assert.deepStrictEqual(
                recorder.runs.map((r) => r.command),
                [compiler],
            );
        } finally {
            await a.server.restart();
        }
    });
});
