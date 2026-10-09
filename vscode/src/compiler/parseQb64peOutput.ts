// Turns the old compiler's console output into messages (design D2). Pure: no `vscode` import, no file access
// except through the `exists` callback, so it can be unit-tested against recorded outputs.
import * as path from "path";

export interface CompilerMessage {
    severity: "error" | "warning";
    /** Absolute path, or undefined for the file that was compiled. */
    file: string | undefined;
    /** 1-based. */
    line: number;
    message: string;
}

export interface ParseOptions {
    /** Absolute path of the compiled file. */
    mainFile: string;
    /** Used to resolve the bare file names in warning lines. Without it, warnings go to the main file. */
    exists?: (file: string) => boolean;
}

// `file:line: warning:` from the old compiler; `file:line:column: warning:` from qb64rust (column ignored).
const WARNING = /^(.+?):(\d+)(?::\d+)?: warning: (.*)$/;
const LINE = /^LINE (\d+):/;
const CAUSED_BY = /^Caused by \(or after\):/;
const INCLUDE_SUFFIX = / in line (\d+) of (.+) included$/;
export const PROGRESS = /^\[[. ]*\]\s*\d+%$/;

/** Splits output into meaningful lines: no blank lines, no control bytes, no progress bar. */
function cleanLines(output: string): string[] {
    const lines: string[] = [];
    for (let line of output.split("\n")) {
        if (line.endsWith("\r")) {
            line = line.slice(0, -1);
        }
        if (line.includes("\r")) {
            continue; // progress bar redrawn with \r
        }
        // eslint-disable-next-line no-control-regex
        line = line.replace(/[\x00-\x08\x0b-\x1f\x7f]/g, "");
        if (line.trim() === "" || PROGRESS.test(line.trim())) {
            continue;
        }
        lines.push(line);
    }
    return lines;
}

export function parseQb64peOutput(output: string, exitCode: number, options: ParseOptions): CompilerMessage[] {
    const lines = cleanLines(output);
    const messages: CompilerMessage[] = [];
    const includeDirs: string[] = [];
    const warnings: { name: string; line: number; message: string }[] = [];
    let block: string[] = [];

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const warning = WARNING.exec(line);
        if (warning) {
            let message = warning[3].trim();
            const next = lines[i + 1];
            if (next !== undefined && /^\s/.test(next) && !WARNING.test(next)) {
                message += ": " + next.trim().replace(/\s+/g, " ");
                i++;
            }
            warnings.push({ name: warning[1], line: Number(warning[2]), message });
            continue;
        }
        const lineMatch = LINE.exec(line);
        if (!lineMatch) {
            block.push(line);
            continue;
        }
        const caused = block.findIndex((l) => CAUSED_BY.test(l));
        const before = caused >= 0 ? block.slice(0, caused) : block;
        let message = (before.length > 0 ? before[before.length - 1] : "").trim();
        let file: string | undefined;
        let lineNo = Number(lineMatch[1]);
        const include = INCLUDE_SUFFIX.exec(message);
        if (include) {
            message = message.slice(0, include.index).trim();
            file = path.normalize(include[2]);
            lineNo = Number(include[1]);
            includeDirs.push(path.dirname(file));
        }
        messages.push({ severity: "error", file, line: Math.max(1, lineNo), message: message || "Compilation failed" });
        block = [];
    }

    for (const w of warnings) {
        messages.push({ severity: "warning", file: resolveWarningFile(w.name, includeDirs, options), line: Math.max(1, w.line), message: w.message });
    }

    // Warnings don't count: a failed C++ step after `-w` warnings must still say why it failed.
    if (exitCode !== 0 && !messages.some((m) => m.severity === "error")) {
        // C++ step failed: "ERROR: C++ compilation failed." then "Check ...compilelog.txt for details."
        const err = lines.findIndex((l) => /^ERROR: /.test(l.trim()));
        const text = err >= 0 ? lines.slice(err, err + 2).map((l) => l.trim()).join(" ") : lines.map((l) => l.trim()).join("\n");
        messages.push({ severity: "error", file: undefined, line: 1, message: text || `The compiler failed (exit code ${exitCode}) without a message.` });
    }
    return messages;
}

/** Warning lines carry only a file name; find which file it is (design D2 step 2). */
function resolveWarningFile(name: string, includeDirs: string[], options: ParseOptions): string | undefined {
    const main = options.mainFile;
    if (path.isAbsolute(name)) {
        return samePath(name, main) ? undefined : path.normalize(name);
    }
    const mainDir = path.dirname(main);
    const candidate = path.join(mainDir, name);
    if (samePath(candidate, main)) {
        return undefined;
    }
    if (options.exists) {
        for (const dir of [mainDir, ...includeDirs]) {
            const p = path.join(dir, name);
            if (options.exists(p)) {
                return samePath(p, main) ? undefined : p;
            }
        }
    }
    return undefined;
}

function samePath(a: string, b: string): boolean {
    const na = path.normalize(a);
    const nb = path.normalize(b);
    return process.platform === "win32" ? na.toLowerCase() === nb.toLowerCase() : na === nb;
}
