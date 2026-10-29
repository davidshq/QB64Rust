import * as path from "path";
import * as vscode from "vscode";
import { copyVersion, removeOtherVersions } from "./compiler/libqbStore";
import { binaryName } from "./compiler/toolchain";
import * as qb64pe from "./compiler/qb64pe";
import { RunQueue } from "./compiler/runQueue";
import { Builder } from "./build";
import { Bundle, chosenCompiler } from "./buildPlan";
import { DebugSupport } from "./debug";
import { Diagnostics } from "./diagnostics";
import { Formatter, registerWhileServerOff, removeStaleTempFiles } from "./format";
import { LanguageServer } from "./languageServer";
import { registerNewProject } from "./newProject";
import { CompilerStatus } from "./statusBar";
import { Qb64TaskProvider, TASK_TYPE } from "./tasks";
import { ToolchainSupport } from "./toolchainSupport";

/** Returned from `activate` so integration tests can observe the parts. */
export interface Api {
    status: CompilerStatus;
    diagnostics: Diagnostics;
    queue: RunQueue;
    server: LanguageServer;
    bundle: Bundle;
    builder: Builder;
    debug: DebugSupport;
    toolchain: ToolchainSupport;
}

let server: LanguageServer | undefined;

const BUSY_TEXT = { check: "checking…", format: "formatting…", build: "building…" } as const;

export function activate(context: vscode.ExtensionContext): Api {
    const queue = new RunQueue();
    const status = new CompilerStatus();
    queue.onBusyChange = (kind) => status.setBusy(kind ? BUSY_TEXT[kind] : undefined);
    // What the platform package carries (design D1-D3 of extension-vsix); a development host has no `bundle\`.
    const libqb = context.asAbsolutePath(path.join("bundle", "libqb"));
    const bundle: Bundle = {
        libqb,
        storage: context.globalStorageUri.fsPath,
        version: copyVersion(context.extension.packageJSON.version, libqb),
    };
    // The bundled binary is named per platform (design D8 of m7-linux).
    const lsp = new LanguageServer(context.asAbsolutePath(path.join("bundle", "bin", binaryName())), status);
    server = lsp;
    status.builder = () => chosenCompiler(lsp);
    status.update();
    const diagnostics = new Diagnostics(queue, status, lsp);
    const builder = new Builder(queue, status, diagnostics, lsp, bundle);
    // The C++ toolchain of a build with qb64rust (change m6-toolchain): looked for now, offered only when needed.
    const toolchain = new ToolchainSupport(status, lsp, builder.output);
    status.toolchain = toolchain;
    status.update();
    const debug = new DebugSupport(builder, status, lsp);
    context.subscriptions.push(
        { dispose: () => queue.dispose() },
        status,
        diagnostics,
        builder,
        toolchain,
        debug,
        lsp,
        registerWhileServerOff(lsp, new Formatter(queue, status, diagnostics, lsp)),
        vscode.tasks.registerTaskProvider(TASK_TYPE, new Qb64TaskProvider(status, lsp, bundle)),
        registerNewProject(),
    );
    void removeStaleTempFiles();
    void removeOtherVersions(bundle.storage, bundle.version);
    return { status, diagnostics, queue, server: lsp, bundle, builder, debug, toolchain };
}

export async function deactivate(): Promise<void> {
    qb64pe.removeSessionTempDir();
    await server?.stop();
}
