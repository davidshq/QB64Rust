// The help tab (OpenSpec change editor-side-panel, design D5; spec editor/help, "The help tab"): one webview panel,
// "QB64 Help", beside the active editor, never in its place. A second page replaces the first in the same tab, with
// Back and Forward. The page is drawn by renderPage.ts; the webview's script only sends what was clicked.
import * as crypto from "crypto";
import * as vscode from "vscode";
import { exampleText, openAsNewProgram, runExample } from "../examples/runExample";
import { HelpIndex } from "./helpIndex";
import { tickWalkthrough } from "../walkthrough";
import { helpHtml, renderPage } from "./renderPage";

export const HELP_PANEL = "qb64rust.help.page";

type Message =
    | { type: "page"; page: string; anchor?: string }
    | { type: "web"; url: string }
    | { type: "back" | "forward" | "search" }
    | { type: "run" | "open" | "copy"; example: string };

export class HelpPanel implements vscode.Disposable {
    private panel: vscode.WebviewPanel | undefined;
    private history: { page: string; anchor?: string }[] = [];
    private at = -1;
    private readonly disposables: vscode.Disposable[] = [];

    constructor(
        private readonly media: vscode.Uri,
        private readonly index: () => HelpIndex | undefined,
        private readonly storage: string,
    ) {
        this.disposables.push(
            vscode.commands.registerCommand("qb64rust.openHelpPage", (page: string, anchor?: string) => this.show(page, anchor)),
            vscode.workspace.onDidGrantWorkspaceTrust(() => this.draw()),
        );
    }

    /** The page shown, undefined without the tab; for tests. */
    get current(): string | undefined {
        return this.panel ? this.history[this.at]?.page : undefined;
    }

    /** The tab's title, undefined without the tab; for tests. */
    get title(): string | undefined {
        return this.panel?.title;
    }

    /** Shows a page in the tab, opened beside the active editor when there is none; false for no such page. */
    async show(page: string, anchor?: string): Promise<boolean> {
        const index = this.index();
        if (!index?.page(page)) {
            void vscode.window.showErrorMessage(index ? `QB64: no help page ${page}.` : "QB64: the help is not installed (run npm run help in a development copy).");
            return false;
        }
        tickWalkthrough("lookedUp");
        this.history = this.history.slice(0, this.at + 1);
        this.history.push({ page, anchor });
        this.at = this.history.length - 1;
        this.open();
        this.draw();
        return true;
    }

    back(): void {
        if (this.at > 0) {
            this.at--;
            this.draw();
        }
    }

    forward(): void {
        if (this.at < this.history.length - 1) {
            this.at++;
            this.draw();
        }
    }

    private open(): void {
        if (this.panel) {
            this.panel.reveal(this.panel.viewColumn, true);
            return;
        }
        const panel = vscode.window.createWebviewPanel(
            HELP_PANEL,
            "QB64 Help",
            { viewColumn: vscode.ViewColumn.Beside, preserveFocus: true },
            { enableScripts: true, localResourceRoots: [this.media], enableFindWidget: true },
        );
        panel.iconPath = vscode.Uri.joinPath(this.media, "qb64.svg");
        panel.webview.onDidReceiveMessage((m: Message) => void this.receive(m));
        panel.onDidDispose(() => {
            if (this.panel === panel) {
                this.panel = undefined;
                this.history = [];
                this.at = -1;
            }
        });
        this.panel = panel;
    }

    private draw(): void {
        const index = this.index();
        const entry = this.history[this.at];
        const panel = this.panel;
        const page = entry && index?.page(entry.page);
        if (!panel || !index || !page) {
            return;
        }
        const body = renderPage(index, page, index.text(page), { trusted: vscode.workspace.isTrusted });
        const anchor = entry.anchor ? ` data-anchor="${entry.anchor.replace(/[^\w-]/g, "")}"` : "";
        panel.title = `QB64 Help: ${page.title}`;
        panel.webview.html = helpHtml(`<div class="page"${anchor}>${body}</div>`, {
            cspSource: panel.webview.cspSource,
            nonce: crypto.randomBytes(16).toString("base64"),
            styleUri: panel.webview.asWebviewUri(vscode.Uri.joinPath(this.media, "help.css")).toString(),
            scriptUri: panel.webview.asWebviewUri(vscode.Uri.joinPath(this.media, "help.js")).toString(),
            canBack: this.at > 0,
            canForward: this.at < this.history.length - 1,
        });
    }

    private async receive(m: Message): Promise<void> {
        const index = this.index();
        switch (m.type) {
            case "page":
                await this.show(m.page, m.anchor);
                return;
            case "web":
                if (/^https?:\/\//i.test(m.url)) {
                    await vscode.env.openExternal(vscode.Uri.parse(m.url));
                }
                return;
            case "back":
                this.back();
                return;
            case "forward":
                this.forward();
                return;
            case "search":
                await vscode.commands.executeCommand("qb64rust.searchHelp");
                return;
            case "run":
                if (index) {
                    await runExample(index, m.example, this.storage);
                }
                return;
            case "open":
                if (index) {
                    await openAsNewProgram(index, m.example);
                }
                return;
            case "copy": {
                const text = index && exampleText(index, m.example);
                if (text !== undefined) {
                    await vscode.env.clipboard.writeText(text);
                    void vscode.window.setStatusBarMessage("QB64: the example was copied.", 3000);
                }
                return;
            }
        }
    }

    dispose(): void {
        this.panel?.dispose();
        for (const d of this.disposables) {
            d.dispose();
        }
    }
}
