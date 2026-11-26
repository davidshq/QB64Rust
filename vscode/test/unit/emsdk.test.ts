import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as em from "../../src/compiler/emsdk";
import * as tc from "../../src/compiler/toolchain";

// OpenSpec change wasm-publish, design D5; spec editor/packaging, "Emscripten SDK lookup in the extension",
// "Emscripten SDK offer" and "Emscripten SDK install command"; spec editor/build-run, "Run in browser". The window
// and the processes are stubbed. Made-up paths (tools\repo_check allows `does\not\exist`).
const dir = "C:\\does\\not\\exist";
const binary = `${dir}\\qb64rust.exe`;

/** A `qb64rust toolchain path --target wasm` that records its arguments and answers `run`. */
function pathRun(run: tc.ProcessRun) {
    const calls: string[][] = [];
    return {
        calls,
        run: (_binary: string, args: string[]) => {
            calls.push(args);
            return Promise.resolve(run);
        },
    };
}

const INSTALLED: tc.ProcessRun = { exitCode: 0, output: `${dir}\\home\\emsdk-7.0.0\r\ninstalled: by qb64rust toolchain install --target wasm\r\n` };
const NONE: tc.ProcessRun = { exitCode: 1, output: "qb64rust: no Emscripten SDK found: a build for the wasm target needs Emscripten 7.0.0. (…)\n" };
const FOUND: em.EmsdkDiscovery = { found: true, folder: `${dir}\\home\\emsdk-7.0.0`, source: "installed" };
const MISSING: em.EmsdkDiscovery = { found: false, reason: "no Emscripten SDK found: (…)", installable: true };

describe("Emscripten SDK lookup", () => {
    it("the installed SDK, when the setting is empty", async () => {
        const p = pathRun(INSTALLED);
        assert.deepStrictEqual(await em.findEmsdk({ setting: "", binary, run: p.run }), FOUND);
        assert.deepStrictEqual(p.calls, [["toolchain", "path", "--target", "wasm"]]);
    });

    it("the setting wins: the compiler checks its folder", async () => {
        const p = pathRun({ exitCode: 0, output: [`${dir}\\mine`, "option: given by --emsdk-root", ""].join("\n") });
        assert.deepStrictEqual(await em.findEmsdk({ setting: `${dir}\\mine`, binary, run: p.run }), { found: true, folder: `${dir}\\mine`, source: "setting" });
        assert.deepStrictEqual(p.calls, [["toolchain", "path", "--target", "wasm", "--emsdk-root", `${dir}\\mine`]]);
    });

    it("a setting that names no SDK is not found, names the setting, and offers no download", async () => {
        const p = pathRun({ exitCode: 1, output: `qb64rust: ${dir}\\mine (given by --emsdk-root) is not an Emscripten SDK folder: (…) no Emscripten SDK found: (…)\n` });
        const found = await em.findEmsdk({ setting: `${dir}\\mine`, binary, run: p.run });
        assert.strictEqual(found.found, false);
        assert.ok(!found.found && found.setting === em.SETTING && !found.installable && found.reason.startsWith("qb64rust.emsdkPath: "), JSON.stringify(found));
    });

    it("none found: the compiler's message, and an install would give one", async () => {
        const found = await em.findEmsdk({ setting: "", binary, run: pathRun(NONE).run });
        assert.deepStrictEqual(found, { found: false, reason: "no Emscripten SDK found: a build for the wasm target needs Emscripten 7.0.0. (…)", installable: true });
    });

    it("a variable that names no SDK: not installable", async () => {
        const found = await em.findEmsdk({ setting: "", binary, run: pathRun({ exitCode: 1, output: "qb64rust: the Emscripten SDK in X (given by QB64RUST_EMSDK_ROOT) is release 6.0.13; (…)\n" }).run });
        assert.ok(!found.found && !found.installable, JSON.stringify(found));
    });

    it("without the binary nothing is run", async () => {
        const p = pathRun(INSTALLED);
        const found = await em.findEmsdk({ setting: "", run: p.run });
        assert.ok(!found.found && found.setting === "qb64rust.path");
        assert.deepStrictEqual(p.calls, []);
    });
});

/** A window that answers the question with `choice`, installs with `run`, and then finds `after`. */
function stubUi(choice: string | undefined, run: tc.InstallRun, after: em.EmsdkDiscovery, platform = "win32") {
    const log: string[] = [];
    let installed = false;
    const ui: em.EmsdkUi = {
        ask: (message, ...choices) => {
            log.push(`ask ${choices.join("|")}`);
            assert.strictEqual(message, em.OFFER);
            return Promise.resolve(choice);
        },
        install: () => {
            log.push("install");
            installed = true;
            return Promise.resolve(run);
        },
        error: (message) => {
            log.push(`error ${message}`);
            return Promise.resolve(undefined);
        },
        info: (message) => {
            log.push(`info ${message}`);
            return Promise.resolve(undefined);
        },
        showOutput: () => log.push("output"),
        find: () => Promise.resolve(installed ? after : MISSING),
        platform,
    };
    return { ui, log };
}

const OK: tc.InstallRun = { exitCode: 0, output: "installed: X\n", cancelled: false };

describe("Emscripten SDK offer", () => {
    it("names the source, the download and the disk space", () => {
        assert.match(em.OFFER, /storage\.googleapis\.com\/webassembly/);
        assert.match(em.OFFER, /815 MB download/);
        assert.match(em.OFFER, /GB on disk/);
    });

    it("Download: installs, and the SDK found afterwards is the result", async () => {
        const { ui, log } = stubUi(tc.DOWNLOAD, OK, FOUND);
        assert.deepStrictEqual(await em.offerEmsdk(ui, MISSING), FOUND);
        assert.deepStrictEqual(log, ["ask Download|Not Now", "install"]);
    });

    it("Not Now, or the question closed: nothing is downloaded, and the command and the setting are named", async () => {
        for (const choice of [tc.NOT_NOW, undefined]) {
            const { ui, log } = stubUi(choice, OK, FOUND);
            assert.strictEqual(await em.offerEmsdk(ui, MISSING), undefined);
            assert.deepStrictEqual(log, ["ask Download|Not Now", `info ${em.DECLINED}`]);
        }
        assert.match(em.DECLINED, /QB64: Install the Emscripten SDK/);
        assert.match(em.DECLINED, /qb64rust\.emsdkPath/);
    });

    it("an install that fails shows its last line", async () => {
        const { ui, log } = stubUi(tc.DOWNLOAD, { exitCode: 1, output: "downloading 5%\nqb64rust: the SHA-256 of the downloaded node.zip is a, not b\n", cancelled: false }, FOUND);
        assert.strictEqual(await em.offerEmsdk(ui, MISSING), undefined);
        assert.deepStrictEqual(log.slice(2), ["error QB64: installing the Emscripten SDK failed: the SHA-256 of the downloaded node.zip is a, not b"]);
    });

    it("a cancelled install says that nothing is installed", async () => {
        const { ui, log } = stubUi(tc.DOWNLOAD, { exitCode: null, output: "", cancelled: true }, FOUND);
        assert.strictEqual(await em.offerEmsdk(ui, MISSING), undefined);
        assert.match(log[2], /^info QB64: the Emscripten SDK download was cancelled; nothing is installed/);
    });

    it("on Linux nothing is asked or downloaded: the compiler's message is shown", async () => {
        const { ui, log } = stubUi(tc.DOWNLOAD, OK, FOUND, "linux");
        assert.strictEqual(await em.offerEmsdk(ui, MISSING), undefined);
        assert.deepStrictEqual(log, [`error QB64: ${MISSING.found ? "" : MISSING.reason}`]);
    });
});

describe("Emscripten SDK install command", () => {
    it("an untrusted workspace: a message, and nothing runs", async () => {
        const { ui, log } = stubUi(tc.DOWNLOAD, OK, FOUND);
        assert.strictEqual(await em.installCommand(false, ui), undefined);
        assert.deepStrictEqual(log, ["info QB64: installing the Emscripten SDK requires a trusted workspace. Nothing was run."]);
    });

    it("found already: its folder is named and nothing is downloaded", async () => {
        const { ui, log } = stubUi(tc.DOWNLOAD, OK, FOUND);
        ui.find = () => Promise.resolve(FOUND);
        assert.deepStrictEqual(await em.installCommand(true, ui), FOUND);
        assert.deepStrictEqual(log, [`info QB64: the Emscripten SDK is found already: ${dir}\\home\\emsdk-7.0.0`]);
    });

    it("none found: the offer", async () => {
        const { ui, log } = stubUi(tc.DOWNLOAD, OK, FOUND);
        assert.deepStrictEqual(await em.installCommand(true, ui), FOUND);
        assert.deepStrictEqual(log, ["ask Download|Not Now", "install"]);
    });

    it("a setting that names no SDK: the reason, and no offer", async () => {
        const { ui, log } = stubUi(tc.DOWNLOAD, OK, FOUND);
        ui.find = () => Promise.resolve({ found: false, reason: "qb64rust.emsdkPath: not an SDK", setting: em.SETTING });
        assert.strictEqual(await em.installCommand(true, ui), undefined);
        assert.deepStrictEqual(log, ["error QB64: qb64rust.emsdkPath: not an SDK"]);
    });
});

describe("run in browser: the plan", () => {
    const file = `${dir}\\w\\game.bas`;

    it("the build: --target wasm, the SDK and the roots, and no -o", () => {
        const roots = { toolchain: `${dir}\\tools`, libqb: `${dir}\\libqb`, include: "" };
        const args = em.wasmBuildArgs(file, roots, `${dir}\\sdk`);
        assert.deepStrictEqual(args, ["-x", "-w", "--target", "wasm", file, "--emsdk-root", `${dir}\\sdk`, "--toolchain-root", `${dir}\\tools`, "--libqb-root", `${dir}\\libqb`]);
        assert.ok(!args.includes("-o"));
    });

    it("the page is the one --print-paths --target wasm names", () => {
        assert.deepStrictEqual(em.pagePathArgs(file), ["--print-paths", file, "--target", "wasm"]);
        const line = JSON.stringify({ exe: `${dir}\\w\\build\\game.js`, page: `${dir}\\w\\build\\web\\index.html`, run_in: `${dir}\\w` });
        assert.strictEqual(em.parsePagePath(`${line}\r\n`), `${dir}\\w\\build\\web\\index.html`);
        // An older compiler's line has no page; a settings file's error is not JSON.
        assert.strictEqual(em.parsePagePath(JSON.stringify({ exe: "a", run_in: "b" })), undefined);
        assert.strictEqual(em.parsePagePath("qb64rust.toml:2: unknown key"), undefined);
    });

    it("a $CONSOLE:ONLY program wrote no page, and an older page does not count", () => {
        const started = 1_000_000;
        assert.strictEqual(em.pageWasBuilt(undefined, started), false);
        assert.strictEqual(em.pageWasBuilt(started - 60_000, started), false);
        assert.strictEqual(em.pageWasBuilt(started + 5000, started), true);
        // A file system that rounds the time down.
        assert.strictEqual(em.pageWasBuilt(started - 1500, started), true);
        assert.match(em.NO_PAGE, /\$CONSOLE:ONLY/);
    });

    it("the server: the folder, and the port of the run before", () => {
        assert.deepStrictEqual(em.serveArgs(`${dir}\\w\\build\\web`), ["serve", `${dir}\\w\\build\\web`]);
        assert.deepStrictEqual(em.serveArgs(`${dir}\\w\\build\\web`, 8065), ["serve", `${dir}\\w\\build\\web`, "--port", "8065"]);
    });

    it("the address is the server's first line", () => {
        assert.deepStrictEqual(em.parseAddress("http://127.0.0.1:8064/"), { url: "http://127.0.0.1:8064/", port: 8064 });
        assert.deepStrictEqual(em.parseAddress("http://127.0.0.1:8070/\r"), { url: "http://127.0.0.1:8070/", port: 8070 });
        assert.strictEqual(em.parseAddress("serving C:\\x until interrupted (Ctrl+C)"), undefined);
        assert.strictEqual(em.parseAddress("see http://127.0.0.1:8064/"), undefined);
    });
});

describe("the manifest: the browser commands", () => {
    const manifest = JSON.parse(fs.readFileSync(path.join(__dirname, "..", "..", "..", "package.json"), "utf8")) as {
        contributes: { commands: { command: string; title: string; enablement?: string }[]; configuration: { properties: Record<string, unknown> } };
    };

    it("both commands, unavailable in an untrusted workspace, and the setting", () => {
        for (const [id, title] of [
            [em.RUN_COMMAND, "Run in Browser"],
            [em.INSTALL_COMMAND, "Install the Emscripten SDK"],
        ]) {
            const command = manifest.contributes.commands.find((c) => c.command === id);
            assert.ok(command, id);
            assert.strictEqual(command.title, title);
            assert.strictEqual(command.enablement, "isWorkspaceTrusted");
        }
        assert.ok(em.SETTING in manifest.contributes.configuration.properties);
        assert.strictEqual(em.INSTALL_TITLE, "QB64: Install the Emscripten SDK");
    });
});
