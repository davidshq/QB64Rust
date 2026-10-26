// Debugging in VS Code (change m5-debug-launch, design D8; spec editor/debug-launch). Needs QB64pe (its toolchain and
// its lldb-dap) and the qb64rust binary (QB64RUST_TEST_QB64RUST); skipped without either. The sessions run
// `qb64rust dap` over the real lldb-dap, with the default console (the integrated terminal).
import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { api, closeAll, compiler, open, server, setSetting, waitFor, workspace, writeFile } from "./helpers";

/** The fixture: `SUB show (n AS LONG)` on line 8, its statement `total = total + n` on line 9. */
const fixture = () => vscode.Uri.file(path.join(workspace, "debug_me.bas"));

/** Every message the adapter sent, of every session since the suite began. */
const sent: { type?: string; event?: string; body?: Record<string, unknown> }[] = [];

/** The next `stopped` event after `from` messages. */
function stoppedAfter(from: number) {
    return waitFor("a stopped event", () => sent.slice(from).find((m) => m.type === "event" && m.event === "stopped"), 600000);
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
        const exe = uri.fsPath.replace(/\.bas$/i, ".exe");
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
            const exe = uri.fsPath.replace(/\.bas$/i, ".exe");
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
