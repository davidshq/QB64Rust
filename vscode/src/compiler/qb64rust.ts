// A build with the new compiler (design D4 of extension-vsix). Only this file and its parser know qb64rust's build
// arguments and output; QB64pe's are in qb64pe.ts.
import * as fs from "fs";
import * as path from "path";
import { executableFor } from "./qb64pe";
import { Qb64rustOutput, parseQb64rustOutput } from "./parseQb64rustOutput";
import { RunQueue, RunResult } from "./runQueue";

/** What a build is given besides the source (spec compiler/cli, "Locating the reference runtime"). */
export interface Roots {
    /** The QB64pe folder: its C++ toolchain, and where `DECLARE LIBRARY` looks for files. */
    qb64pe: string;
    /** The copy of libqb in the extension's storage; undefined for the compiler's own default (no bundle). */
    libqb?: string;
    /** Where included files are looked up after the including file's folder. */
    include: string;
}

/**
 * `-x -w <file> -o <exe>` and the roots, with `--debug` first for a debug build (change m5-debug-launch, design D4).
 * `-c` (QB64pe's console switch) is not one of qb64rust's options.
 */
export function buildArgs(file: string, roots: Roots, debug = false): string[] {
    const args = [...(debug ? ["--debug"] : []), "-x", "-w", file, "-o", executableFor(file), "--qb64pe-root", roots.qb64pe];
    if (roots.libqb) {
        args.push("--libqb-root", roots.libqb);
    }
    args.push("--include-root", roots.include);
    return args;
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
