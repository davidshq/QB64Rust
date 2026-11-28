import * as path from "path";
import * as vscode from "vscode";
import { copyVersion, removeOtherVersions } from "./compiler/libqbStore";
import { binaryName } from "./compiler/toolchain";
import * as qb64pe from "./compiler/qb64pe";
import { RunQueue } from "./compiler/runQueue";
import { BrowserSupport } from "./browser";
import { Builder } from "./build";
import { Bundle, chosenCompiler } from "./buildPlan";
import { DebugSupport } from "./debug";
import { Diagnostics } from "./diagnostics";
import { Formatter, registerWhileServerOff, removeStaleTempFiles } from "./format";
import { LanguageServer } from "./languageServer";
import { registerNewProject } from "./newProject";
import { registerSetUpProject } from "./setUpProject";
import { CompilerStatus } from "./statusBar";
import { TabsNotice } from "./tabs";
import { Qb64TaskProvider, TASK_TYPE } from "./tasks";
import { TestSupport } from "./testing";
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
    tests: TestSupport;
    browser: BrowserSupport;
    tabs: TabsNotice;
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
    // "Run in Browser" and the Emscripten SDK (change wasm-publish): looked for and offered only when it runs.
    const browser = new BrowserSupport(queue, status, diagnostics, lsp, bundle, builder, toolchain);
    // The Test Explorer (change basic-unit-tests): `qb64rust test`, with the binary and toolchain a build uses.
    const tests = new TestSupport(status, lsp, bundle);
    // A file shown with tabs is told of once (the QB64pe IDE turns them into spaces when it loads a file).
    const tabs = new TabsNotice();
    context.subscriptions.push(
        { dispose: () => queue.dispose() },
        status,
        diagnostics,
        builder,
        toolchain,
        debug,
        browser,
        tests,
        tabs,
        lsp,
        registerWhileServerOff(lsp, new Formatter(queue, status, diagnostics, lsp)),
        vscode.tasks.registerTaskProvider(TASK_TYPE, new Qb64TaskProvider(status, lsp, bundle)),
        registerNewProject(),
        registerSetUpProject(lsp),
    );
    void removeStaleTempFiles();
    void removeOtherVersions(bundle.storage, bundle.version);
    return { status, diagnostics, queue, server: lsp, bundle, builder, debug, toolchain, tests, browser, tabs };
}

export async function deactivate(): Promise<void> {
    qb64pe.removeSessionTempDir();
    await server?.stop();
}
