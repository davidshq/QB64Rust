// The HTML of the page "QB64 Home" (OpenSpec change editor-side-panel, design D11): one function of the state. The
// page holds no logic: a click sends a command's name, and the checkbox sends its state. No VS Code import, so a
// unit test compares the string.
import { escapeHtml } from "../panel/programHtml";
import { HomeCommand, HomeState } from "./homeModel";

export interface HomePageOptions {
    /** The webview's `cspSource`. */
    cspSource: string;
    /** A fresh nonce for the page's style and script. */
    nonce: string;
}

const STYLE = `
body { padding: 24px 32px; max-width: 60em; color: var(--vscode-foreground); background: var(--vscode-editor-background);
  font-family: var(--vscode-font-family); font-size: var(--vscode-font-size); line-height: 1.5; }
h1 { font-size: 2em; font-weight: 600; margin: 0; }
h2 { font-size: 1.1em; font-weight: 600; margin: 0 0 6px; }
.lead { color: var(--vscode-descriptionForeground); margin: 2px 0 20px; }
.tiles { display: flex; flex-wrap: wrap; gap: 10px; margin-bottom: 26px; }
button.tile { flex: 1 1 11em; text-align: left; font: inherit; cursor: pointer; padding: 10px 12px; border-radius: 3px;
  color: var(--vscode-button-secondaryForeground); background: var(--vscode-button-secondaryBackground);
  border: 1px solid var(--vscode-button-border, var(--vscode-contrastBorder, transparent)); }
button.tile:hover { background: var(--vscode-button-secondaryHoverBackground); }
button.tile.primary { color: var(--vscode-button-foreground); background: var(--vscode-button-background); }
button.tile.primary:hover { background: var(--vscode-button-hoverBackground); }
button.tile strong { display: block; font-size: 1.1em; }
button.tile span { opacity: 0.85; }
button:focus-visible, a:focus-visible, input:focus-visible { outline: 1px solid var(--vscode-focusBorder); outline-offset: 2px; }
.cols { display: flex; flex-wrap: wrap; gap: 28px; }
.cols > div { flex: 1 1 14em; }
.item { margin: 3px 0; }
.note { color: var(--vscode-descriptionForeground); }
.mark { display: inline-block; width: 1.3em; }
.mark.ok { color: var(--vscode-testing-iconPassed, var(--vscode-foreground)); }
a { color: var(--vscode-textLink-foreground); cursor: pointer; text-decoration: none; }
a:hover { text-decoration: underline; color: var(--vscode-textLink-activeForeground); }
.show { margin-top: 30px; color: var(--vscode-descriptionForeground); }
.show input { vertical-align: middle; margin: 0 6px 0 0; }
`;

const SCRIPT = `
const vscode = acquireVsCodeApi();
document.addEventListener("click", (e) => {
  const t = e.target.closest("[data-command]");
  if (!t || t.tagName === "INPUT") { return; }
  e.preventDefault();
  vscode.postMessage({ command: t.dataset.command, arg: t.dataset.arg });
});
document.addEventListener("change", (e) => {
  if (e.target.dataset.command === "toggle") { vscode.postMessage({ command: "toggle", arg: e.target.checked ? "on" : "off" }); }
});
`;

function link(command: HomeCommand, text: string, arg?: string): string {
    const a = arg === undefined ? "" : ` data-arg="${escapeHtml(arg)}"`;
    return `<a href="#" role="button" data-command="${command}"${a}>${escapeHtml(text)}</a>`;
}

function tile(command: HomeCommand, title: string, what: string, primary = false): string {
    return `<button class="tile${primary ? " primary" : ""}" data-command="${command}"><strong>${escapeHtml(title)}</strong><span>${escapeHtml(what)}</span></button>`;
}

function body(s: HomeState): string {
    const recent = s.recent.length
        ? s.recent.map((r, n) => `<div class="item">${link("recent", r.name, String(n))} <span class="note">${escapeHtml(r.folder)}</span></div>`).join("")
        : `<div class="item note">The programs you open are listed here.</div>`;
    const setup = s.setup
        .map((r) => {
            const mark = r.ok ? `<span class="mark ok">&#10003;</span>` : `<span class="mark">&#9675;</span>`;
            const action = r.action ? ` ${link(r.action.command, r.action.text)}` : "";
            return `<div class="item">${mark}${escapeHtml(r.label)} <span class="note">${escapeHtml(r.note)}</span>${action}</div>`;
        })
        .join("");
    return [
        `<h1>QB64</h1>`,
        `<p class="lead">Write BASIC. Click Run. See it run.</p>`,
        `<div class="tiles">` +
            tile("newProgram", "New Program", "One file, ready to run", true) +
            tile("openProgram", "Open a Program", "A .bas file you have") +
            tile("examples", "Examples", "Small programs to run and change") +
            tile("searchHelp", "Help", "Every statement and function") +
            `</div>`,
        `<div class="cols">`,
        `<div><h2>Recent programs</h2>${recent}</div>`,
        `<div><h2>Setup</h2>${setup}</div>`,
        `<div><h2>New here?</h2>` +
            `<div class="item">${link("getStarted", "Get Started with QB64")}</div>` +
            `<div class="item">${link("page", "First steps: new to programming", "first-steps")}</div>` +
            `<div class="item">${link("page", "Coming from QuickBASIC 4.5", "coming-from-qb45")}</div>` +
            `<div class="item">${link("page", "Coming from QB64pe", "coming-from-qb64pe")}</div></div>`,
        `</div>`,
        `<p class="show"><label><input type="checkbox" data-command="toggle"${s.showOnStartup ? " checked" : ""}>` +
            `Show this page when a QB64 window starts</label> ` +
            `<span>(the command <b>QB64: Home</b> opens it at any time)</span></p>`,
    ].join("\n");
}

/** The whole page for `state`. */
export function homeHtml(state: HomeState, o: HomePageOptions): string {
    const csp = `default-src 'none'; style-src ${o.cspSource} 'nonce-${o.nonce}'; script-src 'nonce-${o.nonce}';`;
    return [
        "<!DOCTYPE html>",
        `<html lang="en"><head><meta charset="UTF-8">`,
        `<meta http-equiv="Content-Security-Policy" content="${csp}">`,
        `<meta name="viewport" content="width=device-width, initial-scale=1.0">`,
        `<style nonce="${o.nonce}">${STYLE}</style></head><body>`,
        body(state),
        `<script nonce="${o.nonce}">${SCRIPT}</script>`,
        "</body></html>",
    ].join("\n");
}
