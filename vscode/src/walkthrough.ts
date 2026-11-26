// The walkthrough "Get Started with QB64" (OpenSpec change editor-side-panel, design D9; spec editor/walkthrough):
// "QB64: Get Started", which opens it, the step "Find and fix a mistake", and the context key that ticks the first
// step once a program has been made.
import * as vscode from "vscode";
import { NewProgram } from "./newProgram";

/** The walkthrough's id, as VS Code names it: the extension's id and the walkthrough's own. */
export const WALKTHROUGH = "qb64rust.qb64rust#getStarted";

/** "Hello" with a `(` never closed on line 3: an error the parser reports as the user types, so the Program view
 * shows it at once, with no save. (A misspelled `PRINT` is found only by the check on save.) */
export const MISTAKE = '\' Hello: a first program. Each PRINT writes one line.\nPRINT "Hello, world!"\nPRINT "Two and three make"; (2 + 3\nEND\n';

/** The steps a context key ticks, besides "write" (set when New Program has written a file). */
export type WalkthroughStep = "ran" | "lookedUp" | "triedExample" | "openedMistake" | "ranInBrowser";

/**
 * Ticks a step of the walkthrough: its `onContext:qb64rust.walkthrough.<step>` completion event. Called by the
 * command that does the step's work, however it was started. An `onCommand:` event alone is not enough: a command
 * the extension runs itself (the side panel's Run button, the status bar's, the help's links) is run in the
 * extension host and never reaches the event, as measured in VS Code 1.100 and 1.146 (task 6.2).
 */
export function tickWalkthrough(step: WalkthroughStep): void {
    void vscode.commands.executeCommand("setContext", `qb64rust.walkthrough.${step}`, true);
}

export function registerWalkthrough(newProgram: NewProgram): vscode.Disposable {
    return vscode.Disposable.from(
        vscode.commands.registerCommand("qb64rust.getStarted", () => vscode.commands.executeCommand("workbench.action.openWalkthrough", WALKTHROUGH, false)),
        vscode.commands.registerCommand("qb64rust.walkthrough.openMistake", async () => {
            tickWalkthrough("openedMistake");
            const doc = await vscode.workspace.openTextDocument({ language: "qb64rust", content: MISTAKE });
            return vscode.window.showTextDocument(doc, vscode.ViewColumn.One);
        }),
        newProgram.onDidMakeProgram(() => vscode.commands.executeCommand("setContext", "qb64rust.walkthrough.madeProgram", true)),
    );
}
