import * as assert from "assert";
import * as crypto from "crypto";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { closeAll, open, setSetting, writeFile } from "./helpers";

describe("language support", () => {
    afterEach(closeAll);

    for (const name of ["lang/hello.bas", "lang/lib.bi", "lang/lib.bm"]) {
        it(`${name} opens as qb64rust`, async () => {
            const editor = await open(writeFile(name, "PRINT 1\r\n"));
            assert.strictEqual(editor.document.languageId, "qb64rust");
        });
    }

    it("Toggle Line Comment uses '", async () => {
        const editor = await open(writeFile("lang/comment.bas", "PRINT 1\r\n"));
        editor.selection = new vscode.Selection(0, 0, 0, 0);
        await vscode.commands.executeCommand("editor.action.commentLine");
        assert.match(editor.document.lineAt(0).text, /^' ?PRINT 1$/);
    });

    it("SUB, FUNCTION and TYPE blocks fold", async () => {
        const text = ["SUB Foo", "    PRINT 1", "END SUB", "FUNCTION Bar%", "    Bar% = 1", "END FUNCTION", "TYPE T", "    x AS LONG", "END TYPE", ""].join("\r\n");
        const uri = writeFile("lang/fold.bas", text);
        await open(uri);
        const ranges = await vscode.commands.executeCommand<vscode.FoldingRange[]>("vscode.executeFoldingRangeProvider", uri);
        const pairs = ranges.map((r) => `${r.start}-${r.end}`);
        for (const expected of ["0-2", "3-5", "6-8"]) {
            assert.ok(pairs.some((p) => p.startsWith(expected.split("-")[0] + "-")), `fold starting at ${expected} in ${pairs.join(",")}`);
        }
    });

    it("CP437 default: bytes 128-255 in a string survive open and save", async () => {
        const bytes: number[] = [...Buffer.from('a$ = "', "latin1")];
        for (let b = 128; b <= 255; b++) {
            bytes.push(b);
        }
        bytes.push(...Buffer.from('"\r\nPRINT a$\r\n', "latin1"));
        const original = Buffer.from(bytes);
        const uri = writeFile("enc/cp437.bas", original);
        const editor = await open(uri);
        assert.ok(editor.document.getText().includes("╔"), "byte 0xC9 decoded as ╔");
        // Make the document dirty without changing its text, then save, so VS Code really writes the bytes.
        await editor.edit((b) => b.insert(new vscode.Position(1, 0), "x"));
        await editor.edit((b) => b.delete(new vscode.Range(1, 0, 1, 1)));
        assert.ok(editor.document.isDirty);
        await editor.document.save();
        assert.deepStrictEqual(fs.readFileSync(uri.fsPath), original);
    });

    // The check described in study/09 uses verification/cp437_all_bytes.bin (bytes 0-255 in order).
    const verification = path.resolve(__dirname, "../../../../../verification");
    const allBytes = path.join(verification, "cp437_all_bytes.bin");

    it("study 09: a file containing byte 0x00 is refused as binary by VS Code", async function () {
        if (!fs.existsSync(allBytes)) {
            this.skip();
        }
        const original = fs.readFileSync(allBytes);
        const expected = fs.readFileSync(path.join(verification, "cp437_all_bytes.sha256"), "utf8").trim().split(/\s+/)[0];
        assert.strictEqual(crypto.createHash("sha256").update(original).digest("hex"), expected);
        const uri = writeFile("enc/allbytes.bas", original);
        await assert.rejects(Promise.resolve(vscode.workspace.openTextDocument(uri)), /binary/);
    });

    it("study 09: bytes 0x01-0xFF survive open, edit, edit back and save, except a lone 0x0D", async function () {
        if (!fs.existsSync(allBytes)) {
            this.skip();
        }
        const original = fs.readFileSync(allBytes).subarray(1);
        const uri = writeFile("enc/bytes1to255.bas", original);
        const editor = await open(uri);
        await editor.edit((b) => b.insert(new vscode.Position(0, 0), "x"));
        await editor.edit((b) => b.delete(new vscode.Range(0, 0, 0, 1)));
        assert.ok(editor.document.isDirty);
        await editor.document.save();
        const saved = fs.readFileSync(uri.fsPath);
        const differing = [...original].map((b, i) => (saved[i] === b ? "" : `0x${b.toString(16)}`)).filter(Boolean);
        // VS Code reads a lone CR as a line break and writes it back as the document's EOL (LF here).
        assert.deepStrictEqual({ length: saved.length, differing }, { length: 255, differing: ["0xd"] });
        assert.strictEqual(saved[original.indexOf(0x0d)], 0x0a);
    });

    it("a workspace override to utf8 wins over the cp437 default", async () => {
        await setSetting("files", "encoding", "utf8", "qb64rust");
        try {
            const uri = writeFile("enc/utf8.bas", Buffer.from('PRINT "é"\r\n', "utf8"));
            const editor = await open(uri);
            assert.strictEqual(editor.document.lineAt(0).text, 'PRINT "é"');
        } finally {
            await setSetting("files", "encoding", undefined, "qb64rust");
        }
    });

    it("format on save is not turned on", () => {
        const value = vscode.workspace.getConfiguration("editor", { languageId: "qb64rust" }).get<boolean>("formatOnSave");
        assert.strictEqual(value, false);
    });
});
