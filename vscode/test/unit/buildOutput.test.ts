import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import { BuildOutputLines, progressText } from "../../src/compiler/buildOutput";

const fixtures = path.resolve(__dirname, "../../../test-fixtures/compiler");

/** Recorded `-c -x -w` build of clean.bas; the progress bar is about 50 redraws on one line. */
const recorded = fs.readFileSync(path.join(fixtures, "build_clean.out.txt"), "latin1");

const expected = [
    "QB64-PE Compiler V4.7.0-GLFW-UNKNOWN",
    "",
    "Beginning C++ output from QB64 code... ",
    "",
    "Compiling C++ code into EXE...",
    "Output: <OUT>\\check.exe",
];

function feed(chunks: string[]): string[] {
    const lines = new BuildOutputLines();
    return [...chunks.flatMap((c) => lines.push(c)), ...lines.flush()];
}

describe("BuildOutputLines", () => {
    it("recorded -x build: progress bar dropped, one line per line", () => {
        assert.deepStrictEqual(feed([recorded]), expected);
    });

    it("same result when the output arrives in small chunks", () => {
        const chunks: string[] = [];
        for (let i = 0; i < recorded.length; i += 7) {
            chunks.push(recorded.slice(i, i + 7));
        }
        assert.deepStrictEqual(feed(chunks), expected);
    });

    it("a line redrawn with \\r shows its last state", () => {
        assert.deepStrictEqual(feed(["a\rb\r\nc"]), ["b", "c"]);
    });
});

describe("BuildOutputLines progress", () => {
    function progress(chunks: string[]): string[] {
        const seen: string[] = [];
        const lines = new BuildOutputLines((p) => seen.push(progressText(p)));
        chunks.forEach((c) => lines.push(c));
        lines.flush();
        return seen;
    }

    // The recorded bar: 2%, 4%, …, 100%, with 100% drawn twice, then the C++ step.
    const expectedProgress = [
        ...Array.from({ length: 50 }, (_, i) => `generating C++ ${2 * (i + 1)}%`),
        "generating C++ 100%",
        "compiling C++…",
    ];

    it("recorded -x build: every redraw of the bar, then the C++ step", () => {
        assert.deepStrictEqual(progress([recorded]), expectedProgress);
    });

    it("reported while the line is still being drawn, not at its end", () => {
        const seen: string[] = [];
        const lines = new BuildOutputLines((p) => seen.push(progressText(p)));
        lines.push("[..   ] 2%\r[...  ] 4");
        assert.deepStrictEqual(seen, ["generating C++ 2%"]);
        lines.push("%\r");
        assert.deepStrictEqual(seen, ["generating C++ 2%", "generating C++ 4%"]);
    });

    it("flush after a killed build reports nothing (the run is over)", () => {
        const seen: string[] = [];
        const lines = new BuildOutputLines((p) => seen.push(progressText(p)));
        lines.push("[..   ] 2%\r[...  ] 4%");
        assert.deepStrictEqual(lines.flush(), []);
        assert.deepStrictEqual(seen, ["generating C++ 2%"]);
    });

    it("same progress when the output arrives in small chunks", () => {
        const chunks: string[] = [];
        for (let i = 0; i < recorded.length; i += 7) {
            chunks.push(recorded.slice(i, i + 7));
        }
        assert.deepStrictEqual(progress(chunks), expectedProgress);
    });
});
