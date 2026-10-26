// Calls to the old QB64pe compiler. Only this file and the parser know its arguments and output format
// (design goals). Plain functions on purpose: in M2 the language server replaces check and format outright.
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import { CompilerMessage, parseQb64peOutput } from "./parseQb64peOutput";
import { RunQueue, RunResult } from "./runQueue";
import { FormatSettings, formatFlags } from "../server/protocol";

export interface CompilerRun extends RunResult {
    messages: CompilerMessage[];
}

let sessionDir: string | undefined;

/** Per-session temp folder for `-o` targets of check and format (design D3, D5). */
export function sessionTempDir(): string {
    if (!sessionDir || !fs.existsSync(sessionDir)) {
        sessionDir = fs.mkdtempSync(path.join(os.tmpdir(), "qb64rust-"));
    }
    return sessionDir;
}

export function removeSessionTempDir(): void {
    if (sessionDir) {
        try {
            fs.rmSync(sessionDir, { recursive: true, force: true });
        } catch {
            // A compiler run still holds a file open on shutdown (Windows); the OS temp cleanup gets it later.
        }
        sessionDir = undefined;
    }
}

/** The executable a build of `source` produces: same folder, same base name. */
export function executableFor(source: string): string {
    const parsed = path.parse(source);
    return path.join(parsed.dir, parsed.name + (process.platform === "win32" ? ".exe" : ""));
}

function withMessages(result: RunResult, mainFile: string): CompilerRun {
    const messages = result.cancelled ? [] : parseQb64peOutput(result.output, result.exitCode, { mainFile, exists: fs.existsSync });
    if (result.timedOut) {
        messages.push({ severity: "error", file: undefined, line: 1, message: "The compiler did not finish in time (qb64rust.checkTimeoutSeconds) and was stopped." });
    }
    return { ...result, messages };
}

/** Generates C++ only, no C++ build, no executable: `-z -q -w` (design D3). */
export async function check(queue: RunQueue, compiler: string, file: string, timeoutMs: number): Promise<CompilerRun> {
    const target = path.join(sessionTempDir(), "check.exe");
    const result = await queue.run({
        kind: "check",
        key: path.normalize(file).toLowerCase(),
        command: compiler,
        args: ["-z", "-q", "-w", file, "-o", target],
        cwd: path.dirname(file),
        timeoutMs,
    });
    return withMessages(result, file);
}

/** `-y -q`, the layout settings as `-f:` settings (so that `config.ini` plays no part), `<source> -o <output>`. */
export function formatArgs(source: string, output: string, settings: FormatSettings): string[] {
    return ["-y", "-q", ...formatFlags(settings), source, "-o", output];
}

/** Formats `source` into `output` with `-y -q` and the settings. Includes resolve relative to `source`'s folder. */
export async function format(queue: RunQueue, compiler: string, source: string, output: string, settings: FormatSettings, timeoutMs: number): Promise<CompilerRun> {
    const result = await queue.run({
        kind: "format",
        command: compiler,
        args: formatArgs(source, output, settings),
        cwd: path.dirname(source),
        timeoutMs,
    });
    return withMessages(result, source);
}

/** Full build with `-c -x -w` into the executable next to the source (design D6). No timeout. */
export async function build(queue: RunQueue, compiler: string, file: string, onOutput?: (text: string) => void): Promise<CompilerRun & { executable: string }> {
    const executable = executableFor(file);
    const result = await queue.run({
        kind: "build",
        command: compiler,
        args: ["-c", "-x", "-w", file, "-o", executable],
        cwd: path.dirname(file),
        onOutput,
    });
    const run = withMessages(result, file);
    if (!run.cancelled && run.exitCode === 0 && !fs.existsSync(executable) && !run.messages.some((m) => m.severity === "error")) {
        // The C++ step failed without a BASIC-level error (design non-goal: no mapping back to BASIC lines).
        const log = path.join(path.dirname(compiler), "internal", "temp", "compilelog.txt");
        run.messages.push({ severity: "error", file: undefined, line: 1, message: `The C++ build failed; see the "QB64" output and ${log}.` });
    }
    return { ...run, executable };
}

/** The command line `build` uses, for tasks that run it in a terminal. */
export function buildArgs(file: string): string[] {
    return ["-c", "-x", "-w", file, "-o", executableFor(file)];
}
