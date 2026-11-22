# Design

## Context

For the motivation see `proposal.md` ("Why"); for the behavior, the delta specs. The study is `study\51`. How
the parts stand (2026-11-19, branch `after-pr-27-real`):

- **The platform is a value.** `qb64rust_base::Platform { Windows, Linux }` (`crates\base\src\platform.rs`).
  `Platform::host()` is read in `main.rs`, `compile.rs` and `lib.rs` of the driver and passed on: to the
  preprocessor (`PpFlags`, for `$IF WIN` and `$IF LINUX`), to `library::find`, to `codegen_cpp::Resolved`, to
  `toolchain::find` and to `build_with_make`. `sema` takes no platform.
- **The build.** `build.rs` `build_with_make` runs the toolchain's `make -C <libqb> OS=win|lnx BITS=64` with
  `AR`, `CC`, `CXX` and `OBJCOPY` set to the tools found, then `-f Makefile -f <program.mk> exe`.
- **libqb's objects are built inside `libqb\`**, beside their sources, under names that do not say what they
  were built for (`libqb\internal\c\libqb\src\qbs.o`). `objects.stamp` names the toolchain; when it differs,
  `refresh_objects` removes every object and the next build makes them again.
- **libqb refuses unknown systems.** `libqb-common.h` lines 19 to 45 know Windows, Apple and `__linux__`, else
  `#error "Unknown system; refusing to build"`. Lines 47 to 51 set `QB64_32` when no 64-bit macro is defined,
  and lines 53 to 58 set `QB64_NOT_X86`.
- **The `Makefile`** has branches for `OS=lnx`, `win` and `osx`. It adds `-m32` or `-m64` from `BITS` (lines
  203 to 213), and on Linux always links `-lGL -lGLU -lX11 -lXrandr -lxcb -ldl -lrt` (line 259), with
  `$CONSOLE:ONLY` too. `threading-$(PLATFORM).o` chooses the thread code.
- **A console program** runs `MAIN_LOOP` on the first thread and the program on a second, `QBMAIN`, with a
  timer thread beside them (`study\03` §1.1, §1.4). `QBMAIN` installs handlers for `SIGFPE` and `SIGSEGV`.
- **The copy of libqb is changed only by edit files** (`tools\libqb\edits\*.edit`, applied by
  `tools\libqb\copy_libqb.py`). An edit names a file under `libqb\`, and the `Makefile` is one of them.
- **The runner** (`tools\legacy_tests\run_legacy_tests.py`) has `--platform` with the values `windows` and
  `linux`. It chooses the files a program is judged by: Linux sidecars, `known.linux.list`, Linux's words for
  libqb's messages, the exit status by its low 8 bits.
- **The suites VM has no network.** What it needs is downloaded by the host and copied in: the toolchain's
  zip, and Node 24.22.3 (`tools\winvm\README.md`). `sema` and the rest of the Rust code need nothing there.
- **Of `tests\verification`, 4158 of 4459 programs are `$CONSOLE:ONLY`** (counted for `study\51` §5.6 on
  2026-11-15; counted again in task 6.1).

### Not measured yet

Nothing below had been run when this design was written. Task group 1 measured each, and the results are under
"Measured", next. Where a result contradicted a decision below, the decision was revised before group 2.

- Whether `emcc` defines `__linux__` (`emcc -dM -E`).
- How long `emcc` takes on `libqb.cpp`, and how long one program takes to link.
- The size of the SDK on disk.
- Whether threads start at once under Node with `-sPTHREAD_POOL_SIZE`, and whether the process ends when the
  program does. A 2020 issue reports a Node program with threads that never exits
  ([Emscripten issue 12801](https://github.com/emscripten-core/emscripten/issues/12801)); its state today is
  not known.
- Whether `-sNODERAWFS` gives the program the real files and the current folder.
- Whether an installed `emsdk` folder still works after it is copied to another path.
- Which release `emsdk` installs as the newest. A search on 2026-11-19 found 6.0.12 dated 2026-10-08 in the
  [ChangeLog](https://github.com/emscripten-core/emscripten/blob/main/ChangeLog.md) and 6.0.13 marked latest
  on the [releases page](https://github.com/emscripten-core/emscripten/releases). The specs say 6.0.12 until
  task 1.1 settles it.

### Measured

Measured on 2026-11-19 on the Windows host, with the SDK in `..\emsdk`.

- **The release.** `emsdk list` offered 7.0.0 as the newest and recommended release, above 6.0.13 and 6.0.12.
  `emsdk install 7.0.0` and `emsdk activate 7.0.0` installed it. `emcc --version` prints `7.0.0`, and
  `upstream\emscripten\emscripten-version.txt` holds `"7.0.0"` with the quotes. The specs pin 7.0.0.
- **The size of the SDK.** 2.2 GB: `upstream` 2.0 GB, Node 106 MB, Python 78 MB. `emsdk` deletes the
  downloaded archive (748 MB) after unpacking it.
- **The SDK's layout.** The launchers are `upstream\emscripten\emcc.exe`, `em++.exe` and `emar.exe`, beside
  their `.py` files. There is no `emcc.bat` (D2 is revised). `.emscripten` names every tool relative to
  `$CFGDIR`, its own folder. Node is `node\24.19.0_64bit\node.exe`.
- **The predefined macros** (`emcc -dM -E`). `__EMSCRIPTEN__` and `__wasm32__` are defined, and `__unix__`.
  `__linux__` is not, so `libqb-common.h` stops at its `#error` without an edit. With `-pthread`,
  `__EMSCRIPTEN_PTHREADS__` and `_REENTRANT` are defined too.
- **Sizes.** A pointer and a `long` are 4 bytes. `long double` is 16 bytes with a 113-bit mantissa: IEEE quad
  precision, as D9 expected.
- **One compile of `libqb.cpp`** with the console-only flags and no `-O`, as libqb's objects are built: 1.6 s
  to the `#error` of `libqb-common.h`. With `-D__linux__` given on the command line (a probe only, not an
  edit) and the GLAD and GLFW include folders, it compiles with no error in 2.1 s.
- **Threads under Node** (task 1.3, `target\scratch\2026-11-19\wasm\threads`). A program that starts two
  threads from `main`, blocks `main` in a `usleep` loop and calls `exit(3)` from a thread runs: both threads
  print, and Node exits with status 3. It does the same without `-sPTHREAD_POOL_SIZE`. Linking it took 3.2 s
  at `-O2`. The 2020 issue of a process that never ends did not happen here.
- **The current folder.** With `-sNODERAWFS`, a relative path is resolved against Node's current folder, not
  the folder of the `.js` file. So a program started from another folder did not find `data.txt` beside it. A
  `--pre-js` file that calls `process.chdir(__dirname)` on the main thread only fixes it (Node refuses
  `process.chdir` in a worker, and the current folder belongs to the process). D4 adds it to the link flags.
- **Link flags that are needed:** `-pthread`, `-sNODERAWFS`, `-sEXIT_RUNTIME`, `-sENVIRONMENT=node` and the
  `--pre-js` of the current folder. `-sALLOW_MEMORY_GROWTH` with `-pthread` gives the warning
  `-Wpthreads-mem-growth` ("may run non-wasm code slowly"); it stays, since libqb allocates as it runs.
- **A copied SDK** (task 1.4). The whole folder copied to another path (85 s) builds and runs task 1.3's
  program with `EM_CONFIG` naming the copy's `.emscripten`. The first build at the new path prints "config
  changed, clearing cache" and builds the system libraries again into the SDK's own
  `upstream\emscripten\cache` (libc 18 s, libc++ 15 s, about 45 s in all). So D8's first way holds: the zip is
  unpacked anywhere in the VM, and the first wasm build there pays for the cache once.
- **The `OS` branches of libqb's build files.** Besides the `Makefile`, five `build.mk` files choose sources
  by `OS` (33 branches in all): `parts\core` (GLFW's `win32_*.c` or `x11_*.c`, linked even for
  `$CONSOLE:ONLY`), `parts\audio\extras`, `parts\input\game_controller`, `parts\network\http` and
  `parts\os\clipboard`. With the host's `OS=win`, each picks Windows sources. Each needs an
  `ifeq ($(WASM),y)` branch of its own (D4, "What the first builds show").
- **The Windows fragments did not change** (task 2.5). `-z` of each program of `tests\corpus\slice.list`
  with the compiler of commit `067b19a8` and with the change's: 489 of 489 programs give the same files byte
  for byte (the list's other 9 lines are scripts of `screen\g12_game`, not programs). 38 s, one at a time.
- **The edits of libqb** (tasks 3.3 and 3.4): seven, each a branch inside `ifeq ($(WASM),y)` or `#ifdef QB64_EMSCRIPTEN`,
  an include, or one small definition. None touches `qbs`, `evnt` or the file code beyond an include.
  - `wasm-makefile-unix`: with `WASM=y` the `Makefile` keeps the host's `OS` for commands and paths, and takes a
    Unix system's objects (`unix`, `PLATFORM := posix`).
  - `wasm-makefile-flags`: no `-m32`, `-pthread` everywhere, none of the host's libraries, no icon object, and
    the link flags. It also leaves `glut-emu.o`, GLFW's `core.a` and `clipboard.a` out of the link: they have
    Windows or X11 code only, and a console program does not reach them. So no `build.mk` needed an edit.
  - `wasm-libqb-common`: `__EMSCRIPTEN__` gives `QB64_EMSCRIPTEN`, `QB64_UNIX` and `QB64_GCC`.
  - `wasm-filesystem-cstdlib`: `<cstdlib>` for `getenv`, `realpath` and `free`, which glibc's headers bring in
    on their own and Emscripten's do not.
  - `wasm-stacktrace-none`: no `_Unwind_Backtrace` (WebAssembly has no unwinder), so logging's stack trace is
    empty.
  - `wasm-console-lock-keys`: `GLUTEmu_KeyboardIsKeyModifierSet` answers false, since `glut-emu.cpp` is not
    linked. That is what `OPTIONAL_GLUT` answers in a desktop console program.

  - `wasm-os-name` (task 3.4): `_OS$` is `[WASM][32BIT]`. It changes the value of `func__os`'s `#else`, which
    only Emscripten reaches, on its own line. A first version added an `#elif` of two lines, and that shifted
    the `__LINE__` of two later log calls in `libqb.cpp`, so the Windows preprocessed text differed.

  Two edits D4 expected were not needed. `qbx.cpp`'s signal handlers compile under Emscripten, whose `signal()`
  only records the handler. A division by zero is still caught by `qb_safe_idiv`, as error 11. `_OS$` is task
  3.4's.
- **The preprocessed text did not change** (task 3.5). `filesystem.cpp`, `stacktrace.cpp`, `window-console.cpp`,
  `keyboard.cpp` and `libqb.cpp` (which includes `libqb-common.h`) were preprocessed with `c++ -E -P` and the
  Windows build's flags, with and without `DEPENDENCY_CONSOLE_ONLY`. All 10 texts are equal before and after the
  edits.
- **Build times on the host.** The first wasm build of a console program, which builds libqb's objects into
  `target\wasm\libqb`: 30 s. The next build of the same program: 4 s. `libqb\` holds the same files after both
  (3.2). Task 3.3's program (`target\scratch\2026-11-19\wasm\prog\w33.bas`) was also built with QB64pe on
  Linux, the reference commit's build in WSL (`~/qb64pe-lnx`, made by `tools\linux\build_qb64pe.sh`), and run
  there. Its output and the wasm build's under Node are the same bytes, and both exit with status 3. The `.js` file is 232 KB and the `.wasm` 450 KB.
- **The slice list on the host** (task 5.2, `tests\wasm\slice.list`, `--jobs 4`). The first run: 21 of 22 passed
  in 90 s. `verification\v20_c_string_edges` printed `_TOSTR$(1 / 3)` as `0.3333333333333333148`, the
  recording `.3333333333333333333`: the right `_FLOAT` value, printed through a 64-bit double. Emscripten's
  `printf` prints a `long double` so unless the program is linked with `-sPRINTF_LONG_DOUBLE` (the SDK's
  `src/settings.js`). A fault of the flags, fixed: the driver adds the flag to the link in `program.mk`, beside
  `--pre-js`, so libqb's copy did not change again. The second run: 22 of 22 in 85 s. Per program 3.8 s to
  24.5 s, half under 8.5 s (`slice\s134_events_timer` waits on its timers), so about 3.9 s per program at 4 jobs.
- **The estimate for the whole family.** About 4,150 `$CONSOLE:ONLY` programs, less those `known.wasm.list`
  skips (task 6.2), at the host's rate: about 4.5 hours at 4 jobs, and about half that at 8 if the suites VM keeps
  the rate. Programs that test a compile error end at the compile and are faster.
- **What the whole family skips** (task 6.2): `-z --target wasm` over the 4,463 programs of `verification\`
  (51 s, 4 at a time, nothing built), written to `tests\verification\known.wasm.list`. `window` 307, `target` 59
  (`SHELL` 30, `DECLARE DYNAMIC LIBRARY` 11, `RUN` 7, the network words 8, `CHAIN` 2, `_SHELLHIDE` 1), `offset`
  244 (`_MEM` 200, `_OFFSET` 43, `_UNSIGNED _OFFSET` 1). Left: 3,853, of which 1,662 run and 2,191 only compile
  (a test of a compile error). 307 + 59 + 244 + 3,853 = 4,463.
- **The estimate for the whole run** (task 6.3): the 1,662 runs at the host's 3.9 s each at 4 jobs are about
  1.8 hours, or about 55 minutes at the VM step's 8 jobs if the VM keeps the host's rate; the compile-only
  programs a few minutes; the SDK's copy into the VM and libqb's wasm objects there a few minutes once. So
  about 1 to 2 hours in the VM.
- **The whole family in the suites VM** (task 6.3, 2026-11-20, `check wasm-verification --where vm`): 82 minutes
  in all (4,938 s). The VM was ready in 42 s, the SDK's first copy into it took 742 s and the tree's 70 s, so the
  programs took about 68 minutes at 8 jobs, near the estimate's low end. Of the 4,470 programs the runner counts:
  3,676 pass, 117 fail, 84 fail as `known.list` and `known.linux.list` expect, and 593 are skipped (the 610
  `window`, `target` and `offset` lines, less those the other lists skip already). The log is kept on the host
  (`target\check\logs\failed\wasm-verification-2026-11-20_130512.log`).
- **The 117 failures, sorted** (task 6.4). Faults, fixed:
  - `check_target` missed values of `_OFFSET` type that no declaration gives: `_CAST(_OFFSET, x)`, `_CV(_OFFSET, s)`
    and a `DECLARE LIBRARY` function named `strlen~%&`. The C++ then failed on `qbs_str` (4 programs, such as
    `v49_a_cast`). They are now rejected as "yet".
  - Member arrays of variable-length `STRING`, fixed or `_DYNAMIC`: the front end lays the elements out 8 bytes
    apart and libqb's array code steps 4 bytes (`qb64_array_copy_1d_whole_at_string`). That is a crash or wrong
    strings (6 programs, such as `v41_k_string_members`), so they are rejected as "yet". The rule also refuses
    16 programs that passed without reaching libqb's array code (such as `v24_x199_member_string_array`). A
    first version also refused every `_DYNAMIC` member, which refused 69 more programs that passed. Their
    descriptor is read through `ptrszint` by the generated code and by libqb alike, so it was narrowed.
  - `_UPTIME` and `_CAPSLOCK _TOGGLE` call `GLUTEmu_TimeGet` and `GLUTEmu_KeyboardToggleLockKeyState`, which
    `glut-emu.cpp` holds (2 programs, link errors). The edit `wasm-console-lock-keys` defines both: a steady
    clock from the program's load, and a toggle that does nothing and answers false, as `glut-emu.cpp` does on
    Linux.
  - `SYSTEM -1` ends Node with 0xFFFFFFFF, which the runner took as a crash (127, as Git Bash says). The runner
    now takes a wasm program's low byte, 255, as on Linux (3 programs, `v33_c03`, `c09`, `c10`).
  - The heap stopped at 2 GB: `-sMAXIMUM_MEMORY=4GB` in the driver's `program.mk` (`v21_c_fixed_len_2147483647`).
  - Stacks of 8 MB (`-sSTACK_SIZE`, `-sDEFAULT_PTHREAD_STACK_SIZE`) were tried for the member programs and
    changed nothing.

  - 73 programs failed in C++ because the wasm build leaves out a part of libqb: audio (54, such as
    `v29_x01_sound_two`), `$DEBUG`'s network part (11), the game controller's (7) and the clipboard (1). With
    `OS=win` their `build.mk` files pick Windows sources (`combaseapi.h`, `windows.h`). They are now rejected at
    compile time by the part the built-in table names (user, `DECISIONS.md` 2026-11-20). That also refuses
    `v29_x79_sound_goto` and `v49_x50_clipboard_call`, which passed.

  On `known.wasm.list`, with what was seen: `offset` 2 (`v70_i1`, `i2`: `$USELIBRARY` includes with `_OFFSET`),
  `target` 3 (`v37_f_static`, `v37_h_customtype`, `v37_x68`: a prebuilt Windows `.a`), `float` 6 (D-069; and
  D-071, a conversion out of range: `v20_d_val`, `v20_f_math_edges`, `v33_c06`), `clib` 1 (D-070), `width` 8
  (`$IF LINUX` and `$IF 32BIT`, `_OS$`, an index past 32 bits, strings past wasm32's 4 GB), `node` 3 (Node's file
  system on a Windows host: paths, the order of a folder, reading a folder), `open` 5 (`v58_e01`–`e04`, `e19`: a
  deep recursion ends in Node's `RangeError`, not error 256). The skipped lines written again by `-z`: `window`
  307, `target` 145, `offset` 268 (of the suite's 4,463; 3,743 left).
- **The closing whole-family run** (task 6.5, 2026-11-20, in the suites VM): 69 minutes (4,140 s). The SDK was
  not copied again (its stamp held). Of the 4,470 programs the runner counts: 3,664 pass, 0 fail, 102 fail as a
  list expects (`known.wasm.list`'s `float`, `clib`, `width`, `node` and `open` lines, and the other lists'), and
  704 are skipped. Of the 3,766 that ran, 97.3% pass; of the whole suite, 82.0%. The runner's two notes
  (`v37_x70` passes though `known.list` lists it; `v16_m6_one_header`'s `lines.list` row) are about the Windows
  lists and came in the first run too; they are not this target's.
- **The other kinds of the slice list, by hand** (`target\scratch\2026-11-19\wasm\prog\w34.bas`): a file
  written, read back, tested with `_FILEEXISTS` and removed; `INPUT` from standard input; `ON ERROR` with
  `RESUME NEXT`; `ON TIMER` with `_LIMIT`; an integer division by zero trapped as error 11. Each worked under
  Node.

## Goals / Non-Goals

**Goals:**

- A `$CONSOLE:ONLY` program builds for `wasm32` on a Windows host and runs under Node, from the same fragments
  and the same libqb sources as the desktop builds.
- A number: how many console programs of `tests\verification` give their recorded output, with every other
  program sorted by reason.
- No wrong program: what the target cannot do right yet is refused at compile time.
- The Windows and Linux builds compile the same libqb code as before, and their C++ fragments do not change
  by a byte.

**Non-Goals:**

- A window, graphics, sound, a browser. A 4-byte `_OFFSET` in `sema`. Atomic thread flags. These are W0.5
  and W1.
- A wasm build on a Linux host. The code takes the host as a value, so it is not ruled out, but it is not
  tested here.
- Speed and size of the built program. They are written down when measured, not worked on.
- Recordings of the wasm target's own.

## Decisions

### D1: `Platform::Wasm`, chosen by `--target`

`Platform` gains `Wasm`. `Platform::host()` never returns it. The driver's options gain `target:
Option<Platform>`; `main.rs` and `compile.rs` use `target.unwrap_or(Platform::host())` at each place that
passes a platform on. `exe_extension` is `js` and `name` is `wasm`. Every `match` on `Platform` gets its arm:

| Place | For `Wasm` |
|---|---|
| `syntax\src\pp.rs` `PpFlags` | D6 |
| `driver\src\library.rs` `find` | Linux's order, headers only; a prebuilt file found is the error of D5 |
| `codegen_cpp::Resolved` and `codegen-cpp\src\library.rs` | Linux's forms |
| `driver\src\lib.rs` `check_exe_icon_on`, `resource_path` | as Linux |
| `driver\src\toolchain.rs`, `build.rs` | D2, D3, D4 |

*Why:* one value already flows through every place that differs by platform (design D3 of `m7-linux`).
*Instead of:* a separate `Target` type beside `Platform`. It would be passed along the same paths and would
ask each place the same question twice. If a later target needs both a host and a target at one place, the
type is split then.

Code: `crates\base\src\platform.rs`; `crates\driver\src\main.rs` (`parse_args`, `USAGE`), `compile.rs`.

### D2: The SDK is found, checked and used through `emsdk`'s own layout

`toolchain::find_emsdk(option, variable)` returns `Emsdk { root, source, emcc, empp, emar, node }`. A folder
counts when it holds `upstream\emscripten\emcc.exe` (Windows; "Measured") or `upstream/emscripten/emcc` and
`upstream\emscripten\emscripten-version.txt`. The version file's text is compared with `EMSDK_RELEASE`.
`node` is the one `emsdk` installed under `node\`. `make` comes from the host toolchain (`toolchain::find`,
`Tool::Make`): `emsdk` installs none on Windows. The driver passes `CC`, `CXX` and `AR` to `make` as it does
now, with Emscripten's programs, and sets `EM_CONFIG` to the SDK's `.emscripten` file so that no shell
activation is needed.

*Why not download it in this change:* `emsdk` fetches several archives and writes a configuration file with
paths. Putting that behind `toolchain install`, with checksums and `--from-zip`, is W2's task, when users
need it. Until then the SDK is installed by hand, once on this machine.

Code: `crates\driver\src\toolchain.rs` (`find_emsdk`, `EMSDK_RELEASE`, `EMSDK_ENV`, `emsdk_missing_message`).

### D3: A wasm copy of libqb for the objects

A wasm build never builds in `libqb\`. It builds in a second tree that holds the same sources:

- `QB64RUST_LIBQB_WASM_ROOT` when set;
- else `target\wasm\libqb` of the checkout, when `libqb\` is a checkout's (it has `..\Cargo.toml`);
- else `wasm\libqb` under the toolchain home (`toolchain::home()`).

Before a build, `sync_wasm_libqb` copies each source file of `libqb\` whose size or time differs, and removes
source files that are gone. Files that builds write (`written_by_builds`) are left alone. The copy is made
under the wasm tree's own objects lock, alone. The tree's `objects.stamp` names the SDK release, so a new
release builds the objects again.

*Why:* objects have one name per source. A wasm build in `libqb\` would replace the Windows objects, and
`refresh_objects` would then build libqb again at every switch, which is minutes each time and would stop a
test run on the other target. *Instead of:* an objects folder per target in the `Makefile`. Its rules are
spread over about 30 `build.mk` files of QB64pe's, and all would need edits. *Cost that stays:* about 27 MB
of sources twice, plus the wasm objects.

Code: `crates\driver\src\build.rs` (`wasm_libqb`, `sync_wasm_libqb`; `build_key` gains the SDK release).

### D4: libqb for Emscripten, by edits inside tests for it

`build_with_make` for `Wasm` runs `make` with the host's `OS` value and `BITS=32`, plus `WASM=y`. The host's
`OS` stays because that branch of the `Makefile` decides how commands and paths are written, which is the
host's matter. The edits, each named `wasm-<what>.edit`:

- **`Makefile`:** blocks under `ifeq ($(WASM),y)` after the `OS` branches. They drop `-m32` and `-no-pie`,
  replace the libraries the `OS` branch links with none, set `PLATFORM := posix` for the thread code, name
  the output `.js`, and add the link flags: `-pthread`, `-sPTHREAD_POOL_SIZE=4`, `-sNODERAWFS`,
  `-sEXIT_RUNTIME`, `-sALLOW_MEMORY_GROWTH`, `-sENVIRONMENT=node`, and a `--pre-js` file that makes the
  folder of the `.js` file the current folder (task 1.3, "Measured"). Task 3.3 settles the rest. The driver's
  `program.mk` adds the `--pre-js` file and `-sPRINTF_LONG_DOUBLE`, which prints a `_FLOAT` at its own precision
  (task 5.2, "Measured").
- **`libqb-common.h`:** `__EMSCRIPTEN__` is tested first and defines `QB64_EMSCRIPTEN`, `QB64_UNIX` and
  `QB64_GCC`. It does not define `QB64_LINUX`, so that no code for X11 or ALSA is taken by accident. A place
  that needs Linux's branch gets `|| defined(QB64_EMSCRIPTEN)` by an edit of its own.
- **`qbx.cpp`:** no signal handlers under Emscripten. Threads there have no POSIX signals
  ([Emscripten: Pthreads support](https://emscripten.org/docs/porting/pthreads.html)), and `qb_safe_idiv`
  already raises the division error itself (`study\03` §9.2 item 11).
- **`_OS$`:** `[WASM][32BIT]` under Emscripten.
- **What the first builds show.** The list is not known before the build runs. `study\51` §3 expects the
  console code, the rounding code (`QB64_NOT_X86`) and the file code. An edit that is more than a branch or a
  stub is named in "Measured" with its reason.

Every edit is inside `#ifdef QB64_EMSCRIPTEN` (or `__EMSCRIPTEN__`) or `ifeq ($(WASM),y)`. So the
preprocessed code of a Windows or Linux build is the same as before. Task 3.5 checks this.

*Why edits, not a fork of the files:* the copy stays "QB64pe's commit plus named edits", which
`copy_libqb.py --check` holds, and the edits can be offered upstream.

Code: `tools\libqb\edits\wasm-*.edit`; `crates\driver\src\build.rs` `build_with_make`, `program_makefile`.

### D5: The target's rejections are a pass after `sema`

`qb64rust_sema::check_target(program, platform, map, main, &mut diagnostics)`, in a new
`crates\sema\src\target.rs`, called from the driver's `check_program` with `parsed.flags.platform()`, where
`check_exe_icon_on` is called now. `main` is the main file, whose first line takes item 1's error; `map` names
QB64pe's `afterlastline.bm`, in which nothing is reported (as built in task 2.4: it is included when a program
names a network function, and it calls `_OPENCLIENT` itself). For `Wasm` it walks the checked program once and
reports:

1. no `$CONSOLE:ONLY`: one error at line 1;
2. each use of a word of the table `WASM_UNAVAILABLE` (statements and functions by name);
3. `DECLARE DYNAMIC LIBRARY`, and a `DECLARE LIBRARY` whose lookup found a prebuilt file (that one in the
   driver's `library::resolve`, which alone looks for the files);
4. each declaration or expression whose type is `Ty::Off`, `Ty::UOff` or the `_MEM` type, and each built-in
   of `MEM_RESULTS` or with an `_OFFSET` or `_MEM` slot (`sema\src\builtins.rs`, `Slot::Offset`,
   `Slot::Mem`): the "yet" message. So is a member array of variable-length `STRING` (as built in task 6.4).
5. each built-in whose `dependency` names a part of libqb of `WASM_PARTS_NOT_YET`, and `ON STRIG`: the "yet"
   message with the part's name (as built in task 6.4; user, `DECISIONS.md` 2026-11-20).

For the other platforms it does nothing. The language server calls `check_program` with the host, so the
editor shows none of these errors; a target setting for the editor is W2's.

*Why a pass after, not inside the checks:* `sema` stays free of the platform, and the rules of one target sit
in one file with one table, which W1 extends and the reference chapter reads. *Why item 4:* the user accepted
a 4-byte `_OFFSET` (`DECISIONS.md` 2026-11-19), but `Ty::int_bits` gives 64 for it, and `size_of`, `LEN`,
`TYPE` layout and constant folding follow. Building such a program now would be wrong code. W0.5 removes the
item when it makes the width the target's.

Code: `crates\sema\src\target.rs`; `crates\driver\src\lib.rs` `check_program`.

### D6: `$IF` flags

`PpFlags` for `Wasm`: `WASM` -1; `WIN`, `WINDOWS`, `LINUX`, `MAC`, `MACOSX` 0; `32BIT` -1; `64BIT` 0. On the
other targets `WASM` is not defined, so it is false there. (This design first said 0 there. As built in task 2.3
it is not defined, because a predefined 0 would make `$IF WASM = UNDEFINED` false on Windows and Linux, where
QB64pe makes it true.)

*Why `WASM` and not `WEB`:* QBJS defines `$IF WEB` (`study\51` §2), and programs written for both QB64 and
QBJS put QBJS-only code inside it, such as `Import`. That code would not compile here. *Why not `LINUX`:* a
program's `$IF LINUX` branch often declares a `.so` library, which this target does not have.

Code: `crates\syntax\src\pp.rs`.

### D7: The runner treats wasm as Linux for judging

`--platform wasm` (`run_legacy_tests.py`, `platform_of`). A helper `judged_as(platform)` returns `linux` for
`wasm`; `expected_file`, `platform_expected`, `platform_neutral`, `exit_code_on`, `program_log` and
`log_without_frames_on` take its result. The compile line gains `--target wasm`, the built file is
`<name>.js`, and the run line is the SDK's `node <name>.js`. The SDK comes from `QB64RUST_EMSDK_ROOT`.
`load_known` reads `known.wasm.list` beside `known.linux.list`; `skipped_on` skips the reasons `target`,
`window` and `offset`. The check of the lists refuses a reason outside the set of spec
`testing/golden-corpus`.

The pass cache and the build cache key on the platform already (design D2 of `m7-linux`), so wasm results
never stand for Windows ones.

*Why Linux's recordings:* under Emscripten libqb takes its Unix branches: "Press enter to continue", the
console's comma spacing (D-011), `/` in paths. QB64pe's Linux build is the nearest thing to an oracle.
*Instead of:* wasm recordings made with our own compiler. A recording made by the thing under test proves
nothing.

Code: `tools\legacy_tests\run_legacy_tests.py`; tests in `test_run_legacy_tests.py`.

### D8: The SDK in the suites VM, and the two `check` steps

`tools\winvm\pins.psd1` gains the Emscripten release. The host packs its SDK folder once into
`%LOCALAPPDATA%\qb64rust\winvm\installers\emsdk-<release>.zip`. `suites.ps1` unpacks it on the cache disk at
the first wasm step and sets `QB64RUST_EMSDK_ROOT` for the step, as `game.ps1` does with Node's zip. This
rests on the SDK folder working at another path, which it does (task 1.4, "Measured"); the first build there
builds Emscripten's system libraries into the SDK's cache once. Had it not worked, the zip would have been
unpacked at the host's own path in the VM.

`tools\check\steps.py` gains `wasm-slice` (host), in the layers `full` and `merge`, and `wasm-verification`
(`vm=True`), in no layer: it runs only when named, `check wasm-verification --where vm` (user, `DECISIONS.md`
2026-11-20). A step whose SDK is missing is "not run" with a reason, and fails only in `merge`.

*Why not in `quick`:* the quick layer runs before every handover. A wasm fault can come only from the crates
the release is built from or from libqb, and the `full` layer covers those. The quick layer stays as long as
it is. *Why the whole family in no layer:* it takes about 2 to 4.5 hours (task 5.2, "Measured"), and the user
wants such runs on a schedule, not after each change or at each merge. W0 runs it once for its number (task
6.3) and once after the fixes (6.5). *Cost that stays:* the `full` and `merge` runs grow by `wasm-slice`,
about 90 s. A regression that only the whole family shows is found at its next named run. (This design first
put both steps in `full` and `merge`.)

Code: `tools\check\steps.py`, `check.py`; `tools\winvm\suites.ps1`, `pins.psd1`, `README.md`.

### D9: What differs on the target, and how it is recorded

Each failure of the whole run gets one of three outcomes:

- **A fault of an edit or of the build flags.** It is fixed here.
- **A defined difference** (`CLAUDE.md` working rule 8): `_FLOAT` is IEEE quad precision in `wasm32`, not
  x87's 80 bits, and number text comes from musl's `printf`. Each kind gets one row in
  `DIVERGENCES-QB64PE.md` that names the wasm target, and its programs go on `known.wasm.list` as `float` or
  `clib`.
- **Not explained in this change:** `open`, with a line in "Measured" saying what was seen. W0 is a
  measurement; it does not have to end with no `open` line, but each one is counted in the closing note.

Nothing is put on `known.wasm.list` to make a run pass without a reason from the set.

### D10: The order of runs, sized to what can break

1. One program by hand (group 3).
2. `tests\wasm\slice.list`, at most 25 programs, on the host (group 5). Console programs under Node open no
   window, and the network words are rejected, so nothing can ask the user anything (`CLAUDE.md` working
   rule 6).
3. The whole family, once, in the suites VM (group 6), after its length is estimated from step 2 and said.
4. After fixes: only the programs that failed, by name or by list. The whole family runs again once, at the
   end.

The Windows suites run again only for group 3's libqb edits, and only as far as task 3.5 says.

## Risks / Trade-offs

- [libqb under Emscripten needs far more edits than a handful] → group 3 stops and reports when the edits
  pass 40 or one of them is more than a branch or stub in a core path (`qbs`, `evnt`, file I/O). That is a
  finding for the user, not something to push through.
- [Threads under Node do not start or the process never ends] → measured first, in task 1.3, with a C++
  program of three lines, before any libqb work. The fallback is `-sPROXY_TO_PTHREAD`, which runs `main` on a
  worker.
- [`emcc` on Windows is slow, so 4,150 builds take hours] → measured in 1.2 and again on the slice list. If
  the family would take more than about 3 hours in the VM, the user is asked before it starts; a sample by
  family is the smaller run.
- [A libqb edit invalidates the Windows pass caches, so the next `just full` starts cold] → all edits land in
  group 3 before any large run, and the user is told when that happens. An edit inside `#ifdef` changes no
  Windows object's code, but the build key reads the file.
- [The Linux recording is not the right judge for some program] → such a program is `node` or `open` on the
  list with what was seen. No recording is changed.
- [A copied SDK does not run in the VM] → D8's second way; if both fail, the family runs in WSL's Ubuntu
  with Emscripten's Linux SDK, after asking the user (`CLAUDE.md` working rule 6).
- [The rejection of `_OFFSET` and `_MEM` hides how those programs behave] → intended. Their count is in the
  closing note as `offset`, and W0.5 is the step that lifts it.
- [Two trees of libqb sources drift] → the wasm tree is only ever written by `sync_wasm_libqb`, before every
  build, from `libqb\`.

## Migration Plan

Nothing changes for a build without `--target`. Rollback is reverting the change; the edit files are inert
without `WASM=y` and `__EMSCRIPTEN__`, and `known.wasm.list` is read only with `--platform wasm`.

## Open Questions

- The exact link flags (pool size, memory). They change no spec and no task; 1.3 and 3.3 settle them.
- Whether the big families, the whole wasm family among them, run on a schedule on another Windows machine
  (user, 2026-11-20: a study, not this change).

## What shows each scenario

Task 7.1: each scenario of the delta specs, and the test or program that shows it. The driver's CLI tests are in
`crates\driver\tests\it\cli.rs` and `wasm_target.rs`; the runner's in `tools\legacy_tests\test_run_legacy_tests.py`;
`check`'s in `tools\check\test_check.py` (class `WasmSteps`).

| Spec | Scenario | Shown by |
|---|---|---|
| `compiler/cli` | An unknown name | CLI test `target_unknown_name` |
| `compiler/cli` | The default | CLI test `target_host_is_the_default` |
| `compiler/cli` | A named output | CLI test `wasm_builds_and_runs` (with the SDK) |
| `compiler/cli` | A wrong ending | CLI test `target_wasm_output_ends_in_js` |
| `compiler/cli` | A failed build | unit test `compile::tests::a_failed_wasm_build_removes_both_files` |
| `compiler/cli` | A debug build | CLI test `target_wasm_refused_options` |
| `compiler/platforms` | Build on Linux | the Linux steps of `check full` (`linux-tier1`, `linux-slice`), as before this change |
| `compiler/platforms` | Checking needs no supported platform | as before this change (spec `compiler/platforms`) |
| `compiler/platforms` | A build for WebAssembly on Windows | CLI test `wasm_builds_and_runs`; unit test `platform::tests::the_toolchain` |
| `compiler/toolchain` | The SDK by variable | unit test `toolchain::tests::emsdk_lookup` |
| `compiler/toolchain` | No SDK | CLI test `target_wasm_without_sdk`; unit test `emsdk_lookup` |
| `compiler/toolchain` | No make | CLI test `target_wasm_without_make` |
| `compiler/toolchain` | Another release | unit test `toolchain::tests::emsdk_another_release` |
| `compiler/toolchain` | Found by variable | CLI test `toolchain_wasm_forms` |
| `compiler/toolchain` | Install is by hand for now | CLI test `toolchain_wasm_forms` |
| `compiler/wasm-target` | Hello | CLI test `wasm_builds_and_runs` |
| `compiler/wasm-target` | A file beside the program | CLI test `wasm_builds_and_runs`; `verification\v22_b_open` on `tests\wasm\slice.list` |
| `compiler/wasm-target` | The exit status | CLI test `wasm_builds_and_runs`; `verification\v33_c01_system_3` on the slice list |
| `compiler/wasm-target` | A window program | CLI test `wasm_target::a_window_program` |
| `compiler/wasm-target` | SHELL | CLI test `wasm_target::shell` |
| `compiler/wasm-target` | The same program for the host | CLI test `wasm_target::shell` |
| `compiler/wasm-target` | A header-only library | CLI test `target_wasm_libraries` |
| `compiler/wasm-target` | Checking only | CLI test `target_wasm_check_only` |
| `compiler/wasm-target` | $DEBUG | CLI test `wasm_target::the_other_words` |
| `compiler/wasm-target` | A new built-in of a part | unit test `target::tests::each_part_is_named_by_builtins`; the rule reads the table's `dependency` |
| `compiler/wasm-target` | SOUND | CLI test `wasm_target::parts_not_built_yet` |
| `compiler/wasm-target` | ON STRIG | CLI test `wasm_target::parts_not_built_yet` |
| `compiler/wasm-target` | A value of the type without a declaration | CLI test `wasm_target::pointer_sized_types` |
| `compiler/wasm-target` | A variable of the type | CLI test `wasm_target::pointer_sized_types` |
| `compiler/wasm-target` | A built-in that returns one | CLI test `wasm_target::pointer_sized_types` |
| `compiler/wasm-target` | A member array of strings | CLI test `wasm_target::pointer_sized_types` |
| `compiler/wasm-target` | A branch for the target | CLI tests `wasm_target::a_branch_for_the_target`, `wasm_builds_and_runs` |
| `compiler/wasm-target` | _OS$ | CLI test `wasm_builds_and_runs`; `verification\v49_c_md5_os` (`width`) |
| `compiler/wasm-target` | Comma zones | `verification\v30_g_iif_min_max` on the slice list |
| `compiler/wasm-target` | A _FLOAT result | `verification\v30_a_using_numbers` on `known.wasm.list` as `float` (D-069) |
| `compiler/wasm-target` | Same fragments | CLI test `wasm_target::same_fragments_as_linux` |
| `testing/check-workflow` | The whole family goes to the VM | `WasmSteps.test_the_vm_job_has_the_command_and_the_sdk`; the runs of tasks 6.3 and 6.5 |
| `testing/check-workflow` | Not in the quick layer | `WasmSteps.test_the_slice_in_full_and_the_family_only_when_named` |
| `testing/check-workflow` | The whole family only when named | `WasmSteps.test_the_slice_in_full_and_the_family_only_when_named` |
| `testing/check-workflow` | A full run on a machine without the SDK | `WasmSteps.test_without_the_sdk_not_run_and_a_failure_only_in_a_merge` |
| `testing/check-workflow` | A merge without the SDK | `WasmSteps.test_without_the_sdk_not_run_and_a_failure_only_in_a_merge` |
| `testing/golden-corpus` | One program by name | `WasmVerification.test_wasm_built_and_run_as_on_linux` |
| `testing/golden-corpus` | Several at a time from a list | `WasmOptions.test_the_slice_list_splits_by_suite`; `check wasm-slice` |
| `testing/golden-corpus` | Recording | `WasmOptions.test_record_is_refused` |
| `testing/golden-corpus` | A program with a Linux recording | `WasmCorpus.test_judged_by_the_linux_sidecar` |
| `testing/golden-corpus` | What the reasons mean | `WasmLists.test_the_reasons_and_what_is_skipped` |
| `testing/golden-corpus` | A window program | `WasmLists.test_a_window_program_is_skipped_without_a_build` |
| `testing/golden-corpus` | A reason outside the set | `WasmLists.test_a_reason_outside_the_set` |
| `testing/golden-corpus` | A listed program that passes | `WasmLists.test_a_listed_program_that_passes_is_news` |
| `testing/golden-corpus` | The list on the host | `check wasm-slice` (22 of 22, task 5.2) |
