import * as vscode from "vscode";
import * as qb64pe from "./compiler/qb64pe";
import { RunQueue } from "./compiler/runQueue";
import { Builder } from "./build";
import { Diagnostics } from "./diagnostics";
import { Formatter, removeStaleTempFiles } from "./format";
import { LanguageServer } from "./languageServer";
import { CompilerStatus } from "./statusBar";
import { Qb64TaskProvider, TASK_TYPE } from "./tasks";

/** Returned from `activate` so integration tests can observe the parts. */
export interface Api {
    status: CompilerStatus;
    diagnostics: Diagnostics;
    queue: RunQueue;
    server: LanguageServer;
}

let server: LanguageServer | undefined;

const BUSY_TEXT = { check: "checking…", format: "formatting…", build: "building…" } as const;

export function activate(context: vscode.ExtensionContext): Api {
    const queue = new RunQueue();
    const status = new CompilerStatus();
    queue.onBusyChange = (kind) => status.setBusy(kind ? BUSY_TEXT[kind] : undefined);
    const diagnostics = new Diagnostics(queue, status);
    const builder = new Builder(queue, status, diagnostics);
    server = new LanguageServer();
    context.subscriptions.push(
        { dispose: () => queue.dispose() },
        status,
        diagnostics,
        builder,
        server,
        vscode.languages.registerDocumentFormattingEditProvider("qb64rust", new Formatter(queue, status, diagnostics)),
        vscode.tasks.registerTaskProvider(TASK_TYPE, new Qb64TaskProvider(status)),
    );
    void removeStaleTempFiles();
    return { status, diagnostics, queue, server };
}

export async function deactivate(): Promise<void> {
    qb64pe.removeSessionTempDir();
    await server?.stop();
}
