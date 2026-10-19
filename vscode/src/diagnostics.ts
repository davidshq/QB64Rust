// The check on save and on command (design D5 of extension-diagnostics): the language server's `qb64rust/check`
// while it runs, whose diagnostics (source `qb64rust`) go to the client's own collection; otherwise the old
// compiler's `-z`, whose messages go to Problems here (source `qb64pe`), cleared on edit and close, as a build's
// are. Neither collection clears the other. A build with qb64rust publishes into a third, `qb64rust-build`
// (source `qb64rust`, design D4 of extension-vsix), cleared the same way.
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import * as config from "./config";
import * as qb64pe from "./compiler/qb64pe";
import { CompilerMessage } from "./compiler/parseQb64peOutput";
import { RunQueue } from "./compiler/runQueue";
import { LanguageServer } from "./languageServer";
import { CompilerStatus } from "./statusBar";

/** Who produced a message: the old compiler (check or build), or a build with the new one. */
export type Source = "qb64pe" | "qb64rust";

export class Diagnostics implements vscode.Disposable {
    readonly collection = vscode.languages.createDiagnosticCollection("qb64pe");
    /** A build with qb64rust's errors (design D4 of extension-vsix); the language server's collection is its own. */
    readonly buildCollection = vscode.languages.createDiagnosticCollection("qb64rust-build");
    /** For each source and checked program, the files it last put diagnostics on (itself and include files). */
    private readonly owned = new Map<string, vscode.Uri[]>();
    private readonly disposables: vscode.Disposable[] = [];
    /** Resolves when the check started by the last save of a file, or by the command, has finished; for tests. */
    readonly lastCheck = new Map<string, Promise<void>>();

    constructor(
        private readonly queue: RunQueue,
        private readonly status: CompilerStatus,
        private readonly server: LanguageServer,
    ) {
        this.disposables.push(
            this.collection,
            this.buildCollection,
            vscode.workspace.onDidSaveTextDocument((doc) => {
                const canCheck = this.server.running !== undefined || this.status.current.found;
                if (doc.languageId === "qb64rust" && config.checkOnSave(doc.uri) && vscode.workspace.isTrusted && canCheck) {
                    this.lastCheck.set(doc.uri.toString(), this.check(doc));
                }
            }),
            vscode.workspace.onDidChangeTextDocument((e) => {
                if (e.contentChanges.length > 0) {
                    this.clear(e.document.uri);
                }
            }),
            vscode.workspace.onDidCloseTextDocument((doc) => this.clear(doc.uri)),
            // A document is disposed long after its editor closes, so also clear when its last tab closes.
            vscode.window.tabGroups.onDidChangeTabs((e) => {
                for (const tab of e.closed) {
                    if (tab.input instanceof vscode.TabInputText && !isShownInTab(tab.input.uri)) {
                        this.clear(tab.input.uri);
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
        if (this.server.running) {
            // The server reads the editor's text, so nothing needs saving, and an untitled document can be checked.
            const run = this.check(doc, true);
            this.lastCheck.set(doc.uri.toString(), run);
            await run;
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
        const run = this.check(doc, true);
        this.lastCheck.set(doc.uri.toString(), run);
        await run;
    }

    /**
     * Checks a document and replaces its program's diagnostics: by the language server while it runs, else a
     * saved document with the old compiler's `-z`. Cancelled checks change nothing.
     */
    async check(doc: vscode.TextDocument, interactive = false): Promise<void> {
        if (this.server.running) {
            try {
                await this.server.check(doc.uri, config.checkTimeoutMs());
            } catch (e) {
                // Shown for the command only: a message on every save after an internal error would be worse.
                if (interactive) {
                    void vscode.window.showErrorMessage(`QB64: the check failed: ${e instanceof Error ? e.message : String(e)}`);
                }
            }
            return;
        }
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

    /** Removes what this class shows for a file: both sources. */
    private clear(uri: vscode.Uri): void {
        this.collection.delete(uri);
        this.buildCollection.delete(uri);
    }

    /** Replaces the diagnostics a program produced from one source (its own file and any include files). */
    publish(main: vscode.Uri, messages: CompilerMessage[], source: Source = "qb64pe"): void {
        const collection = source === "qb64pe" ? this.collection : this.buildCollection;
        const key = main.toString();
        const ownedKey = `${source} ${key}`;
        for (const uri of this.owned.get(ownedKey) ?? []) {
            collection.delete(uri);
        }
        const byFile = new Map<string, { uri: vscode.Uri; items: vscode.Diagnostic[] }>();
        byFile.set(key, { uri: main, items: [] });
        for (const m of messages) {
            const uri = m.file ? vscode.Uri.file(m.file) : main;
            const entry = byFile.get(uri.toString()) ?? { uri, items: [] };
            byFile.set(uri.toString(), entry);
            const diagnostic = new vscode.Diagnostic(
                lineRange(uri, m.line, m.column),
                m.message,
                m.severity === "error" ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning,
            );
            diagnostic.source = source;
            entry.items.push(diagnostic);
        }
        this.owned.set(ownedKey, [...byFile.values()].map((e) => e.uri));
        for (const { uri, items } of byFile.values()) {
            collection.set(uri, items);
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

/**
 * The reported line from its first to its last non-blank character; with a column (the new compiler's, a byte
 * column, which is the character column in CP437 and other single-byte encodings), from that column instead.
 */
function lineRange(uri: vscode.Uri, line: number, column?: number): vscode.Range {
    const index = Math.max(0, line - 1);
    const text = lineText(uri, index);
    if (text === undefined) {
        const at = column ? column - 1 : 0;
        return new vscode.Range(index, at, index, at);
    }
    const start = column ? Math.min(column - 1, text.length) : text.length - text.trimStart().length;
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
