// The user's copy of an example (OpenSpec change editor-side-panel, design D7; spec editor/examples, "An example
// opens as a copy"): a real file in the extension's own storage, which a click opens, the user edits and saves as
// any file, and "Run" builds. The shipped file is never opened. A copy the user has edited is kept; one that was
// not edited follows the shipped example. No VS Code import, so it is unit-tested without it.
import * as fs from "fs";
import * as path from "path";
import { HelpIndex } from "../help/helpIndex";

/** Beside the copy: the shipped text it was made from, to tell an edited copy from an untouched one. */
const ORIGINAL = ".original";

/** The folder an example's copy is in: `<storage>\examples\<name>`, a group's folder for `first-steps/hello`. */
export function exampleFolder(storage: string, name: string): string {
    return path.join(storage, "examples", ...name.split("/"));
}

function read(file: string): Buffer | undefined {
    try {
        return fs.readFileSync(file);
    } catch {
        return undefined;
    }
}

/** Writes the copy from the shipped example, with its `.files` folder's contents beside it, over what is there.
 * The folder is not emptied: a program started from it may still run. */
function write(index: HelpIndex, name: string, storage: string): string | undefined {
    const e = index.example(name);
    if (!e) {
        return undefined;
    }
    const folder = exampleFolder(storage, name);
    const source = index.exampleFile(e);
    const copy = path.join(folder, path.basename(source));
    fs.mkdirSync(folder, { recursive: true });
    fs.copyFileSync(source, copy);
    fs.copyFileSync(source, path.join(folder, ORIGINAL));
    const files = source.replace(/\.bas$/i, ".files");
    if (e.files && fs.existsSync(files)) {
        fs.cpSync(files, folder, { recursive: true, force: true });
    }
    return copy;
}

/**
 * The path of the example's copy, made when there is none. A copy the user has edited is kept as it is. One that
 * is as it was written follows the shipped example, so that an update of the extension shows the new text.
 */
export function ensureExample(index: HelpIndex, name: string, storage: string): string | undefined {
    const e = index.example(name);
    if (!e) {
        return undefined;
    }
    const source = index.exampleFile(e);
    const folder = exampleFolder(storage, name);
    const copy = path.join(folder, path.basename(source));
    const now = read(copy);
    if (now === undefined) {
        return write(index, name, storage);
    }
    const shipped = read(source);
    if (shipped && now.equals(shipped)) {
        return copy;
    }
    const original = read(path.join(folder, ORIGINAL));
    return original && now.equals(original) ? write(index, name, storage) : copy;
}

/** "Reset This Example": the copy as shipped again, whatever the user changed. */
export function resetExample(index: HelpIndex, name: string, storage: string): string | undefined {
    return write(index, name, storage);
}
