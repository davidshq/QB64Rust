// The help tab's script (OpenSpec change editor-side-panel, design D5): it sends what was clicked to the extension,
// which draws the next page. It holds no other logic.
(function () {
    const vscode = acquireVsCodeApi();
    document.addEventListener("click", (e) => {
        const el = e.target instanceof Element ? e.target.closest("[data-page], [data-web], [data-action]") : null;
        if (!el || el.disabled) {
            return;
        }
        e.preventDefault();
        if (el.dataset.page) {
            vscode.postMessage({ type: "page", page: el.dataset.page, anchor: el.dataset.anchor });
        } else if (el.dataset.web) {
            vscode.postMessage({ type: "web", url: el.dataset.web });
        } else {
            const box = el.closest("[data-example]");
            vscode.postMessage({ type: el.dataset.action, example: box ? box.dataset.example : undefined });
        }
    });
    // A link to a heading of the page: scroll to it once the page is drawn.
    // The page's own anchor, not a link's: a link to a heading of another page has one too.
    const page = document.querySelector(".page[data-anchor]");
    const target = page && document.getElementById(page.dataset.anchor);
    if (target) {
        target.scrollIntoView();
    }
})();
