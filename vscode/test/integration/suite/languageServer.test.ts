// The language server (change m2-language-server, tasks 4.2 and 4.3; its check on save and by command,
// extension-diagnostics task 4.3), started from the binary that QB64RUST_TEST_QB64RUST names. Skipped when it is
// unset.
import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import {
    api,
    closeAll,
    compiler,
    dismissingNotifications,
    open,
    qb64peDiagnostics,
    qb64rustDiagnostics,
    saveAndCheck,
    server,
    setSetting,
    setText,
    sleep,
    waitFor,
    waitForAsync,
    writeFile,
} from "./helpers";

const PROGRAM = [
    "$CONSOLE:ONLY", // 0
    "CALL bump(3)", // 1
    "DO", // 2
    "    x = x + 1", // 3
    "    IF x > 2 THEN GOTO done", // 4
    "LOOP", // 5
    "done:", // 6
    "END", // 7
    "SUB bump (n)", // 8
    "    PRINT n", // 9
    "END SUB", // 10
    "",
].join("\r\n");

describe("language server", function () {
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

    it("shows qb64rust in the language status", async () => {
        assert.strictEqual((await api()).server.text, "qb64rust");
    });

    // Spec editor/language-server, "Included files": with qb64rust.includeRoot empty, the QB64pe folder.
    it("include from QB64pe's folder", async function () {
        if (!compiler || !fs.existsSync(path.join(path.dirname(compiler), "internal", "support", "color", "color0.bi"))) {
            this.skip();
        }
        const editor = await open(writeFile("ls/inc_root.bas", "PRINT 1\r\n"));
        const uri = editor.document.uri;
        await setText(editor, "'$INCLUDE:'internal/support/color/nothing.bi'\r\nPRINT 1\r\n");
        await waitFor("the missing include's error", () => qb64rustDiagnostics(uri).length === 1);
        await setText(editor, "'$INCLUDE:'internal/support/color/color0.bi'\r\nPRINT 1\r\n");
        await waitFor("no error: the file is found under the QB64pe folder", () => qb64rustDiagnostics(uri).length === 0);
    });

    it("syntax errors as you type, beside the old compiler's from a build", async function () {
        const editor = await open(writeFile("ls/diag.bas", "PRINT 1\r\n"));
        const uri = editor.document.uri;
        // Without a save: the server's error appears.
        await setText(editor, "PRINT 1\r\nx = 1 +\r\n");
        const live = await waitFor("a qb64rust diagnostic", () => qb64rustDiagnostics(uri).length > 0 && qb64rustDiagnostics(uri));
        assert.strictEqual(live[0].range.start.line, 1);
        assert.deepStrictEqual(qb64peDiagnostics(uri), []);
        // On save, the server's check: still its error alone.
        await saveAndCheck(editor);
        assert.strictEqual(qb64rustDiagnostics(uri).length, 1);
        assert.deepStrictEqual(qb64peDiagnostics(uri), []);
        if (compiler) {
            // A build's error (spec "Two sources") appears beside it; neither replaces the other.
            await dismissingNotifications(vscode.commands.executeCommand("qb64rust.build"));
            assert.strictEqual(qb64peDiagnostics(uri).length, 1);
            assert.strictEqual(qb64rustDiagnostics(uri).length, 1);
        }
        // An edit: the old compiler's go at once, the server's follow the fix.
        await setText(editor, "PRINT 1\r\nx = 1 + 2\r\n");
        assert.deepStrictEqual(qb64peDiagnostics(uri), []);
        await waitFor("the qb64rust diagnostic to go", () => qb64rustDiagnostics(uri).length === 0);
        if (!compiler) {
            this.test!.title += " (without qb64pe: the build part skipped)";
        }
    });

    /** Counts the old compiler's checks the run queue starts while `body` runs. */
    async function qb64peChecks(body: () => Promise<void>): Promise<number> {
        const queue = (await api()).queue;
        const run = queue.run.bind(queue);
        let checks = 0;
        queue.run = (request) => {
            if (request.kind === "check") {
                checks++;
            }
            return run(request);
        };
        try {
            await body();
        } finally {
            queue.run = run;
        }
        return checks;
    }

    it("error on save from the new compiler: every error, each at its column", async () => {
        const editor = await open(writeFile("ls/save.bas", "PRINT 1\r\n"));
        const uri = editor.document.uri;
        await setText(editor, "PRINT 1\r\nx$ = 1\r\nGOTO nowhere\r\n");
        const checks = await qb64peChecks(() => saveAndCheck(editor));
        assert.strictEqual(checks, 0, "the old compiler was run");
        const diags = await waitFor("the check's diagnostics", () => qb64rustDiagnostics(uri).length === 2 && qb64rustDiagnostics(uri));
        // `qb64rust` reports both at column 6: `$` and `nowhere`.
        assert.deepStrictEqual(
            diags.map((d) => [d.range.start.line, d.range.start.character, d.severity]),
            [
                [1, 5, vscode.DiagnosticSeverity.Error],
                [2, 5, vscode.DiagnosticSeverity.Error],
            ],
        );
        assert.match(diags[0].message, /^cannot store a number in a string variable\nQB64pe: /);
        assert.deepStrictEqual(qb64peDiagnostics(uri), []);
    });

    it("fixed on save", async () => {
        const editor = await open(writeFile("ls/fix.bas", "PRINT 1\r\n"));
        const uri = editor.document.uri;
        await setText(editor, "PRINT 1\r\nx$ = 1\r\n");
        await saveAndCheck(editor);
        await waitFor("the type error", () => qb64rustDiagnostics(uri).length === 1);
        await setText(editor, 'PRINT 1\r\nx$ = "1"\r\n');
        await saveAndCheck(editor);
        await waitFor("no diagnostic", () => qb64rustDiagnostics(uri).length === 0);
    });

    it("checkOnSave false: saving runs no check", async () => {
        const a = await api();
        await setSetting("qb64rust", "checkOnSave", false);
        try {
            const editor = await open(writeFile("ls/off.bas", "PRINT 1\r\n"));
            const uri = editor.document.uri;
            await setText(editor, "PRINT 1\r\nx$ = 1\r\n");
            const before = a.diagnostics.lastCheck.get(uri.toString());
            const checks = await qb64peChecks(async () => {
                await editor.document.save();
                await sleep(1500);
            });
            assert.strictEqual(checks, 0);
            assert.strictEqual(a.diagnostics.lastCheck.get(uri.toString()), before);
            // The live parse finds nothing in `x$ = 1`; only a check would.
            assert.deepStrictEqual(qb64rustDiagnostics(uri), []);
        } finally {
            await setSetting("qb64rust", "checkOnSave", undefined);
        }
    });

    it("command qb64rust.check on a saved document without changes", async () => {
        const editor = await open(writeFile("ls/command.bas", "PRINT 1\r\nx$ = 1\r\n"));
        const uri = editor.document.uri;
        assert.strictEqual(editor.document.isDirty, false);
        await vscode.commands.executeCommand("qb64rust.check");
        await waitFor("the command's diagnostics", () => qb64rustDiagnostics(uri).length === 1);
        assert.strictEqual(qb64rustDiagnostics(uri)[0].range.start.line, 1);
    });

    it("outline", async () => {
        const editor = await open(writeFile("ls/outline.bas", PROGRAM));
        const symbols = await waitForAsync("symbols", async () => {
            const s = await vscode.commands.executeCommand<vscode.DocumentSymbol[]>("vscode.executeDocumentSymbolProvider", editor.document.uri);
            return s && s.length > 0 ? s : undefined;
        });
        assert.deepStrictEqual(
            symbols.map((s) => [s.name, s.kind]),
            [
                ["done", vscode.SymbolKind.Key],
                ["bump", vscode.SymbolKind.Method],
            ],
        );
    });

    it("folds a DO loop", async () => {
        const editor = await open(writeFile("ls/fold.bas", PROGRAM));
        const ranges = await waitForAsync("folding ranges", async () => {
            const r = await vscode.commands.executeCommand<vscode.FoldingRange[]>("vscode.executeFoldingRangeProvider", editor.document.uri);
            return r && r.length > 0 ? r : undefined;
        });
        const spans = ranges.map((r) => [r.start, r.end]);
        assert.ok(spans.some(([s, e]) => s === 2 && e === 4), JSON.stringify(spans));
        assert.ok(spans.some(([s, e]) => s === 8 && e === 9), JSON.stringify(spans));
    });

    it("goes to the definition of a CALL and a GOTO", async () => {
        const editor = await open(writeFile("ls/def.bas", PROGRAM));
        const uri = editor.document.uri;
        const definition = async (line: number, character: number) => {
            const pos = new vscode.Position(line, character);
            const found = await waitForAsync(`a definition at ${line}:${character}`, async () => {
                const r = await vscode.commands.executeCommand<(vscode.Location | vscode.LocationLink)[]>("vscode.executeDefinitionProvider", uri, pos);
                return r && r.length > 0 ? r : undefined;
            });
            const first = found[0];
            return "targetUri" in first ? first.targetRange.start : first.range.start;
        };
        // `bump` in `CALL bump(3)` -> `SUB bump` on line 8; `done` in `GOTO done` -> `done:` on line 6.
        assert.deepStrictEqual(await definition(1, 6), new vscode.Position(8, 4));
        assert.deepStrictEqual(await definition(4, 25), new vscode.Position(6, 0));
    });
});
