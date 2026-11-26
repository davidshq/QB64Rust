// The rules of "QB64: New Program" (spec editor/new-program; change editor-side-panel, design D8): where its save
// dialog opens, the name filled in, and the encoding of the file it writes. No VS Code import, so it is
// unit-tested without it.
import * as fs from "fs";
import * as path from "path";

/** The short program the command writes when no example gives one. */
export const NEW_PROGRAM = 'PRINT "Hello, world!"\r\n';

/** The first of `<stem>.bas`, `<stem>2.bas`, … that `folder` does not hold; for `program`, `program1.bas` first
 * (spec "Where the save dialog opens"). */
export function freeName(folder: string, stem = "program"): string {
    const numbered = stem === "program";
    for (let n = numbered ? 1 : 0; ; n++) {
        const name = numbered ? `${stem}${n}.bas` : n === 0 ? `${stem}.bas` : `${stem}${n + 1}.bas`;
        if (!fs.existsSync(path.join(folder, name))) {
            return name;
        }
    }
}

/** The stem of an example's file name: `_clamp_1` is `clamp`, `first-steps/hello` is `hello`, `left-str_2` is
 * `left` (spec "A new program from given text"). */
export function exampleStem(example: string): string {
    const last = example.split("/").pop() ?? example;
    const stem = last
        .replace(/_\d+$/, "")
        .replace(/-str$/, "")
        .replace(/^meta-/, "")
        .replace(/^_+/, "")
        .replace(/[^A-Za-z0-9_-]/g, "");
    return stem || "program";
}

export interface DialogInputs {
    /** The workspace folders, in order. */
    workspaceFolders: string[];
    /** The folder of the active editor's file, when it is a file. */
    activeFileFolder?: string;
    /** Where the command last wrote a program. */
    lastFolder?: string;
    home: string;
    exists: (folder: string) => boolean;
}

/** The folder the save dialog opens in: the workspace folder; with several, the one that holds the active file;
 * with none, the folder last written to when it still exists, else the documents folder (`Documents` in the home
 * folder, else the home folder). */
export function dialogFolder(i: DialogInputs): string {
    if (i.workspaceFolders.length) {
        const holding = i.activeFileFolder && i.workspaceFolders.find((f) => isInside(i.activeFileFolder!, f));
        return holding || i.workspaceFolders[0];
    }
    if (i.lastFolder && i.exists(i.lastFolder)) {
        return i.lastFolder;
    }
    const documents = path.join(i.home, "Documents");
    return i.exists(documents) ? documents : i.home;
}

function isInside(child: string, parent: string): boolean {
    const rel = path.relative(parent, child);
    return rel === "" || (!rel.startsWith("..") && !path.isAbsolute(rel));
}

/** The name the user gave, with `.bas` added when it has no extension of a QB64 file. */
export function withBas(file: string): string {
    return /\.(bas|bi|bm)$/i.test(file) ? file : `${file}.bas`;
}

/**
 * Whether the nearest `qb64rust.toml` at or above `folder` declares the project's files UTF-8 (`encoding =
 * "utf-8"` under `[project]`, spec compiler/source-encoding). A file that cannot be read counts as none.
 */
export function folderIsUtf8(folder: string): boolean {
    for (let dir = path.resolve(folder); ; dir = path.dirname(dir)) {
        const file = path.join(dir, "qb64rust.toml");
        if (fs.existsSync(file)) {
            let text: string;
            try {
                text = fs.readFileSync(file, "utf8");
            } catch {
                return false;
            }
            return tomlProjectEncoding(text)?.toLowerCase().replace("-", "") === "utf8";
        }
        if (path.dirname(dir) === dir) {
            return false;
        }
    }
}

/** The `encoding` string of the `[project]` table of a settings file, read line by line. */
export function tomlProjectEncoding(text: string): string | undefined {
    let table = "";
    for (const raw of text.split(/\r?\n/)) {
        const line = raw.replace(/#.*$/, "").trim();
        const header = /^\[([^\]]+)\]$/.exec(line);
        if (header) {
            table = header[1].trim();
            continue;
        }
        const m = /^encoding\s*=\s*["']([^"']*)["']$/.exec(line);
        if (m && table === "project") {
            return m[1];
        }
    }
    return undefined;
}
