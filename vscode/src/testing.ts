// The tests of the workspace in the Test Explorer (change basic-unit-tests, design D7; spec editor/test-explorer):
// listed and run through `qb64rust test`, which needs the qb64rust binary and its toolchain whatever
// `qb64rust.buildWith` says. The JSON lines and the filters are read and made in compiler/testOutput.ts.
import { spawn } from "child_process";
import * as path from "path";
import * as vscode from "vscode";
import { Bundle, planQb64rust } from "./buildPlan";
import { rootArgs } from "./compiler/qb64rust";
import { killTree } from "./compiler/runQueue";
import { Chosen, JsonLines, listedTests, messageOf, runArgs, stateOf, TestEvent, testKey } from "./compiler/testOutput";
import { LanguageServer } from "./languageServer";
import { CompilerStatus } from "./statusBar";

/** The test files, `*_test.bas` in any case. */
const TEST_FILES = "**/*_[tT][eE][sS][tT].[bB][aA][sS]";

const LIST_TIMEOUT_MS = 30000;

function isTestFile(uri: vscode.Uri): boolean {
    return uri.scheme === "file" && /_test\.bas$/i.test(uri.fsPath);
}

/** Runs `command` and gives its exit code and stdout (`--list` builds nothing, so it is quick). */
function capture(command: string, args: string[], cwd: string): Promise<{ code: number; stdout: string }> {
    return new Promise((resolve) => {
        const chunks: string[] = [];
        const child = spawn(command, args, { cwd, windowsHide: true });
        const timer = setTimeout(() => killTree(child), LIST_TIMEOUT_MS);
        // Decoded as a stream, so a character split between two chunks stays whole.
        child.stdout?.setEncoding("utf8");
        child.stdout?.on("data", (d: string) => chunks.push(d));
        child.on("error", () => resolve({ code: -1, stdout: "" }));
        child.on("close", (code) => {
            clearTimeout(timer);
            resolve({ code: code ?? -1, stdout: chunks.join("") });
        });
    });
}

export class TestSupport implements vscode.Disposable {
    readonly controller = vscode.tests.createTestController("qb64rust", "QB64");
    private readonly disposables: vscode.Disposable[] = [this.controller];
    /** The file of each item, and its SUB for a test. */
    private readonly info = new WeakMap<vscode.TestItem, { file: string; name?: string }>();
    /** The missing binary was told; told again after it was found once. */
    private toldMissing = false;
    /** Discovery at activation and after the binary was looked for again; for tests. */
    ready: Promise<void>;
    /** The arguments of the last run; for tests. */
    lastArgs: string[] | undefined;
    /** The state each test of the last run got, by its key (`testKey`); for tests. */
    readonly lastStates = new Map<string, "passed" | "failed" | "errored">();

    constructor(
        private readonly status: CompilerStatus,
        private readonly server: LanguageServer,
        private readonly bundle: Bundle,
    ) {
        this.controller.refreshHandler = () => this.discoverAll();
        this.controller.createRunProfile("Run", vscode.TestRunProfileKind.Run, (request, token) => this.run(request, token), true);
        const watcher = vscode.workspace.createFileSystemWatcher(TEST_FILES);
        this.disposables.push(
            watcher,
            watcher.onDidCreate((uri) => void this.refresh(uri)),
            watcher.onDidChange((uri) => void this.refresh(uri)),
            watcher.onDidDelete((uri) => this.controller.items.delete(uri.toString())),
            vscode.workspace.onDidSaveTextDocument((doc) => {
                if (isTestFile(doc.uri)) {
                    void this.refresh(doc.uri);
                }
            }),
            // The binary looked for again (a setting changed, the workspace trusted).
            server.onDidChangeRunning(() => {
                this.ready = this.discoverAll();
            }),
        );
        this.ready = this.discoverAll();
    }

    /** Lists the tests of every test file of the workspace; none, and the reason told once, without the binary. */
    async discoverAll(): Promise<void> {
        await this.server.ready;
        const files = await vscode.workspace.findFiles(TEST_FILES);
        const binary = this.server.binary;
        if (!binary.found || !vscode.workspace.isTrusted) {
            this.controller.items.replace([]);
            if (files.length > 0 && !binary.found && !this.toldMissing) {
                this.toldMissing = true;
                const open = "Open Setting";
                void vscode.window.showWarningMessage(`QB64: qb64rust is needed to run tests: ${binary.reason}`, open).then((choice) => {
                    if (choice === open) {
                        void vscode.commands.executeCommand("workbench.action.openSettings", "qb64rust.path");
                    }
                });
            }
            return;
        }
        this.toldMissing = false;
        const present = new Set(files.map((f) => f.toString()));
        this.controller.items.forEach((item) => {
            if (!present.has(item.id)) {
                this.controller.items.delete(item.id);
            }
        });
        await Promise.all(files.map((f) => this.refresh(f)));
    }

    /** Lists the tests of one file again: an item for the file with its tests below it, or none without a test. */
    async refresh(uri: vscode.Uri): Promise<void> {
        const binary = this.server.binary;
        if (!binary.found || !vscode.workspace.isTrusted || !isTestFile(uri)) {
            return;
        }
        const file = uri.fsPath;
        const { stdout } = await capture(binary.path, ["test", "--list", "--format", "json", file], path.dirname(file));
        const tests = listedTests(stdout);
        if (tests.length === 0) {
            this.controller.items.delete(uri.toString());
            return;
        }
        const item = this.controller.items.get(uri.toString()) ?? this.controller.createTestItem(uri.toString(), path.basename(file), uri);
        const folder = vscode.workspace.getWorkspaceFolder(uri);
        item.description = folder ? path.dirname(path.relative(folder.uri.fsPath, file)).replace(/^\.$/, "") : undefined;
        this.info.set(item, { file });
        item.children.replace(
            tests.map((t) => {
                const child = this.controller.createTestItem(`${uri.toString()}::${t.name}`, t.name, uri);
                child.range = new vscode.Range(t.line - 1, 0, t.line - 1, 0);
                this.info.set(child, { file, name: t.name });
                return child;
            }),
        );
        this.controller.items.add(item);
    }

    /** Runs the chosen tests through one `qb64rust test --format json` and marks each as its result comes. */
    async run(request: vscode.TestRunRequest, token: vscode.CancellationToken): Promise<void> {
        const run = this.controller.createTestRun(request);
        this.lastStates.clear();
        const excluded = new Set(request.exclude ?? []);
        const items = new Map<string, vscode.TestItem>();
        const chosen: Chosen[] = [];
        const roots: vscode.TestItem[] = [];
        if (request.include) {
            roots.push(...request.include);
        } else {
            this.controller.items.forEach((i) => roots.push(i));
        }
        for (const item of roots.filter((i) => !excluded.has(i))) {
            const info = this.info.get(item);
            if (!info) {
                continue;
            }
            const tests: vscode.TestItem[] = [];
            if (info.name === undefined) {
                item.children.forEach((c) => {
                    if (!excluded.has(c)) {
                        tests.push(c);
                    }
                });
                // A whole file, unless one of its tests is left out.
                if (tests.length === item.children.size) {
                    chosen.push({ file: info.file });
                } else {
                    chosen.push(...tests.map((t) => ({ file: info.file, name: this.info.get(t)?.name })));
                }
            } else {
                tests.push(item);
                chosen.push({ file: info.file, name: info.name });
            }
            for (const t of tests) {
                items.set(testKey(info.file, this.info.get(t)?.name ?? ""), t);
                run.enqueued(t);
            }
        }
        if (chosen.length === 0) {
            run.end();
            return;
        }
        const plan = await planQb64rust(chosen[0].file, this.status, this.server, this.bundle, true);
        if (plan.compiler === "none") {
            items.forEach((t) => run.errored(t, new vscode.TestMessage(`qb64rust test could not run: ${plan.reason}`)));
            run.end();
            return;
        }
        const args = [...runArgs(chosen), ...rootArgs(plan.roots)];
        this.lastArgs = args;
        const cwd = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? path.dirname(chosen[0].file);
        const marked = new Set<string>();
        await new Promise<void>((resolve) => {
            const child = spawn(plan.command, args, { cwd, env: plan.env, windowsHide: true, detached: process.platform !== "win32" });
            const cancel = token.onCancellationRequested(() => killTree(child));
            const lines = new JsonLines();
            const take = (events: TestEvent[]) => {
                for (const e of events) {
                    if (e.event !== "result") {
                        continue;
                    }
                    const key = testKey(e.file, e.name);
                    const item = items.get(key);
                    // `--filter` matches by containing: a test whose name starts with a chosen one's was not asked for.
                    if (!item || marked.has(key)) {
                        continue;
                    }
                    marked.add(key);
                    if (e.output) {
                        run.appendOutput(e.output.replace(/\r?\n/g, "\r\n"), undefined, item);
                    }
                    const state = stateOf(e.outcome);
                    this.lastStates.set(key, state);
                    if (state === "passed") {
                        run.passed(item, e.ms);
                        continue;
                    }
                    const message = new vscode.TestMessage(messageOf(e));
                    message.location = e.location
                        ? new vscode.Location(vscode.Uri.file(path.resolve(cwd, e.location.file)), new vscode.Position(e.location.line - 1, 0))
                        : item.uri && item.range
                          ? new vscode.Location(item.uri, item.range)
                          : undefined;
                    if (state === "failed") {
                        run.failed(item, message, e.ms);
                    } else {
                        run.errored(item, message, e.ms);
                    }
                }
            };
            // Decoded as streams, so a character split between two chunks stays whole.
            child.stdout?.setEncoding("utf8");
            child.stderr?.setEncoding("utf8");
            child.stdout?.on("data", (d: string) => take(lines.push(d)));
            child.stderr?.on("data", (d: string) => run.appendOutput(d.replace(/\r?\n/g, "\r\n")));
            child.on("error", (err) => run.appendOutput(`${err.message}\r\n`));
            child.on("close", () => {
                take(lines.flush());
                cancel.dispose();
                resolve();
            });
        });
        // Not cancelled, and a test got no result: the run ended without it.
        if (!token.isCancellationRequested) {
            items.forEach((t, key) => {
                if (!marked.has(key)) {
                    run.errored(t, new vscode.TestMessage("qb64rust test gave no result for this test; see the test run's output."));
                }
            });
        }
        run.end();
    }

    dispose(): void {
        this.disposables.forEach((d) => d.dispose());
    }
}
