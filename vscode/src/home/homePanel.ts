// The page "QB64 Home" (OpenSpec change editor-side-panel, design D11; spec editor/home): a tab in the editor area
// with New Program, Open a Program, Examples and Help, the recent programs, the setup's state, and a checkbox that
// stops it opening by itself. It opens when a QB64 window starts with no file open, and by "QB64: Home". The rules
// are in homeModel.ts and the page in homeHtml.ts; this file gathers the state from VS Code and draws it.
import * as crypto from "crypto";
import * as fs from "fs";
import * as vscode from "vscode";
import * as config from "../config";
import { homeHtml } from "./homeHtml";
import { HomeCommand, HomeState, addRecent, isInside, recentRows, setupRows, shouldOpenHome } from "./homeModel";

export const HOME_PANEL = "qb64rust.home";
/** The recent programs, newest first, in the extension's `globalState`. */
const RECENT = "qb64rust.recentPrograms";
/** The files a QB64 folder holds, in either case (a glob is matched by case on Linux). */
const QB64_FILES = "**/*.{bas,BAS,bi,BI,bm,BM}";
/** Stands for the nonce while two drawings are compared. */
const NONCE = "<nonce>";

/** What the page reads of the rest of the extension. */
export interface HomeSources {
    /** Whether the `qb64rust` binary was found. */
    compiler: () => boolean;
    /** The C++ toolchain, looked for again. */
    toolchain: () => Promise<{ found: boolean; installable?: boolean }>;
    /** The Emscripten SDK. */
    emsdk: () => Promise<{ found: boolean; installable?: boolean }>;
    /** The extension's storage: the copies of the examples there are not recent programs. */
    storage: string;
}

export class HomePanel implements vscode.Disposable {
    private panel: vscode.WebviewPanel | undefined;
    private readonly disposables: vscode.Disposable[] = [];
    private shown = "";
    private generation = 0;
    /** The last state drawn; for tests. */
    state: HomeState | undefined;

    constructor(
        private readonly memento: vscode.Memento,
        private readonly sources: HomeSources,
    ) {
        this.disposables.push(
            vscode.commands.registerCommand("qb64rust.home", () => this.show()),
            vscode.workspace.onDidChangeConfiguration((e) => {
                if (config.affectsShowHomeOnStartup(e)) {
                    void this.draw();
                }
            }),
        );
    }

    /** Whether the page's tab is there; for tests. */
    get visible(): boolean {
        return this.panel !== undefined;
    }

    /** The recent programs as kept; for tests. */
    get recent(): string[] {
        return this.memento.get<string[]>(RECENT, []);
    }

    /** Notes a program as the newest of the recent ones: a saved file that is not an example's copy. */
    async noteProgram(uri: vscode.Uri): Promise<void> {
        if (uri.scheme !== "file" || isInside(uri.fsPath, this.sources.storage)) {
            return;
        }
        const before = this.recent;
        const after = addRecent(before, uri.fsPath);
        if (after.join("\n") !== before.join("\n")) {
            await this.memento.update(RECENT, after);
            void this.draw();
        }
    }

    /** Opens the page when this window is one it belongs in (`shouldOpenHome`); called once, as the window starts. */
    async openAtStartup(): Promise<boolean> {
        if (!config.showHomeOnStartup()) {
            return false;
        }
        const folders = vscode.workspace.workspaceFolders?.length ?? 0;
        const found = folders ? await vscode.workspace.findFiles(QB64_FILES, "**/node_modules/**", 1) : [];
        const files = vscode.window.tabGroups.all
            .flatMap((g) => g.tabs)
            .filter(
                (t) =>
                    t.input instanceof vscode.TabInputText ||
                    t.input instanceof vscode.TabInputTextDiff ||
                    t.input instanceof vscode.TabInputCustom ||
                    t.input instanceof vscode.TabInputNotebook ||
                    t.input instanceof vscode.TabInputNotebookDiff,
            ).length;
        const open = shouldOpenHome({ enabled: true, folders, hasQb64Files: found.length > 0, openFiles: files });
        if (open) {
            await this.show();
        }
        return open;
    }

    /** Shows the page in the editor area, in its one tab. */
    async show(): Promise<void> {
        if (this.panel) {
            this.panel.reveal();
        } else {
            const panel = vscode.window.createWebviewPanel(HOME_PANEL, "QB64 Home", vscode.ViewColumn.Active, { enableScripts: true, localResourceRoots: [] });
            panel.webview.onDidReceiveMessage((m: { command?: HomeCommand; arg?: string }) => {
                if (m?.command) {
                    void this.handle(m.command, m.arg);
                }
            });
            panel.onDidChangeViewState(() => {
                if (panel.visible) {
                    void this.draw();
                }
            });
            panel.onDidDispose(() => {
                if (this.panel === panel) {
                    this.panel = undefined;
                    this.shown = "";
                }
            });
            this.panel = panel;
        }
        await this.draw();
    }

    /** Gathers the state and draws it where it changed. */
    private async draw(): Promise<void> {
        if (!this.panel) {
            return;
        }
        const generation = ++this.generation;
        const [toolchain, emsdk] = await Promise.all([this.sources.toolchain(), this.sources.emsdk()]);
        const panel = this.panel;
        if (!panel || generation !== this.generation) {
            return;
        }
        const state: HomeState = {
            recent: recentRows(this.recent, (f) => fs.existsSync(f)),
            setup: setupRows({ compiler: this.sources.compiler(), toolchain, emsdk, platform: process.platform }),
            showOnStartup: config.showHomeOnStartup(),
        };
        this.state = state;
        const html = homeHtml(state, { cspSource: panel.webview.cspSource, nonce: NONCE });
        if (html !== this.shown) {
            this.shown = html;
            panel.webview.html = html.split(NONCE).join(crypto.randomBytes(16).toString("base64"));
        }
    }

    /** What a click on the page does; tests send the command here. */
    async handle(command: HomeCommand, arg?: string): Promise<unknown> {
        switch (command) {
            case "newProgram":
                return vscode.commands.executeCommand("qb64rust.newProgram");
            case "openProgram":
                return vscode.commands.executeCommand("qb64rust.openProgram");
            case "examples":
                return vscode.commands.executeCommand("qb64rust.examples.focus");
            case "searchHelp":
                return vscode.commands.executeCommand("qb64rust.searchHelp");
            case "getStarted":
                return vscode.commands.executeCommand("qb64rust.getStarted");
            case "page":
                return arg ? vscode.commands.executeCommand("qb64rust.openHelpPage", arg) : undefined;
            case "recent": {
                const row = this.state?.recent[Number(arg)];
                return row ? vscode.window.showTextDocument(vscode.Uri.file(row.path)) : undefined;
            }
            case "installToolchain":
                await vscode.commands.executeCommand("qb64rust.installToolchain");
                return this.draw();
            case "installEmsdk":
                await vscode.commands.executeCommand("qb64rust.installEmsdk");
                return this.draw();
            case "toggle":
                return config.setShowHomeOnStartup(arg === "on");
        }
    }

    dispose(): void {
        this.panel?.dispose();
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}
