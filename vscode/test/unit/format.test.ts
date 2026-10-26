import * as assert from "assert";
import * as qb64pe from "../../src/compiler/qb64pe";
import * as qb64rust from "../../src/compiler/qb64rust";
import { chooseFormatter, selectionEdits, splitLines } from "../../src/formatPlan";
import { DEFAULT_FORMAT, formatFlags } from "../../src/server/protocol";

// Change m5b-formatter, design D7; spec editor/formatting. Made-up paths (tools\repo_check allows `does\not\exist`).
const dir = "C:\\does\\not\\exist";

describe("format arguments", () => {
    it("qb64rust -y: -q, the layout settings, the source, -o and the include root", () => {
        const args = qb64rust.formatArgs(`${dir}\\w\\.f.bas`, `${dir}\\t\\out.bas`, DEFAULT_FORMAT, `${dir}\\qb64pe`);
        assert.deepStrictEqual(args, ["-y", "-q", ...formatFlags(DEFAULT_FORMAT), `${dir}\\w\\.f.bas`, "-o", `${dir}\\t\\out.bas`, "--include-root", `${dir}\\qb64pe`]);
    });

    it("qb64rust -y: no --include-root when it is empty, for the compiler's default", () => {
        const args = qb64rust.formatArgs(`${dir}\\w\\.f.bas`, `${dir}\\t\\out.bas`, DEFAULT_FORMAT, "");
        assert.ok(!args.includes("--include-root"));
        assert.deepStrictEqual(args.slice(-2), ["-o", `${dir}\\t\\out.bas`]);
    });

    it("qb64pe -y: the same layout settings, so that config.ini plays no part", () => {
        const settings = { ...DEFAULT_FORMAT, keywordCase: "lower" as const };
        const args = qb64pe.formatArgs(`${dir}\\w\\.f.bas`, `${dir}\\t\\out.bas`, settings);
        assert.deepStrictEqual(args, ["-y", "-q", ...formatFlags(settings), `${dir}\\w\\.f.bas`, "-o", `${dir}\\t\\out.bas`]);
        assert.ok(args.includes("-f:keywordlowercase=1"));
    });
});

describe("which formatter", () => {
    it("the server while it runs, whatever the build", () => {
        for (const b of ["auto", "qb64rust", "qb64pe"] as const) {
            assert.strictEqual(chooseFormatter(true, true, b), "server");
        }
    });

    it("server off: qb64rust -y when the binary is found and the build is not QB64pe's", () => {
        assert.strictEqual(chooseFormatter(false, true, "auto"), "qb64rust");
        assert.strictEqual(chooseFormatter(false, true, "qb64rust"), "qb64rust");
        assert.strictEqual(chooseFormatter(false, true, "qb64pe"), "qb64pe");
    });

    it("server off and no binary: qb64pe -y", () => {
        assert.strictEqual(chooseFormatter(false, false, "auto"), "qb64pe");
        assert.strictEqual(chooseFormatter(false, false, "qb64rust"), "qb64pe");
    });
});

describe("format selection", () => {
    const lines = ["for i=1 to 3", "print   i", "next"];
    const formatted = ["For i = 1 To 3", "    Print i", "Next"];

    it("only the lines the selection touches", () => {
        assert.deepStrictEqual(selectionEdits(lines, formatted, 1, 1, 9), [{ line: 1, text: "    Print i" }]);
        assert.deepStrictEqual(selectionEdits(lines, formatted, 0, 1, 3), [
            { line: 0, text: "For i = 1 To 3" },
            { line: 1, text: "    Print i" },
        ]);
    });

    it("a selection that ends at the start of a line does not touch it", () => {
        assert.deepStrictEqual(selectionEdits(lines, formatted, 1, 2, 0), [{ line: 1, text: "    Print i" }]);
    });

    it("no edit for a line already laid out, or past the end", () => {
        assert.deepStrictEqual(selectionEdits(formatted, formatted, 0, 2, 4), []);
        assert.deepStrictEqual(selectionEdits(lines, formatted, 3, 5, 0), []);
    });

    it("lines: any line end, none after the last line end", () => {
        assert.deepStrictEqual(splitLines("a\r\nb\nc\r"), ["a", "b", "c"]);
        assert.deepStrictEqual(splitLines("a\r\nb"), ["a", "b"]);
        assert.deepStrictEqual(splitLines(""), [""]);
    });
});
