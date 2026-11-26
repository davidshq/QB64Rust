// The SUBs and FUNCTIONs view of the QB64 side panel (OpenSpec change editor-side-panel, design D3): every procedure of
// the Program view's program, from `qb64rust/procedures`, asked again 300 ms after the last change of one of the
// program's documents or a program notification. The view is shown only while the program has an include file
// (`qb64rust.procedures.multiFile`): for a program of one file, VS Code's Outline lists the same.
import * as vscode from "vscode";
import { LanguageServer } from "../languageServer";
import { ProcedureRow, hasIncludeFiles, procedureRows } from "./proceduresModel";

export const PROCEDURES_VIEW = "qb64rust.procedures";
const MULTI_FILE_KEY = "qb64rust.procedures.multiFile";
const BY_NAME_KEY = "qb64rust.procedures.byName";
const REVEAL = "qb64rust.procedures.reveal";

export class ProceduresView implements vscode.TreeDataProvider<ProcedureRow>, vscode.Disposable {
    private readonly changed = new vscode.EventEmitter<void>();
    readonly onDidChangeTreeData = this.changed.event;
    private readonly disposables: vscode.Disposable[] = [];
    /** The QB64 document the list is asked for: the active one, else the last one. */
    private document: vscode.Uri | undefined;
    private byName = false;
    private timer: NodeJS.Timeout | undefined;
    private generation = 0;
    /** The rows shown; for tests. */
    rows: ProcedureRow[] = [];
    /** Whether the view is shown: the program has an include file; for tests. */
    shown = false;
    private readonly filesChange = new vscode.EventEmitter<{ main: string; files: string[] } | undefined>();
    /** Fires with the main file and every file of the program after each answer, undefined without one: the
     * Program view counts the errors of each file, the closed ones too. */
    readonly onDidChangeFiles = this.filesChange.event;
    /** Resolves when the last refresh has its answer; for tests. */
    ready: Promise<void> = Promise.resolve();

    constructor(private readonly server: LanguageServer) {
        this.disposables.push(
            this.changed,
            this.filesChange,
            vscode.window.registerTreeDataProvider(PROCEDURES_VIEW, this),
            vscode.commands.registerCommand("qb64rust.procedures.sortByName", () => this.sort(true)),
            vscode.commands.registerCommand("qb64rust.procedures.sortByPosition", () => this.sort(false)),
            vscode.commands.registerCommand(REVEAL, (row: ProcedureRow) => reveal(row)),
            vscode.window.onDidChangeActiveTextEditor(() => this.follow()),
            vscode.workspace.onDidChangeTextDocument((e) => {
                if (e.document.languageId === "qb64rust" && e.contentChanges.length) {
                    this.soon();
                }
            }),
            vscode.workspace.onDidCloseTextDocument((doc) => {
                if (doc.uri.toString() === this.document?.toString()) {
                    this.document = undefined;
                    this.soon();
                }
            }),
            server.onDidChangeProgram(() => this.soon()),
            server.onDidChangeRunning(() => this.soon()),
            { dispose: () => clearTimeout(this.timer) },
        );
        void vscode.commands.executeCommand("setContext", BY_NAME_KEY, false);
        this.follow();
    }

    private follow(): void {
        const doc = vscode.window.activeTextEditor?.document;
        if (doc?.languageId === "qb64rust" && doc.uri.toString() !== this.document?.toString()) {
            this.document = doc.uri;
            this.soon(0);
        } else if (!this.document) {
            this.soon(0);
        }
    }

    private sort(byName: boolean): void {
        this.byName = byName;
        void vscode.commands.executeCommand("setContext", BY_NAME_KEY, byName);
        this.soon(0);
    }

    /** Asks again after `ms` (300 after an edit, design D3), unless something newer comes first. */
    private soon(ms = 300): void {
        clearTimeout(this.timer);
        let done: () => void = () => undefined;
        this.ready = new Promise((r) => (done = r));
        this.timer = setTimeout(() => void this.refresh().then(done), ms);
    }

    async refresh(): Promise<void> {
        const generation = ++this.generation;
        const running = this.server.running !== undefined;
        const answer = running && this.document ? await this.server.procedures(this.document) : undefined;
        if (generation !== this.generation) {
            return;
        }
        this.rows = procedureRows(answer, this.byName);
        this.filesChange.fire(answer?.main ? { main: answer.main, files: answer.files ?? [] } : undefined);
        this.shown = hasIncludeFiles(answer);
        void vscode.commands.executeCommand("setContext", MULTI_FILE_KEY, this.shown);
        this.changed.fire();
    }

    getTreeItem(row: ProcedureRow): vscode.TreeItem {
        const item = new vscode.TreeItem(row.label, vscode.TreeItemCollapsibleState.None);
        item.description = row.file;
        item.tooltip = row.detail;
        item.iconPath = new vscode.ThemeIcon(row.kind === "main" ? "file-code" : row.kind === "FUNCTION" ? "symbol-function" : "symbol-method");
        item.command = { title: "Go to", command: REVEAL, arguments: [row] };
        item.contextValue = row.kind === "main" ? "procedure.main" : "procedure";
        return item;
    }

    getChildren(row?: ProcedureRow): ProcedureRow[] {
        return row ? [] : this.rows;
    }

    dispose(): void {
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}

/** Opens the row's file with the cursor at its line. */
async function reveal(row: ProcedureRow): Promise<void> {
    const at = new vscode.Position(row.line, 0);
    await vscode.window.showTextDocument(vscode.Uri.parse(row.uri), { selection: new vscode.Range(at, at), preserveFocus: false });
}
