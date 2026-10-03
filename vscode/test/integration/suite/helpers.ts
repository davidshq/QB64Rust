import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import type { Api } from "../../../src/extension";

export const workspace = process.env.QB64RUST_TEST_WORKSPACE!;
export const compiler = process.env.QB64RUST_TEST_COMPILER ?? "";

export async function api(): Promise<Api> {
    const ext = vscode.extensions.getExtension<Api>("qb64rust.qb64rust")!;
    return ext.activate();
}

/** Writes a file into the test workspace (bytes as given; strings as Latin-1, i.e. byte per char). */
export function writeFile(rel: string, content: string | Buffer): vscode.Uri {
    const file = path.join(workspace, rel);
    fs.mkdirSync(path.dirname(file), { recursive: true });
    fs.writeFileSync(file, typeof content === "string" ? Buffer.from(content, "latin1") : content);
    return vscode.Uri.file(file);
}

export async function open(uri: vscode.Uri): Promise<vscode.TextEditor> {
    const doc = await vscode.workspace.openTextDocument(uri);
    return vscode.window.showTextDocument(doc);
}

export async function setText(editor: vscode.TextEditor, text: string): Promise<void> {
    const doc = editor.document;
    await editor.edit((b) => b.replace(new vscode.Range(doc.positionAt(0), doc.positionAt(doc.getText().length)), text));
}

/** Saves and waits for the check that the save started. */
export async function saveAndCheck(editor: vscode.TextEditor): Promise<void> {
    const a = await api();
    const key = editor.document.uri.toString();
    const before = a.diagnostics.lastCheck.get(key);
    await editor.document.save();
    const started = Date.now();
    while (a.diagnostics.lastCheck.get(key) === before && Date.now() - started < 5000) {
        await sleep(20);
    }
    await a.diagnostics.lastCheck.get(key);
}

export function sleep(ms: number): Promise<void> {
    return new Promise((r) => setTimeout(r, ms));
}

export async function waitFor<T>(what: string, probe: () => T | undefined | false, timeoutMs = 10000): Promise<T> {
    const started = Date.now();
    for (;;) {
        const v = probe();
        if (v) {
            return v;
        }
        if (Date.now() - started > timeoutMs) {
            throw new Error(`timed out waiting for ${what}`);
        }
        await sleep(50);
    }
}

/** Resolves a command that waits on a notification by dismissing all notifications after a moment. */
export async function dismissingNotifications<T>(run: Thenable<T>): Promise<T> {
    await sleep(1000);
    await vscode.commands.executeCommand("notifications.clearAll");
    return run;
}

export async function closeAll(): Promise<void> {
    await vscode.commands.executeCommand("workbench.action.closeAllEditors");
}

export async function setSetting(section: string, key: string, value: unknown, languageId?: string): Promise<void> {
    const cfg = vscode.workspace.getConfiguration(section, languageId ? { languageId } : undefined);
    await cfg.update(key, value, vscode.ConfigurationTarget.Workspace, languageId !== undefined);
}
