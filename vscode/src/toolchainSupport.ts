// The C++ toolchain in VS Code (change m6-toolchain, design D6): the lookup kept for the language status, the offer
// before a build or a debug session, and the command "QB64: Install the C++ Toolchain". The rules are in
// compiler/toolchain.ts; here are the window and the processes.
import { execFile, spawn } from "child_process";
import * as path from "path";
import * as vscode from "vscode";
import { refuse } from "./buildPlan";
import * as config from "./config";
import { killTree } from "./compiler/runQueue";
import * as tc from "./compiler/toolchain";
import { LanguageServer } from "./languageServer";
import { CompilerStatus } from "./statusBar";

/** The reason of the lookup before it ran: the status shows no warning for it. */
export const NOT_SEARCHED = "not searched yet";

/** Runs a short command (`qb64rust toolchain path`) and gives its exit code and output. */
function runShort(command: string, args: string[]): Promise<tc.ProcessRun> {
    return new Promise((resolve) => {
        execFile(command, args, { windowsHide: true, timeout: 30000 }, (err, stdout, stderr) => {
            const code = err ? (typeof err.code === "number" ? err.code : null) : 0;
            resolve({ exitCode: code, output: `${stdout}${stderr}` || (err ? err.message : "") });
        });
    });
}

export class ToolchainSupport implements vscode.Disposable {
    private discovery: tc.ToolchainDiscovery = { found: false, reason: NOT_SEARCHED };
    private generation = 0;
    private readonly disposables: vscode.Disposable[];
    /** Replaces the window in the offer; for tests. */
    ui: tc.OfferUi;

    constructor(
        private readonly status: CompilerStatus,
        private readonly server: LanguageServer,
        private readonly output: vscode.OutputChannel,
    ) {
        this.ui = this.windowUi();
        this.disposables = [
            vscode.commands.registerCommand(tc.INSTALL_COMMAND, () => tc.installCommand(vscode.workspace.isTrusted, () => this.lookup(false), this.ui)),
            vscode.workspace.onDidChangeConfiguration((e) => {
                if (config.affectsToolchainPath(e)) {
                    void this.refresh();
                }
            }),
        ];
    }

    /** The last lookup's result. */
    get current(): tc.ToolchainDiscovery {
        return this.discovery;
    }

    /** Looks for the toolchain: with the setting and the QB64pe found (`withSetting`), or neither. */
    private lookup(withSetting: boolean): Promise<tc.ToolchainDiscovery> {
        const qb64pe = this.status.current;
        const binary = this.server.binary;
        return tc.findToolchain({
            setting: withSetting ? config.toolchainPath() : "",
            binary: binary.found ? binary.path : undefined,
            qb64pe: withSetting && qb64pe.found ? path.dirname(qb64pe.path) : undefined,
            run: runShort,
        });
    }

    /** Looks again (at activation, and when the binary, QB64pe or the setting change) and shows the result. */
    async refresh(): Promise<tc.ToolchainDiscovery> {
        const generation = ++this.generation;
        const found = await this.lookup(true);
        if (generation === this.generation) {
            this.discovery = found;
            this.status.render();
        }
        return found;
    }

    /**
     * The toolchain for a build or a debug session: looked for now, and when there is none offered if `interactive`
     * (spec editor/packaging, "Toolchain offer"). A result that is not found means no build: the reason has been
     * shown when `interactive`. The download is offered only when it would give a toolchain: a setting that names
     * no toolchain is shown with a button to it, since the setting would still win after an install.
     */
    async ensure(interactive: boolean): Promise<tc.ToolchainDiscovery> {
        const now = await this.refresh();
        if (now.found || !interactive) {
            return now;
        }
        if (!now.installable) {
            await refuse(now.reason, now.setting);
            return now;
        }
        return (await tc.offerToolchain(this.ui)) ?? { found: false, reason: "no C++ toolchain: the download was not made" };
    }

    private windowUi(): tc.OfferUi {
        return {
            // Modal: a build waits on it, and a notification could be hidden (design D6). "Not Now" is the dialog's
            // close button, so closing it is "Not Now" and no "Cancel" is added.
            ask: async (message, ...choices) => {
                const items: vscode.MessageItem[] = choices.map((title) => ({ title, isCloseAffordance: title === tc.NOT_NOW }));
                return (await vscode.window.showWarningMessage(message, { modal: true }, ...items))?.title;
            },
            install: () => this.install(),
            error: (message, ...items) => vscode.window.showErrorMessage(message, ...items),
            info: (message) => vscode.window.showInformationMessage(message),
            showOutput: () => this.output.show(true),
            find: () => this.refresh(),
        };
    }

    /** `qb64rust toolchain install` under a cancellable notification, its lines in the "QB64" output. */
    private install(): Promise<tc.InstallRun> {
        const binary = this.server.binary;
        if (!binary.found) {
            return Promise.resolve({ exitCode: null, output: `qb64rust: ${binary.reason}`, cancelled: false });
        }
        this.output.show(true);
        this.output.appendLine(`> ${binary.path} toolchain install`);
        return Promise.resolve(
            vscode.window.withProgress(
                { location: vscode.ProgressLocation.Notification, title: "QB64: C++ toolchain", cancellable: true },
                (progress, token) =>
                    new Promise<tc.InstallRun>((resolve) => {
                        const chunks: string[] = [];
                        let shown = 0;
                        let cancelled = false;
                        const child = spawn(binary.path, ["toolchain", "install"], { windowsHide: true });
                        const sub = token.onCancellationRequested(() => {
                            cancelled = true;
                            killTree(child);
                        });
                        let partial = "";
                        const onData = (data: Buffer) => {
                            const text = data.toString("latin1");
                            chunks.push(text);
                            this.output.append(text);
                            const lines = (partial + text).split(/\r?\n/);
                            partial = lines.pop() ?? "";
                            for (const line of lines) {
                                const percent = tc.downloadPercent(line);
                                // The download is 90 % of the bar; checking and unpacking the rest.
                                const at = percent !== undefined ? Math.round(percent * 0.9) : /^unpacking/.test(line) ? 92 : shown;
                                progress.report({ message: line.trim(), increment: Math.max(0, at - shown) });
                                shown = Math.max(shown, at);
                            }
                        };
                        child.stdout?.on("data", onData);
                        child.stderr?.on("data", onData);
                        let done = false;
                        const finish = (exitCode: number | null) => {
                            if (!done) {
                                done = true;
                                sub.dispose();
                                resolve({ exitCode, output: chunks.join(""), cancelled });
                            }
                        };
                        child.on("error", (err) => {
                            chunks.push(`${err.message}\n`);
                            finish(null);
                        });
                        child.on("close", (code) => finish(code));
                    }),
            ),
        );
    }

    dispose(): void {
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}
