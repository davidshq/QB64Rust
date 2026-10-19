// The extension's part of the language server's protocol (design D3 and D6 of m2-language-server, D5 of
// extension-diagnostics): the initialization options, the encoding notification and the check request. No VS Code
// import, so it is unit-tested without it.
import * as path from "path";

/** The notification that tells the server a document's encoding (VS Code's encoding id). */
export const DOCUMENT_ENCODING = "qb64rust/documentEncoding";

export interface DocumentEncodingParams {
    uri: string;
    encoding: string;
}

/** The request that runs the new compiler's whole front end over a document's program and publishes what it finds
 * (spec editor/language-server, "Check request"). */
export const CHECK = "qb64rust/check";

export interface CheckParams {
    /** The document as the client names it to the server (its `didOpen` uri). */
    uri: string;
}

/** The answer: how many errors and warnings the check published. */
export interface CheckResult {
    errors: number;
    warnings: number;
}

export function checkParams(uri: string): CheckParams {
    return { uri };
}

export interface InitializationOptions {
    /** The encoding of documents the extension says nothing about, and of included files read from disk. */
    encoding: string;
    /** Where included files are looked up after the including file's folder; empty for the server's default. */
    includeRoot: string;
}

/**
 * The include root the server and a build with `qb64rust` are given (design D6 of extension-vsix, spec
 * editor/language-server "Included files"): `qb64rust.includeRoot`, a relative one resolved against the workspace
 * folder as `qb64rust.path` is; else the QB64pe folder, where QB64pe looks after the including file's folder. Empty
 * when neither applies, for the default of `qb64rust`, the folder of its binary. Without a workspace folder a
 * relative setting means nothing: the server would resolve it against its working directory, which is whatever
 * VS Code's is.
 */
export function includeRootFor(setting: string, baseDir: string | undefined, qb64peFolder: string | undefined, platform: NodeJS.Platform): string {
    const p = platform === "win32" ? path.win32 : path.posix;
    let root = setting.trim();
    if (root && !p.isAbsolute(root)) {
        root = baseDir ? p.resolve(baseDir, root) : "";
    }
    return root || qb64peFolder || "";
}

/** The options sent with `initialize`: the `[qb64rust]` `files.encoding` (cp437 unless changed) and the include root. */
export function initializationOptions(
    defaultEncoding: string | undefined,
    includeRoot: string,
    baseDir: string | undefined,
    platform: NodeJS.Platform,
    qb64peFolder?: string,
): InitializationOptions {
    return { encoding: defaultEncoding || "cp437", includeRoot: includeRootFor(includeRoot, baseDir, qb64peFolder, platform) };
}

/** Remembers the encoding the server was told for each document, so each change is told once. */
export class EncodingTracker {
    private readonly told = new Map<string, string>();

    /** The notification to send for this document now, or undefined when the server knows its encoding already. */
    update(uri: string, encoding: string): DocumentEncodingParams | undefined {
        if (this.told.get(uri) === encoding) {
            return undefined;
        }
        this.told.set(uri, encoding);
        return { uri, encoding };
    }

    forget(uri: string): void {
        this.told.delete(uri);
    }

    clear(): void {
        this.told.clear();
    }
}
