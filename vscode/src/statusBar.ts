// Holds the discovery result and shows it as a language status item for qb64rust files (study 18, proposal C).
import * as vscode from "vscode";
import * as config from "./config";
import { Discovery, findCompiler } from "./compiler/discovery";

export class CompilerStatus implements vscode.Disposable {
    private readonly item = vscode.languages.createLanguageStatusItem("qb64rust.compiler", "qb64rust");
    private readonly disposables: vscode.Disposable[] = [];
    private discovery: Discovery = { found: false, reason: "not searched yet" };
    private busy: string | undefined;

    constructor() {
        this.item.name = "QB64 compiler";
        this.disposables.push(
            this.item,
            vscode.workspace.onDidChangeConfiguration((e) => {
                if (config.affectsCompilerPath(e)) {
                    this.refresh();
                }
            }),
        );
        this.refresh();
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

    /** The severity shown; exposed for tests. */
    get severity(): vscode.LanguageStatusSeverity {
        return this.item.severity;
    }

    private render(): void {
        const d = this.discovery;
        if (d.found) {
            this.item.text = this.busy ? `qb64pe: ${this.busy}` : "qb64pe";
            this.item.detail = `${d.path} (${d.source})`;
            this.item.severity = vscode.LanguageStatusSeverity.Information;
            this.item.busy = this.busy !== undefined;
            this.item.command = undefined;
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
