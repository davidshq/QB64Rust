// "QB64: New Program" (spec editor/new-program; change editor-side-panel, design D8): a save dialog for one `.bas`
// file, a short program written to it, and the file opened. It writes nothing else: no folder, no `qb64rust.toml`
// and no setting. It builds nothing, so it works in an untrusted workspace. The rules are in newProgramPlan.ts.
import * as fs from "fs";
import * as iconv from "iconv-lite";
import * as os from "os";
import * as path from "path";
import * as vscode from "vscode";
import { NEW_PROGRAM, dialogFolder, exampleStem, folderIsUtf8, freeName, withBas } from "./newProgramPlan";

/** Where the command last wrote a program, kept in the extension's `globalState`, not in a setting. */
const LAST_FOLDER = "qb64rust.newProgram.lastFolder";

export interface NewProgramOptions {
    /** The text to write in place of the short program (an example's). */
    text?: string;
    /** The example the text comes from (`_clamp_1`), for the name filled in. */
    example?: string;
    /** The file to write, given in place of the dialog (tests); null stands for a cancelled dialog. */
    file?: vscode.Uri | null;
}

/** What the command did: the file it wrote and opened, or nothing (cancelled). */
export type NewProgramResult = { file: string; utf8: boolean } | undefined;

export class NewProgram implements vscode.Disposable {
    private readonly disposables: vscode.Disposable[];
    private readonly made = new vscode.EventEmitter<string>();
    /** Fires with the path of each program written (the walkthrough's first step). */
    readonly onDidMakeProgram = this.made.event;

    constructor(private readonly state: vscode.Memento) {
        this.disposables = [this.made, vscode.commands.registerCommand("qb64rust.newProgram", (o?: NewProgramOptions) => this.run(o ?? {}))];
    }

    /** The folder the dialog opens in now. */
    dialogFolder(): string {
        const active = vscode.window.activeTextEditor?.document.uri;
        return dialogFolder({
            workspaceFolders: (vscode.workspace.workspaceFolders ?? []).filter((f) => f.uri.scheme === "file").map((f) => f.uri.fsPath),
            activeFileFolder: active?.scheme === "file" ? path.dirname(active.fsPath) : undefined,
            lastFolder: this.state.get<string>(LAST_FOLDER),
            home: os.homedir(),
            exists: (f) => fs.existsSync(f),
        });
    }

    async run(o: NewProgramOptions): Promise<NewProgramResult> {
        let target = o.file;
        if (target === undefined) {
            const folder = this.dialogFolder();
            const name = freeName(folder, o.example ? exampleStem(o.example) : "program");
            target = await vscode.window.showSaveDialog({
                defaultUri: vscode.Uri.file(path.join(folder, name)),
                filters: { "QB64 programs": ["bas"] },
                saveLabel: "Create Program",
                title: "QB64: New Program",
            });
        }
        if (!target) {
            return undefined;
        }
        let file = target.fsPath;
        if (withBas(file) !== file) {
            file = withBas(file);
            // The dialog asked about the name without `.bas`; this name was not asked about.
            if (fs.existsSync(file) && o.file === undefined) {
                const replace = "Replace";
                const answer = await vscode.window.showWarningMessage(`${path.basename(file)} already exists. Replace it?`, { modal: true }, replace);
                if (answer !== replace) {
                    return undefined;
                }
            }
        }
        const utf8 = folderIsUtf8(path.dirname(file));
        const text = o.text ?? NEW_PROGRAM;
        try {
            fs.writeFileSync(file, utf8 ? Buffer.from(text, "utf8") : iconv.encode(text, "cp437"));
        } catch (e) {
            void vscode.window.showErrorMessage(`QB64: the new program could not be written: ${(e as Error).message}`);
            return undefined;
        }
        await this.state.update(LAST_FOLDER, path.dirname(file));
        // Opened in its folder's encoding, so that it is saved so: a folder's `.vscode\settings.json` applies only
        // when the folder is the workspace (as "QB64: New Project" does).
        const document = await vscode.workspace.openTextDocument(vscode.Uri.file(file), utf8 ? { encoding: "utf8" } : undefined);
        await vscode.window.showTextDocument(document);
        this.made.fire(file);
        return { file, utf8 };
    }

    dispose(): void {
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}
