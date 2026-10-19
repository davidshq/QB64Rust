// Turns the new compiler's console output into messages (design D4 of extension-vsix). The format is spec
// compiler/cli, "Diagnostics output": `<file>:<line>:<column>: error|warning|note: <message>`, then a summary line.
// Pure: no `vscode` import and no file access, so it is unit-tested against recorded outputs.
import * as path from "path";
import { CompilerMessage } from "./parseQb64peOutput";

export interface Qb64rustOutput {
    messages: CompilerMessage[];
    /** True when there is at least one error and every error is marked "not supported yet". */
    allMarked: boolean;
}

// The file may hold a drive's colon (`<drive>:\…\x.bas:2:6: error: …`); the line and column are the last two
// numbers.
const DIAGNOSTIC = /^(.+):(\d+):(\d+): (error|warning|note): (.*)$/;
export const MARKED = "not supported yet:";

/**
 * `mainFile` is the absolute path the compiler was given. A file name in the output that is relative is taken from
 * the main file's folder (the compiler's working directory); the main file itself is reported as `undefined`, as
 * `CompilerMessage` asks. A failure with no diagnostic (a C++ build that failed, an internal compiler error) gives
 * one error on line 1 with the last lines of the output.
 */
export function parseQb64rustOutput(output: string, exitCode: number, mainFile: string): Qb64rustOutput {
    const p = /^[A-Za-z]:[\\/]/.test(mainFile) ? path.win32 : path.posix;
    const same = (a: string, b: string) => (p === path.win32 ? a.toLowerCase() === b.toLowerCase() : a === b);
    const main = p.normalize(mainFile);
    const messages: CompilerMessage[] = [];
    const lines = output.split(/\r?\n/).map((l) => l.trimEnd());
    for (const line of lines) {
        const m = DIAGNOSTIC.exec(line);
        if (!m) {
            continue;
        }
        const [, name, lineText, columnText, kind, text] = m;
        if (kind === "note") {
            const last = messages[messages.length - 1];
            if (last) {
                last.message += `\n${text}`;
            }
            continue;
        }
        const file = p.normalize(p.isAbsolute(name) ? name : p.join(p.dirname(main), name));
        messages.push({
            severity: kind === "error" ? "error" : "warning",
            file: same(file, main) ? undefined : file,
            line: Number(lineText),
            column: Number(columnText),
            message: text,
        });
    }
    const errors = messages.filter((m) => m.severity === "error");
    if (exitCode !== 0 && errors.length === 0) {
        const tail = lines.filter((l) => l.trim() !== "").slice(-3).join("\n");
        const error: CompilerMessage = {
            severity: "error",
            file: undefined,
            line: 1,
            message: `The build failed (exit code ${exitCode}); see the "QB64" output.${tail ? `\n${tail}` : ""}`,
        };
        messages.push(error);
        errors.push(error);
    }
    return { messages, allMarked: errors.length > 0 && errors.every((e) => e.message.startsWith(MARKED)) };
}
