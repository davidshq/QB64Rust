// The first run on a machine without QB64pe or a C++ toolchain (change m6-toolchain, task 4.4; spec editor/packaging,
// "Toolchain offer"). Runs only from `npm run test:integration:first-run` (runTests.ts --first-run), which starts the
// development host with no QB64pe (no qb64rust.compilerPath, none on PATH, QB64RUST_QB64PE_ROOT empty, so not the
// default clone either) and QB64RUST_TOOLCHAIN_HOME at an empty folder; the other suites are left out then. It
// downloads the real toolchain (190 MB). The modal question is answered here (ToolchainSupport.ui.ask); the install,
// its progress notification, the build and the debug session are the extension's own.
import * as assert from "assert";
import { execFileSync } from "child_process";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import * as tc from "../../../src/compiler/toolchain";
import { api, open, server, waitFor, workspace, writeFile } from "./helpers";

const home = process.env.QB64RUST_TOOLCHAIN_HOME ?? "";

describe("first run without QB64pe or a C++ toolchain", function () {
    this.timeout(1800000);
    let answer: string = tc.NOT_NOW;
    const asked: { message: string; choices: string[] }[] = [];
    const infos: string[] = [];
    const errors: string[] = [];
    const installs: tc.InstallRun[] = [];
    let uri: vscode.Uri;
    let exe: string;

    before(async () => {
        assert.ok(server, "needs QB64RUST_TEST_QB64RUST");
        assert.ok(home && (!fs.existsSync(home) || fs.readdirSync(home).length === 0), `QB64RUST_TOOLCHAIN_HOME must be an empty folder: ${home}`);
        // In this process, so that every qb64rust the extension starts has them: empty is unset (toolchain.rs), and an
        // empty QB64RUST_QB64PE_ROOT also leaves out the default clone. runTests.ts cannot pass empty values.
        process.env.QB64RUST_QB64PE_ROOT = "";
        process.env.QB64RUST_TOOLCHAIN_ROOT = "";
        const a = await api();
        await a.server.ready;
        const real = a.toolchain.ui;
        a.toolchain.ui = {
            ...real,
            ask: (message, ...choices) => {
                asked.push({ message, choices });
                return Promise.resolve(answer);
            },
            info: (message) => {
                infos.push(message);
                return real.info(message);
            },
            error: (message, ...items) => {
                errors.push(message);
                return real.error(message, ...items);
            },
            install: async () => {
                const run = await real.install();
                installs.push(run);
                return run;
            },
        };
        uri = writeFile("first/hello.bas", '$CONSOLE:ONLY\r\nOPEN "hello.txt" FOR OUTPUT AS #1\r\nPRINT #1, "first run"\r\nCLOSE #1\r\nSYSTEM\r\n');
        exe = uri.fsPath.replace(/\.bas$/i, ".exe");
    });

    it("starts with no QB64pe and no toolchain, and the language status says so", async () => {
        const a = await api();
        assert.strictEqual(a.status.current.found, false, "QB64pe found");
        const found = await a.toolchain.refresh();
        assert.strictEqual(found.found, false, JSON.stringify(found));
        await waitFor("the toolchain status", () => a.status.text === "C++ toolchain not installed");
    });

    it("Build asks; Not Now builds nothing, downloads nothing and says how to install", async () => {
        await open(uri);
        answer = tc.NOT_NOW;
        assert.strictEqual(await vscode.commands.executeCommand("qb64rust.build", uri), undefined);
        assert.deepStrictEqual(asked, [{ message: tc.OFFER, choices: [tc.DOWNLOAD, tc.NOT_NOW] }]);
        assert.deepStrictEqual(infos, [tc.DECLINED]);
        assert.deepStrictEqual(installs, []);
        assert.ok(!fs.existsSync(exe), "nothing built");
        assert.ok(!fs.existsSync(home) || fs.readdirSync(home).length === 0, "nothing downloaded");
        await vscode.commands.executeCommand("notifications.clearAll");
    });

    it("Download installs with progress and the build goes on", async () => {
        const a = await api();
        answer = tc.DOWNLOAD;
        const started = Date.now();
        const built = await vscode.commands.executeCommand<string | undefined>("qb64rust.build", uri);
        console.log(`    install and build: ${((Date.now() - started) / 1000).toFixed(1)} s`);
        assert.strictEqual(asked.length, 2);
        assert.strictEqual(installs.length, 1);
        const run = installs[0];
        assert.deepStrictEqual([run.exitCode, run.cancelled, errors], [0, false, []], run.output);
        const percents = run.output.split(/\r?\n/).flatMap((l) => tc.downloadPercent(l) ?? []);
        assert.ok(percents.includes(5) && percents.includes(100), `progress lines: ${percents.join(" ")}`);
        console.log(`    progress lines: ${percents.length} (${percents[0]} % to ${percents.at(-1)} %)`);
        assert.strictEqual(built, exe);
        const now = a.toolchain.current;
        assert.ok(now.found && now.source === "installed" && path.dirname(now.folder) === path.resolve(home), JSON.stringify(now));
        await waitFor("the status", () => a.status.text === "qb64rust");
        execFileSync(exe, { cwd: path.dirname(exe), timeout: 60000 });
        assert.strictEqual(fs.readFileSync(path.join(path.dirname(exe), "hello.txt"), "latin1").trim(), "first run");
        await vscode.commands.executeCommand("notifications.clearAll");
    });

    it("a debug session with the installed toolchain stops at a breakpoint, and nothing is offered again", async () => {
        const sent: { type?: string; event?: string; body?: Record<string, unknown> }[] = [];
        const tracker = vscode.debug.registerDebugAdapterTrackerFactory("qb64rust", {
            createDebugAdapterTracker: () => ({ onDidSendMessage: (m) => sent.push(m) }),
        });
        try {
            const fixture = vscode.Uri.file(path.join(workspace, "debug_me.bas"));
            await open(fixture);
            vscode.debug.removeBreakpoints(vscode.debug.breakpoints);
            vscode.debug.addBreakpoints([new vscode.SourceBreakpoint(new vscode.Location(fixture, new vscode.Position(8, 0)))]);
            const ended = new Promise<void>((resolve) => {
                const d = vscode.debug.onDidTerminateDebugSession(() => {
                    d.dispose();
                    resolve();
                });
            });
            assert.strictEqual(await vscode.commands.executeCommand("qb64rust.debug", fixture), true);
            const stopped = await waitFor("a stopped event", () => sent.find((m) => m.type === "event" && m.event === "stopped"), 600000);
            const session = vscode.debug.activeDebugSession!;
            const threadId = stopped.body!.threadId;
            const st = await session.customRequest("stackTrace", { threadId });
            assert.deepStrictEqual(
                st.stackFrames.map((f: { name: string; line: number }) => [f.name, f.line]),
                [
                    ["SUB show", 9],
                    ["Main module", 4],
                ],
            );
            await session.customRequest("continue", { threadId });
            await ended;
            assert.strictEqual(asked.length, 2, "offered again");
        } finally {
            tracker.dispose();
            vscode.debug.removeBreakpoints(vscode.debug.breakpoints);
        }
    });
});
