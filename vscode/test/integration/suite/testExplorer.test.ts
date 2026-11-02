// The Test Explorer (change basic-unit-tests, task 5.2; spec editor/test-explorer). Needs QB64pe (its toolchain) and
// the qb64rust binary (QB64RUST_TEST_QB64RUST); skipped without either.
import * as assert from "assert";
import * as vscode from "vscode";
import { testKey } from "../../../src/compiler/testOutput";
import { api, compiler, server, waitFor, writeFile } from "./helpers";

describe("test explorer", function () {
    this.timeout(900000);

    before(function () {
        if (!compiler || !server) {
            console.log("    skipped: needs QB64pe and QB64RUST_TEST_QB64RUST");
            this.skip();
        }
    });

    it("lists a test file's SUBs at their lines and runs them: one passes, one fails at its SUB", async () => {
        const uri = writeFile(
            "utests/calc_test.bas",
            'SUB TestGood\r\n    AssertEqualInt 4, 2 + 2, "sum"\r\nEND SUB\r\n\r\nSUB TestBad\r\n    AssertEqualInt 4, 2 + 3, "sum"\r\nEND SUB\r\n',
        );
        const a = await api();
        await a.tests.ready;
        await a.tests.refresh(uri);
        const file = await waitFor("the test file's item", () => a.tests.controller.items.get(uri.toString()));
        const tests: [string, number | undefined][] = [];
        file.children.forEach((t) => tests.push([t.label, t.range?.start.line]));
        assert.deepStrictEqual(tests, [
            ["TestGood", 0],
            ["TestBad", 4],
        ]);
        await a.tests.run(new vscode.TestRunRequest([file]), new vscode.CancellationTokenSource().token);
        assert.deepStrictEqual(
            [...a.tests.lastStates].sort(),
            [
                [testKey(uri.fsPath, "TestBad"), "failed"],
                [testKey(uri.fsPath, "TestGood"), "passed"],
            ],
            `the last run: ${a.tests.lastArgs?.join(" ") ?? "none"}`,
        );
    });
});
