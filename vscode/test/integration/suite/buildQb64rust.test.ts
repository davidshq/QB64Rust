// Build, Build and Run and the build task with the new compiler (change extension-vsix, task 4.4; spec
// editor/build-run). Needs QB64pe (its toolchain) and the qb64rust binary (QB64RUST_TEST_QB64RUST); skipped
// without either. The first build in a fresh test profile builds libqb's objects in the prepared copy.
import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { api, closeAll, compiler, dismissingNotifications, open, server, setSetting, setText, waitFor, writeFile } from "./helpers";

const exeOf = (uri: vscode.Uri) => uri.fsPath.replace(/\.bas$/i, process.platform === "win32" ? ".exe" : "");

/** A build's diagnostics with qb64rust (source `qb64rust`, the extension's own collection). */
async function buildDiagnostics(uri: vscode.Uri): Promise<readonly vscode.Diagnostic[]> {
    return (await api()).diagnostics.buildCollection.get(uri) ?? [];
}

describe("build with qb64rust", function () {
    this.timeout(900000);

    before(async function () {
        if (!compiler || !server) {
            console.log("    skipped: needs QB64pe and QB64RUST_TEST_QB64RUST");
            this.skip();
        }
        await setSetting("qb64rust", "buildWith", "auto");
        await (await api()).server.ready;
    });
    after(async () => {
        await setSetting("qb64rust", "buildWith", "qb64pe");
    });
    afterEach(closeAll);

    it("built by qb64rust: the status says so, the executable runs, libqb is prepared in global storage", async () => {
        const a = await api();
        assert.match(a.status.detail ?? "", /builds with qb64rust$/);
        const uri = writeFile("rbuild/hello.bas", '$CONSOLE:ONLY\r\nOPEN "hello.txt" FOR OUTPUT AS #1\r\nPRINT #1, "from qb64rust"\r\nCLOSE #1\r\nSYSTEM\r\n');
        const out = path.join(path.dirname(uri.fsPath), "hello.txt");
        await open(uri);
        const ran = await vscode.commands.executeCommand<boolean>("qb64rust.buildAndRun");
        assert.strictEqual(ran, true, `Build and Run failed; the last command: ${a.builder.lastCommand ?? "none (no build started)"}`);
        assert.ok(a.builder.lastCommand?.startsWith(`${server} -x -w ${uri.fsPath}`), a.builder.lastCommand);
        assert.ok(a.builder.lastCommand?.includes(`--qb64pe-root ${path.dirname(compiler)}`), a.builder.lastCommand);
        const text = await waitFor("hello.txt", () => fs.existsSync(out) && fs.readFileSync(out, "latin1").trim(), 60000);
        assert.strictEqual(text, "from qb64rust");
        if (fs.existsSync(path.join(a.bundle.libqb, "internal", "c", "qbx.cpp"))) {
            const copy = path.join(a.bundle.storage, `libqb-${a.bundle.version}`); // version and libqb.id
            assert.ok(a.builder.lastCommand?.includes(`--libqb-root ${copy}`), a.builder.lastCommand);
            const objects = (dir: string) => fs.readdirSync(path.join(dir, "internal", "c")).filter((n) => n.endsWith(".o"));
            assert.ok(objects(copy).length > 0, "no libqb object in the prepared copy");
            assert.deepStrictEqual(objects(a.bundle.libqb), [], "the bundle was written to");
        }
    });

    it("failed build: the error in an included file at its column, source qb64rust, removed on edit", async () => {
        writeFile("rbuild/inc/lib.bi", "PRINT 2\r\nDIM y AS STRING\r\ny = 5\r\n");
        const uri = writeFile("rbuild/inc/main.bas", "'$INCLUDE:'lib.bi'\r\nPRINT 1\r\n");
        const lib = vscode.Uri.file(path.join(path.dirname(uri.fsPath), "lib.bi"));
        await open(uri);
        const exe = await dismissingNotifications(vscode.commands.executeCommand<string | undefined>("qb64rust.build"));
        assert.strictEqual(exe, undefined);
        const diags = await buildDiagnostics(lib);
        assert.deepStrictEqual(
            diags.map((d) => [d.range.start.line, d.range.start.character, d.source]),
            [[2, 4, "qb64rust"]],
        );
        assert.match(diags[0].message, /^cannot store a number in a string variable\nQB64pe: /);
        const editor = await open(lib);
        await setText(editor, "PRINT 2\r\nDIM y AS STRING\r\ny = \"5\"\r\n");
        assert.deepStrictEqual(await buildDiagnostics(lib), []);
        await editor.document.save();
    });

    it("only unsupported forms: 'Build with QB64pe' builds that program once with QB64pe", async () => {
        const a = await api();
        // A form qb64rust marks "not supported yet" and QB64pe builds; `_DEVICES` was it until M4 built it.
        const uri = writeFile("rbuild/marked.bas", "$CONSOLE:ONLY\r\nSUB _GL\r\nEND SUB\r\n");
        await open(uri);
        const offered: string[][] = [];
        const show = a.builder.showError;
        a.builder.showError = (message, ...items) => {
            offered.push([message, ...items]);
            return Promise.resolve(items[0]);
        };
        try {
            const exe = await vscode.commands.executeCommand<string | undefined>("qb64rust.build");
            assert.deepStrictEqual(offered, [["QB64: build failed: not supported yet: `SUB _GL` (OpenGL) (line 2)", "Build with QB64pe"]]);
            assert.strictEqual(exe, exeOf(uri));
            assert.ok(a.builder.lastCommand?.startsWith(compiler), a.builder.lastCommand);
            assert.deepStrictEqual(await buildDiagnostics(uri), []);
            assert.strictEqual(vscode.workspace.getConfiguration("qb64rust").get("buildWith"), "auto");
        } finally {
            a.builder.showError = show;
        }
    });

    it("an unmarked error offers no QB64pe build", async () => {
        const a = await api();
        const uri = writeFile("rbuild/mixed.bas", "x$ = 1\r\nSUB _GL\r\nEND SUB\r\n");
        await open(uri);
        const offered: string[][] = [];
        const show = a.builder.showError;
        a.builder.showError = (message, ...items) => {
            offered.push([message, ...items]);
            return Promise.resolve(undefined);
        };
        try {
            await dismissingNotifications(vscode.commands.executeCommand<string | undefined>("qb64rust.build"));
            assert.deepStrictEqual(offered, []);
            assert.strictEqual((await buildDiagnostics(uri)).length, 2);
        } finally {
            a.builder.showError = show;
        }
    });

    it("qb64pe chosen: QB64pe builds although qb64rust is found", async () => {
        const a = await api();
        await setSetting("qb64rust", "buildWith", "qb64pe");
        try {
            await waitFor("the status to follow", () => /builds with qb64pe$/.test(a.status.detail ?? ""));
            const uri = writeFile("rbuild/old.bas", '$CONSOLE:ONLY\r\nPRINT "old"\r\n');
            await open(uri);
            assert.strictEqual(await vscode.commands.executeCommand<string | undefined>("qb64rust.build"), exeOf(uri));
            assert.ok(a.builder.lastCommand?.startsWith(compiler), a.builder.lastCommand);
        } finally {
            await setSetting("qb64rust", "buildWith", "auto");
        }
    });

    it("qb64rust chosen but missing: an error, no build, no QB64pe instead", async () => {
        const a = await api();
        const configured = vscode.workspace.getConfiguration("qb64rust").get<string>("path");
        await setSetting("qb64rust", "buildWith", "qb64rust");
        await setSetting("qb64rust", "path", path.join(path.dirname(compiler), "no-qb64rust-here.exe"));
        await a.server.ready;
        try {
            await waitFor("the binary to be missing", () => !a.server.binary.found);
            const uri = writeFile("rbuild/missing.bas", "PRINT 1\r\n");
            await open(uri);
            const before = a.builder.lastCommand;
            const exe = await dismissingNotifications(vscode.commands.executeCommand<string | undefined>("qb64rust.build"));
            assert.strictEqual(exe, undefined);
            assert.strictEqual(a.builder.lastCommand, before, "a build was started");
            assert.ok(!fs.existsSync(exeOf(uri)));

            // The build task does the same: it says why and fails, and does not run QB64pe instead.
            const task = (await vscode.tasks.fetchTasks({ type: "qb64rust" })).find((t) => t.name === "build")!;
            assert.ok(task.execution instanceof vscode.CustomExecution, "the task would run a compiler");
            const ended = new Promise<number | undefined>((resolve) => {
                const sub = vscode.tasks.onDidEndTaskProcess((e) => {
                    if (e.execution.task.name === "build" && e.execution.task.source === "qb64rust") {
                        sub.dispose();
                        resolve(e.exitCode);
                    }
                });
            });
            await vscode.tasks.executeTask(task);
            assert.strictEqual(await ended, 1);
            assert.ok(!fs.existsSync(exeOf(uri)));
        } finally {
            await setSetting("qb64rust", "path", configured);
            await setSetting("qb64rust", "buildWith", "auto");
            await a.server.ready;
            await waitFor("the server to run again", () => a.server.running);
        }
    });

    it("the build task runs qb64rust with the Build command's arguments", async () => {
        const uri = writeFile("rbuild/task.bas", '$CONSOLE:ONLY\r\nPRINT "task"\r\n');
        await open(uri);
        const tasks = await vscode.tasks.fetchTasks({ type: "qb64rust" });
        const build = tasks.find((t) => t.name === "build")!;
        const execution = build.execution as vscode.ProcessExecution;
        assert.strictEqual(execution.process, server);
        assert.deepStrictEqual(execution.args.slice(0, 5), ["-x", "-w", uri.fsPath, "-o", exeOf(uri)]);
        assert.ok(execution.args.includes("--qb64pe-root"));
        const ended = new Promise<number | undefined>((resolve) => {
            const sub = vscode.tasks.onDidEndTaskProcess((e) => {
                if (e.execution.task.name === "build" && e.execution.task.source === "qb64rust") {
                    sub.dispose();
                    resolve(e.exitCode);
                }
            });
        });
        await vscode.tasks.executeTask(build);
        assert.strictEqual(await ended, 0);
        assert.ok(fs.existsSync(exeOf(uri)));
    });
});
