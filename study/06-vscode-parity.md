# 06 — IDE feature parity as a VS Code extension

Decision (2026-10-02): the text-mode IDE is not ported. Feature parity is delivered as a VS Code extension.
This maps every feature in the parity list (`04-ide-debugger.md` Part G.3) to where it lives in the new design.
Section references like "04 B §4" point into the IDE study.

## Architecture

```
VS Code extension (TypeScript)
  ├─ TextMate grammar, language config, snippets, keymap      static, no compiler needed
  ├─ LSP client  ───────────────▶  qb64 language server  ──┐
  ├─ DAP client  ───────────────▶  qb64 debug adapter      ├─ both built on the new compiler as a library
  └─ tasks / commands ──────────▶  qb64 CLI (build, run, format, convert)
```

The compiler has to be a library with an analysis API, not only a CLI. The language server and the debug adapter are
thin layers over it. That is the main design requirement this decision adds.

## What VS Code already provides (no work)

Multi-cursor and column selection, undo/redo, line move (Alt+Up/Down), duplicate line, comment toggle, block indent,
bracket matching, auto-closing brackets and quotes, word-occurrence highlight, bookmarks (via a common extension),
find/replace with regex and scope, go to line, recent files, crash recovery (hot exit), drag-and-drop open,
split views, themes, font and size choice, Unicode text.

Several old limits disappear: whole-line-only multi-line selection, the 608-column limit, CP437-only display.

## Language server (needs compiler support)

| IDE feature | LSP mechanism | Compiler requirement |
|---|---|---|
| Live syntax check, status "OK" / error, error-line highlight | `publishDiagnostics` | Diagnostics with file, line, column, include chain. Today: first error only, line granularity. Report multiple errors where recovery allows |
| Warnings dialog (incl. unused variables) | Diagnostics, severity Warning | Same channel |
| As-you-type auto layout: indent, spacing, keyword case | `textDocument/formatting`, `rangeFormatting`, `onTypeFormatting` | **Formatter separate from code generation.** Must reproduce current layout rules; `tests\format_tests` are the oracle. Old IDE skips the line under the cursor; `onTypeFormatting` on Enter gives the same effect |
| `?` → `PRINT` expansion, closing-quote completion | `onTypeFormatting` or completion | Part of formatter |
| `$IF`-excluded lines drawn uncoloured | Semantic tokens with a custom modifier, or a custom notification (like clangd's inactive regions) | Expose inactive line ranges |
| Highlight user SUB/FUNCTION names | Semantic tokens | Symbol table |
| SUBs dialog (F2), sorting, line counts | `documentSymbol`, outline view | Procedures with ranges, kind, parameters |
| Go to SUB / label | `definition`, `workspaceSymbol` | Labels and procedures with locations |
| Open `$INCLUDE` file | `documentLink` | Resolved include paths |
| Contextual F1 help, help for user SUBs | `hover`, plus a command that opens the help webview | Symbol at position; built-in keyword id |
| Keyword list | `completion`, `signatureHelp` | Built-in table with signatures (`hr_syntax` today) |
| Math evaluator | Command calling the server | Constant evaluator exposed |
| `$NOPREFIX` conversion offer | `codeAction` | Detect and rewrite |
| Shift+F9 manual-check mode | Setting: diagnostics on save only | Cancellable analysis |

Analysis must be cancellable and debounced. The old IDE restarted a full two-pass compile on every keystroke and
relied on polling input once per line.

## Debug adapter (`$DEBUG` replacement)

| IDE feature | DAP mechanism | Notes |
|---|---|---|
| Start paused | `stopOnEntry` | |
| Breakpoints (F9, gutter) | `setBreakpoints` | Native gutter UI |
| Step into / over / out | `stepIn`, `next`, `stepOut` | |
| Run to line | VS Code "Run to Cursor" | Built on a temporary breakpoint |
| Set next line | `gotoTargets` + `goto` | Today: generated `switch` on line numbers in `vw_main_dispatch.txt` |
| Skip lines (Ctrl+P) | No DAP equivalent; custom request plus gutter decoration | Decide whether to keep |
| Watch list with arrays, ranges, UDT members, display formats | `variables`, `evaluate`, Watch view | Needs a debug-info file from the compiler (variables, types, storage, UDT layouts). Today the IDE reads compiler memory |
| Send value | `setVariable` | |
| Watchpoints | `setDataBreakpoints` | Today evaluated inside the debuggee |
| Call stack, viewable after run ends | `stackTrace` | Post-mortem view needs the adapter to keep the last stack |
| Console watch output | Debug console | |

Design requirement: the compiler emits a **debug symbol file** (line table, procedures, variables with storage
locations, UDT layouts). The old wire protocol had defects (frame length off by 4, reversed string watchpoint
comparison; 04 Part F §8); a new protocol should be versioned. An alternative is native debug info plus an existing
native debugger, but BASIC-level variable display then needs a custom layer anyway.

## Tasks and commands (CLI-backed)

| IDE feature | VS Code mechanism |
|---|---|
| F5 run, F11 make EXE only, run without keeping EXE | Task provider + launch configuration; keybindings |
| Modify `COMMAND$` | `args` in launch configuration |
| Output to source folder / default folder | Setting |
| Compile log link | Problems matcher on build output; C++ errors mapped to BASIC lines where possible (`#line` already emitted) |
| Logging options, terminal choice | Launch configuration / settings |
| Licence file generation, purge build files | Commands |
| Compiler settings dialog (optimise, debug info, extra flags) | Settings (`qb64.*`), passed as CLI flags |
| Export as HTML / RTF / Discord / forum / wiki | Commands |
| QB4.5 binary import | Command (converter) |
| QBJS web build | Command |

## Help system

The old IDE downloads and caches wiki pages and renders its own markup (04 E §1). Options: render cached pages in a
webview; or link to the online wiki and use hover text from the built-in table for quick help. Offline parity needs
the webview and a cache.

## Items to decide rather than port

- Code-page handling: the old buffer is raw bytes shown through one of 27 code pages. VS Code works in Unicode, with
  file encodings. Programs depend on CP437 bytes inside string literals. Decide: open as CP437 encoding by default,
  or convert sources to UTF-8 and have the compiler convert literals back.
- Tabs are expanded to spaces on load in the old IDE; VS Code keeps them unless configured.
- Formatting no longer bypasses undo or the dirty flag (old IDE did both). This is an improvement but changes feel.
- Multi-instance behaviour (instance index tied to temp folder, debug port, settings section) disappears.

## Effect on the compiler design

1. Compiler as a library with an incremental, cancellable analysis API.
2. Formatter as its own component, not a by-product of code generation.
3. Diagnostics with columns and more than one error.
4. Debug symbol output.
5. All IDE coupling in the old design (shared globals, `ide(0)` coroutine, line pull, code 100 continuation fetch)
   is dropped entirely. Nothing from 04 Parts A–C needs to be preserved except observable formatting rules.
