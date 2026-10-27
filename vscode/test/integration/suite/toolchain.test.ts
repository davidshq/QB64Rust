// The plan of a build with and without QB64pe and without a C++ toolchain (change m6-toolchain, design D6; spec
// editor/build-run, "Compiler discovery" and "Build command"). The status, the binary and the toolchain are stubbed,
// so nothing is built and nothing needs QB64pe. Made-up paths (tools\repo_check allows `does\not\exist`).
import * as assert from "assert";
import * as path from "path";
import * as vscode from "vscode";
import { Bundle, planBuild } from "../../../src/buildPlan";
import { Discovery } from "../../../src/compiler/discovery";
import { ToolchainDiscovery } from "../../../src/compiler/toolchain";
import { LanguageServer } from "../../../src/languageServer";
import { CompilerStatus } from "../../../src/statusBar";
import { api, setSetting, waitFor } from "./helpers";

const dir = "C:\\does\\not\\exist";
const file = path.join(dir, "w", "hello.bas");
const qb64peExe = path.join(dir, "qb64pe", "qb64pe.exe");
const tools = path.join(dir, "home", "llvm-mingw-20260922-ucrt-x86_64");
const bundle: Bundle = { libqb: path.join(dir, "bundle", "libqb"), storage: path.join(dir, "storage"), version: "0" };
const server = { binary: { found: true, path: path.join(dir, "qb64rust.exe"), source: "setting" } } as unknown as LanguageServer;

/** A status with QB64pe found or not, whose toolchain lookup gives `toolchain` and records how it was asked. */
function stubStatus(qb64pe: boolean, toolchain: ToolchainDiscovery) {
    const asked: boolean[] = [];
    const current: Discovery = qb64pe ? { found: true, path: qb64peExe, source: "setting" } : { found: false, reason: "qb64pe was not found" };
    const status = {
        current,
        require: () => Promise.resolve(undefined),
        toolchain: {
            ensure: (interactive: boolean) => {
                asked.push(interactive);
                return Promise.resolve(toolchain);
            },
        },
    } as unknown as CompilerStatus;
    return { status, asked };
}

const INSTALLED: ToolchainDiscovery = { found: true, folder: tools, source: "installed" };
const NONE: ToolchainDiscovery = { found: false, reason: "no C++ toolchain found", installable: true };

describe("build plan and the C++ toolchain", () => {
    before(async () => {
        await setSetting("qb64rust", "buildWith", "auto");
    });
    after(async () => {
        await setSetting("qb64rust", "buildWith", "qb64pe");
    });

    it("no QB64pe: qb64rust builds with the installed toolchain, no --qb64pe-root, no fallback", async () => {
        const { status } = stubStatus(false, INSTALLED);
        const plan = await planBuild(file, status, server, bundle, false);
        assert.strictEqual(plan.compiler, "qb64rust");
        if (plan.compiler !== "qb64rust") {
            return;
        }
        assert.deepStrictEqual(plan.args.slice(5, 7), ["--toolchain-root", tools]);
        assert.ok(!plan.args.includes("--qb64pe-root"), plan.args.join(" "));
        assert.strictEqual(plan.qb64pe, undefined);
    });

    it("QB64pe found: its folder as --qb64pe-root and the include root, and the fallback", async () => {
        const { status } = stubStatus(true, { found: true, folder: path.join(dir, "qb64pe", "internal", "c", "c_compiler"), source: "qb64pe" });
        const plan = await planBuild(file, status, server, bundle, false);
        assert.strictEqual(plan.compiler, "qb64rust");
        if (plan.compiler !== "qb64rust") {
            return;
        }
        const after = (option: string) => plan.args[plan.args.indexOf(option) + 1];
        assert.strictEqual(after("--toolchain-root"), path.join(dir, "qb64pe", "internal", "c", "c_compiler"));
        assert.strictEqual(after("--qb64pe-root"), path.dirname(qb64peExe));
        assert.strictEqual(after("--include-root"), path.dirname(qb64peExe));
        assert.strictEqual(plan.qb64pe, qb64peExe);
    });

    it("no toolchain: no build, marked as the toolchain's, and a listing does not offer", async () => {
        const { status, asked } = stubStatus(false, NONE);
        const plan = await planBuild(file, status, server, bundle, false);
        assert.deepStrictEqual(plan, { compiler: "none", reason: "no C++ toolchain found", toolchain: true });
        assert.deepStrictEqual(asked, [false]);
    });

    it("an interactive build asks the toolchain to be offered", async () => {
        const { status, asked } = stubStatus(false, NONE);
        await planBuild(file, status, server, bundle, true);
        assert.deepStrictEqual(asked, [true]);
    });

    it("qb64pe chosen: QB64pe is needed, and the toolchain is not looked for", async () => {
        await setSetting("qb64rust", "buildWith", "qb64pe");
        try {
            const without = stubStatus(false, INSTALLED);
            assert.deepStrictEqual(await planBuild(file, without.status, server, bundle, false), { compiler: "none", reason: "qb64pe was not found" });
            const found = stubStatus(true, INSTALLED);
            const plan = await planBuild(file, found.status, server, bundle, false);
            assert.strictEqual(plan.compiler, "qb64pe");
            assert.deepStrictEqual([...without.asked, ...found.asked], []);
        } finally {
            await setSetting("qb64rust", "buildWith", "auto");
        }
    });

    it("the install command is declared, and the language status says so when qb64rust builds without a toolchain", async () => {
        const a = await api();
        assert.ok((await vscode.commands.getCommands(true)).includes("qb64rust.installToolchain"));
        if (!a.server.binary.found) {
            return; // auto builds with QB64pe then
        }
        const saved = a.toolchain.current;
        try {
            (a.toolchain as unknown as { discovery: ToolchainDiscovery }).discovery = NONE;
            a.status.render();
            await waitFor("the toolchain status", () => a.status.text === "C++ toolchain not installed");
            assert.strictEqual(a.status.severity, vscode.LanguageStatusSeverity.Warning);
            // A setting that names no toolchain: an install would not help, so no install link.
            (a.toolchain as unknown as { discovery: ToolchainDiscovery }).discovery = { found: false, reason: "qb64rust.toolchainPath points to x", setting: "qb64rust.toolchainPath" };
            a.status.render();
            await waitFor("the toolchain status", () => a.status.text === "C++ toolchain not found");
        } finally {
            (a.toolchain as unknown as { discovery: ToolchainDiscovery }).discovery = saved;
            a.status.render();
        }
    });
});
