// A build with the new compiler (design D4 of extension-vsix). Only this file and its parser know qb64rust's build
// arguments and output; QB64pe's are in qb64pe.ts.
import { execFile } from "child_process";
import * as fs from "fs";
import * as path from "path";
import { executableFor } from "./qb64pe";
import { Qb64rustOutput, parseQb64rustOutput } from "./parseQb64rustOutput";
import { RunQueue, RunResult } from "./runQueue";
import { FormatSettings, formatDefaultFlags, rewriteFlags } from "../server/protocol";

/** What a build is given besides the source (spec compiler/cli, "Locating the reference runtime"; spec
 * compiler/toolchain). */
export interface Roots {
    /** The C++ toolchain folder (change m6-toolchain); undefined for the system's on Linux (`toolchainRoot`). */
    toolchain: string | undefined;
    /** The QB64pe folder, when QB64pe is found: where `DECLARE LIBRARY` looks for files. */
    qb64pe?: string;
    /** The copy of libqb in the extension's storage; undefined for the compiler's own default (no bundle). */
    libqb?: string;
    /** Where included files are looked up after the including file's folder; empty for the compiler's default. */
    include: string;
}

/** Where a build of a program puts it and the folder it runs in (change build-output-folder, design D5). */
export interface ProgramPaths {
    /** The executable a build writes. */
    exe: string;
    /** The folder the program runs in: the folder it is in (change project-init, design D2). */
    runIn: string;
    /** Given by `qb64rust --print-paths`: the build passes no `-o`, so the compiler puts the program there. False
     * for the paths of a compiler without `--print-paths`, or of QB64pe: next to the source, built with `-o`. */
    fromCompiler: boolean;
    /** Why `--print-paths` gave no paths, when the reason is not an older compiler that lacks it: shown in the
     * build's output, so that a build next to the source is never a surprise. */
    problem?: string;
}

/** The paths of a build with `-o` next to the source: QB64pe's, and an older qb64rust's (design D5). */
export function fallbackPaths(file: string, problem?: string): ProgramPaths {
    const paths: ProgramPaths = { exe: executableFor(file), runIn: path.dirname(file), fromCompiler: false };
    return problem ? { ...paths, problem } : paths;
}

/**
 * Why a call of `--print-paths` that gave no paths failed, or undefined for a compiler that does not know the
 * option, which is expected of a release before it. `output` is what it printed; `error` the process's failure.
 */
export function printPathsProblem(output: string, error: { message: string; killed?: boolean } | null): string | undefined {
    if (/unknown option `--print-paths`/.test(output)) {
        return undefined;
    }
    if (error?.killed) {
        return `qb64rust --print-paths did not answer within ${PRINT_PATHS_TIMEOUT_MS / 1000} s`;
    }
    const first = output.split(/\r?\n/).find((l) => l.trim() !== "");
    return first?.trim() ?? error?.message ?? "qb64rust --print-paths printed nothing";
}

/** `--print-paths <file>`, the command line that asks the compiler for the paths (spec compiler/cli, "The paths of
 * a program"). */
export function printPathsArgs(file: string): string[] {
    return ["--print-paths", file];
}

/** The paths in the one line of JSON that `--print-paths` prints, or undefined when it is not that line. */
export function parsePrintPaths(output: string): ProgramPaths | undefined {
    try {
        const json: unknown = JSON.parse(output.trim());
        if (typeof json === "object" && json !== null) {
            const { exe, run_in: runIn } = json as Record<string, unknown>;
            if (typeof exe === "string" && typeof runIn === "string" && exe && runIn) {
                return { exe, runIn, fromCompiler: true };
            }
        }
    } catch {
        // Not JSON: an older compiler's usage message or a settings file's error.
    }
    return undefined;
}

/** How long `--print-paths` may take before the fallback is used; it reads one small file. */
const PRINT_PATHS_TIMEOUT_MS = 10_000;

/** What `qb64rust init <folder> --plan` says of a folder (spec compiler/project-init, "The plan of a folder"). */
export interface InitPlan {
    folder: string;
    /** The folder's own `qb64rust.toml`, when it has one. */
    settings: string | null;
    newProject: boolean;
    /** The folder's `.bas` files, sorted. */
    programs: string[];
    /** The data files offered for the output folder `build`. */
    dataFiles: string[];
    git: boolean;
}

/** The answers "QB64: Set Up Project" passes to `qb64rust init` (change project-init, design D8). */
export interface InitAnswers {
    /** Set only when the folder has several programs. */
    main?: string;
    /** `.` for beside the source, or `build`. */
    output: string;
    /** The data files to copy; empty for none. Ignored with the output `.`. */
    copy: string[];
}

/** `init <folder> --plan`: the command line that asks the compiler what to ask. */
export function initPlanArgs(folder: string): string[] {
    return ["init", folder, "--plan"];
}

/** The plan in the one line of JSON `init --plan` prints, or undefined when it is not that line, as from a
 * `qb64rust` without `init`, which reads `init` as a file to build. */
export function parseInitPlan(output: string): InitPlan | undefined {
    try {
        const json = JSON.parse(output.trim()) as Record<string, unknown>;
        const strings = (v: unknown) => (Array.isArray(v) && v.every((s) => typeof s === "string") ? (v as string[]) : undefined);
        const programs = strings(json.programs);
        const dataFiles = strings(json.data_files);
        if (typeof json.folder === "string" && programs && dataFiles && typeof json.new_project === "boolean") {
            const settings = typeof json.settings === "string" ? json.settings : null;
            return { folder: json.folder, settings, newProject: json.new_project, programs, dataFiles, git: json.git === true };
        }
    } catch {
        // Not JSON: an older compiler's message.
    }
    return undefined;
}

/** `init <folder> --yes` with one option per answer: the command line that writes the file. */
export function initWriteArgs(folder: string, answers: InitAnswers): string[] {
    const args = ["init", folder, "--yes", "--output", answers.output];
    if (answers.main) {
        args.push("--main", answers.main);
    }
    if (answers.output !== ".") {
        if (answers.copy.length === 0) {
            args.push("--no-copy");
        }
        for (const entry of answers.copy) {
            args.push("--copy", entry);
        }
    }
    return args;
}

/** Runs `binary` with `args` in no particular folder and gives its standard output, or the error with what it
 * printed. */
function runInit(binary: string, args: string[]): Promise<{ ok: boolean; output: string }> {
    return new Promise((resolve) => {
        execFile(binary, args, { timeout: PRINT_PATHS_TIMEOUT_MS, windowsHide: true }, (error, stdout, stderr) => {
            resolve({ ok: !error, output: `${String(stdout)}${String(stderr)}` });
        });
    });
}

/** The plan of `folder`, or undefined from a `qb64rust` without `init` or one that failed. */
export async function initPlan(binary: string, folder: string): Promise<InitPlan | undefined> {
    const { ok, output } = await runInit(binary, initPlanArgs(folder));
    return ok ? parseInitPlan(output) : undefined;
}

/** Writes the folder's settings file; the error is the compiler's message. */
export async function initWrite(binary: string, folder: string, answers: InitAnswers): Promise<{ ok: true } | { error: string }> {
    const { ok, output } = await runInit(binary, initWriteArgs(folder, answers));
    return ok ? { ok: true } : { error: output.trim().replace(/^qb64rust: /, "") || "qb64rust init failed" };
}

/**
 * The paths of `file` from `qb64rust --print-paths`, or [`fallbackPaths`] when the call fails (design D5): without
 * a word for an older compiler named by `qb64rust.path` that has no `--print-paths`, with `problem` for any other
 * failure, such as a settings file that cannot be used or a compiler that does not answer.
 */
export function programPaths(binary: string, file: string): Promise<ProgramPaths> {
    return new Promise((resolve) => {
        execFile(binary, printPathsArgs(file), { cwd: path.dirname(file), timeout: PRINT_PATHS_TIMEOUT_MS, windowsHide: true }, (error, stdout, stderr) => {
            const output = `${String(stdout)}${String(stderr)}`;
            const paths = error ? undefined : parsePrintPaths(String(stdout));
            resolve(paths ?? fallbackPaths(file, printPathsProblem(output, error)));
        });
    });
}

/**
 * The program Run starts and the folder it starts in, given the compiler's paths and those next to the source
 * (spec editor/build-run, "Run commands"): the newer executable of the two, so that a program built with QB64pe
 * ("Build with QB64pe"), by an older compiler or with `-o` from a terminal is run, not an older build in the output
 * folder. `modified` is the time of a file, undefined when there is none.
 */
export function newerProgram(compiler: ProgramPaths, beside: ProgramPaths, modified: (file: string) => number | undefined): ProgramPaths {
    const a = modified(compiler.exe);
    const b = modified(beside.exe);
    return b !== undefined && (a === undefined || b > a) ? beside : compiler;
}

/**
 * `-x -w <file>` and the roots, with `--debug` first for a debug build (change m5-debug-launch, design D4):
 * `--toolchain-root` but for the system's toolchain, `--qb64pe-root` only when QB64pe is found (change m6-toolchain,
 * design D6). `-o <exe>` next to the source unless `paths` came from the compiler, which then puts the program in
 * its output folder and copies the project's data files there (change build-output-folder, design D5). The `-f:`
 * settings `flags` (`config.buildFlags`) before the roots. `-c` (QB64pe's console switch) is not one of qb64rust's
 * options.
 */
export function buildArgs(file: string, roots: Roots, debug = false, paths: ProgramPaths = fallbackPaths(file), flags: string[] = []): string[] {
    const output = paths.fromCompiler ? [] : ["-o", paths.exe];
    return [...(debug ? ["--debug"] : []), "-x", "-w", file, ...output, ...flags, ...rootArgs(roots)];
}

/** The roots as options, as a build passes them (`qb64rust test` takes the same, change basic-unit-tests). */
export function rootArgs(roots: Roots): string[] {
    const args: string[] = [];
    if (roots.toolchain) {
        args.push("--toolchain-root", roots.toolchain);
    }
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

/** `-y -q`, the layout settings as `--format-default` pairs (below the project's `qb64rust.toml`), `<source> -o
 * <output>` and the include root (spec compiler/format, "Format command"; change m5b-formatter, design D7; change
 * project-settings, design D6). For Format Selection (`selection`) both rewrites are turned off with `-f:`, which goes
 * over the settings file, so that each line stays one line (change format-statement-lines, design D8). */
export function formatArgs(source: string, output: string, settings: FormatSettings, includeRoot: string, selection = false): string[] {
    const off = selection ? rewriteFlags({ oneStatementPerLine: false, blockIf: false }) : [];
    const args = ["-y", "-q", ...formatDefaultFlags(settings), ...off, source, "-o", output];
    if (includeRoot) {
        args.push("--include-root", includeRoot);
    }
    return args;
}

export type Qb64rustRun = RunResult & Qb64rustOutput;

/** Formats `source` into `output`, includes relative to `source`'s folder and then the include root (empty for the
 * compiler's default); for Format Selection without the rewrites (`formatArgs`). */
export async function format(
    queue: RunQueue,
    binary: string,
    source: string,
    output: string,
    settings: FormatSettings,
    includeRoot: string,
    timeoutMs: number,
    selection = false,
): Promise<Qb64rustRun> {
    const args = formatArgs(source, output, settings, includeRoot, selection);
    const result = await queue.run({ kind: "format", command: binary, args, cwd: path.dirname(source), timeoutMs });
    const parsed = result.cancelled ? { messages: [], allMarked: false } : parseQb64rustOutput(result.output, result.exitCode, source);
    if (result.timedOut) {
        parsed.messages.push({ severity: "error", file: undefined, line: 1, message: "The compiler did not finish in time (qb64rust.checkTimeoutSeconds) and was stopped." });
    }
    return { ...result, ...parsed };
}

export interface Qb64rustBuild extends RunResult, Qb64rustOutput {
    executable: string;
}

/** Full build into `paths.exe`, a debug build when `debug`, with the `-f:` settings `flags`. No timeout, as a build
 * with QB64pe. */
export async function build(
    queue: RunQueue,
    binary: string,
    file: string,
    roots: Roots,
    env: NodeJS.ProcessEnv,
    debug: boolean,
    paths: ProgramPaths,
    onOutput?: (text: string) => void,
    flags: string[] = [],
): Promise<Qb64rustBuild> {
    const executable = paths.exe;
    const result = await queue.run({ kind: "build", command: binary, args: buildArgs(file, roots, debug, paths, flags), cwd: path.dirname(file), env, onOutput });
    const parsed = result.cancelled ? { messages: [], allMarked: false } : parseQb64rustOutput(result.output, result.exitCode, file);
    if (!result.cancelled && result.exitCode === 0 && !fs.existsSync(executable)) {
        parsed.messages.push({ severity: "error", file: undefined, line: 1, message: `qb64rust reported success but ${executable} does not exist; see the "QB64" output.` });
    }
    return { ...result, ...parsed, executable };
}
