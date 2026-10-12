// The extension's part of the language server's protocol (design D3 and D6 of m2-language-server): the
// initialization options and the encoding notification. No VS Code import, so it is unit-tested without it.
import * as path from "path";

/** The notification that tells the server a document's encoding (VS Code's encoding id). */
export const DOCUMENT_ENCODING = "qb64rust/documentEncoding";

export interface DocumentEncodingParams {
    uri: string;
    encoding: string;
}

export interface InitializationOptions {
    /** The encoding of documents the extension says nothing about, and of included files read from disk. */
    encoding: string;
    /** Where included files are looked up after the including file's folder; empty for the server's default. */
    includeRoot: string;
}

/**
 * The options sent with `initialize`: the `[qb64rust]` `files.encoding` (cp437 unless changed) and
 * `qb64rust.includeRoot`, a relative one resolved against the workspace folder as `qb64rust.path` is. Without a
 * workspace folder a relative one means nothing, so the server's default is used: the server would resolve it
 * against its working directory, which is whatever VS Code's is.
 */
export function initializationOptions(
    defaultEncoding: string | undefined,
    includeRoot: string,
    baseDir: string | undefined,
    platform: NodeJS.Platform,
): InitializationOptions {
    const p = platform === "win32" ? path.win32 : path.posix;
    let root = includeRoot.trim();
    if (root && !p.isAbsolute(root)) {
        root = baseDir ? p.resolve(baseDir, root) : "";
    }
    return { encoding: defaultEncoding || "cp437", includeRoot: root };
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
