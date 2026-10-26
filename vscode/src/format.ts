// Format Document and Format Selection while the language server is off (change m5b-formatter, design D7; the
// server's own formatting needs none of this): `qb64rust -y`, or `qb64pe -y` when the build is QB64pe's or the binary
// is not found, with the layout settings as `-f:` settings. The editor text is written to a sibling temp file (so
// relative includes resolve), formatted into the session temp folder, and read back (design D5 of M1).
import * as crypto from "crypto";
import * as fs from "fs";
import * as path from "path";
import * as iconv from "iconv-lite";
import * as vscode from "vscode";
import * as config from "./config";
import { CompilerMessage } from "./compiler/parseQb64peOutput";
import * as qb64pe from "./compiler/qb64pe";
import * as qb64rust from "./compiler/qb64rust";
import { RunQueue } from "./compiler/runQueue";
import { Diagnostics } from "./diagnostics";
import { chooseFormatter, selectionEdits, splitLines } from "./formatPlan";
import { LanguageServer } from "./languageServer";
import { includeRootFor } from "./server/protocol";
import { CompilerStatus } from "./statusBar";

export const TEMP_PREFIX = ".qb64rust-fmt-";

/** What one run of a compiler's `-y` came to. */
interface Run {
    cancelled: boolean;
    exitCode: number;
    messages: CompilerMessage[];
}

export class Formatter implements vscode.DocumentFormattingEditProvider, vscode.DocumentRangeFormattingEditProvider {
    constructor(
        private readonly queue: RunQueue,
        private readonly status: CompilerStatus,
        private readonly diagnostics: Diagnostics,
        private readonly server: LanguageServer,
    ) {}

    async provideDocumentFormattingEdits(document: vscode.TextDocument, _options: vscode.FormattingOptions, token: vscode.CancellationToken): Promise<vscode.TextEdit[]> {
        const text = await this.formatted(document, token);
        if (text === undefined || text === document.getText()) {
            return [];
        }
        const all = new vscode.Range(document.positionAt(0), document.positionAt(document.getText().length));
        return [vscode.TextEdit.replace(all, text)];
    }

    /** The whole text formatted, and only the lines the selection touches kept (spec "Format selection"). */
    async provideDocumentRangeFormattingEdits(
        document: vscode.TextDocument,
        range: vscode.Range,
        _options: vscode.FormattingOptions,
        token: vscode.CancellationToken,
    ): Promise<vscode.TextEdit[]> {
        const text = await this.formatted(document, token);
        if (text === undefined) {
            return [];
        }
        const edits = selectionEdits(splitLines(document.getText()), splitLines(text), range.start.line, range.end.line, range.end.character);
        return edits.map((e) => vscode.TextEdit.replace(document.lineAt(e.line).range, e.text));
    }

    /** The document's text as the chosen compiler's `-y` lays it out, in the document's line ends; undefined when
     * there is none (cancelled, refused, or a failure, which has been shown). */
    private async formatted(document: vscode.TextDocument, token: vscode.CancellationToken): Promise<string | undefined> {
        if (!vscode.workspace.isTrusted) {
            void vscode.window.showWarningMessage("QB64: formatting runs the compiler and requires a trusted workspace.");
            return undefined;
        }
        const uri = document.uri;
        const binary = this.server.binary;
        // The server is off here: its client formats while it runs (extension.ts registers this provider only then).
        const choice = chooseFormatter(false, binary.found, config.buildWith(uri));
        let compiler: string | undefined;
        if (choice === "qb64rust" && binary.found) {
            compiler = binary.path;
        } else {
            compiler = await this.status.require();
            if (!compiler) {
                return undefined;
            }
        }
        const encoding = config.documentEncoding(document);
        if (!iconv.encodingExists(encoding)) {
            void vscode.window.showErrorMessage(`QB64: cannot format a document in encoding "${encoding}".`);
            return undefined;
        }

        const id = crypto.randomBytes(6).toString("hex");
        const saved = uri.scheme === "file" && !document.isUntitled;
        const dir = saved ? path.dirname(uri.fsPath) : qb64pe.sessionTempDir();
        const source = path.join(dir, `${TEMP_PREFIX}${id}.bas`);
        const output = path.join(qb64pe.sessionTempDir(), `formatted-${id}.bas`);
        const settings = config.formatSettings(uri);
        try {
            fs.writeFileSync(source, iconv.encode(document.getText(), encoding));
            let run: Run;
            if (choice === "qb64rust") {
                const folder = vscode.workspace.workspaceFolders?.[0]?.uri;
                const baseDir = folder?.scheme === "file" ? folder.fsPath : undefined;
                const qb64peFolder = this.status.current.found ? path.dirname(this.status.current.path) : undefined;
                const root = includeRootFor(config.includeRoot(), baseDir, qb64peFolder, process.platform);
                run = await qb64rust.format(this.queue, compiler, source, output, settings, root, config.checkTimeoutMs());
            } else {
                run = await qb64pe.format(this.queue, compiler, source, output, settings, config.checkTimeoutMs());
            }
            if (token.isCancellationRequested || run.cancelled) {
                return undefined;
            }
            const errors = run.messages.filter((m) => m.severity === "error");
            if (run.exitCode !== 0 || errors.length > 0 || !fs.existsSync(output)) {
                // Locations in the temp file belong to the document (the parser reports them as the main file).
                // Not retried with the other compiler (spec "Error is not retried").
                const messages = run.messages.map((m) => (m.file && samePath(m.file, source) ? { ...m, file: undefined } : m));
                if (saved) {
                    this.diagnostics.publish(uri, messages, choice === "qb64rust" ? "qb64rust" : "qb64pe");
                }
                const first = messages.find((m) => m.severity === "error");
                const where = first ? ` (${first.file ? `${path.basename(first.file)} ` : ""}line ${first.line})` : "";
                void vscode.window.showErrorMessage(`QB64: cannot format: ${first?.message ?? "the compiler failed"}${where}`);
                return undefined;
            }
            const eol = document.eol === vscode.EndOfLine.CRLF ? "\r\n" : "\n";
            return iconv.decode(fs.readFileSync(output), encoding).replace(/\r?\n/g, eol);
        } finally {
            fs.rmSync(source, { force: true });
            fs.rmSync(output, { force: true });
        }
    }
}

/**
 * Registers `formatter` for `qb64rust` documents while the language server does not run; while it runs, its client
 * provides formatting (spec editor/formatting, "Which formatter"), and two providers would make VS Code ask which.
 */
export function registerWhileServerOff(server: LanguageServer, formatter: Formatter): vscode.Disposable {
    let registered: vscode.Disposable | undefined;
    const update = (running: boolean): void => {
        if (running && registered) {
            registered.dispose();
            registered = undefined;
        } else if (!running && !registered) {
            registered = vscode.Disposable.from(
                vscode.languages.registerDocumentFormattingEditProvider("qb64rust", formatter),
                vscode.languages.registerDocumentRangeFormattingEditProvider("qb64rust", formatter),
            );
        }
    };
    update(server.running !== undefined);
    const listener = server.onDidChangeRunning(update);
    return { dispose: () => (listener.dispose(), registered?.dispose()) };
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
