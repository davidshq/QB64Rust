import * as assert from "assert";
import * as vscode from "vscode";
import { api, closeAll, compiler, open, setSetting, waitFor, writeFile } from "./helpers";

describe("activation and compiler status", () => {
    after(closeAll);

    it("the extension is installed", () => {
        assert.ok(vscode.extensions.getExtension("qb64rust.qb64rust"));
    });

    it("language status follows qb64rust.compilerPath without reload", async function () {
        if (!compiler) {
            this.skip();
        }
        await open(writeFile("status.bas", "PRINT 1\r\n"));
        const a = await api();
        assert.ok(a.status.current.found, "compiler found from the workspace setting");
        assert.strictEqual(a.status.text, "qb64pe");
        assert.strictEqual(a.status.severity, vscode.LanguageStatusSeverity.Information);
        try {
            await setSetting("qb64rust", "compilerPath", "C:\\does\\not\\exist\\qb64pe.exe");
            await waitFor("status 'not found'", () => /not found/.test(a.status.text));
            assert.strictEqual(a.status.current.found, false);
            assert.strictEqual(a.status.severity, vscode.LanguageStatusSeverity.Error);
        } finally {
            await setSetting("qb64rust", "compilerPath", compiler);
        }
        await waitFor("status found again", () => a.status.text === "qb64pe");
    });

    it("language status shows a running compiler", async function () {
        if (!compiler) {
            this.skip();
        }
        const a = await api();
        a.status.setBusy("checking…");
        assert.strictEqual(a.status.text, "qb64pe: checking…");
        a.status.setBusy(undefined);
        assert.strictEqual(a.status.text, "qb64pe");
    });
});
