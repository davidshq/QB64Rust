import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import { parseQb64rustOutput } from "../../src/compiler/parseQb64rustOutput";

// Recorded with scripts/record-qb64rust-fixtures.sh: the source given by its absolute path, as a build gives it.
const fixtures = path.resolve(__dirname, "../../../test-fixtures/qb64rust");

/** `text` with the <FIXTURES> placeholder put back. The recordings were made on Windows, so the path after it is
 * written with `\`; elsewhere it gets the host's separator, as a build there prints it (m7-linux). */
function withFixtures(text: string): string {
    return text.replace(/<FIXTURES>([^\s:]*)/g, (_, rest: string) => fixtures + rest.split("\\").join(path.sep));
}

function parse(name: string, mainFile: string) {
    const output = withFixtures(fs.readFileSync(path.join(fixtures, `${name}.out.txt`), "latin1"));
    const exit = Number(fs.readFileSync(path.join(fixtures, `${name}.exit`), "utf8").trim());
    return parseQb64rustOutput(output, exit, path.join(fixtures, mainFile));
}

describe("parseQb64rustOutput, recorded fixtures", () => {
    it("errors in the main file, each with its column and the old compiler's wording as a second line", () => {
        assert.deepStrictEqual(parse("build_type_error", "type_error.bas"), {
            messages: [
                { severity: "error", file: undefined, line: 2, column: 6, message: "cannot store a number in a string variable\nQB64pe: Illegal string-number conversion" },
                { severity: "error", file: undefined, line: 3, column: 6, message: "label `nowhere` is not defined in the main module\nQB64pe: Label 'nowhere' not defined" },
            ],
            allMarked: false,
        });
    });

    it("an error in an included file names that file", () => {
        const r = parse("build_include_error", path.join("include_error", "main.bas"));
        assert.deepStrictEqual(r.messages, [
            {
                severity: "error",
                file: path.join(fixtures, "include_error", "lib.bi"),
                line: 3,
                column: 5,
                message: "cannot store a number in a string variable\nQB64pe: Illegal string-number conversion",
            },
        ]);
    });

    it("a marked error beside an unmarked one: not all marked", () => {
        const r = parse("build_marked", "marked.bas");
        assert.strictEqual(r.messages.length, 2);
        assert.strictEqual(r.messages[1].message, "not supported yet: `_DEVICES`");
        assert.strictEqual(r.allMarked, false);
    });

    it("only marked errors", () => {
        const r = parse("build_marked_only", "marked_only.bas");
        assert.deepStrictEqual(r.messages, [{ severity: "error", file: undefined, line: 2, column: 7, message: "not supported yet: `_DEVICES`" }]);
        assert.strictEqual(r.allMarked, true);
    });

    it("a warning of a build that succeeds; the build lines are not messages", () => {
        const r = parse("build_warning", "warning.bas");
        assert.strictEqual(r.messages.length, 1);
        assert.strictEqual(r.messages[0].severity, "warning");
        assert.strictEqual(r.messages[0].line, 1);
        assert.strictEqual(r.messages[0].column, 11);
        assert.strictEqual(r.allMarked, false);
    });
});

describe("parseQb64rustOutput, synthetic cases", () => {
    // Made-up paths (tools\repo_check allows `does\not\exist`).
    const main = "C:\\does\\not\\exist\\x.bas";

    it("a relative file name is taken from the main file's folder", () => {
        const r = parseQb64rustOutput("x.bas:1:1: error: a\r\ninc\\lib.bi:2:3: error: b\r\n2 errors\r\n", 1, main);
        assert.deepStrictEqual(
            r.messages.map((m) => m.file),
            [undefined, "C:\\does\\not\\exist\\inc\\lib.bi"],
        );
    });

    it("the main file in another case is still the main file", () => {
        const r = parseQb64rustOutput("c:\\does\\not\\exist\\X.BAS:1:1: error: a\n1 error\n", 1, main);
        assert.strictEqual(r.messages[0].file, undefined);
    });

    it("a failure without a diagnostic gives one error with the last lines", () => {
        const r = parseQb64rustOutput("building x.exe\nmake: *** [Makefile:1] Error 1\n", 1, main);
        assert.strictEqual(r.messages.length, 1);
        assert.match(r.messages[0].message, /^The build failed \(exit code 1\); see the "QB64" output\.\n/);
        assert.match(r.messages[0].message, /Error 1$/);
        assert.strictEqual(r.allMarked, false);
    });

    it("an internal compiler error (exit code 3)", () => {
        const r = parseQb64rustOutput("qb64rust: internal compiler error: boom at src\\x.rs:1:2\n", 3, main);
        assert.match(r.messages[0].message, /exit code 3/);
        assert.match(r.messages[0].message, /internal compiler error/);
    });

    it("nothing on success", () => {
        assert.deepStrictEqual(parseQb64rustOutput("", 0, main), { messages: [], allMarked: false });
    });
});
