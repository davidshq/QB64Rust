# The wasm target

`--target wasm` builds a program for WebAssembly instead of the machine `qb64rust` runs on. Node runs a console
program, and a browser runs a window program in a page. The target is experimental, and it rejects at compile time
what it does not have.

The pages of this area: [$CONSOLE](../entries/meta-console.md), [$IF](../entries/meta-if.md),
[_OS$](../entries/_os-str.md), [SHELL](../entries/shell.md), [_SHELLHIDE](../entries/_shellhide.md),
[RUN](../entries/run.md), [CHAIN](../entries/chain.md), [_OPENHOST](../entries/_openhost.md),
[_OPENCLIENT](../entries/_openclient.md), [_OPENCONNECTION](../entries/_openconnection.md),
[DECLARE LIBRARY](../entries/declare-library.md), [_OFFSET](../entries/_offset.md), [_MEM](../entries/_mem.md),
[_WINDOWHANDLE](../entries/_windowhandle.md), [_FULLSCREEN](../entries/_fullscreen.md),
[_MESSAGEBOX](../entries/_messagebox.md), [_GL](../entries/_gl.md), [SOUND](../entries/sound.md),
[PLAY](../entries/play.md).

## Building and running

For a `$CONSOLE:ONLY` program, `qb64rust -x --target wasm game.bas` writes `game.js` and `game.wasm` into the
folder `build`, beside the program's `qb64rust.toml` or beside the source, and `node build/game.js` runs the
program. A window program goes into a web folder instead ("Window programs in a page"). `-o` names the `.js`
file and must end in `.js`; the `.wasm` file gets the same name beside it. The two may be moved together, and
renamed together. The program's current folder is the folder of its `.js` file, so a data file it opens by name
alone must be there: list it in `[build]` `copy` of `qb64rust.toml`, which copies it into `build` at each build.
<!-- src: compiler/cli "The output of a wasm build"; compiler/wasm-target "A wasm build"; crates\driver\tests\it\cli.rs -->
<!-- The output folder and `copy`: OpenSpec change build-output-folder; name spec compiler/cli "The output folder" here once it is archived. -->

A build for the wasm target needs the Emscripten SDK, release 7.0.0 and no other. On Windows, `qb64rust toolchain
install --target wasm` downloads it with its Node and Python (815 MB), checks each archive's SHA-256 and installs
it under `%LOCALAPPDATA%\qb64rust\toolchain`. Anywhere, Emscripten's own `emsdk` installs it (`emsdk install
7.0.0` and `emsdk activate 7.0.0`). A build takes the SDK of `--emsdk-root <dir>`, else of the environment
variable `QB64RUST_EMSDK_ROOT`, else the installed one, and `qb64rust toolchain path --target wasm` shows which. A
build downloads nothing. Of the C++ toolchain it uses `make` only.
<!-- src: compiler/toolchain "The Emscripten SDK"; compiler/toolchain "The pinned Emscripten release"; read: crates\driver\src\toolchain.rs -->
<!-- The install and the installed SDK: OpenSpec change wasm-publish; name spec compiler/toolchain "The Emscripten SDK is installed on Windows" here once it is archived. -->

The debug build, the coverage build (`--coverage`), the value-trace build (`--trace-values`), `qb64rust test` and
`qb64rust dap` are not available for this target. Checking with `-z --target wasm` reports the same errors as a
build.
<!-- src: compiler/cli "Options that a wasm build does not take"; crates\driver\tests\it\cli.rs -->

## Window programs in a page

A program without `$CONSOLE:ONLY` is built for a browser. Without `-o` the build writes a web folder, `web` in
the output folder: `build\web\index.html` with `game.js` and `game.wasm` beside it, and its last line names the
page. With `-o out\game.js` it writes `game.html`, `game.js` and `game.wasm` into `out`, and no web folder. The
page runs the program in a canvas, with the keyboard and the mouse over it. A page needs two headers from the host
that serves it, `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp`, to
run the program's threads. It cannot be opened from the disk: `qb64rust serve` serves it on your own machine
("Putting a game on the web").
<!-- src: compiler/wasm-target "A window program runs in a page"; read: crates\driver\src\build.rs; crates\driver\src\compile.rs; crates\driver\tests\it\cli.rs -->

A `$CONSOLE` program's console output goes to the browser's console. libqb's own messages, such as an unhandled
error's "Continue?", are the page's dialogs. `_FULLSCREEN` asks for the browser's full screen; asked outside a key
press or a click, the browser grants it at the next one.
<!-- src: compiler/wasm-target "The page's script"; compiler/wasm-target "libqb's messages in a page"; compiler/wasm-target "Full screen in a page" -->

A browser starts sound only after the page is clicked or touched. Keys the browser keeps for itself, such as
Ctrl+W and F5, never reach the program.
<!-- src: compiler/wasm-target "Parts of libqb the target does not build yet"; read: libqb\internal\c\parts\audio\miniaudio\miniaudio.h -->

A page has only the files it ships: those that `[build]` `copy` of the settings file `qb64rust.toml` copies into
the output folder, as `copy = ["CWS.CFG", "graphics"]` (a folder with all its files). A build without `-o` packs
them into `game.data` beside the page, which puts them in the program's folder before it starts. What the program writes
there is kept in the browser's storage, so a saved game is there at the next visit. A shipped file the program
changed, such as a file of high scores, stays changed until the game ships another version of it.
<!-- src: compiler/wasm-target "A page's files"; read: crates\driver\src\output.rs; crates\driver\src\build.rs -->

A `$CONSOLE:ONLY` program runs under Node, as below.
<!-- src: compiler/wasm-target "A wasm build" -->

## Putting a game on the web

The web folder is what a web host needs, and nothing else of the build. Upload it as it is.
<!-- src: read: crates\driver\src\build.rs; crates\driver\tests\it\cli.rs -->

| File | What it is |
|---|---|
| `index.html` | The page |
| `game.js`, `game.wasm` | The program |
| `game.data` | The files of `[build]` `copy`, when the project lists any |
| `_headers` | Asks a host that reads such a file for the two headers |
| `qb64rust-sw.js` | Adds the two headers in the browser, on a host that sends none |
<!-- src: read: crates\driver\src\build.rs; crates\driver\tests\it\cli.rs -->

The build writes these files anew each time, `game.data` only when `copy` lists files, and leaves any other file
of the folder alone, such as an icon you put there.
<!-- src: read: crates\driver\src\build.rs; crates\driver\tests\it\cli.rs -->

**Try it first.** `qb64rust serve game.bas --open` serves the web folder at `http://127.0.0.1:8064/` with the two
headers and opens the browser. `qb64rust serve <folder>` serves any web folder, and `qb64rust serve` alone serves
`build\web` of the current folder. It listens on `127.0.0.1` only. When port 8064 is taken it uses the next free
one up to 8083, and `--port <n>` names one. A browser keeps a page's saved games for its address with the port, so
keep to one port. `--no-headers` serves the folder as a host that sends neither header, to see what such a host
does. In VS Code, **QB64: Run in Browser** builds, serves and opens the page.
<!-- src: read: crates\driver\src\serve.rs; crates\driver\src\main.rs; crates\driver\tests\it\cli.rs -->

**A page on a host without the headers.** A page that the browser has not isolated registers `qb64rust-sw.js` as
a service worker and reloads once. The worker then adds the two headers to what the page loads, and the program
starts. That costs a player one reload at the first visit. It needs HTTPS, or `127.0.0.1`, and a browser that
allows service workers; a private window of Firefox does not. When the page still cannot start, it says so in
place of "Loading..." and names the two headers.
<!-- src: read: crates\driver\src\build.rs; crates\driver\tests\it\cli.rs -->

What each kind of host needs. Only headless Chrome was tried, with `qb64rust serve` and the test launcher. No
upload to any of these hosts was made, and neither Firefox nor Safari was tried.
<!-- src: read: crates\driver\tests\it\cli.rs; tools\wasm\browser.mjs -->

| Host | What to do |
|---|---|
| Netlify, Cloudflare Pages | Upload the folder. `_headers` asks for the two headers on every path. Cloudflare Pages takes files up to 25 MiB, which a large `game.wasm` or `game.data` may pass |
| GitHub Pages | Upload the folder. The host sends neither header, so the page uses its service worker and reloads once at a player's first visit |
| itch.io | Compress the folder's files into a zip with `index.html` at its top, upload it as an HTML game, and turn on "SharedArrayBuffer support" in the game's embed options. itch.io shows the game inside its own page, where the service worker cannot help. Safari and iPhones are not expected to run it there |
| A server of your own | Send the two headers for the folder, and `application/wasm` for `.wasm` files. Serve it over HTTPS |
<!-- src: read: study\51-web-and-mobile-targets.md; crates\driver\src\build.rs -->

## What the target does not have

A word the target does not have is a compile error at its line, "`<word>` is not available on the wasm target".
It is never built with a routine that does nothing. The words:
<!-- src: compiler/wasm-target "What the target does not have is rejected"; read: crates\sema\src\target.rs -->

| Word | Rejected |
|---|---|
| `SHELL` | as a statement and as a function, with or without `_HIDE` and `_DONTWAIT` |
| `_SHELLHIDE` | the function |
| `RUN` | with a file name; `RUN` alone and `RUN` of a label or a line number are kept |
| `CHAIN` | the statement |
| `_OPENHOST` | the function |
| `_OPENCLIENT` | the function |
| `_OPENCONNECTION` | the function |
| `DECLARE DYNAMIC LIBRARY` | the block |
| `DECLARE LIBRARY` | a block whose library is a prebuilt file (`.a`, `.o`, `.so`, `.lib`); a header alone is compiled with the program |
| `$DEBUG` | the metacommand, whose code talks to the IDE through libqb's network part |
| `_CLIPBOARD$` | the statement and the function: Node has no clipboard |
| `_WINDOWHANDLE` | in a window program, the function: a page has no window of the system |
| `_SCREENCLICK` | in a window program, the statement: a page cannot click outside itself |
| `_SCREENPRINT` | in a window program, the statement: a page cannot type into other programs |
| `_MESSAGEBOX` | in a window program, as a statement and as a function: libqb's dialogs have no browser form yet |
| `_INPUTBOX$`, `_OPENFILEDIALOG$`, `_SAVEFILEDIALOG$`, `_SELECTFOLDERDIALOG$`, `_COLORCHOOSERDIALOG`, `_NOTIFYPOPUP` | as `_MESSAGEBOX` |
| `SUB _GL` | in a window program, the procedure: a page draws through WebGL, which has no OpenGL 1 for a program |
| `_GLRENDER` | in a window program, as `SUB _GL` |
<!-- src: read: crates\sema\src\target.rs; crates\driver\tests\it\wasm_target.rs -->

Three parts of libqb are not built for this target yet. A built-in that needs one is the error "`<word>` is not
available on the wasm target yet: libqb's `<part>` part is not built for it". The part is the one the built-in
table names for it, so a new built-in of a part is covered too.
<!-- src: compiler/wasm-target "Parts of libqb the target does not build yet"; read: crates\sema\src\target.rs -->

| Part | Its words |
|---|---|
| audio | `SOUND`, `PLAY`, `_WAVE`, `_MEMSOUND`, `_MIDISOUNDBANK` and the `_SND` words, in a `$CONSOLE:ONLY` program: Node has no Web Audio (a window program plays them in its page) |
| device-input | `STICK`, `STRIG`, `ON STRIG`, `_DEVICES` and the other words of game controllers and devices |
| screen-image | `_SCREENIMAGE`, `_CLIPBOARDIMAGE` |
<!-- src: read: crates\sema\src\target.rs; crates\driver\tests\it\wasm_target.rs -->

## A pointer is 4 bytes

A pointer has 4 bytes in WebAssembly and 8 on Windows and Linux. On this target `_OFFSET` and
`_UNSIGNED _OFFSET` take 4 bytes and hold 32-bit values, and `LEN` of one is 4.
<!-- src: compiler/wasm-target "A pointer is 4 bytes on this target"; verification\v73_a_type_layout -->

In a `TYPE`, an `_OFFSET` member and a variable-length `STRING` member take 4 bytes each. A `TYPE` of a LONG, an
`_OFFSET` and a `STRING * 2` is 10 bytes here and 14 on Windows. `_MEM` takes 40 bytes, where Windows and Linux
have 56.
<!-- src: compiler/wasm-target "Layouts with pointers on this target"; verification\v73_a_type_layout; verification\v73_b_mem; verification\v73_c_string_members -->

`PUT`, `GET`, `_MK$` and `_CV` write and read 4 bytes for an `_OFFSET`.
<!-- src: compiler/wasm-target "Reading and writing an _OFFSET on this target"; verification\v73_d_values; verification\v73_e_input -->

An operator with an `_OFFSET` is computed in 64 bits, as on Windows, and the result is cut to 32 bits when it is
stored in an `_OFFSET`. `HEX$` of an `_OFFSET` holding -1 is `FFFFFFFF`. An `_OFFSET` variable is passed by
reference to a parameter of another integer type only when that type is also 32 bits wide; to an `_INTEGER64`
parameter a copy is passed. An array parameter of `_OFFSET` takes a `LONG` array here, and an `_INTEGER64`
array is a compile error. A program that must run on both can test `$IF 32BIT`, or use `LEN` of an `_OFFSET`
variable in place of the number 8.
<!-- src: compiler/wasm-target "Arithmetic on an _OFFSET on this target"; compiler/wasm-target "An array for an _OFFSET array parameter on this target"; verification\v73_d_values; read: crates\driver\tests\it\wasm_target.rs -->

## The target's name in a program

On this target `$IF WASM` and `$IF 32BIT` are true, and `$IF WIN`, `WINDOWS`, `LINUX`, `MAC`, `MACOSX` and
`64BIT` false. On Windows and Linux `WASM` is not defined, so `$IF WASM` is false there. `_OS$` is
`[WASM][32BIT]`.
<!-- src: compiler/wasm-target "The target's name in a program"; read: crates\syntax\src\pp.rs:400-427; tools\libqb\edits\wasm-os-name.edit -->

## How a program behaves

A console program's standard input, standard output, command line, exit status and files are those of the Node
process.
Its current folder is the folder of its `.js` file, wherever `node` was started.
<!-- src: compiler/wasm-target "A wasm build"; crates\driver\tests\it\cli.rs -->

A program gives the output and exit status that QB64pe's Linux build gives: its lines end in a line feed, and
the console's comma zones are Linux's.
<!-- src: compiler/wasm-target "A wasm program behaves as on Linux"; openspec\changes\archive\2026-11-20-wasm-console\design.md -->
