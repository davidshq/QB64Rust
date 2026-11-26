// "QB64: Run in Browser" and the Emscripten SDK in VS Code (OpenSpec change wasm-publish, design D5; spec
// editor/build-run, "Run in browser"; spec editor/packaging, "Emscripten SDK offer" and "Emscripten SDK install
// command"). The rules are in compiler/emsdk.ts; here are the window, the build and the page's server.
import { ChildProcess, execFile, spawn } from "child_process";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { Builder } from "./build";
import { Bundle, planQb64rust, refuse } from "./buildPlan";
import * as config from "./config";
import * as em from "./compiler/emsdk";
import { parseQb64rustOutput } from "./compiler/parseQb64rustOutput";
import { tickWalkthrough } from "./walkthrough";
import { RunQueue, killTree } from "./compiler/runQueue";
import * as tc from "./compiler/toolchain";
import { Diagnostics } from "./diagnostics";
import { LanguageServer } from "./languageServer";
import { CompilerStatus } from "./statusBar";
import { ToolchainSupport, runShort } from "./toolchainSupport";

/** How long `qb64rust serve` may take to print its address. */
const ADDRESS_TIMEOUT_MS = 10_000;

/** A running `qb64rust serve` and the terminal that shows it. */
interface Served {
    child: ChildProcess;
    terminal: vscode.Terminal;
    /** Resolves when the process has ended. */
    ended: Promise<void>;
}

export class BrowserSupport implements vscode.Disposable {
    private readonly disposables: vscode.Disposable[];
    private served: Served | undefined;
    /** The port of the last server, asked for again so that the page's address stays the same. */
    private port: number | undefined;
    /** Replaces the window in the offer; for tests. */
    ui: em.EmsdkUi;
    /** Opens the page; tests replace it. */
    open: (url: string) => Thenable<unknown> = (url) => vscode.env.openExternal(vscode.Uri.parse(url));

    constructor(
        private readonly queue: RunQueue,
        private readonly status: CompilerStatus,
        private readonly diagnostics: Diagnostics,
        private readonly server: LanguageServer,
        private readonly bundle: Bundle,
        private readonly builder: Builder,
        private readonly toolchain: ToolchainSupport,
    ) {
        this.ui = this.windowUi();
        this.disposables = [
            vscode.commands.registerCommand(em.RUN_COMMAND, (uri?: vscode.Uri) => this.runInBrowser(uri)),
            vscode.commands.registerCommand(em.INSTALL_COMMAND, () => em.installCommand(vscode.workspace.isTrusted, this.ui)),
        ];
    }

    /** The SDK a build for the browser would use now. */
    find(): Promise<em.EmsdkDiscovery> {
        const binary = this.server.binary;
        return em.findEmsdk({ setting: config.emsdkPath(), binary: binary.found ? binary.path : undefined, run: runShort });
    }

    /** The SDK for a build, offered when there is none and an install would give one. */
    private async ensure(): Promise<em.EmsdkDiscovery | undefined> {
        const now = await this.find();
        if (now.found) {
            return now;
        }
        if (!now.installable) {
            await refuse(now.reason, now.setting);
            return undefined;
        }
        return em.offerEmsdk(this.ui, now);
    }

    /**
     * Builds the program of `uri` or the active editor for the wasm target and, when the build wrote a page, serves
     * its folder and opens the address. Resolves to true when the page was opened.
     */
    async runInBrowser(uri?: vscode.Uri): Promise<boolean> {
        const source = await this.builder.prepare(uri, "Running in a browser");
        if (!source) {
            return false;
        }
        const file = source.fsPath;
        if (config.buildWith(source) === "qb64pe") {
            await refuse(em.NEEDS_QB64RUST, "qb64rust.buildWith");
            return false;
        }
        // The binary, the C++ toolchain (a wasm build uses its `make`) and the copy of libqb, as a desktop build.
        const plan = await planQb64rust(file, this.status, this.server, this.bundle);
        if (plan.compiler === "none") {
            return false;
        }
        const sdk = await this.ensure();
        if (!sdk?.found) {
            return false;
        }
        const page = await pagePath(plan.command, file);
        if (!page) {
            void vscode.window.showErrorMessage("QB64: this qb64rust does not say where a program's page goes (--print-paths --target wasm). It may be older than the browser build.");
            return false;
        }
        const args = em.wasmBuildArgs(file, plan.roots, sdk.folder);
        const output = this.builder.output;
        output.clear();
        output.show(true);
        output.appendLine(`> ${plan.command} ${args.join(" ")}`);
        const started = Date.now();
        const result = await this.queue.run({ kind: "build", command: plan.command, args, cwd: path.dirname(file), env: plan.env, onOutput: (text) => output.append(text) });
        if (result.cancelled) {
            return false;
        }
        const parsed = parseQb64rustOutput(result.output, result.exitCode, file);
        this.diagnostics.publish(source, parsed.messages, "qb64rust");
        const first = parsed.messages.find((m) => m.severity === "error");
        if (result.exitCode !== 0 || first) {
            void vscode.window.showErrorMessage(`QB64: the build for the browser failed${first ? `: ${first.message.split("\n")[0]} (line ${first.line})` : ""}`);
            return false;
        }
        if (!em.pageWasBuilt(modifiedTime(page), started)) {
            void vscode.window.showInformationMessage(em.NO_PAGE);
            return false;
        }
        const url = await this.serve(plan.command, path.dirname(page));
        if (!url) {
            void vscode.window.showErrorMessage(`QB64: the page's server did not start; see the "${em.TERMINAL_NAME}" terminal.`);
            return false;
        }
        await this.open(url);
        tickWalkthrough("ranInBrowser");
        return true;
    }

    /** Stops the server of a run before, and waits until its port is free again. */
    private async stop(): Promise<void> {
        const served = this.served;
        this.served = undefined;
        if (served) {
            killTree(served.child);
            await served.ended;
            served.terminal.dispose();
        }
    }

    /**
     * Serves `folder` with `qb64rust serve` in the "QB64 Web" terminal and resolves to the address it prints, or
     * undefined when it ends without one. The port of the run before is asked for again; when it is taken now, the
     * server picks one.
     */
    private async serve(binary: string, folder: string): Promise<string | undefined> {
        await this.stop();
        const first = await this.start(binary, folder, this.port);
        if (first || this.port === undefined) {
            return first;
        }
        await this.stop();
        return this.start(binary, folder, undefined);
    }

    private start(binary: string, folder: string, port: number | undefined): Promise<string | undefined> {
        const writer = new vscode.EventEmitter<string>();
        const closer = new vscode.EventEmitter<void>();
        const child = spawn(binary, em.serveArgs(folder, port), { windowsHide: true });
        // What the server printed before the terminal opened is shown once it has.
        let waiting: string[] | undefined = [];
        const write = (text: string) => {
            const shown = text.replace(/\r?\n/g, "\r\n");
            if (waiting) {
                waiting.push(shown);
            } else {
                writer.fire(shown);
            }
        };
        const terminal = vscode.window.createTerminal({
            name: em.TERMINAL_NAME,
            pty: {
                onDidWrite: writer.event,
                onDidClose: closer.event,
                open: () => {
                    const held = waiting ?? [];
                    waiting = undefined;
                    held.forEach((text) => writer.fire(text));
                },
                // Closing the terminal stops the server.
                close: () => killTree(child),
            },
        });
        return new Promise((resolve) => {
            let partial = "";
            let answered = false;
            const answer = (url: string | undefined) => {
                if (!answered) {
                    answered = true;
                    resolve(url);
                }
            };
            const timer = setTimeout(() => answer(undefined), ADDRESS_TIMEOUT_MS);
            const onData = (data: Buffer) => {
                const text = data.toString("utf8");
                write(text);
                const lines = (partial + text).split(/\r?\n/);
                partial = lines.pop() ?? "";
                for (const line of lines) {
                    const address = em.parseAddress(line);
                    if (address) {
                        this.port = address.port;
                        answer(address.url);
                    }
                }
            };
            child.stdout?.on("data", onData);
            child.stderr?.on("data", onData);
            const ended = new Promise<void>((done) => {
                const end = (text: string) => {
                    clearTimeout(timer);
                    write(text);
                    answer(undefined);
                    done();
                };
                child.on("error", (err) => end(`${err.message}\n`));
                child.on("close", (code) => end(code ? `[qb64rust serve ended with exit code ${code}]\n` : "[qb64rust serve ended]\n"));
            });
            this.served = { child, terminal, ended };
            terminal.show(true);
        });
    }

    private windowUi(): em.EmsdkUi {
        return {
            ask: async (message, ...choices) => {
                const items: vscode.MessageItem[] = choices.map((title) => ({ title, isCloseAffordance: title === tc.NOT_NOW }));
                return (await vscode.window.showWarningMessage(message, { modal: true }, ...items))?.title;
            },
            install: () => this.toolchain.install(["toolchain", "install", "--target", "wasm"], "QB64: Emscripten SDK"),
            error: (message, ...items) => vscode.window.showErrorMessage(message, ...items),
            info: (message) => vscode.window.showInformationMessage(message),
            showOutput: () => this.builder.output.show(true),
            find: () => this.find(),
            platform: process.platform,
        };
    }

    dispose(): void {
        if (this.served) {
            killTree(this.served.child);
            this.served.terminal.dispose();
        }
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}

/** The page `qb64rust --print-paths --target wasm` names for `file`, or undefined when it names none. */
function pagePath(binary: string, file: string): Promise<string | undefined> {
    return new Promise((resolve) => {
        execFile(binary, em.pagePathArgs(file), { cwd: path.dirname(file), timeout: 10_000, windowsHide: true }, (error, stdout) => {
            resolve(error ? undefined : em.parsePagePath(String(stdout)));
        });
    });
}

/** The modification time of `file` in milliseconds, or undefined when there is no such file. */
function modifiedTime(file: string): number | undefined {
    try {
        return fs.statSync(file).mtimeMs;
    } catch {
        return undefined;
    }
}
