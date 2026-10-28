import * as assert from "assert";
import * as tc from "../../src/compiler/toolchain";

// Change m6-toolchain, design D6; spec editor/packaging, "Toolchain lookup in the extension", "Toolchain offer",
// "Install command". The window and the processes are stubbed. Made-up paths (tools\repo_check allows
// `does\not\exist`).
const dir = "C:\\does\\not\\exist";

/** A `qb64rust toolchain path` that records its arguments and answers `run`. */
function pathRun(run: tc.ProcessRun) {
    const calls: string[][] = [];
    return {
        calls,
        run: (binary: string, args: string[]) => {
            calls.push([binary, ...args]);
            return Promise.resolve(run);
        },
    };
}

const INSTALLED: tc.ProcessRun = {
    exitCode: 0,
    output: `${dir}\\home\\llvm-mingw-20260922-ucrt-x86_64\r\ninstalled: the toolchain \`qb64rust toolchain install\` put in place\r\n`,
};

describe("toolchain lookup", () => {
    it("the setting wins, and the binary is not run", async () => {
        const p = pathRun(INSTALLED);
        const found = await tc.findToolchain({ setting: `${dir}\\mine`, binary: `${dir}\\qb64rust.exe`, qb64pe: `${dir}\\qb64pe`, run: p.run, isToolchain: () => true });
        assert.deepStrictEqual(found, { found: true, folder: `${dir}\\mine`, source: "setting" });
        assert.deepStrictEqual(p.calls, []);
    });

    it("a setting that is no toolchain folder is not found, naming the setting", async () => {
        const found = await tc.findToolchain({ setting: `${dir}\\mine`, binary: `${dir}\\qb64rust.exe`, run: pathRun(INSTALLED).run, isToolchain: () => false });
        assert.strictEqual(found.found, false);
        assert.match(found.found ? "" : found.reason, /qb64rust\.toolchainPath/);
    });

    it("else `qb64rust toolchain path`, given the QB64pe found", async () => {
        const p = pathRun({ exitCode: 0, output: `${dir}\\qb64pe\\internal\\c\\c_compiler\nqb64pe: the toolchain of the QB64pe folder given\n` });
        const found = await tc.findToolchain({ setting: "", binary: `${dir}\\qb64rust.exe`, qb64pe: `${dir}\\qb64pe`, run: p.run });
        assert.deepStrictEqual(found, { found: true, folder: `${dir}\\qb64pe\\internal\\c\\c_compiler`, source: "qb64pe" });
        assert.deepStrictEqual(p.calls, [[`${dir}\\qb64rust.exe`, "toolchain", "path", "--qb64pe-root", `${dir}\\qb64pe`]]);
    });

    it("without QB64pe: no --qb64pe-root, and the installed toolchain", async () => {
        const p = pathRun(INSTALLED);
        const found = await tc.findToolchain({ setting: "", binary: `${dir}\\qb64rust.exe`, run: p.run });
        assert.deepStrictEqual(found, { found: true, folder: `${dir}\\home\\llvm-mingw-20260922-ucrt-x86_64`, source: "installed" });
        assert.deepStrictEqual(p.calls, [[`${dir}\\qb64rust.exe`, "toolchain", "path"]]);
    });

    it("none found: the compiler's message", async () => {
        const p = pathRun({ exitCode: 1, output: "qb64rust: no C++ toolchain found: install it with `qb64rust toolchain install` (…)\n" });
        const found = await tc.findToolchain({ setting: "", binary: `${dir}\\qb64rust.exe`, run: p.run });
        assert.deepStrictEqual(found, { found: false, reason: "no C++ toolchain found: install it with `qb64rust toolchain install` (…)", installable: true });
    });

    it("a lookup that failed otherwise (a variable naming no toolchain): not installable", async () => {
        const p = pathRun({ exitCode: 1, output: "qb64rust: C:\\x (given by QB64RUST_TOOLCHAIN_ROOT) is not a C++ toolchain folder\n" });
        const found = await tc.findToolchain({ setting: "", binary: `${dir}\\qb64rust.exe`, run: p.run });
        assert.deepStrictEqual(found, { found: false, reason: "C:\\x (given by QB64RUST_TOOLCHAIN_ROOT) is not a C++ toolchain folder" });
    });

    it("a setting that names no toolchain: not installable, names the setting", async () => {
        const found = await tc.findToolchain({ setting: `${dir}\\mine`, binary: `${dir}\\qb64rust.exe`, run: pathRun(INSTALLED).run, isToolchain: () => false });
        assert.ok(!found.found && !found.installable && found.setting === tc.SETTING);
    });

    it("no binary: nothing run, not found, not installable", async () => {
        const p = pathRun(INSTALLED);
        const found = await tc.findToolchain({ setting: "", run: p.run });
        assert.ok(!found.found && !found.installable && found.setting === "qb64rust.path");
        assert.deepStrictEqual(p.calls, []);
    });

    it("the progress of a download line", () => {
        assert.strictEqual(tc.downloadPercent("downloading 45%"), 45);
        assert.strictEqual(tc.downloadPercent("downloading https://github.com/x.zip"), undefined);
        assert.strictEqual(tc.downloadPercent("unpacking"), undefined);
    });
});

/** A window and processes that record what happens; `answer` is the choice of the question. */
function stubUi(answer: string | undefined, install: tc.InstallRun, after: tc.ToolchainDiscovery = { found: true, folder: `${dir}\\t`, source: "installed" }) {
    const seen: string[] = [];
    const ui: tc.OfferUi = {
        ask: (message, ...choices) => {
            seen.push(`ask: ${message} [${choices.join(", ")}]`);
            return Promise.resolve(answer);
        },
        install: () => {
            seen.push("install");
            return Promise.resolve(install);
        },
        error: (message, ...items) => {
            seen.push(`error: ${message} [${items.join(", ")}]`);
            return Promise.resolve(items[0]);
        },
        info: (message) => {
            seen.push(`info: ${message}`);
            return Promise.resolve(undefined);
        },
        showOutput: () => seen.push("output"),
        find: () => {
            seen.push("find");
            return Promise.resolve(after);
        },
    };
    return { ui, seen };
}

const OK: tc.InstallRun = { exitCode: 0, output: "installed: x\n", cancelled: false };

describe("toolchain offer", () => {
    it("the question names the source, the download's size and the disk space, with Download and Not Now", async () => {
        const { ui, seen } = stubUi(tc.DOWNLOAD, OK);
        await tc.offerToolchain(ui);
        assert.match(seen[0], /github\.com\/mstorsjo\/llvm-mingw/);
        assert.match(seen[0], /190 MB download, about 460 MB on disk/);
        // The release as the driver's message and RELEASE in toolchain.rs name it.
        assert.match(seen[0], /llvm-mingw 20260922/);
        assert.match(seen[0], /\[Download, Not Now\]$/);
    });

    it("Download: installs, looks again, and the build goes on with the toolchain", async () => {
        const { ui, seen } = stubUi(tc.DOWNLOAD, OK);
        assert.deepStrictEqual(await tc.offerToolchain(ui), { found: true, folder: `${dir}\\t`, source: "installed" });
        assert.deepStrictEqual(seen.slice(1), ["install", "find"]);
    });

    for (const answer of [tc.NOT_NOW, undefined]) {
        it(`${answer ?? "closed"}: nothing installed, the message names the command and the setting`, async () => {
            const { ui, seen } = stubUi(answer, OK);
            assert.strictEqual(await tc.offerToolchain(ui), undefined);
            assert.deepStrictEqual(seen.slice(1), [`info: ${tc.DECLINED}`]);
            assert.match(tc.DECLINED, /QB64: Install the C\+\+ Toolchain/);
            assert.match(tc.DECLINED, /qb64rust\.toolchainPath/);
        });
    }

    it("a failed install: its error with a button to the output, no build", async () => {
        const failed: tc.InstallRun = {
            exitCode: 1,
            output: "downloading https://x\nqb64rust: downloading the toolchain failed (curl: (6) Could not resolve host)\n",
            cancelled: false,
        };
        const { ui, seen } = stubUi(tc.DOWNLOAD, failed);
        assert.strictEqual(await tc.offerToolchain(ui), undefined);
        await new Promise((r) => setImmediate(r));
        assert.deepStrictEqual(seen.slice(1), [
            "install",
            "error: QB64: installing the C++ toolchain failed: downloading the toolchain failed (curl: (6) Could not resolve host) [Show Output]",
            "output",
        ]);
    });

    it("a cancelled install: says so, no build", async () => {
        const { ui, seen } = stubUi(tc.DOWNLOAD, { exitCode: null, output: "", cancelled: true });
        assert.strictEqual(await tc.offerToolchain(ui), undefined);
        assert.match(seen.at(-1) ?? "", /^info: .*cancelled/);
    });
});

describe("install command", () => {
    const installed = (d: tc.ToolchainDiscovery) => () => Promise.resolve(d);

    it("untrusted: a message, nothing run", async () => {
        const { ui, seen } = stubUi(tc.DOWNLOAD, OK);
        assert.strictEqual(await tc.installCommand(false, installed({ found: false, reason: "none" }), ui), undefined);
        assert.strictEqual(seen.length, 1);
        assert.match(seen[0], /^info: .*trusted workspace/);
    });

    it("installed already: a message naming its folder, nothing downloaded", async () => {
        const { ui, seen } = stubUi(tc.DOWNLOAD, OK);
        await tc.installCommand(true, installed({ found: true, folder: `${dir}\\t`, source: "installed" }), ui);
        assert.deepStrictEqual(seen, [`info: QB64: the C++ toolchain is installed already: ${dir}\\t`]);
    });

    it("QB64pe's toolchain only: the same question as a build", async () => {
        const { ui, seen } = stubUi(tc.NOT_NOW, OK);
        await tc.installCommand(true, installed({ found: true, folder: `${dir}\\q`, source: "clone" }), ui);
        assert.match(seen[0], /^ask: /);
    });
});
