// The extension's part of the language server's protocol (design D3 and D6 of m2-language-server, D5 of
// extension-diagnostics, D7 of m5b-formatter, D8 of include-file-program): the initialization options, the encoding
// notification, the check request, the layout settings and the program notification. No VS Code import, so it is
// unit-tested without it.
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

/** The notification that tells the client a document's program (spec editor/language-server, "Program
 * notification"; design D8 of include-file-program): sent when a document opens and each time its program changes. */
export const PROGRAM = "qb64rust/program";

export interface ProgramParams {
    /** The document, as the client named it in `didOpen`. */
    uri: string;
    /** The uri of the program's main file; null when the document is its own program. */
    program: string | null;
    /** Where the program was found: the settings file's `main`, an open document, the disk, or none. */
    source: "settings" | "open" | "disk" | "none";
}

const SOURCES: Record<ProgramParams["source"], string> = {
    settings: "named by main in qb64rust.toml",
    open: "an open document that includes this file",
    disk: "a file on disk that includes this file",
    none: "",
};

/** What the program status item shows for a document (spec editor/language-server, "Program status item"): the
 * program's file name; "no program found" for an included file with none; nothing (the item is hidden) for a
 * `.bas` file that is its own program, or before the server has said. */
export function programStatus(p: ProgramParams | undefined): { text: string; detail: string } | undefined {
    if (!p) {
        return undefined;
    }
    if (p.program === null) {
        // A document that is not a file (`untitled:`) is a program of its own, as a `.bas` file is.
        if (!p.uri.startsWith("file:") || /\.bas$/i.test(p.uri)) {
            return undefined;
        }
        return { text: "no program found", detail: "No .bas file that includes this file was found; it is checked alone." };
    }
    const name = decodeURIComponent(p.program.substring(p.program.lastIndexOf("/") + 1));
    return { text: name, detail: `The program of this file: ${SOURCES[p.source]}.` };
}

/** Remembers the last program the server named for each document. */
export class ProgramTracker {
    private readonly programs = new Map<string, ProgramParams>();

    update(p: ProgramParams): void {
        this.programs.set(p.uri, p);
    }

    get(uri: string): ProgramParams | undefined {
        return this.programs.get(uri);
    }

    /** The uri of the main file of the document's program, when the server named another file; undefined when the
     * document is its own program or the server has not said. */
    mainOf(uri: string): string | undefined {
        return this.programs.get(uri)?.program ?? undefined;
    }

    forget(uri: string): void {
        this.programs.delete(uri);
    }

    clear(): void {
        this.programs.clear();
    }
}

/** The extension's layout settings (spec editor/formatting, "Layout settings"; change m5b-formatter, design D7): sent
 * to the server, and given to either compiler's `-y` as its `-f:` settings. */
export interface FormatSettings {
    autoIndent: boolean;
    /** 1 to 64. */
    indentSize: number;
    indentSubs: boolean;
    autoLayout: boolean;
    keywordCase: "camel" | "upper" | "lower";
}

/** QB64pe's built-in layout defaults, which the settings default to. */
export const DEFAULT_FORMAT: FormatSettings = { autoIndent: true, indentSize: 4, indentSubs: true, autoLayout: true, keywordCase: "camel" };

/** The `-f:` layout settings of `-y`, the same for `qb64rust` and `qb64pe` (spec compiler/format, "Layout settings"),
 * all six given, so that QB64pe's `config.ini` plays no part. */
export function formatFlags(s: FormatSettings): string[] {
    const b = (v: boolean): string => (v ? "1" : "0");
    return [
        `-f:autoindent=${b(s.autoIndent)}`,
        `-f:autoindentsize=${s.indentSize}`,
        `-f:indentsubs=${b(s.indentSubs)}`,
        `-f:autolayout=${b(s.autoLayout)}`,
        `-f:keywordcapitals=${b(s.keywordCase === "upper")}`,
        `-f:keywordlowercase=${b(s.keywordCase === "lower")}`,
    ];
}

/** The same six settings as `qb64rust -y`'s `--format-default <setting>=<value>` pairs, which rank below the
 * `[format]` table of the project's `qb64rust.toml` (spec editor/formatting, "Layout settings"; change
 * project-settings, design D6). */
export function formatDefaultFlags(s: FormatSettings): string[] {
    return formatFlags(s).flatMap((f) => ["--format-default", f.slice("-f:".length)]);
}

/** The parameters of `workspace/didChangeConfiguration` that give the server new layout settings. */
export function configurationParams(format: FormatSettings): { settings: { format: FormatSettings } } {
    return { settings: { format } };
}

export interface InitializationOptions {
    /** The encoding of documents the extension says nothing about, and of included files read from disk. */
    encoding: string;
    /** Where included files are looked up after the including file's folder; empty for the server's default. */
    includeRoot: string;
    /** The layout settings of formatting. */
    format: FormatSettings;
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

/** The options sent with `initialize`: the `[qb64rust]` `files.encoding` (cp437 unless changed), the include root and
 * the layout settings (the defaults unless given). */
export function initializationOptions(
    defaultEncoding: string | undefined,
    includeRoot: string,
    baseDir: string | undefined,
    platform: NodeJS.Platform,
    qb64peFolder?: string,
    format: FormatSettings = DEFAULT_FORMAT,
): InitializationOptions {
    return { encoding: defaultEncoding || "cp437", includeRoot: includeRootFor(includeRoot, baseDir, qb64peFolder, platform), format };
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
