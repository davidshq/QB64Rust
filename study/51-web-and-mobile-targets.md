# 51 — Web and mobile targets

2026-11-15. The user asked how QBJS compares with QB64Rust, then asked for a deeper look and recommendations:
"I'd like QB64Rust to eventually be able to deploy both to the web and to mobile apps". Then they asked for
the options besides Emscripten, what it takes to put a WebAssembly game on the web, and how Civil War Strategy
would work on a phone: "the issue more than whether it is native or not is whether it works on a mobile
device" (user, 2026-11-15). The user answered questions 1 to 3 of section 10 on 2026-11-19 (`DECISIONS.md`):
Emscripten with the same libqb, start now, a 4-byte `_OFFSET`. Questions 4 and 5 are open.

What this rests on:

- QBJS's README and two wiki pages, read 2026-11-15, not run.
- This repo's `libqb\` copy and `crates\`, read.
- `study\03` (the runtime) and `SOMEDAY.md`.
- Civil War Strategy's repo at `69e0cec` plus the user's uncommitted edits, read 2026-11-15.
- Web searches on browsers, tools and hosting. Each fact from them carries its source where it is used. The
  same links are in section 11.

Nothing here was built or measured; the measurements wait on the Emscripten SDK (§11, "Still open").
On 2026-11-16 the statements first marked "general knowledge" were checked in sources (`CLAUDE.md` working
rule 17). Those that no source confirmed are marked "unchecked", each with the reason.

## 1. Summary

- **Build the web target from the same C++ and the same libqb, compiled with Emscripten to WebAssembly.** The
  front end, the IR and `codegen-cpp` stay as they are. QB64pe fidelity, the project's purpose, carries over
  almost unchanged. The work is in libqb, as edit files, and in the driver.
- **The alternatives were weighed** (section 4): Cheerp, plain `clang` with our own browser glue, Emscripten's
  JavaScript output, running the native build in an x86 emulator in the browser, a runtime rewritten in Rust,
  our own JavaScript output, and QBJS. None keeps libqb and gives a browser window, sound and input with less
  work.
- **A WebAssembly game is a folder of static files**: an `.html` page, a `.js` loader, the `.wasm` code and the
  game's data. Any web host can serve it. There is one condition: libqb runs on threads, and a page gets threads
  only when it is served with two HTTP headers (section 6). Netlify, Cloudflare Pages and your own server can
  send them. GitHub Pages needs a workaround script. itch.io has a setting, but the header it sends does not
  work in Safari, so a threaded game there would likely not run on iPhones (§6.3).
- **The first step is cheap and gives an answer:** build the 4158 `$CONSOLE:ONLY` programs of `verification\`
  for WebAssembly and run them under Node against their recordings.
- **On a phone, the web build is the app.** It runs in the phone's browser and can be installed to the home
  screen. A store wrapper puts the same build in the stores. The real work for phones is input and screen size,
  not native code. Civil War Strategy needs a touch key bar, tappable menus and a way to type one name
  (section 7.3).
- **Two things must be fixed for any of this:** libqb's thread handshakes, which rely on x86 memory order, and
  the front end's assumption that every target is 64-bit.

## 2. QBJS, more closely

`boxgaming/qbjs` runs QB64-style BASIC in the browser
([README](https://github.com/boxgaming/qbjs): MIT, about 85 stars, 615 commits, no tagged release). Its
compiler is `tools\qb2js.bas`, a BASIC program that writes JavaScript. It is written in QBJS's own dialect
(`Import`, `As Object`, `AndAlso`), so QBJS compiles it (`node qbc tools/qb2js.bas qb2js.js`) and QB64pe cannot.
`boxgaming/gx` holds a copy that `study\archive\44` describes as written in QB64.
A JavaScript runtime and the GX game engine come with it. QB64pe's IDE has a "QBJS Web Build" dialog that runs
it (`study\archive\04` §2.1). Programs can be written and run in its online IDE, but they do not need it. QB64pe's
`internal\support\converter\qbjs-build.bas` compiles ahead of time with Node (`node qbc.js prog.bas
program.js`, its `CompileSource`). It writes a folder of static files, `<source folder>_web`, holding
`index.html`, `program.js`, QBJS's runtime and the program's data files. It can be put on any host and needs no
special headers, because it runs on one thread.

Its page [QBasic Language Support](https://github.com/boxgaming/qbjs/wiki/QBasic-Language-Support) says it
supports "a large subset". It lists these differences:

- no `GOTO` or `GOSUB`; labels only for `DATA`, `READ` and `RESTORE`;
- parameters passed by value, so a SUB that assigns to a numeric or string parameter does not change the
  caller's variable (changes to array elements and `TYPE` members do reach it);
- no `STRING * n`, no `_BIT * n`, no `_MEM`;
- metacommands mostly ignored; it adds `$If Javascript` (inline JavaScript) and `$If WEB`.

Its page [Unsupported Keywords](https://github.com/boxgaming/qbjs/wiki/Unsupported-Keywords) is longer. It lists,
among others: `ON ERROR`, `RESUME`, `ERR`, `ERL`, `DEFINT` and the other `DEF` statements, `FIELD`, `LSET`,
`RSET`, `RANDOM` files, `GET` and `PUT` for graphics, `PALETTE`, `PCOPY`, `VIEW`, `VIEW PRINT`, `WIDTH`, `TAB`,
`SPC`, `PEEK`, `POKE`, `DEF SEG`, `ON TIMER`, `ON KEY`, `CHAIN`, `SHELL`, `DECLARE LIBRARY`, `$CONSOLE`,
`$RESIZE`, `_MAPTRIANGLE`, `_SNDRAW`, `_SNDCOPY`, the whole `_MEM` family, `_OPENHOST` and the other network
words, and several QB64pe words (`$EMBED`, `_ARRAYCOPY`, `_FILES$`, `_CRC32`, `_MD5$`). The page sets some
entries in italics without saying what the italics mean, so a few of these may be partial.

What follows for us:

- QBJS is a way to put suitable programs on the web, not a QB64pe-compatible compiler. The old programs this
  project exists for use `GOSUB`, `ON ERROR`, `DEFINT`, `FIELD`, `STRING * n` and SUBs that change their
  arguments. Such programs would not build under QBJS, or would build and run differently. Passing by value is
  the dangerous case, because nothing reports it.
- Civil War Strategy uses `$RESIZE:STRETCH` (`cws.bas` line 5) and `DEF SEG` with `BLOAD` for its pictures
  (`load_picture`, `draw\pictures.bm`; the game's `TODO.md`). Both are on the unsupported list.
  What QBJS lacks for the game, measured by compiling it on 2026-11-16, and the plan to add it in our fork
  are in the game's repo: `..\civil-war-strategy\docs\qbjs-plan.md`.
- A "QBJS-compatible" lint is possible, taking a few hours of Claude's time. Once we have a web target of our
  own its value is small, and the lists change with QBJS's releases. **Not recommended.**

## 3. What a new target touches today

The pipeline is `syntax` → `sema` → `ir` → `codegen-cpp`, which writes the C++ fragments that libqb's `qbx.cpp`
includes. The driver then builds with the copy of QB64pe's `Makefile` (`libqb\`) and a C++ toolchain
(`crates\README.md`). So a new target is mostly a new C++ toolchain and a libqb that builds for it. What stands
in the way, found by reading the code:

| Item | Where | What it means for the web or a phone |
|---|---|---|
| libqb refuses unknown systems | `libqb-common.h`: `#error "Unknown system; refusing to build"` unless Windows, Apple or `__linux__` | Emscripten defines `__EMSCRIPTEN__`, `__wasm__` and `__wasm32__` ([CPython's Emscripten notes](https://chromium.googlesource.com/external/github.com/python/cpython/+show/1e4ed932109c4f8ee4f43bc3e6fdb7710a3606bd/Platforms/emscripten/README.md)). Whether it also defines `__linux__` is unchecked: no source says either way, and `emcc -dM -E` settles it (CPython's notes give that command). Either way libqb needs a `QB64_EMSCRIPTEN` branch, as a libqb edit, so that it does not take Linux's paths (X11, ALSA) |
| Thread handshakes rely on x86 memory order | `study\03` §1.5: `lock_display`, `autodisplay`, `display_frame[].state`, `exit_ok`, `qbevent` are plain `int32` polled across threads | ARM phones, and WebAssembly threads, can break them. `SOMEDAY.md`'s row "Redesign the runtime ABI" names this as a reason to bring the redesign back. The narrow fix is to make these flags `std::atomic`, as libqb edits, without the rest of that redesign |
| Fixed-function OpenGL in the presentation path | `libqb.cpp` 24226–25715: `glMatrixMode`, `glEnableClientState`, `glVertexPointer`, `glDrawArrays`, about 34 call sites, and GLU's `gluBuild2DMipmaps` | WebGL 2 and OpenGL ES have no fixed-function pipeline. The path draws textured triangles from client arrays, so it can be rewritten as a small shader path. One rewrite serves the web and phones |
| `SUB _GL` | User code calls OpenGL 1.x directly (`study\03` §3.7) | Emscripten can emulate some of OpenGL 1.x, but its [OpenGL support page](https://emscripten.org/docs/porting/multimedia_and_graphics/OpenGL-support.html) says that emulation "is by no means complete". So report `SUB _GL` as not available on these targets |
| GLFW | `glut-emu.cpp` wraps GLFW 3.5.1 | Emscripten has a fuller GLFW port than its built-in one, `--use-port=contrib.glfw3` ([Contrib ports](https://emscripten.org/docs/compiling/Contrib-Ports.html)), so the web can keep GLFW. Its [README](https://github.com/pongasoft/emscripten-glfw) (Apache 2.0, release 3.5.1.20261109) says it implements GLFW 3.5.1, the version libqb uses, and lists keyboard, joystick and gamepad, fullscreen and threads. Touch came in 3.4.0.20250209 ("Added support for touch (mobile)"); the README does not say whether a touch becomes a mouse click. GLFW has no Android or iOS port: the [GLFM](https://awesome.ecosyste.ms/projects/github.com%2Fbrackeen%2Fglfm) README says to use GLFW on the desktop and GLFM on phones. A native phone build needs another layer, such as SDL3 ([SDL3 README-emscripten](https://wiki.libsdl.org/SDL3/README-emscripten) shows the same API on the web) or GLFM |
| The main loop blocks | `GLUTEmu::MainLoop`: `while (!glfwWindowShouldClose) glfwPollEvents()` on the first thread | A browser's main thread must return to the browser each frame. On the web this loop becomes `emscripten_set_main_loop`. One function changes |
| Every target is 64-bit | `sema`: `_OFFSET` is `int64` (`crates\sema\src\lib.rs` line 96, "the targets are 64-bit"); `ptrszint` in `codegen-cpp` | `wasm32` has 32-bit pointers. Either use 64-bit WebAssembly memory, or make the pointer width a property of the target in `sema`. See §5.3 |
| `_FLOAT` is x87 80-bit | `rounding.h` uses x87 assembly unless `QB64_NOT_X86`; `long double` is 16 bytes | On `wasm32`, `long double` is IEEE quad precision: 16 bytes, 113 bits of mantissa, and a maintainer says this "can't be changed at this point" ([emscripten-discuss, "The long double"](https://groups.google.com/g/emscripten-discuss/c/GLGQ8AaeLZk/m/W36jCz8gDAAJ)). The same size as x87's 80-bit value stored in 16 bytes, but a different result, done in software. `CLAUDE.md` working rule 8 (user, 2026-10-18) covers this: a well-defined value and a `DIVERGENCES-QB64PE.md` row |
| Number formatting depends on the C library | `qbs_str.cpp` post-processes `sprintf("% .6E")` (`study\03` §9.1) | Emscripten uses musl's `printf` ([musl list, 2019](https://openwall.com/lists/musl/2019/09/13/6)), changed to print floats at `double` precision only ([musl list, 2024](https://openwall.com/lists/musl/2024/06/13/12)). That does not hurt `STR$` of a `_FLOAT`: libqb already converts it to `double` first (`qbs_str(long double)`, `libqb\src\qbs_str.cpp` lines 252–255, "not fully implemented"). Any difference from MinGW's output shows in the Node runs of §5.6 |
| Errors | No `setjmp` and no C++ exceptions in `qbx.cpp`, `libqb.cpp` or `libqb\src` (one textual match only) | The error model is return-based, so it compiles to WebAssembly unchanged |
| Features with no browser equivalent | `SHELL`, `RUN "file"`, `CHAIN`, TCP (`_OPENHOST`, `_OPENCLIENT`), `COM`, printing, `_SCREENIMAGE`, clipboard images, `DECLARE LIBRARY` of a `.dll` or `.so` | libqb already has stub modules for some (`qb_http-stub.cpp`, `stub_audio.cpp`, `stub_font.cpp`, `window-console.cpp`). See §5.5 |

Upstream QB64pe has no web or Emscripten work of its own. In `..\QB64pe-latest\internal\c` only vendored
libraries mention it. miniaudio has a Web Audio backend (`miniaudio.h`), which matters for §5.4.

## 4. Ways to reach the web, Emscripten and the others

### 4.1 What the browser runs

A browser runs two kinds of code: JavaScript, and WebAssembly. WebAssembly is a compact binary format for
compiled code. Every current browser runs it: Chrome since 57, Firefox 52, Safari and iOS Safari 11, about
97% of users ([caniuse](https://caniuse.com/wasm)). MDN says it "brings near-native performance to the web"
([MDN](https://developer.mozilla.org/en-US/docs/WebAssembly)). A WebAssembly module cannot reach the page by itself: it cannot draw, play sound or read
keys. So every way below has two parts:

1. the program's code, compiled to WebAssembly or JavaScript;
2. **glue**: JavaScript that connects the code to the browser. It creates the canvas, sends OpenGL calls to
   WebGL, passes keys and the mouse in, feeds the audio, and provides files and threads.

The choice is mostly a choice of who writes the glue.

### 4.2 The options

| Option | What it is | Who writes the glue | Fidelity to QB64pe | Verdict |
|---|---|---|---|---|
| **A. Emscripten, to WebAssembly** | `emcc`, a `clang` with a C library and browser glue, compiles the same C++ fragments and libqb | Emscripten. It ports GLFW and OpenGL ES to WebGL, and provides a virtual file system, threads as Web Workers, and a Node mode. miniaudio already knows it | The same runtime code. Differences come from the C library, `_FLOAT` and pointer width, and the Node runs measure them | **Recommended** |
| A2. Emscripten, to JavaScript | The same, with `-sWASM=0` | Emscripten | Same as A | **No**: see §4.3 |
| B. Cheerp | Another `clang`-based C++ compiler for the web, by Leaning Technologies. It writes WebAssembly or JavaScript. Apache 2.0 with LLVM exceptions since 3.0 ([Cheerp 3.0 announcement](https://labs.leaningtech.com/blog/cheerp-3-now-permissively-licensed), [FAQ](https://labs.leaningtech.com/cheerp/faq)) | Mostly us: it has no GLFW port, and miniaudio does not target it (not checked further) | Same as A for the code it compiles | No: A's glue is the reason to pick A |
| C. Plain `clang` or `wasi-sdk` to WebAssembly | The LLVM toolchain alone, targeting `wasm32` | Us, all of it: window, WebGL, audio, input, files, threads | Same as A | No: weeks of glue that Emscripten already has |
| D. Run the native build in an emulator | [CheerpX](https://labs.leaningtech.com/cheerpx) JIT-compiles x86 Linux programs to WebAssembly in the browser; its demo, [WebVM](https://labs.leaningtech.com/blog/webvm-server-less-x86-virtual-machines-in-the-browser), is a whole Linux in a page. [Boxedwine](https://github.com/danoon2/Boxedwine) runs 32-bit Windows programs through Wine, compiled with Emscripten. [js-dos](https://js-dos.com/overview.html) runs DOS programs in DOSBox | The emulator | The real executable, so the best possible, but slow. Its [own TODO](https://github.com/danoon2/Boxedwine) says the web Boxedwine is "still slow"; [XDA](https://www.xda-developers.com/boxedwine-emulate-windows-applications-web-browsers/) measured its desktop build at about 25% of native speed in 2021 | No, as a build target. A whole operating system image is the download. CheerpX is free only for personal and most open-source use, and needs a license otherwise ([CheerpX licensing](https://labs-cheerpx.pages.dev/docs/licensing)). js-dos is GPL-2.0 ([js-dos](https://github.com/js-dos/dosbox)). Note: js-dos could run the original DOS build of Civil War Strategy, but that is the old PDS 7.1 program, not our compiler's |
| E. A runtime in Rust | A second backend and libqb rewritten in Rust, compiled to WebAssembly with Rust's own web tools (`wasm-bindgen`) | Us, in Rust | Only as good as the rewrite. libqb is about 27,000 lines in `libqb.cpp` plus 24,500 in `libqb\src`, without its vendored parts | Not for the web alone. Revisit only if the "Redesign the runtime ABI" row comes back for its own reasons |
| F. Our own JavaScript | `codegen-js` and a JavaScript runtime, as QBJS does | Us | Low, unless the whole runtime is rewritten in JavaScript | No |
| G. Hand off to QBJS | Write QBJS-flavored source and run its compiler | QBJS | QBJS's subset (§2) | No: it changes what programs do without saying so |

Option A keeps the project's main asset. The 4393 verification programs, the corpus and the upstream tests all
judge the same runtime code, so a web build can be held to the same recordings. Any option that replaces libqb
(E, F, G) starts that over.

### 4.3 Can Emscripten write JavaScript instead of WebAssembly?

Yes. `-sWASM=0` writes plain JavaScript and no `.wasm` file
([Building to WebAssembly](https://emscripten.org/docs/compiling/WebAssembly.html)). The choice is made at link
time. It does not change the object files. Emscripten still compiles to WebAssembly first, then converts the
result to JavaScript at the end, with Binaryen's `wasm2js`
([emscripten-discuss](https://mail-archive.com/emscripten-discuss@googlegroups.com/msg09846.html)). A
maintainer describes the output as similar to asm.js, but not the same.

It is not worth having here:

- Every current browser runs WebAssembly (§4.1, [caniuse](https://caniuse.com/wasm)), so the JavaScript
  output reaches no extra players.
- 64-bit integer arithmetic, which BASIC's `_INTEGER64` and libqb use, is split into pairs of 32-bit values
  by Binaryen's `I64ToI32Lowering` pass, which wasm2js runs (`runner.add("i64-to-i32-lowering")` in
  [`src/wasm2js.h`](https://github.com/WebAssembly/binaryen/blob/main/src/wasm2js.h);
  [the pass](https://chromium.googlesource.com/external/github.com/WebAssembly/binaryen/+/HEAD/src/passes/I64ToI32Lowering.cpp)).
  That it makes the output larger and slower is unchecked: no published measurement was found.
- Binaryen's [README](https://github.com/WebAssembly/binaryen) says wasm2js cannot "match WebAssembly
  semantics 100% precisely with fast JavaScript code". It assumes that loads, stores and float-to-integer
  conversions do not trap, and warns of "slight differences in corner cases of conversions". BASIC's numeric
  rules live in exactly those conversions.
- Whether it supports threads, which libqb needs (§5.1), is not documented. Neither Emscripten's
  [threads page](https://emscripten.org/docs/porting/pthreads.html) nor Binaryen's README mentions wasm2js with
  threads, and no search found an answer. One build with the SDK would settle it (§11).

## 5. The web target in detail (option A)

### 5.1 Threads, or one thread

libqb already runs the BASIC program on its own thread (`QBMAIN`), with the window and OpenGL on the first
thread, and a timer thread and a housekeeping thread (`MAIN_LOOP`) beside them (`study\03` §1.1). That maps
onto the browser well:

- The page's main thread runs GLFW's callbacks and drawing through `emscripten_set_main_loop`.
- `QBMAIN`, the timer thread and `MAIN_LOOP` are Web Workers (Emscripten's threads). A worker may block and
  spin. So `INKEY$` loops, `SLEEP`, `INPUT` and the spin handshakes keep working as they do natively, without
  freezing the page.
- libqb starts its threads in `main()` and expects them to run at once (`study\03` §1.2). Emscripten's
  [threads page](https://emscripten.org/docs/porting/pthreads.html) says that creating a new Web Worker
  "requires returning the main event loop", and that a pool (`PTHREAD_POOL_SIZE`) "creates the Web Workers
  before main is called". So the build sets a pool large enough for `QBMAIN`, the timer, `MAIN_LOOP`,
  miniaudio's thread and libcurl's, if HTTP is ever mapped: about 4 to 6.
- The same page says that blocking on the browser's main thread is "a busy-wait … which can make the browser
  tab unresponsive". In libqb, the main thread waits for `QBMAIN` only to run `SUB _GL` (`study\03` §1.5), and
  `SUB _GL` is not available on the web (§5.5). Every other wait is on a worker.
- The page also lists what threads cannot do: no POSIX signals and no `fork()`. libqb installs `SIGFPE` and
  `SIGSEGV` handlers in `QBMAIN` (`study\03` §1.2). The division handler is only a backstop, because
  `qb_safe_idiv` raises the error itself (`study\03` §9.2 item 11). On the web the handlers are left out.

The cost is hosting: threads need two HTTP headers (§6.2).

The other way is one thread with **Asyncify** or **JSPI**. The whole program runs on the main thread and is
suspended at each wait (`evnt()`, `INKEY$`, `_LIMIT`, `_DISPLAY`, `SLEEP`). It needs no headers. But a loop
that waits without such a call freezes the tab. Civil War Strategy's `menu_wait_key$` (`ui\menu.bm` line 222)
is `DO: pressed$ = INKEY$: LOOP WHILE pressed$ = ""`, so `INKEY$` itself would have to suspend. Asyncify adds
"both code size and slowness", "usually not extreme, something like 50% or so"; JSPI does not
([Emscripten: Asyncify](https://emscripten.org/docs/porting/asyncify.html)). JSPI ships in Chrome 137 and later
([Chromium intent to ship](https://groups.google.com/a/chromium.org/g/blink-dev/c/w_jCD4gf7Bc),
[caniuse](https://caniuse.com/wf-wasm-jspi)). A 2026 summary says Firefox plans to turn it on by default in
Firefox 153 and Safari has it in the beta of Safari 27
([OpenReplay](https://blog.openreplay.com/jspi-javascript-wasm-bridge/)). That summary is secondhand.

**Recommendation:** threads first. They match libqb's design and need no change to how the generated code
waits. Keep JSPI as a later option for hosts that cannot send the headers, once Firefox and Safari ship it.

#### What a one-thread build would cost

Added 2026-11-19, when the user asked "how much work is it to have a one-thread fallback?". It is an estimate
from reading `study\03` §1, not a measurement. Nothing is decided.

On one thread, the BASIC program, the timer and the housekeeping loop (`MAIN_LOOP`) share the page's main
thread. The program is suspended wherever it waits, and the browser runs the rest meanwhile. What has to
change in libqb, as edits behind a build flag:

- **Every wait becomes a suspension.** `_LIMIT`, `_DELAY` and `SLEEP` already call `evnt(0)` about every 9 ms
  while they wait (`study\03` §1.6), so they share one place. `INKEY$`, `_KEYHIT`, `INPUT` and `_DISPLAY` need
  one each.
- **The spin handshakes become plain calls.** `lock_display`, `lock_mainloop`, `autodisplay` and
  `ontimerthread_lock` have one thread set a flag and spin until another answers (`study\03` §1.5). On one
  thread nobody answers, so each of them would hang. Each is replaced by doing the other side's work directly.
- **The timer thread and `MAIN_LOOP` become callbacks.** `ON TIMER` is checked at each suspension. `display()`,
  `snd_update()` and the BIOS tick run from the browser's frame callback.
- **A loop that never waits must still let the page run.** The generated code already tests `qbevent` between
  statements. On one thread nothing sets it, so the test would have to read a clock or a counter instead, and
  suspend about every 10 ms. That slows every program a little. How much is not measured.
- **`END` and shutdown.** `end()` parks the program's thread forever, and `MAIN_LOOP` waits for `exit_ok`
  (`study\03` §1.7). Both are rewritten.
- **Sound.** Whether miniaudio's Web Audio backend plays without threads was not checked.

That is about 15 to 25 places in `libqb.cpp`, `qbx.cpp` and `libqb\src`. In Claude's time it is two to four
sessions after W1. The larger cost stays: two forms of the runtime, so every browser test runs twice, and
every later libqb edit near a wait is written twice.

The suspension itself is JSPI or Asyncify. JSPI is in Chrome 137 and later. Mozilla posted an intent to turn
it on by default in Firefox 153, on 2026-06-10
([Mozilla intent to ship](https://groups.google.com/a/mozilla.org/g/dev-platform/c/8cOxM-dWaNs), read through
a search result's summary, not the post). Whether Firefox has shipped it, and Safari's state, are unchecked.
A browser without JSPI needs the Asyncify build, which is about 50% larger and slower (the Asyncify page
cited above).

Which hosts need it:

| Host | Threaded build | One-thread build needed? |
|---|---|---|
| Your own server, Netlify, Cloudflare Pages | Works: the host sends the two headers | No |
| GitHub Pages | Works with `coi-serviceworker` (§6.3), one reload on the first visit | No |
| itch.io | Works in Chrome and Firefox; likely not in Safari or on iPhones (§6.3, not tested) | Yes, for Safari |
| An app's web view (Capacitor, §7.1) | Reports say the headers do not take effect | Yes, or a local server inside the app |

**Recommendation (Claude, 2026-11-19): do not build it now, and decide after W1.** By then a threaded game
exists and can be tried on itch.io in Safari, which settles the one case that is uncertain. Nothing in W0 or
W1 makes the one-thread build harder, provided W1 adds no new spin handshake.

The panel, with the web platform and mobile engineers of §9:

- **Pragmatic engineer:** wait. Three of the four kinds of host need nothing. The work is real and the
  doubled testing never goes away.
- **QB64 engineer:** the threaded build is the one that behaves like QB64pe. On one thread, timing changes:
  `ON TIMER` fires only at suspensions, and a tight loop runs slower.
- **Compiler/languages engineer:** the `qbevent` test between statements is the only place the generated C++
  would change. Keep it a libqb macro, so that the fragments stay the same for both builds.
- **Rust engineer:** for the driver it is one more build variant, with its own libqb objects. Small.
- **Test engineer:** it doubles the browser runs. Take that on only for a host a real game needs.
- **Web platform engineer:** itch.io is where hobby games are published, so this will likely be wanted one
  day. Test the threaded build there before paying for it.
- **Mobile engineer:** store wrappers need it, or a local server. That is P2, which is not planned yet.

### 5.2 Graphics

- Keep GLFW through Emscripten's `contrib.glfw3` port, so `glut-emu.cpp` keeps its shape. Change its main loop
  (§3).
- Rewrite the presentation path (`libqb.cpp` 24226–25715) to WebGL 2 / OpenGL ES 3 with one textured-triangle
  shader. Put it behind a switch, so the desktop build keeps its fixed-function path until the new one has
  passed the screen tests there too. The software drawing (`LINE`, `CIRCLE`, `PAINT`, `_PUTIMAGE` to software
  images, text) is unchanged, because it draws into memory (`study\03` §3.3).
- Hardware images (`_LOADIMAGE(…, 33)`, `_PUTIMAGE` to `_DISPLAY` layers) go through the same path, so they
  come with the rewrite.
- `SUB _GL`, `_GLRENDER` and the `_gl*` routines: a compile-time error on the web target ("not available on the
  web target").

### 5.3 Pointer width

Two choices:

1. **`wasm32`, with `_OFFSET` 4 bytes on that target.** `sema` takes the pointer width from the target. The
   width is visible to programs: `LEN` of an `_OFFSET`, the layout of a `TYPE` holding one, and `LEN` of a
   `_MEM`. QB64pe does the same: a 32-bit build sets `OFFSETTYPE` to 32 bits (`..\QB64pe\source\utilities\type.bi`
   line 88), and the setting `-f:TargetBits=32` asks for one (`source\qb64pe.bas` lines 14537–14538), so it has a precedent. It runs in
   every browser, including Safari on iPhones.
2. **64-bit memory (memory64, `-sMEMORY64`).** Nothing in `sema` changes. Chrome and Firefox support it
   ([Chromium intent](https://groups.google.com/a/chromium.org/g/blink-dev/c/5vTbd1dttwc),
   [Mozilla intent](https://groups.google.com/a/mozilla.org/g/dev-platform/c/I-MywEnFxKc)). Safari, including
   iOS, does not, through Safari 26.5 ([caniuse](https://caniuse.com/wf-wasm-memory64)). WebKit has an open
   bug for it ([WebKit bug 300538](https://bugs.webkit.org/show_bug.cgi?id=300538)). That would rule out
   iPhones and iPads (§7).

**Recommendation:** `wasm32`. Start from `sema`'s `_OFFSET` width, the `LEN` rules, `TYPE` layout and the C types
`codegen-cpp` writes. The 32-bit path in libqb (`QB64_32`) exists already. Programs that assume 8 bytes for an
`_OFFSET` in a file record get a recording for this target.

### 5.4 Sound, files, input

- **Sound:** miniaudio's Web Audio backend. Browsers block audio started "in a tab which has not yet had any
  user interaction", and Web Audio's `start()` outside an input event "is subject to autoplay rules"
  ([MDN: Autoplay guide](https://developer.mozilla.org/en-US/docs/Web/Media/Guides/Autoplay)). So the page shows "click to start" or starts audio at the first input.
- **Files:** a virtual file system. The program's own files are packed with it (Emscripten's `--preload-file`)
  and appear in the folder that libqb changes to at start. Written files go to the browser's storage, so that
  saved games survive a reload. The list of files to pack goes in `qb64rust.toml`; `$EMBED` already covers
  single files.
- **Keyboard and mouse:** GLFW's port maps browser events. Some keys, such as Escape, Alt+Tab and Ctrl+N,
  "are typically captured by the user agent or the underlying operating system" and may never reach the
  program. The experimental Keyboard Lock API can claim some of them in full screen
  ([MDN: Keyboard API](https://developer.mozilla.org/en-US/docs/Web/API/Keyboard_API)). `_FULLSCREEN` maps to the browser's fullscreen
  request, which needs a click or a key press.
- **Gamepads:** the port lists "joystick/gamepad" among its features
  ([emscripten-glfw](https://github.com/pongasoft/emscripten-glfw)), so libqb's game controller code may need
  only a backend for it. That backend was not looked at.
- **Touch:** the port supports touch since 3.4.0.20250209 (same README). Whether a touch reaches libqb as a
  mouse click is tested in P1 (§7.2).
- **Clipboard:** text through the browser's clipboard API. It is asynchronous, and a read needs a recent user
  action, a permission or a "Paste" prompt, which differ by browser
  ([MDN: Clipboard API](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard_API)). First cut: `_CLIPBOARD$` reads what the program itself set.

### 5.5 What the web target does not have

A program that uses any of these is rejected at compile time with "not available on the web target". It is
never built with a stub that quietly does nothing. This is the project's rule for unsupported things ("not
supported yet", never wrong code), applied per target:

- `SHELL`, `_SHELLHIDE`, `RUN "file"`, `CHAIN`;
- `_OPENHOST`, `_OPENCLIENT("TCP/IP:…")`, `_OPENCONNECTION`: browsers have no raw sockets. `_OPENCLIENT("HTTP:…")`
  could map to `fetch` later;
- `OPEN "COMn:"`, `LPRINT`, `_PRINTIMAGE`, `_SCREENIMAGE`, `_SCREENCLICK`, `_SCREENPRINT`;
- `SUB _GL` and the `_gl*` routines;
- `DECLARE LIBRARY` of a prebuilt `.dll` or `.so`. A `DECLARE LIBRARY` whose library is a C header or source
  file can be compiled by `emcc` with the program, so it may stay;
- the native dialogs (`_OPENFILEDIALOG$` and the others), which could later map to the browser's file picker.

Mechanism: a per-target column in `tools\builtins\builtins.json`, which `sema` reads like the "not supported
yet" marks, plus a check of `DECLARE LIBRARY` and `SUB _GL`. One table then answers "what does the web target
support" for the compiler, the language reference and the editor.

### 5.6 Testing

- **Console programs under Node.** Emscripten can build a program to run under Node with access to the real
  file system (`NODERAWFS`). It "replaces all normal filesystem access with direct Node.js operations", and the
  starting folder is the process's own ([Emscripten: File System API](https://emscripten.org/docs/api_reference/Filesystem-API.html)). Of `verification\`, 4158 of 4459 programs are `$CONSOLE:ONLY`;
  of `tests\corpus`, 427 of 516. The runner gains `--target wasm`, which builds with `emcc` and runs
  `node prog.js` against the same recordings. A `known.wasm.list` sits beside `known.linux.list`, as `m7-linux`
  set up per platform. Large runs go in the suites VM (`CLAUDE.md` working rule 6).
- **Screen programs in a headless browser.** `tools\screen` reads a program's final screen state. The web
  build can expose the same state to a headless Chromium driven by Playwright, so the `screen/` corpus
  programs are judged the same way.
- **CI:** Node and a headless Chromium both run on GitHub's Linux runners.

## 6. What it takes to put a WebAssembly game on the web

### 6.1 The files

A build for the web is a folder of static files:

| File | What it is |
|---|---|
| `game.html` | The page: a canvas, and a line that loads `game.js` |
| `game.js` | Emscripten's glue (§4.1) |
| `game.wasm` | The compiled program and libqb |
| `game.data` | The packed files: for Civil War Strategy, `graphics\` and `scenarios\` |
| `coi-serviceworker.js` (optional) | Adds the two headers on hosts that cannot send them (§6.3) |

Before Emscripten 3.1.58 (April 2024), a threaded build also wrote a `.worker.js` file. Since then, the
workers start from `game.js` itself
([emscripten-discuss PSA](https://groups.google.com/g/emscripten-discuss/c/rNEdu0qUhH0),
[pull request 21701](https://github.com/emscripten-core/emscripten/pull/21701)).

The player installs nothing. They open a link in a current browser, on a desktop or on a phone.

### 6.2 What the server must do

1. **Serve over HTTP, not from the disk.** Opening `game.html` from a folder (`file://`) does not work. The
   fastest way to load WebAssembly, `WebAssembly.instantiateStreaming`, needs a response whose content type is
   `application/wasm`. A file read from disk has none, and the call then fails
   ([W3C WebAssembly Web API](https://www.w3.org/TR/2024/WD-wasm-web-api-2-20240822/),
   [Deno manual](https://docs.deno.com/runtime/manual/runtime/webassembly/using_streaming_wasm)). Chrome also
   checks it for `file://`
   ([chromium-discuss](https://groups.google.com/a/chromium.org/g/chromium-discuss/c/_tO-GvjjTvY)). So the
   player always reaches the game through a web server. For testing on your own machine, that is a local
   server, which `qb64rust serve` would be (§6.4).
2. **Send `.wasm` as `application/wasm`.** nginx's own `mime.types` maps it (changeset "MIME: added
   application/wasm type", [ticket 1606](https://trac.nginx.org/nginx/ticket/1606)). That the hosts of §6.3
   do is unchecked: their documents do not say. The first upload settles it.
3. **Send the two headers that turn threads on.** Browsers give `SharedArrayBuffer`, which threads need, only
   to a "cross-origin isolated" page. That is a page served with `Cross-Origin-Opener-Policy: same-origin` and
   `Cross-Origin-Embedder-Policy: require-corp` ([web.dev](https://goo.gle/2NATe2S)). Safari has supported
   these headers, and threads, since 15.2 on macOS and iOS
   ([WebKit, Safari 15.2](https://webkit.org/blog/12140/new-webkit-features-in-safari-15-2/)). Use
   `require-corp`, not `credentialless`, for Safari
   ([Shen.ai troubleshooting](https://developer.shen.ai/getting-started/web-troubleshooting)). Once the page is
   isolated, everything it loads from other sites must allow it. A self-contained game folder loads nothing
   from other sites, so this costs it nothing.
4. **HTTPS**, for the service worker of §6.3 and for installing to a phone's home screen (§7). The hosts
   below serve HTTPS: GitHub Pages for `github.io` sites
   ([GitHub Docs](https://docs.github.com/en/pages/getting-started-with-github-pages/securing-your-github-pages-site-with-https)),
   Netlify on "all sites" ([Netlify Docs](https://docs.netlify.com/domains-https/https-ssl/)), and itch.io,
   "an HTTPS website with no exceptions" ([itch.io](https://itch.io/docs/creators/html5)). Cloudflare Pages'
   limits page does not say; that it serves `*.pages.dev` over HTTPS is unchecked.

### 6.3 Where it can be hosted

| Host | Can it send the headers? | Notes |
|---|---|---|
| Your own server (nginx, Apache, Caddy) | Yes: two lines of configuration | Full control |
| Netlify, Cloudflare Pages | Yes: a `_headers` file in the folder ([Wasmer's guide](https://docs.wasmer.io/sdk/wasmer-js/how-to/coop-coep-headers), [engineered.at](https://engineered.at/articles/setting-the-coop-and-coep-headers-on-static-hosting-like-github-pages)) | Both have free plans ([Netlify pricing](https://www.netlify.com/pricing/), [Cloudflare Pages limits](https://developers.cloudflare.com/pages/platform/limits/)). Cloudflare Pages takes at most 20,000 files, and **25 MiB per file**, so a large `game.wasm` may not fit there (same page) |
| GitHub Pages | No ([GitHub community discussion 13309](https://github.com/orgs/community/discussions/13309)) | The usual workaround is [`coi-serviceworker`](https://github.com/gzuidhof/coi-serviceworker) (MIT). It is a service worker that adds the headers. The page reloads once on a player's first visit, so that the worker can take over. The script must be its own file, served from the game's own site rather than a CDN, over HTTPS or `localhost` ([engineered.at](https://engineered.at/articles/setting-the-coop-and-coep-headers-on-static-hosting-like-github-pages), [stefnotch on dev.to](https://dev.to/stefnotch/enabling-coop-coep-without-touching-the-server-2d3n)) |
| itch.io | Yes, by a setting: "SharedArrayBuffer support", under Frame Options in the game's Embed Options ([itch.io forum](https://itch.io/t/2025776/experimental-sharedarraybuffer-support), [a Godot developer's notes](https://gist.github.com/nisovin/577e5596a36e899e8932747de1b7a353)) | Marked experimental. With it on, requests to other sites fail and other embeds on the page stop working. It sends `COEP: credentialless`, not `require-corp` ([gist](https://gist.github.com/nisovin/577e5596a36e899e8932747de1b7a353)). Firefox has worked with it since February 2023, through an origin trial for itch.io's domain (same gist; secondhand). The gist says Safari lags. Safari needs `require-corp` ([Shen.ai](https://developer.shen.ai/getting-started/web-troubleshooting)), so **a threaded game on itch.io will likely not run in Safari or on iPhones**. Not tested. A single-thread build (JSPI, §5.1) would avoid this once Safari ships JSPI |

A single file is also possible. `-sSINGLE_FILE` embeds the `.wasm` in the `.js`, and since the change above it
works with threads ([pull request 21701](https://github.com/emscripten-core/emscripten/pull/21701)). It still
needs a server that sends the headers.

### 6.4 What QB64Rust would ship

- `qb64rust -x --target web prog.bas` writes the folder of §6.1.
- `qb64rust serve <folder>` serves it on `localhost` with the right headers and the right content types.
- The extension gets "Run in browser".
- The toolchain is the Emscripten SDK, pinned to one version and downloaded by `qb64rust toolchain install
  --target web`, as llvm-mingw is now. `emsdk install` fetches prebuilt tools
  ([Download and install](https://emscripten.org/docs/getting_started/downloads)). No source gives the size
  of an install. The official Docker image, which holds the SDK and a Linux base, is 770 to 810 MB compressed
  ([Docker Hub](https://hub.docker.com/r/emscripten/emsdk/tags)). That is only a rough guide.
- The size of a build and its startup time were not measured. libqb is large, so dropping unused parts by the
  `DEPENDENCY_*` flags (`study\03` §8.3) matters more here than on the desktop.

## 7. Phones and tablets

### 7.1 Working on a phone is not the same as a native app

The user's point is that the program must work on a phone; whether the app is native matters less. Three ways
reach a phone, and the first two use the web build unchanged:

| Way | What | Needs | Verdict |
|---|---|---|---|
| **P1. The web build, in the phone's browser** | The player opens a link. With a web app manifest, served over HTTPS, they can also install it to the home screen and start it like an app, with no browser bar (`display: standalone`). iOS 16.4 and later installs from the Share menu of Safari, Chrome and others; Chrome on Android installs it as an app ([MDN: Making PWAs installable](https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps/Guides/Making_PWAs_installable)) | The web target, plus the input and screen work of §7.2 | **First.** No store, no signing, no Mac. Threads work on iOS Safari 15.2 and later ([WebKit](https://webkit.org/blog/12140/new-webkit-features-in-safari-15-2/)) and in Chrome on Android 88 and later, for cross-origin isolated pages ([Chrome blog](https://developer.chrome.com/blog/enabling-shared-array-buffer)) |
| **P2. The same build in a store wrapper** | On Android, a **Trusted Web Activity**: an app that opens the game's own site full screen in Chrome, with no browser bar. The site proves it belongs to the app through Digital Asset Links, which are tied to the app's signing key. If the check fails, the app falls back to a Chrome tab ([Android: TWA quick start](https://developer.android.com/develop/ui/views/layout/webapps/guide-trusted-web-activities-version2)). Google's [Bubblewrap](https://unpkg.com/@bubblewrap/cli@1.24.1/README.md) command builds it and checks the site against the PWA criteria; the Play Store takes an app bundle ([web.dev](https://web.dev/articles/using-a-pwa-in-your-android-app)). On iOS and Android, [Capacitor](https://forum.ionicframework.com/t/what-code-language-capacitor-runs-on/213244) (MIT) wraps the files in an app with its own web view | TWA: the site hosted with the headers, Android SDK, signing keys. Capacitor on iOS: a Mac with Xcode and an Apple developer account | **Second**, when a game should be in a store. **On Android, prefer a TWA**: it is Chrome, so threads should work as on the site (my inference, not tested). In a web view they may not. A 2022 report says Android's WebView ignores the opener header, so it cannot enable threads ([Capacitor issue 6182](https://github.com/ionic-team/capacitor/issues/6182); the reporter's claim). On iOS, headers served through a custom URL scheme left `crossOriginIsolated` false on a real iPad ([a 2026 test](https://github.com/logos-fleet/logos-workspace/issues/287)). The same 2022 reporter found the headers lost after the iOS web view reloaded, and used a local web server inside the app instead (issue 6182). So threads in Capacitor need a local server in the app, or the single-thread build |
| P3. A native app | libqb built with the Android NDK or for iOS, with SDL3 or GLFM in place of GLFW, the OpenGL ES path of §5.2 and the atomics fix of §3 | Gradle or Xcode packaging, the app's life cycle | **Only if P1 and P2 fall short**: speed, `DECLARE LIBRARY` of native code, or a store refusing a wrapper. iOS also needs a Mac, and macOS was postponed (user, `DECISIONS.md` 2026-11-07) |

P1 needs the `wasm32` choice of §5.3. With memory64, iPhones could not run the game at all.

### 7.2 What any QB64 program meets on a phone

These hold for every way above. They are the real work of "mobile":

- **No keyboard.** Programs read `INKEY$`, `INPUT` and `_KEYHIT`. The phone's own keyboard opens only for a
  focused text field, and the browser then shrinks the page to make room for it
  ([MDN: VirtualKeyboard API](https://developer.mozilla.org/en-US/docs/Web/API/VirtualKeyboard_API)). So the page needs a key bar of
  its own: buttons drawn by the page that send keys to the program.
- **Touch as a mouse.** A tap is a click of button 1. A long press could be button 2.
- **Screen size.** A 640×480 screen drawn on a phone. libqb's letterboxing and `$RESIZE:STRETCH` (`study\03`
  §3.8) already scale it to the canvas. Landscape fits a 4:3 screen best.
- **The app's life cycle.** Browsers freeze pages in the background, and timers do not run while a page is
  frozen. They may also discard a page, and the page is not told when that happens
  ([Chrome: Page Lifecycle API](https://developer.chrome.com/docs/web-platform/page-lifecycle-api)). So timers
  jump when it resumes, and the program should save often.
- **Battery.** A loop that spins on `INKEY$` keeps one core at full speed, on the desktop as on a phone. On a
  phone that drains the battery and warms the device.
- **`END`'s "Press any key to continue"** needs a tap to count as a key.

### 7.3 Worked example: Civil War Strategy on a phone

Read from the game's repo (2026-11-15).

**What the game is now:**

- `SCREEN 12`: 640×480, 16 colors, an 80×30 text grid of 8×16 characters (`cws.bas` lines 158, and
  `draw\pictures.bm`, `draw\map.bm`, `game\victory_screens.bm`).
- `$RESIZE:STRETCH` (`cws.bas` line 5). It opens a 1280×960 window when `_DESKTOPWIDTH` and `_DESKTOPHEIGHT`
  allow (`cws.bas` line 159).
- Keys only; no mouse. The game's `TODO.md` lists "Mouse menus" as not done: "The menus read only keys
  (`menu_select`, `ui\menu.bm`)."
- The keys it uses: Enter, Esc, Up, Down, PgUp and PgDn, F3 (redraw), F4 (force report) and F7 (end turn), all
  defined in `cws.bi` lines 220–228. A letter picks the menu row that starts with it (`menu_letter_row`,
  `ui\menu.bm`).
- Typed text once: `INPUT commander$` for the Hall of Fame (`ui\victory.bm` line 65).
- Waiting for a key spins: `menu_wait_key$` (`ui\menu.bm` line 222) loops on `INKEY$` with no `_LIMIT`.
- Sound: `PLAY` tunes (`game\tunes.bm`).
- Files: pictures from `graphics\` through `DEF SEG` and `BLOAD`, maps from `scenarios\`. It writes `CWS.CFG`,
  `HISCORE.CWS`, `cws1.sav` to `cws9.sav`, `cws.his` and `battsumm` in the current folder (the game's
  `TODO.md`, "One known folder").
- No `SHELL` any more. The `SHELL "copy cws.his oldhist.ory"` that `STATUS.md` describes was in an earlier
  version of the game.

**What it needs for a phone, and who does it:**

| Need | How | Where the work is |
|---|---|---|
| Runs in the browser | The web target, W0 to W2 of §8 | This repo |
| Its files | `graphics\` and `scenarios\` packed into `game.data`. Saves, settings and the Hall of Fame kept in browser storage, so that they survive a reload | This repo (packing and storage) |
| `DEF SEG` and `BLOAD` | They work, because libqb emulates DOS memory (`study\03` §2.2) | Nothing |
| Its keys without a keyboard | A key bar on the page: Up, Down, Enter, Esc, PgUp, PgDn, F3, F4, F7. Its keys and labels set per game in `qb64rust.toml`, for example `F7 = "End turn"` | This repo (the key bar); the game's repo (its list of keys) |
| Picking a menu row | Best: the game reads taps as clicks with `_MOUSEINPUT` and picks the row tapped. That is the game's own "Mouse menus" item in its `TODO.md`. Without it, the player taps Up and Down on the key bar and then Enter, which works but is slow | The game's repo |
| Typing the commander's name | The page opens the phone's keyboard while the program waits in `INPUT`, and closes it after | This repo (libqb's `INPUT` tells the page it is waiting) |
| Screen size | Landscape, letterboxed. On a phone held sideways, the 480 rows fill the screen's height; the text is small but readable (not measured). On a tablet it is roomy. The game's 1280×960 choice comes from `_DESKTOPWIDTH`, which on the web is the browser window's size; stretching then fills the page | This repo (`_DESKTOPWIDTH` on the web) |
| Battery | `_LIMIT 30` in `menu_wait_key$`, or the runtime letting the thread rest when `INKEY$` returns nothing many times in a row. The first is the game's choice. The second changes only timing, not output, and would need a `DIVERGENCES-QB64PE.md` row | Either |
| Music | Starts at the first tap, as browsers require | This repo |
| Being suspended | The game saves when the player asks. An autosave at each end of turn would protect a game the phone closes | The game's repo, optional |

So for Civil War Strategy the work in this repo is the web target plus a key bar, the keyboard for `INPUT`, and
storage for saves. The work in the game's repo is tappable menus, a key list, and optionally `_LIMIT` and
autosave. The game's part belongs in its own `TODO.md` (`CLAUDE.md` working rule 2), once the user decides.

## 8. Order of work

Each step ends at a point where the user can stop and still have something.

| Step | What | Ends with | Claude's time, rough |
|---|---|---|---|
| W0 | `QB64_EMSCRIPTEN` branch and stubs as libqb edits; `emcc` as a toolchain in the driver (`--target wasm`); console programs only; the runner's `--target wasm` under Node | The share of the 4158 console verification programs that pass, and a list of where the rest differ | One to two sessions, plus the VM run |
| W0.5 | `sema`'s pointer width per target (§5.3); the thread flags as `std::atomic` (libqb edits, also run on the desktop suites) | W0's lists without the pointer-width failures; the handshakes safe on ARM | One session |
| W1 | Graphics in a browser: GLFW port, `emscripten_set_main_loop`, the WebGL 2 path, Web Audio, the virtual file system and storage, the checks of §5.5 | Civil War Strategy and the `screen/` corpus running in Chromium, judged by the screen state | Several sessions; the WebGL path is the largest part |
| W2 | `-x --target web` output, `qb64rust serve`, the toolchain download, "Run in browser", a reference chapter on the web target | A user can build a game and put it on Netlify, itch.io or GitHub Pages | One to two sessions |
| P1 | Web app manifest, touch as mouse, the key bar and its settings, the keyboard for `INPUT`, tested in Safari on iOS and Chrome on Android | Civil War Strategy playable on a phone from a link | One to two sessions, plus the user's time on real devices |
| P2 | Trusted Web Activity packaging with Bubblewrap, as documentation and a template; Capacitor for iOS later, with a local server in the app (§7.1) | A store-ready Android package | One session, plus the user's store accounts |
| P3 | Native Android, and iOS after macOS | — | Not estimated; decide after P1 |

W0 is an OpenSpec change (`CLAUDE.md` working rule 11): it adds a toolchain and a test kind. W1 and P1 would be
changes of their own. Together they would be a new milestone in `ROADMAP.md`. `STATUS.md` says there is no
open milestone now.

## 9. The panel

Roles: the five of `CLAUDE.md` working rule 4, plus a **web platform engineer**, for hosting, threads and
WebGL, and a **mobile engineer**, for phones, input and stores. Both topics are outside the usual five.

- **Pragmatic engineer:** W0 first, then look at its numbers. It reuses everything, costs little and tells us
  whether libqb under Emscripten is close. Promise "works on a phone" only after P1 has run on a real iPhone
  and a real Android phone. The user's time goes on hosting choices and devices, not on the build.
- **QB64 engineer:** Option A is the only one that keeps old programs honest. QBJS's by-value parameters would
  silently break many SUBs in older games. The per-target rejection list matters: a QB64 user must learn at
  compile time that `SHELL` is gone, not when the game reaches it.
- **Compiler/languages engineer:** The front end needs one real change: pointer width as a target property.
  Carry it as a proper target value from the driver to `sema`, not as a flag, because ARM Linux and macOS will
  want the same value.
- **Rust engineer:** The Rust side is small: a target value, a toolchain module for `emcc`, the runner option,
  `qb64rust serve`. A playground that compiles in the browser is a different project and stays out of scope.
  The build needs `emcc`, which does not run in a browser.
- **Test engineer:** The 4158 console programs under Node are the strongest early signal available, and they
  come almost free. Per-target known lists follow `m7-linux`'s pattern. Screen programs need a browser harness:
  new infrastructure, its own task in W1. Expect `_FLOAT` and number formatting to fill most of the first known
  list.
- **Web platform engineer:** Threads with the two headers are the right call for libqb. Test the hosts early,
  in Safari and Firefox as well as Chrome: itch.io's setting, `coi-serviceworker` on GitHub Pages, Netlify's
  `_headers`. That is where a build that works locally fails. itch.io's `credentialless` header already looks
  like such a failure for Safari. Keep `wasm32`, or iPhones are lost.
- **Mobile engineer:** For a hobby audience, the browser route is right: no store review, no fees, no Mac.
  Input is the hard part, not rendering. A key bar configured per game, and tappable menus in the game, matter
  more than any native work. Civil War Strategy is a good first case: a 4:3 screen, nine special keys, one
  typed name.

The panel recommends: approve W0 as the next OpenSpec change. If W0's numbers are good, plan W0.5 to W2 as a
web milestone, with P1 after it. Leave P2 and P3 in `SOMEDAY.md`.

## 10. Questions for the user

1. **Is a web target by option A (Emscripten, the same libqb) the approach?** The panel recommends it, over
   Cheerp, an emulator, a Rust runtime, our own JavaScript or QBJS (§4).
2. **Start W0 now, or after the open questions of `study\47` and `study\50`?** W0 changes nothing that those
   studies decide.
3. **Is it acceptable that `_OFFSET` is 4 bytes on the web target (`wasm32`)?** The alternative, memory64,
   rules out Safari, iPhones and iPads.
4. **Phones: is "works in the phone's browser, installable to the home screen" (P1) the goal for now, with
   store packaging (P2) and native apps (P3) left in `SOMEDAY.md`?** For Civil War Strategy it also means
   tappable menus in the game's own repo (§7.3).
5. **Hosting:** where must a built game run? A local server, itch.io, Netlify or Cloudflare Pages, GitHub Pages,
   your own site. This decides how much the one-thread fallback (JSPI) matters. itch.io in particular likely
   rules out iPhones for a threaded build (§6.3). Netlify, Cloudflare Pages or your own site do not.

## 11. Sources

Read 2026-11-15. The links are also given where each fact is used.

QBJS:
- [README](https://github.com/boxgaming/qbjs)
- [QBasic Language Support](https://github.com/boxgaming/qbjs/wiki/QBasic-Language-Support)
- [Unsupported Keywords](https://github.com/boxgaming/qbjs/wiki/Unsupported-Keywords)

Emscripten and other compilers:
- [Emscripten: Building to WebAssembly](https://emscripten.org/docs/compiling/WebAssembly.html) (`-sWASM=0`)
- [emscripten-discuss: WASM=0 still compiles to wasm, then converts](https://mail-archive.com/emscripten-discuss@googlegroups.com/msg09846.html)
- [Emscripten: Contrib ports](https://emscripten.org/docs/compiling/Contrib-Ports.html) (`contrib.glfw3`)
- [Emscripten: OpenGL support](https://emscripten.org/docs/porting/multimedia_and_graphics/OpenGL-support.html)
- [Emscripten: Pthreads support](https://emscripten.org/docs/porting/pthreads.html) (the pool, blocking on the main thread, no signals)
- [emscripten-discuss: no separate worker file](https://groups.google.com/g/emscripten-discuss/c/rNEdu0qUhH0), [pull request 21701](https://github.com/emscripten-core/emscripten/pull/21701) (and `SINGLE_FILE` with threads)
- [emscripten-glfw README](https://github.com/pongasoft/emscripten-glfw) (the `contrib.glfw3` port)
- [Binaryen README](https://github.com/WebAssembly/binaryen) (wasm2js)
- [CPython's Emscripten notes](https://chromium.googlesource.com/external/github.com/python/cpython/+show/1e4ed932109c4f8ee4f43bc3e6fdb7710a3606bd/Platforms/emscripten/README.md) (predefined macros)
- [emscripten-discuss: "The long double"](https://groups.google.com/g/emscripten-discuss/c/GLGQ8AaeLZk/m/W36jCz8gDAAJ)
- musl's `printf` in Emscripten: [musl list, 2019](https://openwall.com/lists/musl/2019/09/13/6), [musl list, 2024](https://openwall.com/lists/musl/2024/06/13/12)
- [Emscripten: Download and install](https://emscripten.org/docs/getting_started/downloads), [the emsdk Docker image](https://hub.docker.com/r/emscripten/emsdk/tags)
- [Cheerp 3.0, permissively licensed](https://labs.leaningtech.com/blog/cheerp-3-now-permissively-licensed), [Cheerp FAQ](https://labs.leaningtech.com/cheerp/faq)

Emulators:
- [CheerpX](https://labs.leaningtech.com/cheerpx), [WebVM](https://labs.leaningtech.com/blog/webvm-server-less-x86-virtual-machines-in-the-browser), [CheerpX licensing](https://labs-cheerpx.pages.dev/docs/licensing)
- [Boxedwine](https://github.com/danoon2/Boxedwine), [XDA on Boxedwine](https://www.xda-developers.com/boxedwine-emulate-windows-applications-web-browsers/)
- [js-dos overview](https://js-dos.com/overview.html), [js-dos DOSBox](https://github.com/js-dos/dosbox)

Browser features:
- JSPI: [Chromium intent to ship](https://groups.google.com/a/chromium.org/g/blink-dev/c/w_jCD4gf7Bc), [caniuse](https://caniuse.com/wf-wasm-jspi), [OpenReplay summary](https://blog.openreplay.com/jspi-javascript-wasm-bridge/) (secondhand)
- Memory64: [caniuse](https://caniuse.com/wf-wasm-memory64), [Chromium intent](https://groups.google.com/a/chromium.org/g/blink-dev/c/5vTbd1dttwc), [Mozilla intent](https://groups.google.com/a/mozilla.org/g/dev-platform/c/I-MywEnFxKc), [WebKit bug 300538](https://bugs.webkit.org/show_bug.cgi?id=300538)
- Cross-origin isolation: [web.dev](https://goo.gle/2NATe2S), [WebKit: Safari 15.2](https://webkit.org/blog/12140/new-webkit-features-in-safari-15-2/), [Shen.ai: Safari needs require-corp](https://developer.shen.ai/getting-started/web-troubleshooting)
- Loading WebAssembly: [W3C WebAssembly Web API](https://www.w3.org/TR/2024/WD-wasm-web-api-2-20240822/), [Deno manual](https://docs.deno.com/runtime/manual/runtime/webassembly/using_streaming_wasm), [chromium-discuss on file://](https://groups.google.com/a/chromium.org/g/chromium-discuss/c/_tO-GvjjTvY)

Hosting:
- [Wasmer: COOP and COEP headers](https://docs.wasmer.io/sdk/wasmer-js/how-to/coop-coep-headers)
- [engineered.at: the headers on static hosting](https://engineered.at/articles/setting-the-coop-and-coep-headers-on-static-hosting-like-github-pages)
- [GitHub community discussion 13309](https://github.com/orgs/community/discussions/13309)
- [stefnotch: COOP/COEP without touching the server](https://dev.to/stefnotch/enabling-coop-coep-without-touching-the-server-2d3n)
- [itch.io: experimental SharedArrayBuffer support](https://itch.io/t/2025776/experimental-sharedarraybuffer-support)
- [A Godot developer's notes on itch.io's setting](https://gist.github.com/nisovin/577e5596a36e899e8932747de1b7a353) (`credentialless`, Firefox, Safari)
- [coi-serviceworker](https://github.com/gzuidhof/coi-serviceworker)

Phones:
- [SDL3 README-emscripten](https://wiki.libsdl.org/SDL3/README-emscripten)
- [GLFM](https://awesome.ecosyste.ms/projects/github.com%2Fbrackeen%2Fglfm)
- [Capacitor is MIT (Ionic forum)](https://forum.ionicframework.com/t/what-code-language-capacitor-runs-on/213244)
- [Capacitor issue 6182](https://github.com/ionic-team/capacitor/issues/6182) (the headers in iOS's and Android's web views)
- [logos-workspace issue 287](https://github.com/logos-fleet/logos-workspace/issues/287) (a custom URL scheme on a real iPad)
- [Android: Trusted Web Activities quick start](https://developer.android.com/develop/ui/views/layout/webapps/guide-trusted-web-activities-version2), [Bubblewrap](https://unpkg.com/@bubblewrap/cli@1.24.1/README.md), [web.dev: a PWA in your Android app](https://web.dev/articles/using-a-pwa-in-your-android-app)

Read 2026-11-16, to check the statements once marked "general knowledge":
- WebAssembly: [caniuse](https://caniuse.com/wasm), [MDN](https://developer.mozilla.org/en-US/docs/WebAssembly)
- wasm2js and 64-bit integers: [`src/wasm2js.h`](https://github.com/WebAssembly/binaryen/blob/main/src/wasm2js.h), [`I64ToI32Lowering.cpp`](https://chromium.googlesource.com/external/github.com/WebAssembly/binaryen/+/HEAD/src/passes/I64ToI32Lowering.cpp)
- [Emscripten: Asyncify](https://emscripten.org/docs/porting/asyncify.html), [Emscripten: File System API](https://emscripten.org/docs/api_reference/Filesystem-API.html) (`NODERAWFS`)
- QB64pe's 32-bit `_OFFSET`: `..\QB64pe\source\utilities\type.bi` line 88, `source\global\constants.bas` lines 18–27, `source\qb64pe.bas` lines 14536–14538
- MDN: [Autoplay guide](https://developer.mozilla.org/en-US/docs/Web/Media/Guides/Autoplay), [Keyboard API](https://developer.mozilla.org/en-US/docs/Web/API/Keyboard_API), [Clipboard API](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard_API), [VirtualKeyboard API](https://developer.mozilla.org/en-US/docs/Web/API/VirtualKeyboard_API), [Making PWAs installable](https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps/Guides/Making_PWAs_installable)
- [Chrome: Page Lifecycle API](https://developer.chrome.com/docs/web-platform/page-lifecycle-api), [Chrome: SharedArrayBuffer on Android Chrome 88](https://developer.chrome.com/blog/enabling-shared-array-buffer)
- Hosting: [nginx ticket 1606](https://trac.nginx.org/nginx/ticket/1606) (`application/wasm`), [GitHub Pages and HTTPS](https://docs.github.com/en/pages/getting-started-with-github-pages/securing-your-github-pages-site-with-https), [Netlify HTTPS](https://docs.netlify.com/domains-https/https-ssl/), [Netlify pricing](https://www.netlify.com/pricing/), [Cloudflare Pages limits](https://developers.cloudflare.com/pages/platform/limits/), [itch.io HTML5 docs](https://itch.io/docs/creators/html5)

### Still open

The web lookups of 2026-11-15 settled what sources could settle. They also found the itch.io and Safari limit
(§6.3) and the web-view limits (§7.1). Left open:

- **Measured with the SDK, not yet done:**
  - how long `emcc` takes on `libqb.cpp`;
  - the size and startup time of a build;
  - the size of an SDK install;
  - whether `__linux__` is defined (`emcc -dM -E`);
  - whether `-sWASM=0` builds with threads;
  - whether `NODERAWFS` runs the console programs as §5.6 expects.

  An attempt to install the SDK in `target\scratch\` on 2026-11-15 was refused by the session's permission
  check, so these wait on the user's leave to download and run the Emscripten SDK. With it they take one
  session and become the first tasks of W0.
- **Tested on devices or hosts, by design:**
  - whether itch.io's setting runs a threaded build in Safari and Firefox today;
  - whether `contrib.glfw3` turns a touch into a mouse click;
  - whether a Trusted Web Activity gets threads as Chrome does;
  - whether each host of §6.3 sends `.wasm` as `application/wasm`, and whether `game.wasm` fits Cloudflare
    Pages' 25 MiB limit per file.

  These belong to W2 and P1.
