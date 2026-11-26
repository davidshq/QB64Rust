// The rows of the Examples view (OpenSpec change editor-side-panel, spec editor/examples, "The Examples view"): the
// groups of `examples\index.toml`, then "One for every keyword" with the reference's examples by keyword. No VS Code
// import.
import type { HelpExample, HelpIndexData } from "../help/helpIndex";

export type ExampleRow =
    | { kind: "group"; label: string; children: ExampleRow[]; open?: boolean }
    | { kind: "example"; label: string; example: HelpExample };

export const EVERY_KEYWORD = "One for every keyword";

/** The tree of the view. A keyword with one example is one row; with several, a row of its own with them below.
 * The first group ("First steps") starts open, so that the view shows programs to click, not only folders. */
export function exampleTree(data: HelpIndexData): ExampleRow[] {
    const row = (e: HelpExample, label = e.title): ExampleRow => ({ kind: "example", label, example: e });
    const groups: ExampleRow[] = data.groups.map((g, n) => ({ kind: "group", label: g.title, children: g.examples.map((e) => row(e)), open: n === 0 }));
    const keywords: ExampleRow[] = data.keywords.map((k) =>
        k.examples.length === 1 ? row(k.examples[0], k.title) : { kind: "group", label: k.title, children: k.examples.map((e) => row(e)) },
    );
    return [...groups, { kind: "group", label: EVERY_KEYWORD, children: keywords }];
}

/** The context value of an example's row: one that opens a port gets no "Run" (design D7). */
export function exampleContext(e: HelpExample): "example.runnable" | "example.network" {
    return e.network ? "example.network" : "example.runnable";
}
