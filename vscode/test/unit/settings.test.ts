import * as assert from "assert";
import { compilerFlags } from "../../src/compiler/buildSettings";
import { externalStart } from "../../src/compiler/externalRun";
import { runEnvironment } from "../../src/compiler/runEnv";
import * as qb64pe from "../../src/compiler/qb64pe";
import { buildArgs, fallbackPaths } from "../../src/compiler/qb64rust";
import { expandTabs, tabEdits } from "../../src/tabsPlan";

// Made-up paths (tools\repo_check allows `does\not\exist`).
const dir = "C:\\does\\not\\exist";

describe("build settings", () => {
    const none = { optimize: false, licenseFile: false, extraLinkerFlags: "" };

    it("a setting at its default gives no -f: setting", () => {
        assert.deepStrictEqual(compilerFlags(none), []);
        assert.deepStrictEqual(compilerFlags({ ...none, extraLinkerFlags: "   " }), []);
    });

    it("each setting gives the -f: setting both compilers take", () => {
        assert.deepStrictEqual(compilerFlags({ optimize: true, licenseFile: true, extraLinkerFlags: " -lfoo " }), [
            "-f:OptimizeCppProgram=true",
            "-f:GenerateLicenseFile=true",
            "-f:ExtraLinkerFlags=-lfoo",
        ]);
    });

    it("qb64rust: the -f: settings after -o and before the roots", () => {
        const file = `${dir}\\w\\hello.bas`;
        const args = buildArgs(file, { toolchain: "", include: `${dir}\\inc` }, false, fallbackPaths(file), ["-f:OptimizeCppProgram=true"]);
        assert.deepStrictEqual(args.slice(3), ["-o", fallbackPaths(file).exe, "-f:OptimizeCppProgram=true", "--include-root", `${dir}\\inc`]);
    });

    it("qb64pe: the -f: settings last", () => {
        const args = qb64pe.buildArgs(`${dir}\\w\\hello.bas`, ["-f:GenerateLicenseFile=true"]);
        assert.strictEqual(args[0], "-c");
        assert.strictEqual(args[args.length - 1], "-f:GenerateLicenseFile=true");
    });
});

describe("the run environment", () => {
    const off = { level: "none", scopes: ["qb64"], handlers: ["console"], file: "" };

    it("nothing to add at the defaults; QB64PE_NOPROMPT alone for noPrompt", () => {
        assert.strictEqual(runEnvironment(false, off), undefined);
        assert.deepStrictEqual(runEnvironment(true, off), { QB64PE_NOPROMPT: "y" });
    });

    it("a log level sets the four QB64PE_LOG_* variables, the file qb64pe-log.txt when none is named", () => {
        assert.deepStrictEqual(runEnvironment(false, { ...off, level: "trace", scopes: ["qb64", "libqb-audio"], handlers: ["console", "file"] }), {
            QB64PE_LOG_LEVEL: "trace",
            QB64PE_LOG_SCOPES: "qb64,libqb-audio",
            QB64PE_LOG_HANDLERS: "console,file",
            QB64PE_LOG_FILE_PATH: "qb64pe-log.txt",
        });
        assert.strictEqual(runEnvironment(false, { ...off, level: "error", file: " run.log " })?.QB64PE_LOG_FILE_PATH, "run.log");
    });
});

describe("run in a window of its own", () => {
    it("Windows: cmd's start, with the folder and the arguments as typed", () => {
        const s = externalStart("win32", `${dir}\\w`, `${dir}\\w\\hello.exe`, "a b", "");
        assert.deepStrictEqual(s, { command: "cmd.exe", args: ["/d", "/c", `start "QB64" /D "${dir}\\w" "${dir}\\w\\hello.exe" a b`], verbatim: true });
    });

    it("Linux: the external terminal with -e, x-terminal-emulator when none is set", () => {
        const s = externalStart("linux", "/w/it's", "/w/it's/hello", "", "");
        assert.strictEqual(s.command, "x-terminal-emulator");
        assert.deepStrictEqual(s.args, ["-e", "sh", "-c", "cd '/w/it'\\''s' && '/w/it'\\''s/hello'"]);
        assert.strictEqual(externalStart("linux", "/w", "/w/hello", "", " xterm ").command, "xterm");
    });
});

describe("tabs", () => {
    it("each tab goes to the next column that is a multiple of 4, as the QB64pe IDE loads a file", () => {
        assert.strictEqual(expandTabs("\tPRINT"), "    PRINT");
        assert.strictEqual(expandTabs("ab\tc"), "ab  c");
        assert.strictEqual(expandTabs("abc\t\tx"), "abc     x");
    });

    it("a tab inside a string stays; one in a comment is turned", () => {
        assert.strictEqual(expandTabs('\tPRINT "a\tb"\t\' x\ty'), '    PRINT "a\tb" \' x y');
        assert.strictEqual(expandTabs('x = 1 \' "\tq'), 'x = 1 \' "   q');
    });

    it("a tab in a DATA statement stays, after a line number or a `:` too; the indent before it is turned", () => {
        assert.strictEqual(expandTabs("\tDATA a\tb, 1"), "    DATA a\tb, 1");
        assert.strictEqual(expandTabs("10 data\ta"), "10 data\ta");
        assert.strictEqual(expandTabs("x = 1: DATA a\tb"), "x = 1: DATA a\tb");
        assert.strictEqual(expandTabs("dataset = 1\t' x"), "dataset = 1 ' x", "a name that starts with DATA");
        assert.strictEqual(expandTabs('PRINT "DATA\t": y\t= 2'), 'PRINT "DATA\t": y    = 2');
    });

    it("only the lines that change are edited", () => {
        assert.deepStrictEqual(tabEdits(["PRINT", '\tPRINT "\t"', 'PRINT "\t"']), [[1, '    PRINT "\t"']]);
    });
});
