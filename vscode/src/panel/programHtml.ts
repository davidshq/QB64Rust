// The HTML of the Program view (OpenSpec change editor-side-panel, design D1 and D2): one function of the state. The
// page holds no logic: a click on a button or a link sends its name, and the extension draws the page again when the
// state changes. No VS Code import, so a unit test compares the string for each state.
import { ProgramState, plural } from "./programModel";

/** The names a click sends back (`data-command`). */
export type ProgramCommand = "run" | "debug" | "browser" | "build" | "goto" | "help" | "newProgram" | "openProgram" | "getStarted" | "trust" | "searchHelp";

export interface PageOptions {
    /** The webview's `cspSource`. */
    cspSource: string;
    /** A fresh nonce for the page's style and script. */
    nonce: string;
}

export function escapeHtml(text: string): string {
    return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;").replace(/'/g, "&#39;");
}

const STYLE = `
body { padding: 0 12px 8px; color: var(--vscode-foreground); font-family: var(--vscode-font-family); font-size: var(--vscode-font-size); }
.name { font-weight: 600; margin: 6px 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
button { font: inherit; color: var(--vscode-button-secondaryForeground); background: var(--vscode-button-secondaryBackground);
  border: 1px solid var(--vscode-button-border, var(--vscode-contrastBorder, transparent)); border-radius: 2px; padding: 4px 8px; cursor: pointer; }
button:hover:not(:disabled) { background: var(--vscode-button-secondaryHoverBackground); }
button.primary { color: var(--vscode-button-foreground); background: var(--vscode-button-background); }
button.primary:hover:not(:disabled) { background: var(--vscode-button-hoverBackground); }
button:focus-visible, a:focus-visible { outline: 1px solid var(--vscode-focusBorder); outline-offset: 2px; }
button:disabled { opacity: 0.5; cursor: default; }
button.run { display: block; width: 100%; padding: 6px; font-size: 1.15em; font-weight: 600; margin-bottom: 6px; }
.row { display: flex; flex-wrap: wrap; gap: 6px; }
.row button { flex: 1 1 auto; }
a { color: var(--vscode-textLink-foreground); cursor: pointer; text-decoration: none; }
a:hover { text-decoration: underline; color: var(--vscode-textLink-activeForeground); }
.note { color: var(--vscode-descriptionForeground); }
.status { margin-top: 8px; padding: 6px 8px; border-radius: 2px; border: 1px solid var(--vscode-contrastBorder, transparent);
  background: var(--vscode-textBlockQuote-background); }
.status.error { border-left: 3px solid var(--vscode-editorError-foreground, var(--vscode-errorForeground)); }
.status.ok .mark { color: var(--vscode-testing-iconPassed, var(--vscode-foreground)); }
.status .where { font-weight: 600; }
.status .message { margin: 2px 0 4px; word-wrap: break-word; }
.status code { font-family: var(--vscode-editor-font-family); font-weight: 600; color: inherit; background: none; }
.status .links { display: flex; flex-wrap: wrap; gap: 12px; }
.bar { height: 2px; margin-top: 6px; overflow: hidden; background: var(--vscode-contrastBorder, transparent); }
.bar i { display: block; width: 30%; height: 100%; background: var(--vscode-progressBar-background); animation: bar 1.6s linear infinite; }
@keyframes bar { from { margin-left: -30%; } to { margin-left: 100%; } }
@media (prefers-reduced-motion: reduce) { .bar i { animation: none; width: 100%; } }
.foot { margin: 10px 0 0; }
`;

const SCRIPT = `
const vscode = acquireVsCodeApi();
document.addEventListener("click", (e) => {
  const t = e.target.closest("[data-command]");
  if (t && !t.disabled) { e.preventDefault(); vscode.postMessage({ command: t.dataset.command }); }
});
`;

/** A compiler message as HTML: what it marks with backticks (`` expected `)` ``) drawn as code. */
export function messageHtml(message: string): string {
    return escapeHtml(message).replace(/`([^`]+)`/g, "<code>$1</code>");
}

function link(command: ProgramCommand, text: string): string {
    return `<a href="#" role="button" data-command="${command}">${escapeHtml(text)}</a>`;
}

/** The last line of the page in every state: the help's search, since the Help view's own button is often below
 * the fold. */
const FOOT = `<p class="foot">${link("searchHelp", "Search Help")}</p>`;

function body(state: ProgramState): string {
    if (state.kind === "none") {
        return [
            `<p class="name">No program is open.</p>`,
            `<div class="row"><button class="primary" data-command="newProgram">New Program</button>` +
                `<button data-command="openProgram">Open a Program</button></div>`,
            `<p>${link("getStarted", "Get Started with QB64")}</p>`,
            FOOT,
        ].join("\n");
    }
    const off = state.trusted ? "" : " disabled";
    const parts = [
        `<p class="name" title="${escapeHtml(state.uri)}">${escapeHtml(state.name)}</p>`,
        `<button class="run primary" data-command="run"${off}>&#9654; Run</button>`,
        `<div class="row"><button data-command="debug"${off}>Debug</button>` +
            `<button data-command="browser"${off}>Run in Browser</button>` +
            `<button data-command="build"${off}>Build</button></div>`,
    ];
    if (!state.trusted) {
        parts.push(`<p class="note">Running needs a trusted workspace. ${link("trust", "Manage Workspace Trust")}</p>`);
    }
    const s = state.status;
    switch (s.kind) {
        case "busy":
            parts.push(`<div class="status busy" role="status">${escapeHtml(s.text)}${s.long ? '<div class="bar"><i></i></div>' : ""}</div>`);
            break;
        case "ok": {
            const warnings = s.warnings ? `, ${plural(s.warnings, "warning")}` : "";
            parts.push(`<div class="status ok" role="status"><span class="mark">&#10003;</span> OK${warnings}</div>`);
            break;
        }
        case "error": {
            const main = s.first.uri === state.uri;
            const where = main ? `Line ${s.first.line}` : `${s.first.file}, line ${s.first.line}`;
            const more = s.count > 1 ? ` <span class="note">(${plural(s.count - 1, "more error")})</span>` : "";
            const links = [link("goto", `Go to line ${s.first.line}`)];
            if (s.page) {
                links.push(link("help", s.pageTitle ? `Help on ${s.pageTitle}` : "Help"));
            }
            parts.push(
                `<div class="status error" role="status"><div class="where">${escapeHtml(where)}${more}</div>` +
                    `<div class="message">${messageHtml(s.first.message)}</div>` +
                    `<div class="links">${links.join("")}</div></div>`,
            );
            break;
        }
    }
    parts.push(FOOT);
    return parts.join("\n");
}

/** The whole page for `state`. */
export function programHtml(state: ProgramState, o: PageOptions): string {
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
