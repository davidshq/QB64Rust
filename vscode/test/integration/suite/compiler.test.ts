import * as assert from "assert";
import * as path from "path";
import * as qb64pe from "../../../src/compiler/qb64pe";
import { RunQueue } from "../../../src/compiler/runQueue";
import { compiler, writeFile } from "./helpers";

describe("compiler/qb64pe against the real compiler", () => {
    before(function () {
        if (!compiler) {
            this.skip();
        }
    });

    it("check: clean file gives no messages", async () => {
        const uri = writeFile("compiler/clean.bas", 'PRINT "Hello"\r\n');
        const run = await qb64pe.check(new RunQueue(), compiler, uri.fsPath, 30000);
        assert.strictEqual(run.exitCode, 0);
        assert.deepStrictEqual(run.messages, []);
    });

    it("check: erroneous file gives the error at its line", async () => {
        const uri = writeFile("compiler/error.bas", 'PRINT "start"\r\nx = 1 +\r\n');
        const run = await qb64pe.check(new RunQueue(), compiler, uri.fsPath, 30000);
        assert.strictEqual(run.exitCode, 1);
        assert.deepStrictEqual(run.messages, [{ severity: "error", file: undefined, line: 2, message: "Expected variable/value after '+'" }]);
    });

    it("check writes nothing next to the source", async () => {
        const uri = writeFile("compiler/nothing.bas", "PRINT 1\r\n");
        await qb64pe.check(new RunQueue(), compiler, uri.fsPath, 30000);
        const fs = await import("fs");
        assert.deepStrictEqual(fs.readdirSync(path.dirname(uri.fsPath)).filter((n) => n.startsWith("nothing")), ["nothing.bas"]);
    });
});
