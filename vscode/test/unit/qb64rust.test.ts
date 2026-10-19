import * as assert from "assert";
import { buildArgs } from "../../src/compiler/qb64rust";

// Design D4 of extension-vsix: the command line of a build with qb64rust. Made-up paths (tools\repo_check allows
// `does\not\exist`).
const dir = "C:\\does\\not\\exist";

describe("qb64rust build arguments", () => {
    it("the executable next to the source, the QB64pe folder, the prepared libqb and the include root", () => {
        const args = buildArgs(`${dir}\\w\\hello.bas`, { qb64pe: `${dir}\\qb64pe`, libqb: `${dir}\\s\\libqb-0.2.0`, include: `${dir}\\qb64pe` });
        const exe = process.platform === "win32" ? `${dir}\\w\\hello.exe` : `${dir}\\w\\hello`;
        assert.deepStrictEqual(args, [
            "-x",
            "-w",
            `${dir}\\w\\hello.bas`,
            "-o",
            exe,
            "--qb64pe-root",
            `${dir}\\qb64pe`,
            "--libqb-root",
            `${dir}\\s\\libqb-0.2.0`,
            "--include-root",
            `${dir}\\qb64pe`,
        ]);
    });

    it("no --libqb-root without a bundle: the compiler's own default", () => {
        const args = buildArgs(`${dir}\\w\\hello.bas`, { qb64pe: `${dir}\\qb64pe`, include: `${dir}\\inc` });
        assert.ok(!args.includes("--libqb-root"));
        assert.deepStrictEqual(args.slice(-2), ["--include-root", `${dir}\\inc`]);
    });
});
