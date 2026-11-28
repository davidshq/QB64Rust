// "QB64: Set Up Project" (change project-init, design D8; spec editor/project-setup): the questions of
// `qb64rust init` as pick lists, and the compiler writes the file. A folder with a settings file is only opened, and
// a folder with no program gets "QB64: New Project".
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { InitAnswers, initPlan, initWrite } from "./compiler/qb64rust";
import type { LanguageServer } from "./languageServer";
import { newProject } from "./newProject";
import { Question, answersOf, copyQuestion, startOf } from "./setUpPlan";

/** What the command did, for the tests. */
export type SetUpResult = { settings: string } | { message: string } | undefined;

/** Shows one question; the values picked, or undefined when the list was left without a choice. */
type Ask = (q: Question) => Promise<string[] | undefined>;

async function pickList(q: Question): Promise<string[] | undefined> {
    const items = q.choices.map((c) => ({ label: c.label, description: c.description, value: c.value, picked: q.many }));
    if (q.many) {
        const picked = await vscode.window.showQuickPick(items, { title: q.title, canPickMany: true, ignoreFocusOut: true });
        return picked?.map((p) => p.value);
    }
    const picked = await vscode.window.showQuickPick(items, { title: q.title, ignoreFocusOut: true });
    return picked ? [picked.value] : undefined;
}

/** The folder of the active QB64 file, else one asked for. */
async function folderToSetUp(): Promise<vscode.Uri | undefined> {
    const doc = vscode.window.activeTextEditor?.document;
    if (doc?.languageId === "qb64rust" && doc.uri.scheme === "file") {
        return vscode.Uri.file(path.dirname(doc.uri.fsPath));
    }
    const picked = await vscode.window.showOpenDialog({ canSelectFolders: true, canSelectFiles: false, canSelectMany: false, openLabel: "Set Up", title: "QB64: Set Up Project" });
    return picked?.[0];
}

function fail(message: string): SetUpResult {
    void vscode.window.showErrorMessage(message);
    return { message };
}

/**
 * Sets up `folder` (or the active file's, or one asked for). `picks`, given by a test in place of the lists, is the
 * values picked at each question in order; `undefined` at a place is a list left without a choice.
 */
export async function setUpProject(server: LanguageServer, folder?: vscode.Uri, picks?: (string[] | undefined)[]): Promise<SetUpResult> {
    if (!vscode.workspace.isTrusted) {
        return fail("QB64: Set Up Project runs the compiler, so it needs a trusted workspace.");
    }
    folder ??= await folderToSetUp();
    if (!folder) {
        return undefined;
    }
    const binary = server.binary;
    if (!binary.found) {
        return fail("QB64: Set Up Project needs the qb64rust compiler, which was not found (setting qb64rust.path).");
    }
    const plan = await initPlan(binary.path, folder.fsPath);
    if (!plan) {
        return fail("QB64: Set Up Project needs a newer qb64rust, one with `qb64rust init`.");
    }
    const start = startOf(plan);
    if (start.kind === "open") {
        await vscode.window.showTextDocument(vscode.Uri.file(start.settings));
        void vscode.window.showInformationMessage(`QB64: ${folder.fsPath} is set up already; its qb64rust.toml is open.`);
        return { message: "set up already" };
    }
    if (start.kind === "newProject") {
        const made = await newProject(folder);
        return made && "main" in made ? { settings: path.join(folder.fsPath, "qb64rust.toml") } : made;
    }
    let asked = 0;
    const ask: Ask = (q) => (picks ? Promise.resolve(picks[asked++]) : pickList(q));
    const picked: Partial<Record<Question["key"], string[]>> = {};
    for (const q of start.questions) {
        const values = await ask(q);
        if (!values || values.length === 0) {
            return undefined;
        }
        picked[q.key] = values;
    }
    const copy = copyQuestion(plan, picked.output?.[0] ?? ".");
    if (copy) {
        // An empty pick is an answer here: no data files.
        const values = await ask(copy);
        if (!values) {
            return undefined;
        }
        picked.copy = values;
    }
    const answers: InitAnswers = answersOf(picked);
    const written = await initWrite(binary.path, folder.fsPath, answers);
    if ("error" in written) {
        return fail(`QB64: the settings file could not be written: ${written.error}`);
    }
    const settings = path.join(folder.fsPath, "qb64rust.toml");
    if (fs.existsSync(settings)) {
        await vscode.window.showTextDocument(vscode.Uri.file(settings));
    }
    return { settings };
}

export function registerSetUpProject(server: LanguageServer): vscode.Disposable {
    return vscode.commands.registerCommand("qb64rust.setUpProject", (folder?: vscode.Uri, picks?: (string[] | undefined)[]) => setUpProject(server, folder, picks));
}
