// Reads the extension's settings. Every `qb64rust.*` lookup goes through here.
import * as vscode from "vscode";

const SECTION = "qb64rust";

function settings(scope?: vscode.ConfigurationScope): vscode.WorkspaceConfiguration {
    return vscode.workspace.getConfiguration(SECTION, scope);
}

export function compilerPath(): string {
    return settings().get<string>("compilerPath", "").trim();
}

export function checkOnSave(uri: vscode.Uri): boolean {
    return settings(uri).get<boolean>("checkOnSave", true);
}

/** Timeout for check and format runs (design D4). */
export function checkTimeoutMs(): number {
    const seconds = settings().get<number>("checkTimeoutSeconds", 30);
    return Math.max(1, seconds) * 1000;
}

export function runArguments(uri: vscode.Uri): string {
    return settings(uri).get<string>("runArguments", "").trim();
}

export function noPrompt(uri: vscode.Uri): boolean {
    return settings(uri).get<boolean>("noPrompt", false);
}

export function affectsCompilerPath(e: vscode.ConfigurationChangeEvent): boolean {
    return e.affectsConfiguration(`${SECTION}.compilerPath`);
}

/**
 * The encoding VS Code uses for this document's bytes (design D10). This is what the document was opened with:
 * the `[qb64rust]` default (cp437), a user or workspace override, or an encoding chosen by hand.
 */
export function documentEncoding(document: vscode.TextDocument): string {
    return document.encoding;
}

export function openCompilerPathSetting(): Thenable<unknown> {
    return vscode.commands.executeCommand("workbench.action.openSettings", `${SECTION}.compilerPath`);
}
