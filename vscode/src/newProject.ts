// "QB64: New Project" (spec editor/new-project; design D7 of fixes-switch): a folder with `main.bas` and a
// `qb64rust.toml` that turns the fixes on, since a new project defaults to the fixes on and an existing program to
// off. It builds nothing, so it works in an untrusted workspace too.
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";

/** The program the command writes. */
export const MAIN_BAS = 'PRINT "Hello, world!"\r\n';

/** The settings file the command writes: every fix on (spec compiler/fixes). */
export const SETTINGS_TOML = "# qb64rust settings for this program (fixes: qb64rust --list-fixes).\n[fixes]\nall = true\n";

/** The files the command writes, by name. */
export const PROJECT_FILES: ReadonlyArray<[string, string]> = [
    ["main.bas", MAIN_BAS],
    ["qb64rust.toml", SETTINGS_TOML],
];

/**
 * Writes the project into `folder` and gives the path of `main.bas`, or, when one of its files is there already,
 * writes nothing and gives that file's name as the error.
 */
export function writeProject(folder: string): { main: string } | { exists: string } {
    const there = PROJECT_FILES.find(([name]) => fs.existsSync(path.join(folder, name)));
    if (there) {
        return { exists: there[0] };
    }
    const written: string[] = [];
    try {
        for (const [name, text] of PROJECT_FILES) {
            // `wx`: never over a file made since the test above.
            const file = path.join(folder, name);
            fs.writeFileSync(file, text, { flag: "wx" });
            written.push(file);
        }
    } catch (e) {
        // A file that could not be written: none of the project is left, only what this call wrote is removed.
        for (const file of written) {
            fs.rmSync(file, { force: true });
        }
        throw e;
    }
    return { main: path.join(folder, "main.bas") };
}

/** What the command did: the `main.bas` it wrote and opened, or the message it showed, or nothing (no folder). */
export type NewProjectResult = { main: string } | { message: string } | undefined;

/**
 * Asks for a folder (or takes `folder`, as a test gives it in place of the dialog), writes the project into it and
 * opens `main.bas`.
 */
export async function newProject(folder?: vscode.Uri): Promise<NewProjectResult> {
    if (!folder) {
        const picked = await vscode.window.showOpenDialog({
            canSelectFolders: true,
            canSelectFiles: false,
            canSelectMany: false,
            openLabel: "Create Project Here",
            title: "QB64: New Project",
        });
        folder = picked?.[0];
    }
    if (!folder) {
        return undefined;
    }
    let result: ReturnType<typeof writeProject>;
    try {
        result = writeProject(folder.fsPath);
    } catch (e) {
        const message = `QB64: the new project could not be written: ${(e as Error).message}`;
        void vscode.window.showErrorMessage(message);
        return { message };
    }
    if ("exists" in result) {
        const message = `QB64: ${result.exists} is already in ${folder.fsPath}; nothing was written.`;
        void vscode.window.showErrorMessage(message);
        return { message };
    }
    await vscode.window.showTextDocument(vscode.Uri.file(result.main));
    return { main: result.main };
}

export function registerNewProject(): vscode.Disposable {
    return vscode.commands.registerCommand("qb64rust.newProject", (folder?: vscode.Uri) => newProject(folder));
}
