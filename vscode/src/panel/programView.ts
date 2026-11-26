// The Program view of the QB64 side panel and the status bar items (OpenSpec change editor-side-panel, design D1 and
// D2): which program Run builds, the four buttons, and one status line. The state comes from `programModel.ts`;
// this file gathers its inputs from VS Code and draws it.
import * as crypto from "crypto";
import * as path from "path";
import * as vscode from "vscode";
import { RunKind } from "../compiler/runQueue";
import { LanguageServer } from "../languageServer";
import { ProgramCommand, programHtml } from "./programHtml";
import { ProgramStatusBar } from "../statusBar";
import { Activity, ProgramDiagnostic, ProgramState, mainActivity, programState } from "./programModel";

export const PROGRAM_VIEW = "qb64rust.program";
/** Stands for the nonce while two drawings are compared. */
const NONCE = "<nonce>";

/** The commands each button runs, with the program's uri. */
const BUTTONS: Partial<Record<ProgramCommand, string>> = {
    run: "qb64rust.buildAndRun",
    debug: "qb64rust.debug",
    browser: "qb64rust.runInBrowser",
    build: "qb64rust.build",
};

export class ProgramView implements vscode.WebviewViewProvider, vscode.Disposable {
    private view: vscode.WebviewView | undefined;
    private readonly disposables: vscode.Disposable[] = [];
    private program: vscode.Uri | undefined;
    private queueActivity: Activity | undefined;
    private firstBuild = false;
    private download: string | undefined;
    /** The reference page of the first error, by the error's place (`uri:line`), as the server answered it. */
    private errorPage: { at: string; page?: string } | undefined;
    private shown = "";
    /** Every file of a program, by its main file, as the language server listed them (`qb64rust/procedures`). */
    private files: { main: string; files: Set<string> } | undefined;
    private timer: NodeJS.Timeout | undefined;
    /** The two status bar items, drawn from the same state; for tests. */
    readonly statusBar = new ProgramStatusBar();
    private readonly stateChange = new vscode.EventEmitter<ProgramState>();
    /** Fires with each new state; the SUBs and FUNCTIONs view follows the program by it. */
    readonly onDidChangeState = this.stateChange.event;
    /** The last state drawn; for tests. */
    state: ProgramState = { kind: "none", trusted: vscode.workspace.isTrusted };
    /** The title of a help page by its name (`if` is `IF`), for the link "Help on IF"; set once the help is read. */
    pageTitle: (page: string) => string | undefined = () => undefined;

    constructor(private readonly server: LanguageServer) {
        this.disposables.push(
            this.statusBar,
            this.stateChange,
            vscode.window.registerWebviewViewProvider(PROGRAM_VIEW, this),
            vscode.window.onDidChangeActiveTextEditor(() => this.follow()),
            vscode.languages.onDidChangeDiagnostics(() => this.soon()),
            vscode.workspace.onDidGrantWorkspaceTrust(() => this.update()),
            server.onDidChangeProgram(() => this.follow()),
            { dispose: () => clearTimeout(this.timer) },
        );
        this.follow();
    }

    /** What the run queue runs (its `onBusyChange`). */
    setQueueActivity(kind: RunKind | undefined): void {
        this.queueActivity = kind ? { kind } : undefined;
        this.update();
    }

    setFirstBuild(on: boolean): void {
        this.firstBuild = on;
        this.update();
    }

    setDownload(what: string | undefined): void {
        this.download = what;
        this.update();
    }

    /** Every file of the program `main` (uris as the server gives them), so that an error in a closed include file
     * counts: a program such as Civil War Strategy has 47 of them. */
    setProgramFiles(list: { main: string; files: string[] } | undefined): void {
        const norm = (u: string) => vscode.Uri.parse(u).toString();
        this.files = list && { main: norm(list.main), files: new Set(list.files.map(norm)) };
        this.soon();
    }

    /** The program of the active editor, when it is a QB64 file; else the last one stays (spec editor/side-panel,
     * "The program named in the Program view"). */
    private follow(): void {
        const doc = vscode.window.activeTextEditor?.document;
        if (doc?.languageId === "qb64rust") {
            this.program = this.server.programOf(doc.uri) ?? doc.uri;
        }
        this.update();
    }

    /** The program's uri, for the buttons; undefined before any. */
    get programUri(): vscode.Uri | undefined {
        return this.program;
    }

    /** An update a moment later, so that a burst of diagnostics draws once. */
    private soon(): void {
        clearTimeout(this.timer);
        this.timer = setTimeout(() => this.update(), 100);
    }

    /** The diagnostics of the program's files: the main file, every file the server listed for the program, open or
     * not, and each open file the server names the program for. */
    private diagnostics(main: vscode.Uri): ProgramDiagnostic[] {
        const key = main.toString();
        const listed = this.files?.main === key ? this.files.files : undefined;
        const out: ProgramDiagnostic[] = [];
        for (const [uri, list] of vscode.languages.getDiagnostics()) {
            const u = uri.toString();
            const mine = u === key || listed?.has(u) || this.server.programOf(uri)?.toString() === key;
            if (!mine) {
                continue;
            }
            for (const d of list) {
                if (d.severity !== vscode.DiagnosticSeverity.Error && d.severity !== vscode.DiagnosticSeverity.Warning) {
                    continue;
                }
                out.push({
                    uri: uri.toString(),
                    file: path.basename(uri.path),
                    line: d.range.start.line + 1,
                    message: d.message,
                    severity: d.severity === vscode.DiagnosticSeverity.Error ? "error" : "warning",
                });
            }
        }
        return out;
    }

    /** Works out the state again and draws it where it changed. */
    update(): void {
        const p = this.program;
        const diagnostics = p ? this.diagnostics(p) : [];
        const inputs = {
            program: p ? { uri: p.toString(), name: path.basename(p.path) || p.toString() } : undefined,
            activity: mainActivity(this.queueActivity, this.firstBuild, this.download),
            diagnostics,
            trusted: vscode.workspace.isTrusted,
        };
        let state = programState(inputs);
        if (state.kind === "program" && state.status.kind === "error") {
            const first = state.status.first;
            const at = `${first.uri}:${first.line}`;
            if (this.errorPage?.at !== at) {
                this.errorPage = { at };
                void this.lookUpPage(first.uri, first.line, at);
            }
            const page = this.errorPage.page;
            state = programState({ ...inputs, errorPage: page, errorPageTitle: page ? this.pageTitle(page) : undefined });
        }
        this.state = state;
        this.statusBar.render(state, vscode.window.activeTextEditor?.document.languageId === "qb64rust");
        if (this.view) {
            // Drawn again only when the page changes; each drawing gets a fresh nonce. The placeholder has a `<`,
            // which no escaped name or message holds.
            const html = programHtml(state, { cspSource: this.view.webview.cspSource, nonce: NONCE });
            if (html !== this.shown) {
                this.shown = html;
                this.view.webview.html = html.split(NONCE).join(crypto.randomBytes(16).toString("base64"));
            }
        }
        this.stateChange.fire(state);
    }

    /** Asks the server for the page of the statement on the error's line (its first word), for the "Help" link. */
    private async lookUpPage(uri: string, line: number, at: string): Promise<void> {
        const doc = vscode.workspace.textDocuments.find((d) => d.uri.toString() === uri);
        if (!doc || line - 1 >= doc.lineCount) {
            return;
        }
        const text = doc.lineAt(line - 1);
        const position = new vscode.Position(line - 1, text.firstNonWhitespaceCharacterIndex);
        const topic = await this.server.helpTopic(doc.uri, position);
        if (this.errorPage?.at === at && topic && "page" in topic) {
            this.errorPage.page = topic.page;
            this.update();
        }
    }

    resolveWebviewView(view: vscode.WebviewView): void {
        this.view = view;
        this.shown = "";
        view.webview.options = { enableScripts: true, localResourceRoots: [] };
        view.webview.onDidReceiveMessage((m: { command?: ProgramCommand }) => {
            if (m?.command) {
                void this.handle(m.command);
            }
        });
        view.onDidDispose(() => {
            if (this.view === view) {
                this.view = undefined;
            }
        });
        this.update();
    }

    /** What a click on a button or link of the page does; tests send the message here. */
    async handle(command: ProgramCommand): Promise<unknown> {
        const run = BUTTONS[command];
        if (run) {
            const p = this.program;
            if (p && p.scheme !== "file") {
                // An unsaved program: the command acts on the active editor, which asks to save it first.
                await vscode.window.showTextDocument(p);
                return vscode.commands.executeCommand(run);
            }
            return vscode.commands.executeCommand(run, p);
        }
        switch (command) {
            case "goto":
                return this.goToError();
            case "help": {
                const s = this.state;
                const page = s.kind === "program" && s.status.kind === "error" ? s.status.page : undefined;
                return page ? vscode.commands.executeCommand("qb64rust.openHelpPage", page) : undefined;
            }
            case "newProgram":
                return vscode.commands.executeCommand("qb64rust.newProgram");
            case "openProgram":
                return vscode.commands.executeCommand("qb64rust.openProgram");
            case "getStarted":
                return vscode.commands.executeCommand("qb64rust.getStarted");
            case "trust":
                return vscode.commands.executeCommand("workbench.trust.manage");
            case "searchHelp":
                return vscode.commands.executeCommand("qb64rust.searchHelp");
            default:
                return undefined;
        }
    }

    /** Opens the file of the first error with the cursor at the start of its line. */
    private async goToError(): Promise<void> {
        const s = this.state;
        if (s.kind !== "program" || s.status.kind !== "error") {
            return;
        }
        const first = s.status.first;
        const at = new vscode.Position(first.line - 1, 0);
        await vscode.window.showTextDocument(vscode.Uri.parse(first.uri), { selection: new vscode.Range(at, at) });
    }

    dispose(): void {
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}

/** "Open a Program": a `.bas` file chosen in VS Code's open dialog, opened in the editor. */
export async function openProgram(): Promise<vscode.Uri | undefined> {
    const picked = await vscode.window.showOpenDialog({
        canSelectFiles: true,
        canSelectMany: false,
        filters: { "QB64 programs": ["bas"] },
        openLabel: "Open Program",
        title: "QB64: Open a Program",
    });
    const uri = picked?.[0];
    if (uri) {
        await vscode.window.showTextDocument(uri);
    }
    return uri;
}
