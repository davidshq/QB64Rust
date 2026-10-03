// Compiler messages → Problems. Check on save and on command; cleared on edit and close.
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import * as config from "./config";
import * as qb64pe from "./compiler/qb64pe";
import { CompilerMessage } from "./compiler/parseQb64peOutput";
import { RunQueue } from "./compiler/runQueue";
import { CompilerStatus } from "./statusBar";

export class Diagnostics implements vscode.Disposable {
    readonly collection = vscode.languages.createDiagnosticCollection("qb64rust");
    /** For each checked program, the files it last put diagnostics on (itself and include files). */
    private readonly owned = new Map<string, vscode.Uri[]>();
    private readonly disposables: vscode.Disposable[] = [];
    /** Resolves when the check started by the last save of a file has finished; for tests. */
    readonly lastCheck = new Map<string, Promise<void>>();

    constructor(
        private readonly queue: RunQueue,
        private readonly status: CompilerStatus,
    ) {
        this.disposables.push(
            this.collection,
            vscode.workspace.onDidSaveTextDocument((doc) => {
                if (doc.languageId === "qb64rust" && config.checkOnSave(doc.uri) && vscode.workspace.isTrusted && this.status.current.found) {
                    this.lastCheck.set(doc.uri.toString(), this.check(doc));
                }
            }),
            vscode.workspace.onDidChangeTextDocument((e) => {
                if (e.contentChanges.length > 0) {
                    this.collection.delete(e.document.uri);
                }
            }),
            vscode.workspace.onDidCloseTextDocument((doc) => this.collection.delete(doc.uri)),
            // A document is disposed long after its editor closes, so also clear when its last tab closes.
            vscode.window.tabGroups.onDidChangeTabs((e) => {
                for (const tab of e.closed) {
                    if (tab.input instanceof vscode.TabInputText && !isShownInTab(tab.input.uri)) {
                        this.collection.delete(tab.input.uri);
                    }
                }
            }),
            vscode.commands.registerCommand("qb64rust.check", () => this.checkCommand()),
        );
    }

    private async checkCommand(): Promise<void> {
        const doc = vscode.window.activeTextEditor?.document;
        if (!doc || doc.languageId !== "qb64rust") {
            void vscode.window.showErrorMessage("QB64: open a QB64 file to check it.");
            return;
        }
        if (!vscode.workspace.isTrusted) {
            void vscode.window.showWarningMessage("QB64: checking runs the compiler and requires a trusted workspace.");
            return;
        }
        if (doc.isUntitled) {
            void vscode.window.showWarningMessage("QB64: save the program to a file before checking it.");
            return;
        }
        if (doc.isDirty && !(await doc.save())) {
            return;
        }
        // Saving may already have started a check; this one replaces it (same file, design D4).
        await this.check(doc, true);
    }

    /** Checks a saved document with `-z` and replaces its program's diagnostics. Cancelled checks change nothing. */
    async check(doc: vscode.TextDocument, interactive = false): Promise<void> {
        const compiler = interactive ? await this.status.require() : this.status.current.found ? this.status.current.path : undefined;
        if (!compiler || doc.uri.scheme !== "file") {
            return;
        }
        const run = await qb64pe.check(this.queue, compiler, doc.uri.fsPath, config.checkTimeoutMs());
        if (run.cancelled) {
            return;
        }
        this.publish(doc.uri, run.messages);
    }

    /** Replaces the diagnostics a program produced (its own file and any include files). */
    publish(main: vscode.Uri, messages: CompilerMessage[]): void {
        const key = main.toString();
        for (const uri of this.owned.get(key) ?? []) {
            this.collection.delete(uri);
        }
        const byFile = new Map<string, { uri: vscode.Uri; items: vscode.Diagnostic[] }>();
        byFile.set(key, { uri: main, items: [] });
        for (const m of messages) {
            const uri = m.file ? vscode.Uri.file(m.file) : main;
            const entry = byFile.get(uri.toString()) ?? { uri, items: [] };
            byFile.set(uri.toString(), entry);
            const diagnostic = new vscode.Diagnostic(
                lineRange(uri, m.line),
                m.message,
                m.severity === "error" ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning,
            );
            diagnostic.source = "qb64pe";
            entry.items.push(diagnostic);
        }
        this.owned.set(key, [...byFile.values()].map((e) => e.uri));
        for (const { uri, items } of byFile.values()) {
            this.collection.set(uri, items);
        }
    }

    dispose(): void {
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}

function isShownInTab(uri: vscode.Uri): boolean {
    const key = uri.toString();
    return vscode.window.tabGroups.all.some((g) => g.tabs.some((t) => t.input instanceof vscode.TabInputText && t.input.uri.toString() === key));
}

/** The reported line from its first to its last non-blank character; no column is claimed. */
function lineRange(uri: vscode.Uri, line: number): vscode.Range {
    const index = Math.max(0, line - 1);
    const text = lineText(uri, index);
    if (text === undefined) {
        return new vscode.Range(index, 0, index, 0);
    }
    const start = text.length - text.trimStart().length;
    const end = text.trimEnd().length;
    return end > start ? new vscode.Range(index, start, index, end) : new vscode.Range(index, 0, index, text.length);
}

function lineText(uri: vscode.Uri, index: number): string | undefined {
    const open = vscode.workspace.textDocuments.find((d) => d.uri.toString() === uri.toString());
    if (open) {
        return index < open.lineCount ? open.lineAt(index).text : undefined;
    }
    try {
        // Single-byte encodings keep one character per byte, so Latin-1 gives the right positions for CP437 too.
        const lines = fs.readFileSync(path.normalize(uri.fsPath), "latin1").split(/\r?\n/);
        return lines[index];
    } catch {
        return undefined;
    }
}
