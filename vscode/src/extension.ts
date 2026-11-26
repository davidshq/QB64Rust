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
import { ExamplesView } from "./examples/examplesView";
import { HelpIndex } from "./help/helpIndex";
import { HelpPanel } from "./help/helpPanel";
import { HelpView } from "./help/helpView";
import { HomePanel } from "./home/homePanel";
import { NewProgram } from "./newProgram";
import { registerNewProject } from "./newProject";
import { ProgramView, openProgram } from "./panel/programView";
import { ProceduresView } from "./panel/proceduresView";
import { CompilerStatus } from "./statusBar";
import { Qb64TaskProvider, TASK_TYPE } from "./tasks";
import { TestSupport } from "./testing";
import { registerWalkthrough } from "./walkthrough";
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
    /** The QB64 side panel (change editor-side-panel). */
    program: ProgramView;
    procedures: ProceduresView;
    help: HelpPanel;
    helpView: HelpView;
    examples: ExamplesView;
    newProgram: NewProgram;
    /** The page "QB64 Home". */
    home: HomePanel;
    /** The help folder's index, undefined without one. */
    helpIndex: () => HelpIndex | undefined;
}

let server: LanguageServer | undefined;

const BUSY_TEXT = { check: "checking…", format: "formatting…", build: "building…" } as const;

export function activate(context: vscode.ExtensionContext): Api {
    const queue = new RunQueue();
    const status = new CompilerStatus();
    // The Program view is made below, once the language server exists.
    const panel: { program?: ProgramView } = {};
    queue.onBusyChange = (kind) => {
        status.setBusy(kind ? BUSY_TEXT[kind] : undefined);
        panel.program?.setQueueActivity(kind);
    };
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
    // The QB64 side panel (change editor-side-panel): the Program view, the status bar items and the procedures.
    const programView = new ProgramView(lsp);
    panel.program = programView;
    const procedures = new ProceduresView(lsp);
    context.subscriptions.push(
        programView,
        procedures,
        procedures.onDidChangeFiles((files) => programView.setProgramFiles(files)),
        builder.onDidChangeFirstBuild((on) => programView.setFirstBuild(on)),
        toolchain.onDidChangeDownload((what) => programView.setDownload(what)),
        vscode.commands.registerCommand("qb64rust.openProgram", () => openProgram()),
    );
    // Help and examples from the help folder the package carries (design D4 of editor-side-panel), read once.
    let index: HelpIndex | undefined | null = null;
    const helpIndex = (): HelpIndex | undefined => {
        if (index === null) {
            index = HelpIndex.load(context.asAbsolutePath("help"));
        }
        return index;
    };
    programView.pageTitle = (page) => helpIndex()?.page(page)?.title;
    const help = new HelpPanel(vscode.Uri.joinPath(context.extensionUri, "media"), helpIndex, bundle.storage);
    const helpView = new HelpView(helpIndex, lsp);
    const examples = new ExamplesView(helpIndex, bundle.storage);
    const newProgram = new NewProgram(context.globalState);
    context.subscriptions.push(help, helpView, examples, newProgram, registerWalkthrough(newProgram));
    // "QB64 Home" (design D11 of editor-side-panel): the page a QB64 window starts with while no file is open.
    const home = new HomePanel(context.globalState, {
        compiler: () => lsp.binary.found,
        toolchain: async () => {
            await lsp.ready;
            return toolchain.refresh();
        },
        emsdk: () => browser.find(),
        storage: bundle.storage,
    });
    context.subscriptions.push(
        home,
        // A program the Program view names is a recent program.
        programView.onDidChangeState((s) => {
            if (s.kind === "program") {
                void home.noteProgram(vscode.Uri.parse(s.uri));
            }
        }),
    );
    void home.openAtStartup();
    context.subscriptions.push(
        { dispose: () => queue.dispose() },
        status,
        diagnostics,
        builder,
        toolchain,
        debug,
        browser,
        tests,
        lsp,
        registerWhileServerOff(lsp, new Formatter(queue, status, diagnostics, lsp)),
        vscode.tasks.registerTaskProvider(TASK_TYPE, new Qb64TaskProvider(status, lsp, bundle)),
        registerNewProject(),
    );
    void removeStaleTempFiles();
    void removeOtherVersions(bundle.storage, bundle.version);
    return { status, diagnostics, queue, server: lsp, bundle, builder, debug, toolchain, tests, browser, program: programView, procedures, help, helpView, examples, newProgram, home, helpIndex };
}

export async function deactivate(): Promise<void> {
    qb64pe.removeSessionTempDir();
    await server?.stop();
}
