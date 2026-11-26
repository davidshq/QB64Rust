import * as assert from "assert";
import { fileName, hasIncludeFiles, procedureRows } from "../../src/panel/proceduresModel";
import type { Procedure, ProceduresResult } from "../../src/server/protocol";

const MAIN = "file:///c%3A/games/bounce.bas";
const RULES = "file:///c%3A/games/my%20rules.bm";

function proc(name: string, kind: "SUB" | "FUNCTION", params: string, uri: string, line: number): Procedure {
    return { name, kind, params, uri, range: { start: { line, character: 0 }, end: { line, character: 10 } } };
}

const answer: ProceduresResult = {
    main: MAIN,
    files: [MAIN, RULES],
    procedures: [proc("DrawBall", "SUB", "(x, y)", MAIN, 20), proc("Speed!", "FUNCTION", "(level)", RULES, 0), proc("bump", "SUB", "", MAIN, 30)],
};

describe("the SUBs and FUNCTIONs view's rows (design D3 of editor-side-panel)", () => {
    it("the main program first, then the program's order, each with its file", () => {
        const rows = procedureRows(answer, false);
        assert.deepStrictEqual(
            rows.map((r) => [r.label, r.file, r.kind, r.line]),
            [
                ["Main program", "bounce.bas", "main", 0],
                ["DrawBall (x, y)", "bounce.bas", "SUB", 20],
                ["Speed! (level)", "my rules.bm", "FUNCTION", 0],
                ["bump", "bounce.bas", "SUB", 30],
            ],
        );
        assert.strictEqual(rows[1].detail, "SUB DrawBall (x, y)");
        assert.strictEqual(rows[3].detail, "SUB bump");
        assert.strictEqual(rows[2].uri, RULES);
    });

    it("A to Z, whatever the case, the main program still first", () => {
        const rows = procedureRows(answer, true);
        assert.deepStrictEqual(
            rows.map((r) => r.label),
            ["Main program", "bump", "DrawBall (x, y)", "Speed! (level)"],
        );
    });

    it("the view is shown only for a program with an include file", () => {
        assert.strictEqual(hasIncludeFiles(answer), true);
        assert.strictEqual(hasIncludeFiles({ main: MAIN, files: [MAIN], procedures: answer.procedures }), false);
        assert.strictEqual(hasIncludeFiles({ main: null, files: [], procedures: [] }), false);
        assert.strictEqual(hasIncludeFiles(undefined), false);
    });

    it("no parse, no rows", () => {
        assert.deepStrictEqual(procedureRows(undefined, false), []);
        assert.deepStrictEqual(procedureRows({ main: null, files: [], procedures: [] }, false), []);
        assert.strictEqual(procedureRows({ main: MAIN, files: [MAIN], procedures: [] }, false).length, 1);
    });

    it("a file name is decoded", () => {
        assert.strictEqual(fileName(RULES), "my rules.bm");
    });
});
