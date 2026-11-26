// The Examples view (OpenSpec change editor-side-panel, design D7; spec editor/examples): a click opens the user's
// copy of an example as a file; the row's menu has "Run", "Open as a New Program" and "Reset This Example".
import * as vscode from "vscode";
import { HelpIndex } from "../help/helpIndex";
import { ExampleRow, exampleContext, exampleTree } from "./examplesModel";
import { openAsNewProgram, openExample, resetExampleCopy, runExample } from "./runExample";

export const EXAMPLES_VIEW = "qb64rust.examples";

/** The name of an example from a command's argument: a row of the view, or the name itself. */
function nameOf(arg: ExampleRow | string | undefined): string | undefined {
    return typeof arg === "string" ? arg : arg?.kind === "example" ? arg.example.name : undefined;
}

export class ExamplesView implements vscode.TreeDataProvider<ExampleRow>, vscode.Disposable {
    private readonly disposables: vscode.Disposable[];

    constructor(
        private readonly index: () => HelpIndex | undefined,
        storage: string,
    ) {
        const withIndex = <T>(arg: ExampleRow | string | undefined, f: (i: HelpIndex, name: string) => T): T | undefined => {
            const i = this.index();
            const name = nameOf(arg);
            return i && name ? f(i, name) : undefined;
        };
        this.disposables = [
            vscode.window.registerTreeDataProvider(EXAMPLES_VIEW, this),
            vscode.commands.registerCommand("qb64rust.examples.open", (arg?: ExampleRow | string) => withIndex(arg, (i, n) => openExample(i, n, storage))),
            vscode.commands.registerCommand("qb64rust.examples.run", (arg?: ExampleRow | string) => withIndex(arg, (i, n) => runExample(i, n, storage))),
            vscode.commands.registerCommand("qb64rust.examples.openAsNewProgram", (arg?: ExampleRow | string) => withIndex(arg, openAsNewProgram)),
            vscode.commands.registerCommand("qb64rust.examples.reset", (arg?: ExampleRow | string) => withIndex(arg, (i, n) => resetExampleCopy(i, n, storage))),
        ];
    }

    /** The rows at the top; for tests. */
    get rows(): ExampleRow[] {
        return this.getChildren();
    }

    getTreeItem(row: ExampleRow): vscode.TreeItem {
        if (row.kind === "group") {
            return new vscode.TreeItem(row.label, row.open ? vscode.TreeItemCollapsibleState.Expanded : vscode.TreeItemCollapsibleState.Collapsed);
        }
        const item = new vscode.TreeItem(row.label, vscode.TreeItemCollapsibleState.None);
        item.iconPath = new vscode.ThemeIcon("file-code");
        item.contextValue = exampleContext(row.example);
        item.tooltip = row.example.network ? `${row.example.name}: opens a network port, so it has no Run here.` : row.example.name;
        item.command = { title: "Open", command: "qb64rust.examples.open", arguments: [row] };
        return item;
    }

    getChildren(row?: ExampleRow): ExampleRow[] {
        if (row) {
            return row.kind === "group" ? row.children : [];
        }
        const i = this.index();
        return i ? exampleTree(i.data) : [];
    }

    dispose(): void {
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}
