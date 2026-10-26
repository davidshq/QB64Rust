// Debugging a program in VS Code (change m5-debug-launch): the configuration provider (design D3), the adapter
// `qb64rust dap` (D2) and the Debug command (D5). The rules are in debugConfig.ts; the debug build is Build's
// (`Builder.debugBuild`, D4).
import * as path from "path";
import * as vscode from "vscode";
import * as config from "./config";
import { Builder } from "./build";
import { refuse } from "./buildPlan";
import { DEBUG_TYPE, Facts, LaunchConfig, fileConfiguration, initialConfiguration, resolveLaunch } from "./debugConfig";
import { LanguageServer } from "./languageServer";
import { CompilerStatus } from "./statusBar";

export class DebugSupport implements vscode.Disposable, vscode.DebugConfigurationProvider, vscode.DebugAdapterDescriptorFactory {
    private readonly disposables: vscode.Disposable[];
    /** The last refusal's message; for tests. */
    lastRefusal: string | undefined;

    constructor(
        private readonly builder: Builder,
        private readonly status: CompilerStatus,
        private readonly server: LanguageServer,
    ) {
        this.disposables = [
            vscode.debug.registerDebugConfigurationProvider(DEBUG_TYPE, this),
            vscode.debug.registerDebugAdapterDescriptorFactory(DEBUG_TYPE, this),
            vscode.commands.registerCommand("qb64rust.debug", (uri?: vscode.Uri) => this.debugCommand(uri)),
        ];
    }

    provideDebugConfigurations(): vscode.DebugConfiguration[] {
        return [initialConfiguration()];
    }

    /**
     * Step 1 of design D3: F5 with no `launch.json` gives an empty configuration, which debugs the active QB64 file;
     * with another file active it is returned unchanged.
     */
    resolveDebugConfiguration(_folder: vscode.WorkspaceFolder | undefined, debugConfig: vscode.DebugConfiguration): vscode.DebugConfiguration {
        if (!debugConfig.type && !debugConfig.request && !debugConfig.name && activeQb64File()) {
            return initialConfiguration();
        }
        return debugConfig;
    }

    /** Step 2 of design D3: refuse, launch the executable, or build with `--debug` and launch what was built. */
    async resolveDebugConfigurationWithSubstitutedVariables(
        folder: vscode.WorkspaceFolder | undefined,
        debugConfig: vscode.DebugConfiguration,
    ): Promise<vscode.DebugConfiguration | undefined> {
        const launch = resolveLaunch(debugConfig as LaunchConfig, this.facts(folder));
        switch (launch.kind) {
            case "refuse":
                this.lastRefusal = launch.message;
                // Not awaited: the launch ends now, the message stays with its button.
                void refuse(launch.message, launch.setting);
                return undefined;
            case "launch":
                return launch.config;
            case "build": {
                const exe = await this.builder.debugBuild(vscode.Uri.file(launch.source));
                // No executable: the reason is in Problems and the output already. `undefined` ends the launch.
                return exe ? { ...launch.config, program: exe } : undefined;
            }
        }
    }

    /** Design D2: `qb64rust dap --qb64pe-root <folder>`, from the binary and the QB64pe folder a build uses. */
    createDebugAdapterDescriptor(): vscode.DebugAdapterDescriptor {
        const binary = this.server.binary;
        if (!binary.found) {
            throw new Error(`QB64: debugging needs qb64rust, but ${binary.reason}`);
        }
        const qb64pe = this.status.current;
        if (!qb64pe.found) {
            throw new Error(`QB64: debugging needs QB64pe's toolchain, but ${qb64pe.reason}`);
        }
        return new vscode.DebugAdapterExecutable(binary.path, ["dap", "--qb64pe-root", path.dirname(qb64pe.path)]);
    }

    /** Design D5: debugs the file of `uri` or the active editor, as Build and Run runs it. */
    async debugCommand(uri?: vscode.Uri): Promise<boolean> {
        const file = uri?.scheme === "file" ? uri.fsPath : activeQb64File();
        if (!file) {
            this.lastRefusal = "open a QB64 program first.";
            void vscode.window.showErrorMessage(`QB64: ${this.lastRefusal}`);
            return false;
        }
        const folder = vscode.workspace.getWorkspaceFolder(vscode.Uri.file(file));
        return vscode.debug.startDebugging(folder, fileConfiguration(file));
    }

    private facts(folder: vscode.WorkspaceFolder | undefined): Facts {
        const resource = folder?.uri;
        return {
            trusted: vscode.workspace.isTrusted,
            activeFile: activeQb64File(),
            folder: folder?.uri.scheme === "file" ? folder.uri.fsPath : undefined,
            runArguments: config.runArguments(resource),
            noPrompt: config.noPrompt(resource),
            buildWith: config.buildWith(resource),
            binary: this.server.binary,
        };
    }

    dispose(): void {
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}

/** The active editor's file when it is a QB64 program on disk. */
function activeQb64File(): string | undefined {
    const doc = vscode.window.activeTextEditor?.document;
    return doc && doc.languageId === "qb64rust" && doc.uri.scheme === "file" ? doc.uri.fsPath : undefined;
}
