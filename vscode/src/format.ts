// Format Document through the compiler's `-y` mode (design D5): the editor text is written to a sibling temp
// file (so relative includes resolve), formatted into the session temp folder, and read back.
import * as crypto from "crypto";
import * as fs from "fs";
import * as path from "path";
import * as iconv from "iconv-lite";
import * as vscode from "vscode";
import * as config from "./config";
import * as qb64pe from "./compiler/qb64pe";
import { RunQueue } from "./compiler/runQueue";
import { Diagnostics } from "./diagnostics";
import { CompilerStatus } from "./statusBar";

export const TEMP_PREFIX = ".qb64rust-fmt-";

export class Formatter implements vscode.DocumentFormattingEditProvider {
    constructor(
        private readonly queue: RunQueue,
        private readonly status: CompilerStatus,
        private readonly diagnostics: Diagnostics,
    ) {}

    async provideDocumentFormattingEdits(document: vscode.TextDocument, _options: vscode.FormattingOptions, token: vscode.CancellationToken): Promise<vscode.TextEdit[]> {
        if (!vscode.workspace.isTrusted) {
            void vscode.window.showWarningMessage("QB64: formatting runs the compiler and requires a trusted workspace.");
            return [];
        }
        const compiler = await this.status.require();
        if (!compiler) {
            return [];
        }
        const encoding = config.documentEncoding(document);
        if (!iconv.encodingExists(encoding)) {
            void vscode.window.showErrorMessage(`QB64: cannot format a document in encoding "${encoding}".`);
            return [];
        }

        const id = crypto.randomBytes(6).toString("hex");
        const dir = document.uri.scheme === "file" && !document.isUntitled ? path.dirname(document.uri.fsPath) : qb64pe.sessionTempDir();
        const source = path.join(dir, `${TEMP_PREFIX}${id}.bas`);
        const output = path.join(qb64pe.sessionTempDir(), `formatted-${id}.bas`);
        try {
            fs.writeFileSync(source, iconv.encode(document.getText(), encoding));
            const run = await qb64pe.format(this.queue, compiler, source, output, config.checkTimeoutMs());
            if (token.isCancellationRequested || run.cancelled) {
                return [];
            }
            const errors = run.messages.filter((m) => m.severity === "error");
            if (run.exitCode !== 0 || errors.length > 0 || !fs.existsSync(output)) {
                // Locations in the temp file belong to the document (the parser reports them as the main file).
                const messages = run.messages.map((m) => (m.file && samePath(m.file, source) ? { ...m, file: undefined } : m));
                if (document.uri.scheme === "file" && !document.isUntitled) {
                    this.diagnostics.publish(document.uri, messages);
                }
                const first = messages.find((m) => m.severity === "error");
                const where = first ? ` (${first.file ? `${path.basename(first.file)} ` : ""}line ${first.line})` : "";
                void vscode.window.showErrorMessage(`QB64: cannot format: ${first?.message ?? "the compiler failed"}${where}`);
                return [];
            }
            const eol = document.eol === vscode.EndOfLine.CRLF ? "\r\n" : "\n";
            const text = iconv.decode(fs.readFileSync(output), encoding).replace(/\r?\n/g, eol);
            if (text === document.getText()) {
                return [];
            }
            const all = new vscode.Range(document.positionAt(0), document.positionAt(document.getText().length));
            return [vscode.TextEdit.replace(all, text)];
        } finally {
            fs.rmSync(source, { force: true });
            fs.rmSync(output, { force: true });
        }
    }
}

function samePath(a: string, b: string): boolean {
    return path.normalize(a).toLowerCase() === path.normalize(b).toLowerCase();
}

/** Removes temp files left behind by an earlier session that ended during a format (design risks). */
export async function removeStaleTempFiles(): Promise<void> {
    if (!vscode.workspace.workspaceFolders) {
        return;
    }
    const stale = await vscode.workspace.findFiles(`**/${TEMP_PREFIX}*.bas`, undefined, 100);
    const cutoff = Date.now() - 60_000; // another window may be formatting right now
    for (const uri of stale) {
        if (uri.scheme !== "file") {
            continue;
        }
        try {
            if (fs.statSync(uri.fsPath).mtimeMs < cutoff) {
                fs.rmSync(uri.fsPath, { force: true });
            }
        } catch {
            // already gone
        }
    }
}
