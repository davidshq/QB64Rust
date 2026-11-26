// The Help view and the commands "QB64: Search Help" and "QB64: Help for This Word" (OpenSpec change
// editor-side-panel, design D5 and D6; spec editor/help). They read only the help folder, so they work in an
// untrusted workspace and without the `qb64rust` binary. No key is bound: F1 stays VS Code's (user, 2026-11-25).
import * as vscode from "vscode";
import { LanguageServer } from "../languageServer";
import { HelpIndex, HelpRow, plainText, wordAt } from "./helpIndex";

export const HELP_VIEW = "qb64rust.help";

export class HelpView implements vscode.TreeDataProvider<HelpRow>, vscode.Disposable {
    private readonly disposables: vscode.Disposable[];

    constructor(
        private readonly index: () => HelpIndex | undefined,
        private readonly server: LanguageServer,
    ) {
        this.disposables = [
            vscode.window.registerTreeDataProvider(HELP_VIEW, this),
            vscode.commands.registerCommand("qb64rust.searchHelp", (query?: string) => this.search(query)),
            // From a hover's link with the hovered place; from the menu or the palette, the cursor's.
            vscode.commands.registerCommand("qb64rust.helpForWord", (uri?: string, line?: number, character?: number) =>
                this.helpForWord(uri === undefined ? undefined : { uri: vscode.Uri.parse(uri), position: new vscode.Position(line ?? 0, character ?? 0) }),
            ),
        ];
    }

    getTreeItem(row: HelpRow): vscode.TreeItem {
        const item = new vscode.TreeItem(row.label, row.children ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None);
        if (row.page) {
            const page = this.index()?.page(row.page);
            item.tooltip = page ? plainText(page.sentence) : undefined;
            item.command = { title: "Open", command: "qb64rust.openHelpPage", arguments: [row.page] };
            item.iconPath = new vscode.ThemeIcon(row.children ? "book" : page?.kind === "keyword" ? "symbol-keyword" : "book");
        }
        return item;
    }

    getChildren(row?: HelpRow): HelpRow[] {
        return row ? (row.children ?? []) : (this.index()?.data.tree ?? []);
    }

    /** "QB64: Search Help": every page by title and first sentence, narrowed as the user types (`search` of the
     * index); the chosen page opens in the help tab. Gives the page chosen; for tests, `pick` stands for the user. */
    async search(query = "", pick?: (items: string[]) => string | undefined): Promise<string | undefined> {
        const index = this.index();
        if (!index) {
            void vscode.window.showErrorMessage("QB64: the help is not installed (run npm run help in a development copy).");
            return undefined;
        }
        if (pick) {
            const chosen = pick(index.search(query).map((p) => p.name));
            if (chosen) {
                await vscode.commands.executeCommand("qb64rust.openHelpPage", chosen);
            }
            return chosen;
        }
        type Item = vscode.QuickPickItem & { page: string };
        const qp = vscode.window.createQuickPick<Item>();
        qp.title = "QB64: Search Help";
        qp.placeholder = "A word, or what it does: bounds, random, file";
        qp.matchOnDescription = false;
        qp.matchOnDetail = false;
        const fill = (q: string) => {
            // The index ranks the pages; VS Code's own filter is kept out of the way with `alwaysShow`.
            qp.items = index.search(q).slice(0, 200).map((p) => ({ label: p.title, detail: plainText(p.sentence), page: p.name, alwaysShow: true }));
        };
        qp.value = query;
        fill(query);
        qp.onDidChangeValue(fill);
        const chosen = await new Promise<string | undefined>((resolve) => {
            qp.onDidAccept(() => {
                resolve(qp.selectedItems[0]?.page);
                qp.hide();
            });
            qp.onDidHide(() => resolve(undefined));
            qp.show();
        });
        qp.dispose();
        if (chosen) {
            await vscode.commands.executeCommand("qb64rust.openHelpPage", chosen);
        }
        return chosen;
    }

    /**
     * "QB64: Help for This Word" (spec editor/help, "Help for the word under the cursor"): the page of a built-in or
     * keyword, the definition of a name the program defines, else the search with the word typed in. Without the
     * language server the page is found by the word's spelling. Gives what it did, for tests.
     */
    async helpForWord(at?: { uri: vscode.Uri; position: vscode.Position }): Promise<"page" | "definition" | "search" | undefined> {
        const editor = vscode.window.activeTextEditor;
        const doc = at ? vscode.workspace.textDocuments.find((d) => d.uri.toString() === at.uri.toString()) : editor?.document;
        if (!doc || doc.languageId !== "qb64rust") {
            return undefined;
        }
        const pos = at?.position ?? editor!.selection.active;
        if (at && editor?.document !== doc) {
            await vscode.window.showTextDocument(doc);
        }
        if (at) {
            // The definition is gone to from the cursor: put it on the hovered word.
            vscode.window.activeTextEditor!.selection = new vscode.Selection(pos, pos);
        }
        const found = pos.line < doc.lineCount ? wordAt(doc.lineAt(pos.line).text, pos.character) : undefined;
        const topic = await this.server.helpTopic(doc.uri, pos);
        if (topic && "page" in topic) {
            await vscode.commands.executeCommand("qb64rust.openHelpPage", topic.page);
            return "page";
        }
        if (topic && "definition" in topic) {
            await vscode.commands.executeCommand("editor.action.revealDefinition");
            return "definition";
        }
        // No server (undefined): the word's spelling against the index. The server's null (a comment, a word with
        // no page) goes to the search.
        const byWord = topic === undefined && found ? this.index()?.pageOfWord(found.word) : undefined;
        if (byWord) {
            await vscode.commands.executeCommand("qb64rust.openHelpPage", byWord.name);
            return "page";
        }
        void this.search(found?.word ?? "");
        return "search";
    }

    dispose(): void {
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}
