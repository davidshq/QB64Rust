// Tasks of type `qb64rust` (`action: build | run`, optional `file`), so build and run can be bound in tasks.json.
import { ChildProcess, spawn } from "child_process";
import * as path from "path";
import * as vscode from "vscode";
import * as config from "./config";
import { Bundle, chosenCompiler, planBuild } from "./buildPlan";
import * as qb64pe from "./compiler/qb64pe";
import { killTree } from "./compiler/runQueue";
import { LanguageServer } from "./languageServer";
import { CompilerStatus } from "./statusBar";

export const TASK_TYPE = "qb64rust";

interface Qb64TaskDefinition extends vscode.TaskDefinition {
    action: "build" | "run";
    file?: string;
}

export class Qb64TaskProvider implements vscode.TaskProvider {
    constructor(
        private readonly status: CompilerStatus,
        private readonly server: LanguageServer,
        private readonly bundle: Bundle,
    ) {}

    async provideTasks(): Promise<vscode.Task[]> {
        if (!vscode.workspace.isTrusted) {
            return [];
        }
        const tasks: vscode.Task[] = [];
        for (const action of ["build", "run"] as const) {
            const task = await this.makeTask({ type: TASK_TYPE, action }, vscode.TaskScope.Workspace);
            if (task) {
                tasks.push(task);
            }
        }
        return tasks;
    }

    async resolveTask(task: vscode.Task): Promise<vscode.Task | undefined> {
        if (!vscode.workspace.isTrusted) {
            return undefined;
        }
        const def = task.definition as Qb64TaskDefinition;
        if (def.action !== "build" && def.action !== "run") {
            return undefined;
        }
        return this.makeTask(def, task.scope ?? vscode.TaskScope.Workspace);
    }

    private async makeTask(def: Qb64TaskDefinition, scope: vscode.TaskScope | vscode.WorkspaceFolder): Promise<vscode.Task | undefined> {
        const file = this.targetFile(def, scope);
        const name = def.action;
        if (!file) {
            // No file yet: the task still shows in the list and explains itself when run.
            return new vscode.Task(def, scope, name, TASK_TYPE, new vscode.ShellExecution("echo Open a QB64 program first, or set \"file\" in tasks.json."));
        }
        const cwd = path.dirname(file);
        if (def.action === "build") {
            // The Build command's compiler and command line (spec editor/build-run, "Tasks"). QB64pe chosen and not
            // found: `qb64pe` by name, whose failure in the terminal says what is wrong. No C++ toolchain for
            // qb64rust: the task is listed and offers it when it runs (design D6 of m6-toolchain). When qb64rust
            // would build but cannot otherwise (not found, libqb not prepared), the task fails with the reason, never
            // building with QB64pe.
            const plan = await planBuild(file, this.status, this.server, this.bundle, false);
            const execution =
                plan.compiler !== "none"
                    ? new vscode.ProcessExecution(plan.command, plan.args, { cwd, env: plan.compiler === "qb64rust" ? lockEnv(plan.env) : undefined })
                    : plan.toolchain
                      ? this.offering(file, cwd)
                      : chosenCompiler(this.server, vscode.Uri.file(file)) === "qb64pe" && !this.status.current.found
                        ? new vscode.ProcessExecution("qb64pe", qb64pe.buildArgs(file), { cwd })
                        : failing(plan.reason);
            const task = new vscode.Task(def, scope, name, TASK_TYPE, execution);
            task.group = vscode.TaskGroup.Build;
            return task;
        }
        const uri = vscode.Uri.file(file);
        const env = config.noPrompt(uri) ? { QB64PE_NOPROMPT: "y" } : undefined;
        return new vscode.Task(def, scope, name, TASK_TYPE, new vscode.ProcessExecution(qb64pe.executableFor(file), splitArguments(config.runArguments(uri)), { cwd, env }));
    }

    private targetFile(def: Qb64TaskDefinition, scope: vscode.TaskScope | vscode.WorkspaceFolder): string | undefined {
        if (def.file) {
            if (path.isAbsolute(def.file)) {
                return def.file;
            }
            const folder = typeof scope === "object" ? scope : vscode.workspace.workspaceFolders?.[0];
            return folder?.uri.scheme === "file" ? path.join(folder.uri.fsPath, def.file) : undefined;
        }
        const doc = vscode.window.activeTextEditor?.document;
        return doc && doc.languageId === "qb64rust" && doc.uri.scheme === "file" && !doc.isUntitled ? doc.uri.fsPath : undefined;
    }

    /** A build task that found no C++ toolchain when it was listed: when it runs, it plans again as the Build command
     * does, which offers the toolchain, and builds with qb64rust in its terminal once there is one. */
    private offering(file: string, cwd: string): vscode.CustomExecution {
        return new vscode.CustomExecution(async () => {
            const write = new vscode.EventEmitter<string>();
            const close = new vscode.EventEmitter<number>();
            let child: ChildProcess | undefined;
            const show = (text: string) => write.fire(text.replace(/\r?\n/g, "\r\n"));
            return {
                onDidWrite: write.event,
                onDidClose: close.event,
                open: () => {
                    void (async () => {
                        const plan = await planBuild(file, this.status, this.server, this.bundle, true);
                        if (plan.compiler === "none") {
                            show(`QB64: ${plan.reason}\n`);
                            close.fire(1);
                            return;
                        }
                        show(`> ${plan.command} ${plan.args.join(" ")}\n`);
                        child = spawn(plan.command, plan.args, { cwd, env: plan.compiler === "qb64rust" ? plan.env : process.env, windowsHide: true });
                        child.stdout?.on("data", (d: Buffer) => show(d.toString("latin1")));
                        child.stderr?.on("data", (d: Buffer) => show(d.toString("latin1")));
                        // A process that does not start gives "error" and then "close": the task ends once.
                        let done = false;
                        const finish = (code: number) => {
                            if (!done) {
                                done = true;
                                close.fire(code);
                            }
                        };
                        child.on("error", (e) => {
                            show(`${e.message}\n`);
                            finish(1);
                        });
                        child.on("close", (code) => finish(code ?? 1));
                    })();
                },
                close: () => killTree(child),
            };
        });
    }
}

/** A task that prints why it cannot build and ends with exit code 1. */
function failing(reason: string): vscode.CustomExecution {
    return new vscode.CustomExecution(async () => {
        const write = new vscode.EventEmitter<string>();
        const close = new vscode.EventEmitter<number>();
        return {
            onDidWrite: write.event,
            onDidClose: close.event,
            open: () => {
                write.fire(`QB64: ${reason}\r\n`);
                close.fire(1);
            },
            close: () => undefined,
        };
    });
}

/** The variables a task adds to the terminal's environment: the objects' lock, when there is one. */
function lockEnv(env: NodeJS.ProcessEnv): Record<string, string> | undefined {
    const lock = env.QB64RUST_OBJECTS_LOCK;
    return lock ? { QB64RUST_OBJECTS_LOCK: lock } : undefined;
}

/** Splits `qb64rust.runArguments` for a process start: whitespace separates, double quotes group. */
export function splitArguments(text: string): string[] {
    const args: string[] = [];
    const re = /"([^"]*)"|(\S+)/g;
    let m: RegExpExecArray | null;
    while ((m = re.exec(text)) !== null) {
        args.push(m[1] ?? m[2]);
    }
    return args;
}
