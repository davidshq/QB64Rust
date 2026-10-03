// Tasks of type `qb64rust` (`action: build | run`, optional `file`), so build and run can be bound in tasks.json.
import * as path from "path";
import * as vscode from "vscode";
import * as config from "./config";
import * as qb64pe from "./compiler/qb64pe";
import { CompilerStatus } from "./statusBar";

export const TASK_TYPE = "qb64rust";

interface Qb64TaskDefinition extends vscode.TaskDefinition {
    action: "build" | "run";
    file?: string;
}

export class Qb64TaskProvider implements vscode.TaskProvider {
    constructor(private readonly status: CompilerStatus) {}

    provideTasks(): vscode.Task[] {
        if (!vscode.workspace.isTrusted) {
            return [];
        }
        return (["build", "run"] as const).flatMap((action) => {
            const task = this.makeTask({ type: TASK_TYPE, action }, vscode.TaskScope.Workspace);
            return task ? [task] : [];
        });
    }

    resolveTask(task: vscode.Task): vscode.Task | undefined {
        if (!vscode.workspace.isTrusted) {
            return undefined;
        }
        const def = task.definition as Qb64TaskDefinition;
        if (def.action !== "build" && def.action !== "run") {
            return undefined;
        }
        return this.makeTask(def, task.scope ?? vscode.TaskScope.Workspace);
    }

    private makeTask(def: Qb64TaskDefinition, scope: vscode.TaskScope | vscode.WorkspaceFolder): vscode.Task | undefined {
        const file = this.targetFile(def, scope);
        const name = def.action;
        if (!file) {
            // No file yet: the task still shows in the list and explains itself when run.
            return new vscode.Task(def, scope, name, TASK_TYPE, new vscode.ShellExecution("echo Open a QB64 program first, or set \"file\" in tasks.json."));
        }
        const cwd = path.dirname(file);
        if (def.action === "build") {
            const compiler = this.status.current.found ? this.status.current.path : "qb64pe";
            const task = new vscode.Task(def, scope, name, TASK_TYPE, new vscode.ProcessExecution(compiler, qb64pe.buildArgs(file), { cwd }));
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
