import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import { parseQb64peOutput, CompilerMessage } from "../../src/compiler/parseQb64peOutput";

const fixtures = path.resolve(__dirname, "../../../test-fixtures/compiler");

/** Reads a recorded output with the <FIXTURES> placeholder put back, as the parser would see it. */
function recorded(name: string): { output: string; exit: number } {
    const output = fs.readFileSync(path.join(fixtures, `${name}.out.txt`), "latin1").split("<FIXTURES>").join(fixtures);
    const exit = Number(fs.readFileSync(path.join(fixtures, `${name}.exit`), "utf8").trim());
    return { output, exit };
}

function parse(name: string, mainFile: string): CompilerMessage[] {
    const { output, exit } = recorded(name);
    return parseQb64peOutput(output, exit, { mainFile: path.join(fixtures, mainFile), exists: fs.existsSync });
}

const expectedError: CompilerMessage = { severity: "error", file: undefined, line: 2, message: "Expected variable/value after '+'" };

describe("parseQb64peOutput, recorded fixtures", () => {
    it("clean file: nothing", () => {
        assert.deepStrictEqual(parse("check_clean", "clean.bas"), []);
    });

    it("error in the main file", () => {
        assert.deepStrictEqual(parse("check_error_main", "error_main.bas"), [expectedError]);
    });

    it("error on an indented line", () => {
        assert.deepStrictEqual(parse("check_error_indented", "error_indented.bas"), [expectedError]);
    });

    it("error in an include: include file and its line, LINE n of the $INCLUDE ignored", () => {
        assert.deepStrictEqual(parse("check_include_error", "include_error/main.bas"), [
            { severity: "error", file: path.join(fixtures, "include_error", "inc.bi"), line: 2, message: "Expected variable/value after '+'" },
        ]);
    });

    it("missing include: reported on the $INCLUDE line of the main file", () => {
        assert.deepStrictEqual(parse("check_missing_include", "missing_include/main.bas"), [
            { severity: "error", file: undefined, line: 2, message: "File nothere.bi not found" },
        ]);
    });

    it("unused-variable warning with the detail appended, whitespace collapsed", () => {
        assert.deepStrictEqual(parse("check_warning_unused", "warning_unused.bas"), [
            { severity: "warning", file: undefined, line: 1, message: "Unused variable: unusedvar% INTEGER" },
        ]);
    });

    it("-x output: banner and progress bar ignored", () => {
        assert.deepStrictEqual(parse("check_error_progress", "error_main.bas"), [expectedError]);
    });

    it("-y success: nothing", () => {
        assert.deepStrictEqual(parse("format_ok", "format_ok.bas"), []);
        assert.deepStrictEqual(parse("format_cp437", "format_cp437.bas"), []);
    });

    it("-y error: same as a check", () => {
        assert.deepStrictEqual(parse("format_error", "error_main.bas"), [expectedError]);
    });
});

describe("parseQb64peOutput, synthetic cases", () => {
    const main = path.resolve("/proj/main.bas");

    it("CRLF and LF give the same result", () => {
        const lf = "\nExpected variable/value after '+'\nCaused by (or after):X = 1 +\nLINE 2:x = 1 +\n";
        const crlf = lf.replace(/\n/g, "\r\n");
        assert.deepStrictEqual(parseQb64peOutput(crlf, 1, { mainFile: main }), parseQb64peOutput(lf, 1, { mainFile: main }));
        assert.deepStrictEqual(parseQb64peOutput(lf, 1, { mainFile: main }), [expectedError]);
    });

    it("unrecognised failure: one error on line 1 with the output", () => {
        const result = parseQb64peOutput("Something unexpected\r\n", 1, { mainFile: main });
        assert.strictEqual(result.length, 1);
        assert.strictEqual(result[0].severity, "error");
        assert.strictEqual(result[0].line, 1);
        assert.strictEqual(result[0].file, undefined);
        assert.match(result[0].message, /Something unexpected/);
    });

    it("C++ step failure: the ERROR line and its hint, not the banner", () => {
        // Shape from verification/v06_else_level.compile.txt (-c -x build).
        const out = "QB64-PE Compiler V4.7.0-GLFW-UNKNOWN\r\n\r\nBeginning C++ output from QB64 code... \r\n[....] 50%\r[.....] 100%\r\n\r\nCompiling C++ code into executable...\r\nERROR: C++ compilation failed.\r\nCheck .\\internal\\temp\\compilelog.txt for details.\r\n";
        assert.deepStrictEqual(parseQb64peOutput(out, 1, { mainFile: main }), [
            { severity: "error", file: undefined, line: 1, message: "ERROR: C++ compilation failed. Check .\\internal\\temp\\compilelog.txt for details." },
        ]);
    });

    it("C++ step failure after warnings: the warnings and the failure", () => {
        const out = "main.bas:1: warning: Unused variable\r\n    q&   LONG\r\n\r\nCompiling C++ code into executable...\r\nERROR: C++ compilation failed.\r\nCheck .\\internal\\temp\\compilelog.txt for details.\r\n";
        assert.deepStrictEqual(parseQb64peOutput(out, 1, { mainFile: main }), [
            { severity: "warning", file: undefined, line: 1, message: "Unused variable: q& LONG" },
            { severity: "error", file: undefined, line: 1, message: "ERROR: C++ compilation failed. Check .\\internal\\temp\\compilelog.txt for details." },
        ]);
    });

    it("failure without any output still gives an error", () => {
        const result = parseQb64peOutput("", 3, { mainFile: main });
        assert.strictEqual(result.length, 1);
        assert.match(result[0].message, /exit code 3/);
    });

    it("unparsable output with exit 0 is not an error", () => {
        assert.deepStrictEqual(parseQb64peOutput("some chatter\n", 0, { mainFile: main }), []);
    });

    it("message not in any known list is still reported (structure, not prefixes)", () => {
        const out = "\nA brand new message nobody has seen\nCaused by (or after):FOO\nLINE 7:foo\n";
        assert.deepStrictEqual(parseQb64peOutput(out, 1, { mainFile: main }), [
            { severity: "error", file: undefined, line: 7, message: "A brand new message nobody has seen" },
        ]);
    });

    it("warning in another file resolves against the main folder, then include folders", () => {
        const incDir = path.resolve("/proj/lib");
        const exists = (p: string) => p === path.join(incDir, "util.bi");
        const out = "util.bi:4: warning: Unused variable\n    q&   LONG\n" + "\nOops\x01 in line 3 of " + path.join(incDir, "other.bi") + " included\nCaused by (or after):X\nLINE 1:x\n";
        const result = parseQb64peOutput(out, 1, { mainFile: main, exists });
        assert.deepStrictEqual(result, [
            { severity: "error", file: path.join(incDir, "other.bi"), line: 3, message: "Oops" },
            { severity: "warning", file: path.join(incDir, "util.bi"), line: 4, message: "Unused variable: q& LONG" },
        ]);
    });

    it("warning whose file cannot be found goes to the main file", () => {
        const result = parseQb64peOutput("nowhere.bi:2: warning: Something\n", 0, { mainFile: main, exists: () => false });
        assert.deepStrictEqual(result, [{ severity: "warning", file: undefined, line: 2, message: "Something" }]);
    });

    it("warning with a column (qb64rust's form): the column is not read as the line", () => {
        const result = parseQb64peOutput("p.bas:2:11: warning: Something\n", 0, { mainFile: main, exists: () => false });
        assert.deepStrictEqual(result, [{ severity: "warning", file: undefined, line: 2, message: "Something" }]);
    });
});
