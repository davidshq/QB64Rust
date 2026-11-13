// Debugging in VS Code (change m5-debug-launch, design D8; spec editor/debug-launch). Needs QB64pe (its toolchain and
// its lldb-dap) and the qb64rust binary (QB64RUST_TEST_QB64RUST); skipped without either. The sessions run
// `qb64rust dap` over the real lldb-dap, with the default console (the integrated terminal).
import * as assert from "assert";
import { execFileSync } from "child_process";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import * as vscode from "vscode";
import { api, closeAll, compiler, open, server, setSetting, waitFor, workspace, writeFile } from "./helpers";

/** The fixture: `SUB show (n AS LONG)` on line 8, its statement `total = total + n` on line 9. */
const fixture = () => vscode.Uri.file(path.join(workspace, "debug_me.bas"));

/** Every message the adapter sent, of every session since the suite began. */
const sent: { type?: string; event?: string; body?: Record<string, unknown> }[] = [];

/** The next `stopped` event after `from` messages. A timeout's error lists what the adapter sent instead, and what
 * `note` says, so that a failure in CI shows whether the breakpoint was verified and how far the program got. */
async function stoppedAfter(from: number, timeout = 600000, note: () => string = () => "") {
    try {
        return await waitFor("a stopped event", () => sent.slice(from).find((m) => m.type === "event" && m.event === "stopped"), timeout);
    } catch (e) {
        // The threads as the adapter names them, read in a pause: one named "Program" once its own breakpoint on the
        // main module was hit.
        let threads: string;
        // lldb's own view in the pause: whether the proxy's breakpoint on the main module resolved, and the thread's
        // addresses (the proxy shows only BASIC frames). libqb's `main` sets `device_last` to 2 just before
        // `libqb_http_init` and the window; its other globals have no debug information.
        let lldb = "";
        const commands = ["`breakpoint list", "`bt all", "`target variable device_last"];
        try {
            const session = vscode.debug.activeDebugSession!;
            const paused = sent.length;
            await session.customRequest("pause", { threadId: 0 });
            await waitFor("the pause", () => sent.slice(paused).find((m) => m.type === "event" && m.event === "stopped"), 30000);
            threads = JSON.stringify((await session.customRequest("threads")).threads);
            for (const command of commands) {
                try {
                    lldb += `\n${command}:\n${(await session.customRequest("evaluate", { expression: command, context: "repl" })).result}`;
                } catch (c) {
                    lldb += `\n${command}: failed (${(c as Error).message})`;
                }
            }
        } catch (t) {
            threads = `not read (${(t as Error).message})`;
        }
        const pid = sent.slice(from).find((m) => m.type === "event" && m.event === "process")?.body?.systemProcessId;
        const windows = process.platform === "win32" && typeof pid === "number" ? windowsStacks(pid, lldb) : "";
        const seen = sent.slice(from).map((m) => JSON.stringify(m).slice(0, 600));
        throw new Error(
            `${(e as Error).message}; ${note()}; threads ${threads}; the adapter sent ${seen.length} messages:\n${seen.join("\n")}${lldb}${windows}`,
            { cause: e },
        );
    }
}

/** What Windows says of a program that lldb shows only by address (it loads no module but the executable): the
 * module each of `lldb`'s addresses is in, and the stacks `cdb` reads without stopping the program, with
 * Microsoft's symbols, where the Windows SDK's `cdb` is installed. */
function windowsStacks(pid: number, lldb: string): string {
    let out = "";
    try {
        const list = execFileSync(
            "powershell",
            [
                "-NoProfile",
                "-Command",
                `(Get-Process -Id ${pid}).Modules | ForEach-Object { "$($_.BaseAddress.ToInt64()) $($_.ModuleMemorySize) $($_.ModuleName)" }`,
            ],
            { encoding: "utf8", timeout: 60000 },
        );
        const modules = list
            .split(/\r?\n/)
            .map((l) => l.trim().split(" "))
            .filter((f) => f.length === 3)
            .map(([base, size, name]) => ({ base: BigInt(base), size: BigInt(size), name }));
        const addresses = [...new Set(lldb.match(/0x[0-9a-f]{16}/gi) ?? [])];
        out += "\nmodules of lldb's addresses:";
        for (const a of addresses) {
            const at = BigInt(a);
            const m = modules.find((m) => at >= m.base && at < m.base + m.size);
            out += `\n  ${a} ${m ? `${m.name}+0x${(at - m.base).toString(16)}` : "in no module"}`;
        }
    } catch (m) {
        out += `\nmodules not read (${(m as Error).message})`;
    }
    const cdb = path.join(process.env["ProgramFiles(x86)"] ?? "C:\\Program Files (x86)", "Windows Kits", "10", "Debuggers", "x64", "cdb.exe");
    if (!fs.existsSync(cdb)) {
        return `${out}\ncdb: not installed (${cdb})`;
    }
    try {
        const symbols = path.join(os.tmpdir(), "qb64rust-it-symbols");
        const stacks = execFileSync(cdb, ["-pv", "-p", String(pid), "-c", "~*kn 30; q"], {
            encoding: "utf8",
            timeout: 300000,
            env: { ...process.env, _NT_SYMBOL_PATH: `srv*${symbols}*https://msdl.microsoft.com/download/symbols` },
        });
        out += `\ncdb's stacks:\n${stacks}`;
    } catch (c) {
        const err = c as { message: string; stdout?: string };
        out += `\ncdb failed (${err.message}):\n${err.stdout ?? ""}`;
    }
    return out;
}

/** Resolves when the next debug session ends. */
function nextEnd(): Promise<void> {
    return new Promise((resolve) => {
        const d = vscode.debug.onDidTerminateDebugSession(() => {
            d.dispose();
            resolve();
        });
    });
}

function breakAt(uri: vscode.Uri, line: number): void {
    vscode.debug.removeBreakpoints(vscode.debug.breakpoints);
    vscode.debug.addBreakpoints([new vscode.SourceBreakpoint(new vscode.Location(uri, new vscode.Position(line - 1, 0)))]);
}

describe("debugging with qb64rust", function () {
    this.timeout(900000);
    let tracker: vscode.Disposable | undefined;

    before(async function () {
        if (!compiler || !server) {
            console.log("    skipped: needs QB64pe and QB64RUST_TEST_QB64RUST");
            this.skip();
        }
        await setSetting("qb64rust", "buildWith", "auto");
        await (await api()).server.ready;
        tracker = vscode.debug.registerDebugAdapterTrackerFactory("qb64rust", {
            createDebugAdapterTracker: () => ({ onDidSendMessage: (m) => sent.push(m) }),
        });
    });
    after(async () => {
        tracker?.dispose();
        vscode.debug.removeBreakpoints(vscode.debug.breakpoints);
        await vscode.debug.stopDebugging();
        await setSetting("qb64rust", "buildWith", "qb64pe");
        await closeAll();
    });

    it("the Debug command builds with --debug and stops at a breakpoint in a SUB, its parameter read by BASIC name", async () => {
        const a = await api();
        const uri = fixture();
        await open(uri);
        breakAt(uri, 9);
        const from = sent.length;
        const ended = nextEnd();
        assert.strictEqual(await vscode.commands.executeCommand("qb64rust.debug", uri), true);
        assert.match(a.builder.lastCommand ?? "", /--debug/);
        const exe = uri.fsPath.replace(/\.bas$/i, process.platform === "win32" ? ".exe" : "");
        assert.ok(fs.existsSync(exe) && fs.existsSync(`${exe}.debug.json`), "the debug build and its symbol file");
        const stopped = await stoppedAfter(from);
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
        const scopes = await session.customRequest("scopes", { frameId: st.stackFrames[0].id });
        assert.strictEqual(scopes.scopes[0].name, "Locals");
        const locals = await session.customRequest("variables", { variablesReference: scopes.scopes[0].variablesReference });
        const n = locals.variables.find((v: { name: string }) => v.name === "N");
        assert.deepStrictEqual([n?.value, n?.type], ["5", "LONG"]);
        await session.customRequest("continue", { threadId });
        await ended;
    });

    it("F5 with no launch.json debugs the active QB64 file", async () => {
        const uri = fixture();
        await open(uri);
        breakAt(uri, 3);
        const from = sent.length;
        const ended = nextEnd();
        // F5 itself: the test workspace has no launch.json, so VS Code picks the debugger of the active file's language.
        await vscode.commands.executeCommand("workbench.action.debug.start");
        const stopped = await stoppedAfter(from);
        const session = vscode.debug.activeDebugSession!;
        const st = await session.customRequest("stackTrace", { threadId: stopped.body!.threadId, levels: 1 });
        assert.deepStrictEqual([st.stackFrames[0].name, st.stackFrames[0].line], ["Main module", 3]);
        await vscode.debug.stopDebugging(session);
        await ended;
    });

    it("F5 with an included file active debugs its program, in the program's folder", async () => {
        const a = await api();
        const main = writeFile("debug_inc/main.bas", "'$INCLUDE:'rules/part.bm'\r\nSUB greet\r\nPRINT \"hi\"\r\nEND SUB\r\n");
        // The two files show in a failure how far the program got (the CI-only hang, STATUS.md).
        const part = writeFile(
            "debug_inc/rules/part.bm",
            'OPEN "started.txt" FOR OUTPUT AS #1: CLOSE #1\r\nCALL greet\r\nOPEN "ended.txt" FOR OUTPUT AS #1: CLOSE #1\r\n',
        );
        await open(part);
        await waitFor("the program of part.bm", () => a.server.programText === "main.bas");
        breakAt(main, 3);
        const from = sent.length;
        const started = new Promise<vscode.DebugSession>((resolve) => {
            const d = vscode.debug.onDidStartDebugSession((s) => {
                d.dispose();
                resolve(s);
            });
        });
        const ended = nextEnd();
        await vscode.commands.executeCommand("workbench.action.debug.start");
        const session = await started;
        const exe = main.fsPath.replace(/\.bas$/i, process.platform === "win32" ? ".exe" : "");
        assert.deepStrictEqual([session.configuration.program, session.configuration.cwd], [exe, path.dirname(main.fsPath)]);
        // The build is done once the session starts, so the stop comes in seconds (3.5 s in CI for the test above).
        const marks = () =>
            ["started.txt", "ended.txt"]
                .map((f) => `${f} ${fs.existsSync(path.join(path.dirname(main.fsPath), f)) ? "written" : "not written"}`)
                .join(", ");
        const stopped = await stoppedAfter(from, 120000, marks);
        const st = await session.customRequest("stackTrace", { threadId: stopped.body!.threadId, levels: 1 });
        assert.deepStrictEqual([st.stackFrames[0].name, st.stackFrames[0].line], ["SUB greet", 3]);
        await vscode.debug.stopDebugging(session);
        await ended;
    });

    it("a build that fails starts no session, and its error is in Problems", async () => {
        const a = await api();
        const uri = writeFile("debug_error.bas", "x = 1\ny = \n");
        await open(uri);
        const before = vscode.debug.activeDebugSession;
        assert.strictEqual(await vscode.commands.executeCommand("qb64rust.debug", uri), false);
        assert.strictEqual(vscode.debug.activeDebugSession, before);
        const errors = a.diagnostics.buildCollection.get(uri) ?? [];
        assert.ok(
            errors.some((d) => d.range.start.line === 1),
            JSON.stringify(errors),
        );
    });

    it("qb64rust.buildWith qb64pe refuses, naming the setting, and builds nothing", async () => {
        const a = await api();
        await setSetting("qb64rust", "buildWith", "qb64pe");
        try {
            const uri = fixture();
            const exe = uri.fsPath.replace(/\.bas$/i, process.platform === "win32" ? ".exe" : "");
            // Run alone, or after the first test failed, there may be no executable yet: none must appear.
            const built = fs.existsSync(exe) ? fs.statSync(exe).mtimeMs : undefined;
            a.debug.lastRefusal = undefined;
            assert.strictEqual(await vscode.commands.executeCommand("qb64rust.debug", uri), false);
            assert.match(a.debug.lastRefusal ?? "", /qb64rust\.buildWith/);
            assert.strictEqual(fs.existsSync(exe) ? fs.statSync(exe).mtimeMs : undefined, built, "nothing built");
            await vscode.commands.executeCommand("notifications.clearAll");
        } finally {
            await setSetting("qb64rust", "buildWith", "auto");
        }
    });
});
