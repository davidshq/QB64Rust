import * as assert from "assert";
import * as path from "path";
import { buildArgs, fallbackPaths, newerProgram, parsePrintPaths, printPathsArgs, printPathsProblem, programPaths } from "../../src/compiler/qb64rust";
import { adapterArgs } from "../../src/debugConfig";
import { toolchainRoot } from "../../src/compiler/toolchain";

// Design D4 of extension-vsix, design D6 of m6-toolchain: the command line of a build with qb64rust. Made-up paths
// (tools\repo_check allows `does\not\exist`).
const dir = "C:\\does\\not\\exist";

describe("qb64rust build arguments", () => {
    it("the executable next to the source, the toolchain, the QB64pe folder, the prepared libqb and the include root", () => {
        const args = buildArgs(`${dir}\\w\\hello.bas`, {
            toolchain: `${dir}\\qb64pe\\internal\\c\\c_compiler`,
            qb64pe: `${dir}\\qb64pe`,
            libqb: `${dir}\\s\\libqb-0.2.0`,
            include: `${dir}\\qb64pe`,
        });
        const exe = process.platform === "win32" ? `${dir}\\w\\hello.exe` : `${dir}\\w\\hello`;
        assert.deepStrictEqual(args, [
            "-x",
            "-w",
            `${dir}\\w\\hello.bas`,
            "-o",
            exe,
            "--toolchain-root",
            `${dir}\\qb64pe\\internal\\c\\c_compiler`,
            "--qb64pe-root",
            `${dir}\\qb64pe`,
            "--libqb-root",
            `${dir}\\s\\libqb-0.2.0`,
            "--include-root",
            `${dir}\\qb64pe`,
        ]);
    });

    it("no --libqb-root without a bundle: the compiler's own default", () => {
        const args = buildArgs(`${dir}\\w\\hello.bas`, { toolchain: `${dir}\\t`, qb64pe: `${dir}\\qb64pe`, include: `${dir}\\inc` });
        assert.ok(!args.includes("--libqb-root"));
        assert.deepStrictEqual(args.slice(-2), ["--include-root", `${dir}\\inc`]);
    });

    it("without QB64pe: --toolchain-root only, no --qb64pe-root, and no --include-root when it is empty", () => {
        const args = buildArgs(`${dir}\\w\\hello.bas`, { toolchain: `${dir}\\t`, include: "" });
        assert.deepStrictEqual(args.slice(-2), ["--toolchain-root", `${dir}\\t`]);
        assert.ok(!args.includes("--qb64pe-root"));
        assert.ok(!args.includes("--include-root"));
    });

    it("--debug first only for a debug build (change m5-debug-launch, design D4)", () => {
        const roots = { toolchain: `${dir}\\t`, qb64pe: `${dir}\\qb64pe`, include: `${dir}\\inc` };
        const plain = buildArgs(`${dir}\\w\\hello.bas`, roots);
        assert.ok(!plain.includes("--debug"));
        assert.deepStrictEqual(buildArgs(`${dir}\\w\\hello.bas`, roots, true), ["--debug", ...plain]);
    });

    it("the system's toolchain on Linux: no --toolchain-root, the compiler finds it on PATH (m7-linux)", () => {
        const root = toolchainRoot({ folder: "/usr/bin", source: "system" });
        assert.strictEqual(root, undefined);
        assert.ok(!buildArgs("/w/hello.bas", { toolchain: root, include: "" }).includes("--toolchain-root"));
        assert.strictEqual(toolchainRoot({ folder: `${dir}\\t`, source: "installed" }), `${dir}\\t`);
    });
});

// Change build-output-folder, design D5: the paths from `qb64rust --print-paths`, or next to the source without it.
describe("program paths", () => {
    const file = `${dir}\\w\\game.bas`;
    const roots = { toolchain: `${dir}\\t`, include: "" };

    it("asks the compiler with --print-paths and the file", () => {
        assert.deepStrictEqual(printPathsArgs(file), ["--print-paths", file]);
    });

    it("reads exe and run_in from the compiler's line of JSON", () => {
        const line = JSON.stringify({ exe: `${dir}\\w\\build\\game.exe`, run_in: `${dir}\\w\\build` });
        assert.deepStrictEqual(parsePrintPaths(`${line}\r\n`), { exe: `${dir}\\w\\build\\game.exe`, runIn: `${dir}\\w\\build`, fromCompiler: true });
    });

    it("anything else is no paths: an older compiler's usage, a settings file's error, a part missing", () => {
        assert.strictEqual(parsePrintPaths("unknown option `--print-paths`\nusage: qb64rust ..."), undefined);
        assert.strictEqual(parsePrintPaths(`${dir}\\w\\qb64rust.toml:2: unknown build setting \`out\``), undefined);
        assert.strictEqual(parsePrintPaths(JSON.stringify({ exe: "x.exe" })), undefined);
        assert.strictEqual(parsePrintPaths(""), undefined);
    });

    it("the fallback: next to the source, run in its folder, built with -o", () => {
        const exe = process.platform === "win32" ? `${dir}\\w\\game.exe` : `${dir}\\w\\game`;
        const paths = fallbackPaths(file);
        assert.deepStrictEqual(paths, { exe, runIn: path.dirname(file), fromCompiler: false });
        assert.deepStrictEqual(buildArgs(file, roots, false, paths).slice(0, 5), ["-x", "-w", file, "-o", exe]);
    });

    it("no -o when the paths came from the compiler", () => {
        const paths = { exe: `${dir}\\w\\build\\game.exe`, runIn: `${dir}\\w`, fromCompiler: true };
        assert.deepStrictEqual(buildArgs(file, roots, false, paths), ["-x", "-w", file, "--toolchain-root", `${dir}\\t`]);
        assert.deepStrictEqual(buildArgs(file, roots, true, paths)[0], "--debug");
    });

    it("a compiler that cannot be started gives the fallback, with the reason", async () => {
        const paths = await programPaths(`${dir}\\no-qb64rust.exe`, file);
        assert.deepStrictEqual([paths.exe, paths.runIn, paths.fromCompiler], [fallbackPaths(file).exe, path.dirname(file), false]);
        assert.ok(paths.problem, "no reason given");
    });

    it("the reason for the fallback: none for an older compiler, else what it printed, or the timeout", () => {
        const usage = "unknown option `--print-paths`\nusage: qb64rust [-x|-c] ...";
        assert.strictEqual(printPathsProblem(usage, { message: "exit 1" }), undefined);
        const bad = `${dir}\\w\\qb64rust.toml:2: unknown build setting \`out\`; the settings are output, copy\n`;
        assert.strictEqual(printPathsProblem(bad, { message: "exit 1" }), bad.trim());
        assert.strictEqual(printPathsProblem("", { message: "x", killed: true }), "qb64rust --print-paths did not answer within 10 s");
        assert.strictEqual(printPathsProblem("", { message: "spawn ENOENT" }), "spawn ENOENT");
        assert.strictEqual(fallbackPaths(file, "why").problem, "why");
        assert.ok(!("problem" in fallbackPaths(file)));
    });
});

// Spec editor/build-run, "Run commands": Run takes the newer of the compiler's program and the one next to the
// source (the review's finding 1).
describe("the program Run starts", () => {
    const compiler = { exe: `${dir}\\w\\build\\game.exe`, runIn: `${dir}\\w\\build`, fromCompiler: true };
    const beside = { exe: `${dir}\\w\\game.exe`, runIn: `${dir}\\w`, fromCompiler: false };
    const times = (t: Record<string, number>) => (f: string) => t[f];

    it("the compiler's when only it exists, or when it is newer", () => {
        assert.strictEqual(newerProgram(compiler, beside, times({ [compiler.exe]: 5 })), compiler);
        assert.strictEqual(newerProgram(compiler, beside, times({ [compiler.exe]: 5, [beside.exe]: 4 })), compiler);
        assert.strictEqual(newerProgram(compiler, beside, times({})), compiler, "neither: Run offers to build");
    });

    it("the one next to the source when only it exists, or when it is newer, as after Build with QB64pe", () => {
        assert.strictEqual(newerProgram(compiler, beside, times({ [beside.exe]: 1 })), beside);
        assert.strictEqual(newerProgram(compiler, beside, times({ [compiler.exe]: 4, [beside.exe]: 5 })), beside);
    });
});

describe("debug adapter arguments", () => {
    it("dap with the toolchain folder a build uses (spec editor/debug-launch, 'The adapter')", () => {
        assert.deepStrictEqual(adapterArgs(`${dir}\\t`), ["dap", "--toolchain-root", `${dir}\\t`]);
        assert.deepStrictEqual(adapterArgs(undefined), ["dap"]);
    });
});
