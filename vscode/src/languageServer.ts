// The new compiler's language server (design D6 of m2-language-server): `qb64rust lsp` started over stdio when the
// binary is found, its diagnostics live in their own collection (source `qb64rust`), and the outline, folding
// ranges and go to definition come from it. Without the binary nothing changes from M1. While it runs, a check on
// save or by command is its `qb64rust/check` (design D5 of extension-diagnostics), and formatting is its own, with
// the layout settings sent at the start and on each change (design D7 of m5b-formatter).
import * as path from "path";
import * as vscode from "vscode";
import {
    DidChangeConfigurationNotification,
    LSPErrorCodes,
    LanguageClient,
    LanguageClientOptions,
    ResponseError,
    ServerOptions,
    State as ClientState,
} from "vscode-languageclient/node";
import * as config from "./config";
import { Discovery, QB64RUST, findCompiler } from "./compiler/discovery";
import { CHECK, CheckResult, DOCUMENT_ENCODING, EncodingTracker, checkParams, configurationParams, initializationOptions } from "./server/protocol";
import { CompilerStatus } from "./statusBar";

/** The reason shown when the server stopped by itself (`clientState`). */
const STOPPED = "the server stopped";

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
    /** The check under way for each document, to cancel when a newer one starts. */
    private readonly checks = new Map<string, vscode.CancellationTokenSource>();
    private readonly runningChange = new vscode.EventEmitter<boolean>();
    private wasRunning = false;
    /** Fires with `running` when the server starts running or stops (the extension's formatter is registered while it
     * does not run). */
    readonly onDidChangeRunning = this.runningChange.event;

    /**
     * `bundled` is the binary the extension carries (design D2 of extension-vsix), which may not exist. `status`
     * gives the QB64pe folder, the include root's default (design D6), and is told when the binary was looked for.
     */
    constructor(
        private readonly bundled: string,
        private readonly status: CompilerStatus,
    ) {
        this.item.name = "QB64 language server";
        this.disposables.push(
            this.item,
            this.runningChange,
            vscode.workspace.onDidChangeConfiguration((e) => {
                if (config.affectsLanguageServer(e)) {
                    this.ready = this.restart();
                } else if (config.affectsFormatSettings(e)) {
                    // Without a restart (spec editor/formatting, "Setting changed while the server runs").
                    this.running?.sendNotification(DidChangeConfigurationNotification.type, configurationParams(config.formatSettings())).catch(() => undefined);
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

    /** Where the `qb64rust` binary was looked for and found, as of the last start; a build uses the same one. */
    get binary(): Discovery {
        return this.discovery;
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
        // Looked for before the stop's wait, so that a build started at activation sees the binary (`binary`).
        const folder = vscode.workspace.workspaceFolders?.[0]?.uri;
        const baseDir = folder?.scheme === "file" ? folder.fsPath : undefined;
        this.discovery = findCompiler({ setting: config.serverPath(), baseDir, bundled: this.bundled, envPath: process.env.PATH, platform: process.platform }, QB64RUST);
        this.status.update();
        await this.stop();
        if (generation !== this.generation) {
            return;
        }
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
            initializationOptions: initializationOptions(
                config.defaultEncoding(),
                config.includeRoot(),
                baseDir,
                process.platform,
                this.status.current.found ? path.dirname(this.status.current.path) : undefined,
                config.formatSettings(),
            ),
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
            // A later restart (or the dispose) owns the state. Its stop came while this client was starting, which
            // need not stop it (two clients then publish everything twice): stop it here.
            if (failure === undefined) {
                try {
                    await client.stop();
                } catch {
                    // Already stopped.
                }
            }
            return;
        }
        if (failure === undefined) {
            client.onDidChangeState((e) => this.clientState(client, e.newState));
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

    /**
     * Asks the server to check a document's program; the server publishes the diagnostics into its own collection.
     * Undefined when the server does not run, or the check was cancelled: after `timeoutMs`, by a newer check of the
     * same document (the server then answers the older one with ContentModified), or by the server stopping.
     * Throws when the check failed otherwise (an internal error of the server, which it logs).
     */
    async check(uri: vscode.Uri, timeoutMs: number): Promise<CheckResult | undefined> {
        const client = this.running;
        if (!client) {
            return undefined;
        }
        const key = uri.toString();
        this.checks.get(key)?.cancel();
        const source = new vscode.CancellationTokenSource();
        this.checks.set(key, source);
        const timer = setTimeout(() => source.cancel(), timeoutMs);
        try {
            const params = checkParams(client.code2ProtocolConverter.asUri(uri));
            return await client.sendRequest<CheckResult>(CHECK, params, source.token);
        } catch (e) {
            const cancelled = e instanceof ResponseError && (e.code === LSPErrorCodes.RequestCancelled || e.code === LSPErrorCodes.ContentModified);
            if (cancelled || source.token.isCancellationRequested || this.running !== client) {
                return undefined;
            }
            throw e;
        } finally {
            clearTimeout(timer);
            if (this.checks.get(key) === source) {
                this.checks.delete(key);
            }
            source.dispose();
        }
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
        this.tellRunning();
        if (client) {
            try {
                await client.stop();
            } catch {
                // A server that failed to start cannot be stopped; nothing is left to clean up.
            }
        }
    }

    /**
     * A change of the running client's own state: the server died, and the client stopped (or gave up restarting it),
     * or came back after a restart of its own. While it is stopped the server does not run, so the extension's
     * formatter is registered again (spec editor/formatting, "Which formatter"). A client this class stopped is no
     * longer `this.client`.
     */
    private clientState(client: LanguageClient, state: ClientState): void {
        if (this.client !== client) {
            return;
        }
        if (state === ClientState.Stopped && this.state.kind === "running") {
            this.state = { kind: "failed", reason: STOPPED };
            this.render();
        } else if (state === ClientState.Running && this.state.kind === "failed" && this.state.reason === STOPPED) {
            this.state = { kind: "running" };
            this.render();
        }
    }

    private tellRunning(): void {
        const running = this.state.kind === "running";
        if (running !== this.wasRunning) {
            this.wasRunning = running;
            this.runningChange.fire(running);
        }
    }

    private render(): void {
        this.tellRunning();
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
                this.item.text = d.source === "bundle" ? "qb64rust (bundled)" : "qb64rust";
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
