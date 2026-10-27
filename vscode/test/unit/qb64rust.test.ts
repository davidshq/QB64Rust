import * as assert from "assert";
import { buildArgs } from "../../src/compiler/qb64rust";
import { adapterArgs } from "../../src/debugConfig";

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
});

describe("debug adapter arguments", () => {
    it("dap with the toolchain folder a build uses (spec editor/debug-launch, 'The adapter')", () => {
        assert.deepStrictEqual(adapterArgs(`${dir}\\t`), ["dap", "--toolchain-root", `${dir}\\t`]);
    });
});
