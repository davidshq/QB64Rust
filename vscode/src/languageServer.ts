// The new compiler's language server (design D6 of m2-language-server): `qb64rust lsp` started over stdio when the
// binary is found, its diagnostics live in their own collection (source `qb64rust`), and the outline, folding
// ranges and go to definition come from it. Without the binary nothing changes from M1.
import * as vscode from "vscode";
import { LanguageClient, LanguageClientOptions, ServerOptions } from "vscode-languageclient/node";
import * as config from "./config";
import { Discovery, QB64RUST, findCompiler } from "./compiler/discovery";
import { DOCUMENT_ENCODING, EncodingTracker, initializationOptions } from "./server/protocol";

type State = { kind: "stopped" } | { kind: "untrusted" } | { kind: "starting" } | { kind: "running" } | { kind: "failed"; reason: string };

export class LanguageServer implements vscode.Disposable {
    private readonly item = vscode.languages.createLanguageStatusItem("qb64rust.server", "qb64rust");
    private readonly disposables: vscode.Disposable[] = [];
    private readonly encodings = new EncodingTracker();
    private client: LanguageClient | undefined;
    private discovery: Discovery = { found: false, reason: "not searched yet" };
    private state: State = { kind: "stopped" };
    /** Counts restarts (and the dispose); a restart that a later one overtook leaves the state alone. */
    private generation = 0;
    /** Resolves when the last start or restart has finished; for tests. */
    ready: Promise<void>;

    constructor() {
        this.item.name = "QB64 language server";
        this.disposables.push(
            this.item,
            vscode.workspace.onDidChangeConfiguration((e) => {
                if (config.affectsLanguageServer(e)) {
                    this.ready = this.restart();
                }
            }),
            vscode.workspace.onDidGrantWorkspaceTrust(() => {
                this.ready = this.restart();
            }),
            vscode.workspace.onDidOpenTextDocument((doc) => this.tellEncoding(doc)),
            // An encoding change (Reopen with Encoding) comes as a change of the document.
            vscode.workspace.onDidChangeTextDocument((e) => this.tellEncoding(e.document)),
            vscode.workspace.onDidCloseTextDocument((doc) => this.encodings.forget(doc.uri.toString())),
        );
        this.ready = this.restart();
    }

    /** The client while the server runs; for tests. */
    get running(): LanguageClient | undefined {
        return this.state.kind === "running" ? this.client : undefined;
    }

    /** The text shown in the language status item; for tests. */
    get text(): string {
        return this.item.text;
    }

    /** Stops the server if it runs, looks for the binary again and starts it. */
    async restart(): Promise<void> {
        const generation = ++this.generation;
        await this.stop();
        if (generation !== this.generation) {
            return;
        }
        const folder = vscode.workspace.workspaceFolders?.[0]?.uri;
        const baseDir = folder?.scheme === "file" ? folder.fsPath : undefined;
        this.discovery = findCompiler({ setting: config.serverPath(), baseDir, envPath: process.env.PATH, platform: process.platform }, QB64RUST);
        if (!this.discovery.found) {
            this.render();
            return;
        }
        // The binary comes from a setting a workspace may set: run it only in a trusted workspace.
        if (!vscode.workspace.isTrusted) {
            this.state = { kind: "untrusted" };
            this.render();
            return;
        }
        const serverOptions: ServerOptions = { command: this.discovery.path, args: ["lsp"] };
        const clientOptions: LanguageClientOptions = {
            documentSelector: [
                { language: "qb64rust", scheme: "file" },
                { language: "qb64rust", scheme: "untitled" },
            ],
            diagnosticCollectionName: "qb64rust",
            initializationOptions: initializationOptions(config.defaultEncoding(), config.includeRoot(), baseDir, process.platform),
        };
        const client = new LanguageClient("qb64rust", "QB64 language server (qb64rust)", serverOptions, clientOptions);
        this.client = client;
        this.state = { kind: "starting" };
        this.render();
        let failure: string | undefined;
        try {
            await client.start();
        } catch (e) {
            failure = e instanceof Error ? e.message : String(e);
        }
        if (generation !== this.generation) {
            // A later restart (or the dispose) has stopped this client already and owns the state.
            return;
        }
        if (failure === undefined) {
            this.state = { kind: "running" };
            this.encodings.clear();
            for (const doc of vscode.workspace.textDocuments) {
                this.tellEncoding(doc);
            }
        } else {
            this.state = { kind: "failed", reason: failure };
        }
        this.render();
    }

    private tellEncoding(doc: vscode.TextDocument): void {
        if (this.state.kind !== "running" || !this.client || doc.languageId !== "qb64rust") {
            return;
        }
        const params = this.encodings.update(doc.uri.toString(), config.documentEncoding(doc));
        if (params) {
            this.client.sendNotification(DOCUMENT_ENCODING, params).catch(() => undefined);
        }
    }

    async stop(): Promise<void> {
        const client = this.client;
        this.client = undefined;
        this.state = { kind: "stopped" };
        if (client) {
            try {
                await client.stop();
            } catch {
                // A server that failed to start cannot be stopped; nothing is left to clean up.
            }
        }
    }

    private render(): void {
        const d = this.discovery;
        const s = this.state;
        this.item.command = undefined;
        this.item.busy = s.kind === "starting";
        if (!d.found) {
            // Optional: without it the extension works as in M1, so this is information, not an error.
            this.item.text = "qb64rust: not found";
            this.item.detail = d.reason;
            this.item.severity = vscode.LanguageStatusSeverity.Information;
            this.item.command = { title: "Open Setting", command: "workbench.action.openSettings", arguments: ["qb64rust.path"] };
            return;
        }
        this.item.detail = d.path;
        switch (s.kind) {
            case "running":
            case "starting":
                this.item.text = "qb64rust";
                this.item.severity = vscode.LanguageStatusSeverity.Information;
                break;
            case "untrusted":
                this.item.text = "qb64rust: not started";
                this.item.detail = "The language server runs in a trusted workspace only.";
                this.item.severity = vscode.LanguageStatusSeverity.Information;
                break;
            case "failed":
                this.item.text = "qb64rust: failed to start";
                this.item.detail = s.reason;
                this.item.severity = vscode.LanguageStatusSeverity.Error;
                break;
            case "stopped":
                this.item.text = "qb64rust: stopped";
                this.item.severity = vscode.LanguageStatusSeverity.Information;
                break;
        }
    }

    dispose(): void {
        // A restart still under way must not start a client after this.
        this.generation++;
        void this.stop();
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}
