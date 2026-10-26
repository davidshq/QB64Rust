// Hover, references, rename and completion from the language server (change m5b-language-features, task 6.1; spec
// editor/language-server), started from the binary that QB64RUST_TEST_QB64RUST names. Skipped when it is unset.
import * as assert from "assert";
import * as path from "path";
import * as vscode from "vscode";
import { api, closeAll, open, server, writeFile } from "./helpers";

const PROGRAM = [
    "'$INCLUDE:'lib.bi'", // 0
    "DIM count AS _UNSIGNED LONG", // 1
    "count = 1", // 2
    "total = total + count", // 3
    "x$ = LEFT$(\"abc\", 2)", // 4
    "show", // 5
    "SUB show", // 6
    "    inner = 2", // 7
    "    PRINT in", // 8
    "END SUB", // 9
    "",
].join("\r\n");

/** The plain text of a hover's contents. */
function hoverText(hovers: vscode.Hover[]): string {
    return hovers
        .flatMap((h) => h.contents)
        .map((c) => (typeof c === "string" ? c : c.value))
        .join("\n");
}

describe("language features from the server", () => {
    let uri: vscode.Uri;

    before(async function () {
        if (!server) {
            console.log("    skipped: QB64RUST_TEST_QB64RUST is not set");
            this.skip();
        }
        const a = await api();
        await a.server.ready;
        assert.ok(a.server.running, `the server did not start: ${a.server.text}`);
        writeFile("features/lib.bi", "DIM SHARED total AS LONG\r\n");
        uri = writeFile("features/main.bas", PROGRAM);
    });
    afterEach(closeAll);

    it("hover on a variable and on a built-in", async () => {
        await open(uri);
        const variable = await vscode.commands.executeCommand<vscode.Hover[]>("vscode.executeHoverProvider", uri, new vscode.Position(2, 1));
        assert.ok(hoverText(variable).includes("count AS _UNSIGNED LONG"), hoverText(variable));
        const builtin = await vscode.commands.executeCommand<vscode.Hover[]>("vscode.executeHoverProvider", uri, new vscode.Position(4, 6));
        assert.ok(hoverText(builtin).includes("Left$(STRING, LONG) AS STRING"), hoverText(builtin));
    });

    it("references across an include", async () => {
        await open(uri);
        const refs = await vscode.commands.executeCommand<vscode.Location[]>("vscode.executeReferenceProvider", uri, new vscode.Position(3, 1));
        const places = refs.map((r) => [path.basename(r.uri.fsPath), r.range.start.line, r.range.start.character]);
        places.sort();
        assert.deepStrictEqual(places, [
            ["lib.bi", 0, 11],
            ["main.bas", 3, 0],
            ["main.bas", 3, 8],
        ]);
    });

    it("a rename, and a refused one", async () => {
        await open(uri);
        const edit = await vscode.commands.executeCommand<vscode.WorkspaceEdit>("vscode.executeDocumentRenameProvider", uri, new vscode.Position(1, 5), "tally");
        const edits = edit.get(uri).map((e) => [e.range.start.line, e.range.start.character, e.newText]);
        assert.deepStrictEqual(edits, [
            [1, 4, "tally"],
            [2, 0, "tally"],
            [3, 16, "tally"],
        ]);
        await assert.rejects(
            Promise.resolve(vscode.commands.executeCommand("vscode.executeDocumentRenameProvider", uri, new vscode.Position(1, 5), "print")),
            /reserved word/,
        );
    });

    it("completion inside a SUB", async () => {
        await open(uri);
        // The first request starts the index; ask until it answers from it.
        let labels: string[] = [];
        for (let i = 0; i < 50 && !labels.includes("inner"); i++) {
            const list = await vscode.commands.executeCommand<vscode.CompletionList>("vscode.executeCompletionItemProvider", uri, new vscode.Position(8, 12));
            labels = list.items.map((c) => (typeof c.label === "string" ? c.label : c.label.label));
            await new Promise((r) => setTimeout(r, 100));
        }
        assert.ok(labels.includes("inner"), "the SUB's own local");
        assert.ok(labels.includes("total"), "a shared variable");
        assert.ok(!labels.includes("count"), "the main module's own variable");
        assert.ok(labels.includes("_RGB32"), "a built-in");
    });
});
