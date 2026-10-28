// Holds the discovery result and shows it as a language status item for qb64rust files (study 18, proposal C): which
// compiler builds, and whether its needs are met: for qb64rust the C++ toolchain, for QB64pe QB64pe (spec
// editor/build-run, "Compiler discovery").
import * as vscode from "vscode";
import * as config from "./config";
import { Discovery, findCompiler } from "./compiler/discovery";
import { INSTALL_COMMAND } from "./compiler/toolchain";
import type { ToolchainSupport } from "./toolchainSupport";

export class CompilerStatus implements vscode.Disposable {
    private readonly item = vscode.languages.createLanguageStatusItem("qb64rust.compiler", "qb64rust");
    private readonly disposables: vscode.Disposable[] = [];
    private discovery: Discovery = { found: false, reason: "not searched yet" };
    private busy: string | undefined;
    /** Which compiler builds now (design D5 of extension-vsix); set once the language server exists. */
    builder: (() => "qb64rust" | "qb64pe") | undefined;
    /** The C++ toolchain of a build with qb64rust (change m6-toolchain); set once the language server exists. */
    toolchain: ToolchainSupport | undefined;

    constructor() {
        this.item.name = "QB64 compiler";
        this.disposables.push(
            this.item,
            vscode.workspace.onDidChangeConfiguration((e) => {
                if (config.affectsCompilerPath(e)) {
                    this.refresh();
                } else if (config.affectsBuildWith(e)) {
                    this.render();
                }
            }),
        );
        this.refresh();
    }

    /** The status shown again after the `qb64rust` binary was looked for, and the toolchain looked for again. */
    update(): void {
        this.render();
        void this.toolchain?.refresh();
    }

    /** Runs discovery again (on activation and whenever `qb64rust.compilerPath` changes). */
    refresh(): void {
        const folder = vscode.workspace.workspaceFolders?.[0]?.uri;
        this.discovery = findCompiler({
            setting: config.compilerPath(),
            baseDir: folder?.scheme === "file" ? folder.fsPath : undefined,
            envPath: process.env.PATH,
            platform: process.platform,
        });
        this.render();
    }

    get current(): Discovery {
        return this.discovery;
    }

    /** The compiler path, or undefined after showing an error with a button that opens the setting. */
    async require(): Promise<string | undefined> {
        if (this.discovery.found) {
            return this.discovery.path;
        }
        const open = "Open Setting";
        const choice = await vscode.window.showErrorMessage(`QB64: ${this.discovery.reason}`, open);
        if (choice === open) {
            await config.openCompilerPathSetting();
        }
        return undefined;
    }

    /** Shows "checking…", "building…" etc. while a compiler run is active; undefined clears it. */
    setBusy(activity: string | undefined): void {
        this.busy = activity;
        this.render();
    }

    /** The text shown; exposed for tests. */
    get text(): string {
        return this.item.text;
    }

    /** The detail shown; exposed for tests. */
    get detail(): string | undefined {
        return this.item.detail;
    }

    /** The severity shown; exposed for tests. */
    get severity(): vscode.LanguageStatusSeverity {
        return this.item.severity;
    }

    /** Shows the state; also after the toolchain was looked for. */
    render(): void {
        const d = this.discovery;
        const builder = this.builder?.() ?? "qb64pe";
        this.item.busy = this.busy !== undefined;
        this.item.command = undefined;
        if (builder === "qb64rust") {
            const t = this.toolchain?.current;
            const qb64pe = d.found ? ` · QB64pe ${d.path}` : "";
            if (!t || t.found || t.reason === "not searched yet") {
                this.item.text = this.busy ? `qb64rust: ${this.busy}` : "qb64rust";
                const tools = t?.found ? `toolchain ${t.folder} (${t.source})` : "looking for the C++ toolchain";
                this.item.detail = `${tools}${qb64pe} · builds with qb64rust`;
                this.item.severity = vscode.LanguageStatusSeverity.Information;
            } else {
                // The install only where it would give a toolchain; else the setting to change, if there is one.
                this.item.text = t.installable ? "C++ toolchain not installed" : "C++ toolchain not found";
                this.item.detail = `${t.reason} · builds with qb64rust`;
                this.item.severity = vscode.LanguageStatusSeverity.Warning;
                this.item.busy = false;
                this.item.command = t.installable
                    ? { title: "Install the C++ Toolchain", command: INSTALL_COMMAND }
                    : t.setting
                      ? { title: "Open Setting", command: "workbench.action.openSettings", arguments: [t.setting] }
                      : undefined;
            }
        } else if (d.found) {
            this.item.text = this.busy ? `qb64pe: ${this.busy}` : "qb64pe";
            this.item.detail = `${d.path} (${d.source}) · builds with qb64pe`;
            this.item.severity = vscode.LanguageStatusSeverity.Information;
        } else {
            this.item.text = "compiler not found";
            this.item.detail = d.reason;
            this.item.severity = vscode.LanguageStatusSeverity.Error;
            this.item.busy = false;
            this.item.command = { title: "Open Setting", command: "workbench.action.openSettings", arguments: ["qb64rust.compilerPath"] };
        }
    }

    dispose(): void {
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}
