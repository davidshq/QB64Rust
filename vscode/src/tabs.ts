// The notice for a QB64 file shown with tabs. The QB64pe IDE turns tabs into spaces when it loads a file; VS Code
// keeps them, and the file is never changed when it opens. The notice says so once per file and session and offers
// to turn them into spaces as the QB64pe IDE does (`tabsPlan.ts`); `qb64rust.tabsNotice` turns it off.
import * as vscode from "vscode";
import * as config from "./config";
import { tabEdits } from "./tabsPlan";

const CONVERT = "Turn Tabs into Spaces";
const NEVER = "Don't Show Again";

export class TabsNotice implements vscode.Disposable {
    private readonly told = new Set<string>();
    private readonly disposables: vscode.Disposable[] = [];
    /** Shows the notice and gives the button chosen; tests replace it. */
    show: (message: string, ...items: string[]) => Thenable<string | undefined> = (message, ...items) => vscode.window.showInformationMessage(message, ...items);
    /** The notices shown, by file; for tests. */
    readonly shown: string[] = [];

    constructor() {
        // Files shown in an editor only: a build opens documents no editor shows, and a diff view shows another
        // scheme's.
        const shown = (editors: readonly vscode.TextEditor[]) => editors.forEach((e) => void this.opened(e.document));
        this.disposables.push(
            vscode.window.onDidChangeVisibleTextEditors(shown),
            vscode.commands.registerCommand("qb64rust.convertTabs", (uri?: vscode.Uri) => this.convertCommand(uri)),
        );
        shown(vscode.window.visibleTextEditors);
    }

    /** Tells of a QB64 file's tabs once per session, unless the setting is off. */
    private async opened(document: vscode.TextDocument): Promise<void> {
        const key = document.uri.toString();
        const scheme = document.uri.scheme;
        if (document.languageId !== "qb64rust" || (scheme !== "file" && scheme !== "untitled") || this.told.has(key) || !config.tabsNotice(document.uri)) {
            return;
        }
        if (tabEdits(lines(document)).length === 0) {
            return;
        }
        this.told.add(key);
        this.shown.push(key);
        const name = vscode.workspace.asRelativePath(document.uri);
        const choice = await this.show(`QB64: ${name} has tab characters. The QB64pe IDE turns them into spaces when it loads a file.`, CONVERT, NEVER);
        if (choice === CONVERT) {
            await convert(document);
        } else if (choice === NEVER) {
            await config.turnOffTabsNotice();
        }
    }

    /** `QB64: Turn Tabs into Spaces`, for the file named or the active editor's. */
    private async convertCommand(uri?: vscode.Uri): Promise<boolean> {
        const document = uri ? await vscode.workspace.openTextDocument(uri) : vscode.window.activeTextEditor?.document;
        if (!document || document.languageId !== "qb64rust") {
            void vscode.window.showErrorMessage("QB64: open a QB64 file first.");
            return false;
        }
        return convert(document);
    }

    dispose(): void {
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}

function lines(document: vscode.TextDocument): string[] {
    const out: string[] = [];
    for (let i = 0; i < document.lineCount; i++) {
        out.push(document.lineAt(i).text);
    }
    return out;
}

/** Turns the document's tabs into spaces as one edit, which Undo takes back; the file is not saved. */
async function convert(document: vscode.TextDocument): Promise<boolean> {
    const edit = new vscode.WorkspaceEdit();
    for (const [i, text] of tabEdits(lines(document))) {
        edit.replace(document.uri, document.lineAt(i).range, text);
    }
    return vscode.workspace.applyEdit(edit);
}
