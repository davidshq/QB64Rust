// An example of the help, opened, run or reset (OpenSpec change editor-side-panel, design D7; specs editor/examples
// and editor/help). A click opens the user's copy of it, a real file in the extension's own storage
// (exampleCopy.ts), so it closes without a question and Run builds it at once. "Run" builds that same copy, so
// nothing is written into the user's folders. "Open as a New Program" gives the shipped text to "QB64: New Program".
// The shipped file is never opened for editing.
import * as fs from "fs";
import * as vscode from "vscode";
import { HelpIndex } from "../help/helpIndex";
import { tickWalkthrough } from "../walkthrough";
import { ensureExample, resetExample } from "./exampleCopy";

/** The program's text, as shipped (UTF-8, as the reference keeps it). */
export function exampleText(index: HelpIndex, name: string): string | undefined {
    const e = index.example(name);
    return e ? fs.readFileSync(index.exampleFile(e), "utf8") : undefined;
}

/** "Run": the user's copy built and run by "QB64: Build and Run". A network example has no Run (design D7). */
export async function runExample(index: HelpIndex, name: string, storage: string): Promise<boolean> {
    const e = index.example(name);
    if (!e || e.network) {
        return false;
    }
    if (!vscode.workspace.isTrusted) {
        void vscode.window.showWarningMessage("QB64: running an example requires a trusted workspace. Nothing was executed.");
        return false;
    }
    const copy = ensureExample(index, name, storage);
    tickWalkthrough("triedExample");
    return copy ? Boolean(await vscode.commands.executeCommand("qb64rust.buildAndRun", vscode.Uri.file(copy))) : false;
}

/** A click on an example: its copy in an editor, as any file. */
export async function openExample(index: HelpIndex, name: string, storage: string): Promise<vscode.TextEditor | undefined> {
    const copy = ensureExample(index, name, storage);
    if (copy === undefined) {
        return undefined;
    }
    tickWalkthrough("triedExample");
    return vscode.window.showTextDocument(await vscode.workspace.openTextDocument(vscode.Uri.file(copy)));
}

/** "Reset This Example": the copy as shipped again. An open editor of it with unsaved changes is reverted. */
export async function resetExampleCopy(index: HelpIndex, name: string, storage: string): Promise<boolean> {
    const copy = resetExample(index, name, storage);
    if (copy === undefined) {
        return false;
    }
    const uri = vscode.Uri.file(copy).toString();
    const doc = vscode.workspace.textDocuments.find((d) => d.uri.toString() === uri);
    if (doc?.isDirty) {
        await vscode.window.showTextDocument(doc);
        await vscode.commands.executeCommand("workbench.action.files.revert");
    }
    return true;
}

/** "Open as a New Program": the text given to "QB64: New Program", with the example's name filled in. */
export function openAsNewProgram(index: HelpIndex, name: string): Thenable<unknown> {
    const text = exampleText(index, name);
    return vscode.commands.executeCommand("qb64rust.newProgram", { text: text?.replace(/\r?\n/g, "\r\n"), example: name });
}
