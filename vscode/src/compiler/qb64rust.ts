// A build with the new compiler (design D4 of extension-vsix). Only this file and its parser know qb64rust's build
// arguments and output; QB64pe's are in qb64pe.ts.
import * as fs from "fs";
import * as path from "path";
import { executableFor } from "./qb64pe";
import { Qb64rustOutput, parseQb64rustOutput } from "./parseQb64rustOutput";
import { RunQueue, RunResult } from "./runQueue";
import { FormatSettings, formatFlags } from "../server/protocol";

/** What a build is given besides the source (spec compiler/cli, "Locating the reference runtime"; spec
 * compiler/toolchain). */
export interface Roots {
    /** The C++ toolchain folder (change m6-toolchain). */
    toolchain: string;
    /** The QB64pe folder, when QB64pe is found: where `DECLARE LIBRARY` looks for files. */
    qb64pe?: string;
    /** The copy of libqb in the extension's storage; undefined for the compiler's own default (no bundle). */
    libqb?: string;
    /** Where included files are looked up after the including file's folder; empty for the compiler's default. */
    include: string;
}

/**
 * `-x -w <file> -o <exe>` and the roots, with `--debug` first for a debug build (change m5-debug-launch, design D4):
 * `--toolchain-root` always, `--qb64pe-root` only when QB64pe is found (change m6-toolchain, design D6). `-c`
 * (QB64pe's console switch) is not one of qb64rust's options.
 */
export function buildArgs(file: string, roots: Roots, debug = false): string[] {
    const args = [...(debug ? ["--debug"] : []), "-x", "-w", file, "-o", executableFor(file), "--toolchain-root", roots.toolchain];
    if (roots.qb64pe) {
        args.push("--qb64pe-root", roots.qb64pe);
    }
    if (roots.libqb) {
        args.push("--libqb-root", roots.libqb);
    }
    if (roots.include) {
        args.push("--include-root", roots.include);
    }
    return args;
}

/** `-y -q`, the layout settings, `<source> -o <output>` and the include root (spec compiler/format, "Format
 * command"; change m5b-formatter, design D7). */
export function formatArgs(source: string, output: string, settings: FormatSettings, includeRoot: string): string[] {
    const args = ["-y", "-q", ...formatFlags(settings), source, "-o", output];
    if (includeRoot) {
        args.push("--include-root", includeRoot);
    }
    return args;
}

export type Qb64rustRun = RunResult & Qb64rustOutput;

/** Formats `source` into `output`, includes relative to `source`'s folder and then the include root (empty for the
 * compiler's default). */
export async function format(
    queue: RunQueue,
    binary: string,
    source: string,
    output: string,
    settings: FormatSettings,
    includeRoot: string,
    timeoutMs: number,
): Promise<Qb64rustRun> {
    const result = await queue.run({ kind: "format", command: binary, args: formatArgs(source, output, settings, includeRoot), cwd: path.dirname(source), timeoutMs });
    const parsed = result.cancelled ? { messages: [], allMarked: false } : parseQb64rustOutput(result.output, result.exitCode, source);
    if (result.timedOut) {
        parsed.messages.push({ severity: "error", file: undefined, line: 1, message: "The compiler did not finish in time (qb64rust.checkTimeoutSeconds) and was stopped." });
    }
    return { ...result, ...parsed };
}

export interface Qb64rustBuild extends RunResult, Qb64rustOutput {
    executable: string;
}

/** Full build into the executable next to the source, a debug build when `debug`. No timeout, as a build with QB64pe. */
export async function build(
    queue: RunQueue,
    binary: string,
    file: string,
    roots: Roots,
    env: NodeJS.ProcessEnv,
    debug: boolean,
    onOutput?: (text: string) => void,
): Promise<Qb64rustBuild> {
    const executable = executableFor(file);
    const result = await queue.run({ kind: "build", command: binary, args: buildArgs(file, roots, debug), cwd: path.dirname(file), env, onOutput });
    const parsed = result.cancelled ? { messages: [], allMarked: false } : parseQb64rustOutput(result.output, result.exitCode, file);
    if (!result.cancelled && result.exitCode === 0 && !fs.existsSync(executable)) {
        parsed.messages.push({ severity: "error", file: undefined, line: 1, message: `qb64rust reported success but ${executable} does not exist; see the "QB64" output.` });
    }
    return { ...result, ...parsed, executable };
}
