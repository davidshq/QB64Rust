// Build, Run, and Build and Run (design D6; with qb64rust, design D4 and D5 of extension-vsix).
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import * as config from "./config";
import { BuildPlan, Bundle, planBuild } from "./buildPlan";
import { BuildOutputLines, progressText } from "./compiler/buildOutput";
import * as qb64pe from "./compiler/qb64pe";
import * as qb64rust from "./compiler/qb64rust";
import { RunQueue } from "./compiler/runQueue";
import { Diagnostics } from "./diagnostics";
import { LanguageServer } from "./languageServer";
import { CompilerStatus } from "./statusBar";

const TERMINAL_NAME = "QB64";

export class Builder implements vscode.Disposable {
    readonly output = vscode.window.createOutputChannel("QB64");
    private readonly disposables: vscode.Disposable[] = [this.output];
    private terminal: vscode.Terminal | undefined;
    /** The last build's command line as shown in the output channel; for tests. */
    lastCommand: string | undefined;
    /** Shows a failed build's message with its buttons; tests replace it to choose a button. */
    showError: (message: string, ...items: string[]) => Thenable<string | undefined> = (message, ...items) => vscode.window.showErrorMessage(message, ...items);

    constructor(
        private readonly queue: RunQueue,
        private readonly status: CompilerStatus,
        private readonly diagnostics: Diagnostics,
        private readonly server: LanguageServer,
        private readonly bundle: Bundle,
    ) {
        this.disposables.push(
            vscode.commands.registerCommand("qb64rust.build", (uri?: vscode.Uri) => this.buildCommand(uri)),
            vscode.commands.registerCommand("qb64rust.run", (uri?: vscode.Uri) => this.runCommand(uri)),
            vscode.commands.registerCommand("qb64rust.buildAndRun", (uri?: vscode.Uri) => this.buildAndRunCommand(uri)),
            vscode.window.onDidCloseTerminal((t) => {
                if (this.terminal === t) {
                    this.terminal = undefined;
                }
            }),
        );
    }

    /** Returns the path of the executable, or undefined if the build did not succeed. */
    buildCommand(uri?: vscode.Uri): Promise<string | undefined> {
        return this.build(uri, true);
    }

    /**
     * The debug build of the program of `uri` or the active editor (`--debug`, change m5-debug-launch, design D4):
     * the executable, or undefined when there is none (the reason already shown).
     */
    async debugBuild(uri?: vscode.Uri): Promise<string | undefined> {
        const doc = await this.prepare(uri, "Debugging");
        if (!doc) {
            return undefined;
        }
        const plan = await planBuild(doc.uri.fsPath, this.status, this.server, this.bundle, true, true);
        return plan.compiler === "qb64rust" ? this.buildWithQb64rust(doc, plan, false) : undefined;
    }

    /**
     * Builds; `notify` shows "built X" when it succeeds. Build and Run passes false: the program starting is the
     * confirmation (study 18, proposal D). Failures are always shown.
     */
    private async build(uri: vscode.Uri | undefined, notify: boolean): Promise<string | undefined> {
        const doc = await this.prepare(uri, "Building");
        if (!doc) {
            return undefined;
        }
        const plan = await planBuild(doc.uri.fsPath, this.status, this.server, this.bundle);
        if (plan.compiler === "none") {
            return undefined;
        }
        return plan.compiler === "qb64pe" ? this.buildWithQb64pe(doc, plan.command, notify) : this.buildWithQb64rust(doc, plan, notify);
    }

    /**
     * Builds with the new compiler (design D4 and D5 of extension-vsix). Its errors go to their own collection
     * (source `qb64rust`). A failure under `auto` whose errors are all marked "not supported yet" offers to build
     * the program once with QB64pe when QB64pe is found, except for a debug build, which QB64pe cannot make.
     */
    private async buildWithQb64rust(doc: vscode.TextDocument, plan: Extract<BuildPlan, { compiler: "qb64rust" }>, notify: boolean): Promise<string | undefined> {
        const file = doc.uri.fsPath;
        this.output.clear();
        this.output.show(true);
        this.lastCommand = `${plan.command} ${plan.args.join(" ")}`;
        this.output.appendLine(`> ${this.lastCommand}`);
        if (plan.firstLibqbBuild) {
            this.output.appendLine("This first build after installing or updating the extension also builds QB64pe's runtime library (libqb) once.");
        }
        const result = await qb64rust.build(this.queue, plan.command, file, plan.roots, plan.env, plan.debug, (text) => {
            this.output.append(text);
            if (plan.firstLibqbBuild && /^building /m.test(text)) {
                this.status.setBusy("building libqb (first build)…");
            }
        });
        if (result.cancelled) {
            return undefined;
        }
        this.diagnostics.publish(doc.uri, result.messages, "qb64rust");
        const ok = result.exitCode === 0 && fs.existsSync(result.executable) && !result.messages.some((m) => m.severity === "error");
        if (ok) {
            this.output.appendLine(`Built ${result.executable}`);
            if (notify) {
                void vscode.window.showInformationMessage(`QB64: built ${path.basename(result.executable)}`);
            }
            return result.executable;
        }
        const first = result.messages.find((m) => m.severity === "error");
        const text = `QB64: build failed${first ? `: ${first.message.split("\n")[0]} (line ${first.line})` : ""}`;
        if (result.allMarked && config.buildWith(doc.uri) === "auto" && !plan.debug && plan.qb64pe) {
            const fallback = "Build with QB64pe";
            const qb64peCompiler = plan.qb64pe;
            if ((await this.showError(text, fallback)) === fallback) {
                // The marked errors say what qb64rust lacks, not what is wrong with the program: QB64pe builds it.
                this.diagnostics.publish(doc.uri, [], "qb64rust");
                return this.buildWithQb64pe(doc, qb64peCompiler, notify);
            }
            return undefined;
        }
        void vscode.window.showErrorMessage(text);
        return undefined;
    }

    private async buildWithQb64pe(doc: vscode.TextDocument, compiler: string, notify: boolean): Promise<string | undefined> {
        const file = doc.uri.fsPath;
        this.output.clear();
        this.output.show(true);
        this.lastCommand = `${compiler} ${qb64pe.buildArgs(file).join(" ")}`;
        this.output.appendLine(`> ${this.lastCommand}`);
        // Progress goes to the status item, not the output channel, which cannot redraw a line.
        const lines = new BuildOutputLines((p) => this.status.setBusy(progressText(p)));
        const show = (completed: string[]) => completed.forEach((l) => this.output.appendLine(l));
        const result = await qb64pe.build(this.queue, compiler, file, (text) => show(lines.push(text)));
        show(lines.flush());
        if (result.cancelled) {
            return undefined;
        }
        this.diagnostics.publish(doc.uri, result.messages);
        const ok = result.exitCode === 0 && fs.existsSync(result.executable) && !result.messages.some((m) => m.severity === "error");
        if (ok) {
            this.output.appendLine(`Built ${result.executable}`);
            if (notify) {
                void vscode.window.showInformationMessage(`QB64: built ${path.basename(result.executable)}`);
            }
            return result.executable;
        }
        const first = result.messages.find((m) => m.severity === "error");
        void vscode.window.showErrorMessage(`QB64: build failed${first ? `: ${first.message} (line ${first.line})` : ""}`);
        return undefined;
    }

    async runCommand(uri?: vscode.Uri): Promise<boolean> {
        const doc = await this.prepare(uri, "Running", false);
        if (!doc) {
            return false;
        }
        const exe = qb64pe.executableFor(doc.uri.fsPath);
        if (!fs.existsSync(exe)) {
            const build = "Build and Run";
            const choice = await vscode.window.showErrorMessage(`QB64: ${path.basename(exe)} does not exist yet. Build it first?`, build);
            return choice === build ? this.buildAndRunCommand(doc.uri) : false;
        }
        this.startInTerminal(exe, doc.uri);
        return true;
    }

    async buildAndRunCommand(uri?: vscode.Uri): Promise<boolean> {
        const exe = await this.build(uri, false);
        if (!exe) {
            return false;
        }
        this.startInTerminal(exe, vscode.Uri.file(exe));
        return true;
    }

    /**
     * Starts the program in a new "QB64" terminal, in the source folder, with `qb64rust.runArguments`. The previous
     * one is closed first: a program still running in it (or waiting for a key) would receive the command line as
     * input. Only terminals this extension created are closed.
     */
    private startInTerminal(exe: string, resource: vscode.Uri): void {
        this.terminal?.dispose();
        const env = config.noPrompt(resource) ? { QB64PE_NOPROMPT: "y" } : undefined;
        const terminal = vscode.window.createTerminal({ name: TERMINAL_NAME, cwd: path.dirname(exe), env });
        this.terminal = terminal;
        terminal.show(true);
        terminal.sendText(shellCommand(vscode.env.shell, path.dirname(exe), exe, config.runArguments(resource)));
    }

    /**
     * The document to act on, saved. Refuses in untrusted workspaces and asks to save untitled documents
     * (the action does not start until the document has a file path).
     */
    private async prepare(uri: vscode.Uri | undefined, verb: string, save = true): Promise<vscode.TextDocument | undefined> {
        if (!vscode.workspace.isTrusted) {
            void vscode.window.showWarningMessage(`QB64: ${verb.toLowerCase()} requires a trusted workspace. Nothing was executed.`);
            return undefined;
        }
        let doc: vscode.TextDocument | undefined;
        if (uri && uri.scheme === "file" && !uri.fsPath.toLowerCase().endsWith(".exe")) {
            doc = await vscode.workspace.openTextDocument(uri);
        } else if (uri && uri.fsPath.toLowerCase().endsWith(".exe")) {
            return undefined;
        } else {
            doc = vscode.window.activeTextEditor?.document;
        }
        if (!doc || doc.languageId !== "qb64rust") {
            void vscode.window.showErrorMessage("QB64: open a QB64 program first.");
            return undefined;
        }
        if (doc.isUntitled || doc.uri.scheme !== "file") {
            const saveAs = "Save As…";
            const choice = await vscode.window.showWarningMessage("QB64: save the program to a file first.", saveAs);
            if (choice === saveAs) {
                await vscode.commands.executeCommand("workbench.action.files.saveAs");
            }
            return undefined;
        }
        if (save && doc.isDirty && !(await doc.save())) {
            return undefined;
        }
        return doc;
    }

    dispose(): void {
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}

/** A command line that changes to `dir` and starts `exe` with `args` in the given shell. `args` is passed as typed. */
export function shellCommand(shell: string, dir: string, exe: string, args: string): string {
    const name = path.basename(shell).toLowerCase().replace(/\.exe$/, "");
    const tail = args ? ` ${args}` : "";
    if (name === "pwsh" || name === "powershell") {
        const q = (s: string) => `'${s.replace(/'/g, "''")}'`;
        return `Set-Location -LiteralPath ${q(dir)}; & ${q(exe)}${tail}`;
    }
    if (name === "cmd") {
        return `cd /d "${dir}" && "${exe}"${tail}`;
    }
    const q = (s: string) => `'${s.replace(/'/g, "'\\''")}'`;
    return `cd ${q(dir)} && ${q(exe)}${tail}`;
}
