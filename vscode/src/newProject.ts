// "QB64: New Project" (spec editor/new-project; design D7 of fixes-switch): a folder with `main.bas` and a
// `qb64rust.toml` that turns the fixes on, since a new project defaults to the fixes on and an existing program to
// off. The project is UTF-8 (design D10 of utf8-source). It builds nothing, so it works in an untrusted workspace
// too.
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";

/** The program the command writes. */
export const MAIN_BAS = 'PRINT "Hello, world!"\r\n';

/**
 * The settings file the command writes: every fix on (spec compiler/fixes), and the project's files UTF-8 (spec
 * compiler/source-encoding; OpenSpec change utf8-source).
 */
export const SETTINGS_TOML =
    "# qb64rust settings for this program (fixes: qb64rust --list-fixes).\n[fixes]\nall = true\n\n" +
    "# The source files are saved as UTF-8; their strings hold code page 437 characters.\n[project]\n" +
    'encoding = "utf-8"\n';

/** The workspace setting that makes VS Code save the folder's QB64 files as UTF-8, over the extension's cp437. */
export const WORKSPACE_SETTINGS = '{\n    "[qb64rust]": {\n        "files.encoding": "utf8"\n    }\n}\n';

/** The files the command writes, by name. */
export const PROJECT_FILES: ReadonlyArray<[string, string]> = [
    ["main.bas", MAIN_BAS],
    ["qb64rust.toml", SETTINGS_TOML],
    [path.join(".vscode", "settings.json"), WORKSPACE_SETTINGS],
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
    const made: string[] = [];
    try {
        for (const [name, text] of PROJECT_FILES) {
            // `wx`: never over a file made since the test above.
            const file = path.join(folder, name);
            const dir = path.dirname(file);
            if (!fs.existsSync(dir)) {
                fs.mkdirSync(dir, { recursive: true });
                made.push(dir);
            }
            fs.writeFileSync(file, text, { flag: "wx" });
            written.push(file);
        }
    } catch (e) {
        // A file that could not be written: none of the project is left, only what this call wrote or made is
        // removed.
        for (const file of written) {
            fs.rmSync(file, { force: true });
        }
        // Only an empty folder is removed, so a folder that was there after all is never emptied.
        for (const dir of made.reverse()) {
            try {
                fs.rmdirSync(dir);
            } catch {
                // Not empty, or already gone: left as it is.
            }
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
    // Opened as UTF-8 here, so that it is saved so in this window too: the folder's `.vscode/settings.json` applies
    // only once the folder is opened as the workspace (`TextDocument.encoding` is the encoding a save uses).
    const document = await vscode.workspace.openTextDocument(vscode.Uri.file(result.main), { encoding: "utf8" });
    await vscode.window.showTextDocument(document);
    return { main: result.main };
}

export function registerNewProject(): vscode.Disposable {
    return vscode.commands.registerCommand("qb64rust.newProject", (folder?: vscode.Uri) => newProject(folder));
}
