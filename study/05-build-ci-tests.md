# 05 - Build system, bootstrap, CI/release and test suite (QB64pe)

Repo: `..\QB64pe` (HEAD 16f629784e, "Automatic update of ./internal/source"; `git describe` says `v4.1.0-751-...` only because later tags are absent from the clone; `source\global\version.bas:13` says `4.7.0-GLFW`). Paths below are repo-relative. Study was read-only; nothing was built or run.

## 0. Big picture

QB64pe is a self-hosting BASIC-to-C++ translator. `source\qb64pe.bas` (28,828 lines plus includes) emits C++ fragments into `internal\temp\`; `Makefile` compiles them (through `internal\c\qbx.cpp`, which `#include`s them) together with the runtime `internal\c\libqb.cpp` (27,119 lines) + `libqb\` + `parts\` libraries into one native exe. The same pipeline builds the compiler itself from a checked-in, pre-translated copy of the compiler in `internal\source\`.

## 1. Bootstrap (fresh checkout to working qb64pe)

`internal\source\` is the compiler's own C++ output (the contents of `internal\temp` after compiling `source\qb64pe.bas`), committed to git. Only a C++ compiler and make are needed.

| OS | Entry | Toolchain | Steps |
|---|---|---|---|
| Windows | `setup_win.cmd` | LLVM-MinGW (clang-based, UCRT), downloaded into `internal\c\c_compiler\` by `setup_mingw.cmd` | detect bitness (`setup_win.cmd:65-80`, 60 s prompt for 64/32 default 64); `setup_mingw.cmd <bits>` (:85); `mingw32-make -jN OS=win clean` (:130); `mingw32-make -jN OS=win BUILD_QB64=y` (:134). `/s` or `-s` uses system MinGW from PATH (`USE_SYSTEM_MINGW=y`, :53-57). |
| Linux | `setup_lnx.sh` | system g++/make | reads `internal/source/.bits` (64 in repo; :26-30), refuses 64-bit build on 32-bit host (:32-36) and root (:39); installs packages for arch/pacman, debian family/apt, fedora family/yum, void/xbps (:88-111; Debian list e.g. build-essential mesa-common-dev libglu1-mesa-dev libasound2-dev libpng-dev libcurl4-openssl-dev libx11/xcursor/xrandr/xinerama/xi-dev, :96); `make clean OS=lnx BITS=$BITS`; `make OS=lnx BITS=$BITS BUILD_QB64=y -j3` (:125-126); creates `run_qb64pe.sh` and a `.desktop` entry (:128-151). |
| macOS | `setup_osx.command` | Xcode CLT clang++ (`xcode-select --install` if missing, :20-27) | `make OS=osx clean; make OS=osx BUILD_QB64=y -j3` (:30-31). BITS defaults to 64 (`Makefile:168`). |

### MinGW acquisition (`setup_mingw.cmd`, 475 lines)
- Fetches the latest release of `mstorsjo/llvm-mingw` from GitHub (:3-4, :151-152): `llvm-mingw-<tag>-ucrt-<target>.zip`; target x86_64 / i686 / aarch64 / armv7 from `PROCESSOR_ARCHITECTURE`/`PROCESSOR_ARCHITEW6432` (:54-95). No version pin and no checksum verification found.
- Download chain (no PowerShell, for Win7): curl on PATH, else curl from Git for Windows, else GitHubDesktop.exe as Electron/Node HTTPS runtime (:108-141). Extraction: unzip.exe, tar, or cscript Shell.Application VBS helper (:195-236). Result moved to `internal\c\c_compiler\` (:248). Skipped if `c++.exe` exists (:37-42).

### How `internal\source` is compiled (`BUILD_QB64=y`)
`Makefile:292-308`: copies `internal\source\*` into `internal\temp<TEMP_ID>\` at make-parse time (`$(shell $(CP) ...)`), forces `DEP_FONT, DEP_ICON, DEP_ICON_RC, DEP_SOCKETS, DEP_HTTP, DEP_CONSOLE, DEP_ZLIB = y`, and `EXE ?= qb64pe[.exe]` (:215-221). It does not set `DEP_GL`, `DEP_IMAGE_CODEC`, `DEP_AUDIO_MINIAUDIO`, `DEP_SCREENIMAGE`, `DEP_DEVICEINPUT` (see open items). Then the normal link pipeline (section 2) runs; Windows resources come from `internal\source\icon.rc/icon.ico/qb64pe.manifest`, linked by `ICON_OBJ`.

### Contents of `internal\source` (2,078 files, ~25 MB, tracked in git)
It is exactly the per-compile output directory:
- `main.txt`: list of `#include "mainN.txt"` (written at `qb64pe.bas:17594`). `main<N>.txt`, N=0..511: N=0 is the main module (3.8 MB), N>=1 one file per SUB/FUNCTION (`qb64pe.bas:5430`), i.e. 511 subs/functions. Statements are prefixed by `#line <n> "file.bas"`.
- `ret<N>.txt` per-sub RETURN dispatch (`:5433`), `data<N>.txt`, `free<N>.txt` per-sub static data init/free, `maindata.txt` (global var init, 511 KB), `mainfree.txt`, `mainerr.txt`, `runline.txt`, `ontimer/onkey/onstrig(.j).txt`, `chain.txt`, `inpchain.txt`, `clear.txt`.
- `global.txt` (globals, helper templates such as `qb_safe_idiv`, extern declarations), `regsf.txt` (function registration table), `dyninfo.txt`, `incone.txt`.
- Metadata: `extdep.txt` (INCL:/ICON: list with absolute CI paths `/home/runner/work/QB64pe/...`), `compilelog.txt` (the make command line, shows `g++ -no-pie -fdebug-prefix-map ... -m64 -std=gnu++20`), `format.out`, `embedded.cpp` (empty `_EMBEDDED$` stub), `data.bin`, `temp.bin`, `regsf_ignore.txt`, `vw_main_dispatch.txt`, `vw_main_skip.txt`.
- Windows resources: `icon.ico`, `icon.rc` (VERSIONINFO 4,7,0,0), `manifest.h`, `qb64pe.manifest`, `qb64pe.exe.manifest`.
- `.bits` = `64`.
Consumed by `internal\c\qbx.cpp` (1,608 lines, checked-in template): `#include "../temp/global.txt"` (:500), `regsf.txt`, `dyninfo.txt`, `clear.txt`, `inpchain.txt`, `chain.txt`, `onstrig/onkey/ontimer.txt`, `maindata.txt`, `mainerr.txt`, `runline.txt`, `...j.txt`, `main.txt` (:606). For `TEMP_ID>1` the compiler writes `qbx<N>.cpp` with the `../temp/` path rewritten (`qb64pe.bas:345-355`).
The generated text was produced on Linux x64 (CI), yet Windows and macOS bootstrap from the same text.

### Regeneration and commit by CI
`.ci\bootstrap.*` builds `qb64pe_bootstrap` from `internal\source`; `.ci\compile.*` runs `qb64pe_bootstrap -x -w source/qb64pe.bas` (`compile.sh:11`, `compile.bat:9`), deletes `internal/source/*` and moves `internal/temp/*` there (Linux first removes debug_*, recompile_*, *.sym, and writes `.bits`; `compile.sh:15-25`). `.ci\push-internal-source.sh` commits as github-actions[bot], message `Automatic update of ./internal/source`, only if `git diff --cached` is non-empty (297 such commits in history). The workflow step (`build-process.yml:180-182`) runs only on: push event, Linux x64 leg, `refs/heads/main`, head commit message contains "Merge pull request". A deploy SSH key (`ACTION_DEPLOY_KEY`, :53-56) lets the push trigger a new build. Only Linux x64 output is ever pushed.
Rewrite implication: the bootstrap is a fixed point; a rewrite needs its own seed strategy (checked-in generated output, a seed binary, or a different self-host ladder).

## 2. The Makefile (535 lines)

### Inputs
| Variable | Lines | Meaning |
|---|---|---|
| `OS` | 6-20 | `win`/`lnx`/`osx`; auto-detected only if `OS` is `Windows_NT`/unset (uname). Selects paths, tools (`cp -r`/`xcopy`, `rm -fr`/`del /Q`, `FIXPATH`, `ESCAPENAME`, `ADDQUOTES`), `PLATFORM` (posix/windows), `EXTENSION`. Windows: `SHELL := cmd` (:92), backslash paths. |
| `BITS` | 39-48, 69-77, 121-128, 168, 195-213 | 32/64. lnx: `getconf LONG_BIT`; win: from `$(CC) -dumpmachine`; osx: 64. Adds `-m32`/`-m64`. |
| `EXE` | 215-232 | output name, required unless `BUILD_QB64` (`clean`/`build-tests` fake it). |
| `BUILD_QB64` | 292-308 | bootstrap mode. |
| `TEMP_ID` | 24, 52, 89, 159, 283 | suffix for `internal\temp<id>` and `qbx<id>.cpp/.o` (multiple IDE instances). |
| `CXXFLAGS_EXTRA`, `CXXLIBS_EXTRA`, `CFLAGS_EXTRA` | 30-34 | appended to flags (extra libs first in list). |
| `STRIP_SYMBOLS` | 270, 278, 519 | `n` keeps symbols (unix adds `-rdynamic` for dladdr stack traces, :277-281). Default: `.sym` split + strip. |
| `GENERATE_LICENSE`, `LICENSE` | 233-236, 508-533 | concatenate `licenses/license_<x>.txt` for each `LICENSE_IN_USE` entry into `<exe>.license.txt`. |
| `USE_SYSTEM_MINGW` | 94-106, 146 | Windows: use PATH `gcc/c++/ar/objcopy/windres` instead of `internal\c\c_compiler\bin\`. |
| `DEP_*` | below | feature flags. |
CC/CXX are not set on unix (make defaults: cc/g++, clang on macOS).

Always-on flags: `-std=gnu++20 -fno-strict-aliasing -Wno-conversion-null` (:244-253; "libqb does some illegal type punning"), `-lpthread` (:256), `-no-pie` on Linux (:67), debug-path flags (:79-81, 146-150, 173, 192-193): `-fdebug-compilation-dir=.` (clang) or `-fdebug-prefix-map=$(CURDIR)=.` (gcc) so `#line`/debug paths are relative. `MAKEFLAGS += --no-builtin-rules`.

### Per-OS link flags
- Linux (:259): `-lGL -lGLU -lX11 -lXrandr -lxcb -ldl -lrt`; audio adds `-lm -lasound` (:422); http `-lcurl` (`network/http/build.mk:56`); clipboard `-lpng` (`clipboard/build.mk:16`). Dynamic linking to system libs.
- Windows (:263): `-static-libgcc -static-libstdc++ -lopengl32 -lglu32 -lgdi32 -lcomdlg32 -lole32 -luuid -lshlwapi -lwindowscodecs -lwinmm`; subsystem: `-mconsole` if `DEP_CONSOLE` or `DEP_CONSOLE_ONLY` else `-mwindows` (:455-462); `-lws2_32` (sockets), `-lwinspool` (printer), deviceinput `-lxinput -ldinput8 -ldxguid -lwbemuuid -loleaut32` (:408), audio `-lm -lksguid -ldxguid` (:426), curl `-lcrypt32 -lbcrypt -lwldap32 -lws2_32 -lsecur32`. `_WIN32_WINNT=0x0600` (`libqb-common.h:19`).
- macOS (:267): `-framework OpenGL IOKit Cocoa CoreVideo ApplicationServices CoreFoundation QuartzCore`; audio adds CoreAudio CoreMIDI AudioUnit AudioToolbox; `-s` strip (:270).

### DEP_* flags
| Flag | Makefile effect | Parts/libs | Compiler sets when |
|---|---|---|---|
| `DEP_GL` | `-DDEPENDENCY_GL` (:332) | gl helper code (`parts/core/gl_header_for_parsing`); GLFW+glad (core.a) always linked (:451) | `_GL` used (`qb64pe.bas:3066`) |
| `DEP_IMAGE_CODEC` | `-DDEPENDENCY_IMAGE_CODEC`; image.a (`video/image/build.mk:25`) | stb_image, nanosvg, tiny_webp, qoi, jo_gif, sg_curico, sg_pcx, hqx/mmpx/sxbr | `_LOADIMAGE`, `_SAVEIMAGE` |
| `DEP_SCREENIMAGE` | `-DDEPENDENCY_SCREENIMAGE`, implies image codec (:339, 385) | clip (clipboard.a is always linked, `os/clipboard/build.mk:56`) | `_SCREENIMAGE`, `_CLIPBOARDIMAGE` |
| `DEP_CONSOLE_ONLY` | `-DDEPENDENCY_CONSOLE_ONLY` (no `QB64_GUI`, no GL headers); `window-console.o`/`main-thread-console.o` instead of gui versions (`libqb/build.mk:30-31, 52-53`); `-mconsole` | removes graphics | `$CONSOLE:ONLY` (`qb64pe.bas:3597`) |
| `DEP_CONSOLE` | `-mconsole` (Windows) | - | `$CONSOLE` (ConsoleOn, :13717) |
| `DEP_SOCKETS` | `-DDEPENDENCY_SOCKETS` / `NO_SOCKETS`; `-lws2_32` | emitted with `DEP_HTTP=y` (:13709) | `_OPENHOST/_OPENCLIENT` |
| `DEP_HTTP` | libcurl (`network/http/build.mk`): bundled static build on Windows (`-DUSE_SCHANNEL -DUSE_WINDOWS_SSPI -DHTTP_ONLY`), system `-lcurl` elsewhere; `qb_http.o` vs `qb_http-stub.o` | libcurl | with sockets |
| `DEP_PRINTER` | `-DDEPENDENCY_PRINTER`/`NO_PRINTER`; `-lwinspool` | - | `LPRINT`, `_PRINTIMAGE` |
| `DEP_ICON`, `DEP_ICON_RC` | `-DDEPENDENCY_ICON`/`NO_ICON`; `ICON_RC` links `temp\icon.o` built by `windres` (:379-386, 482-486) | - | `_ICON`; `$EXEICON`/`$VERSIONINFO` (:13719) |
| `DEP_FONT` | font.a + freetype.a; else `stub_font.o` | FreeType | `_LOADFONT` etc. |
| `DEP_DEVICEINPUT` | game_controller.a + Windows libs | libstem_gamepad | `STRIG/STICK/_DEVICE*`, `ON STRIG` (:7535) |
| `DEP_AUDIO_MINIAUDIO` | audio.a (`audio/build.mk`, `audio/extras/build.mk`); else `stub_audio.o` | miniaudio, stb_vorbis, libxmp-lite, radv2, hivelytracker, qoa, foo_midi(+VSTiPlayer win), libmidi, primesynth, tinysoundfont, ymfmidi, 5 ma_vtable glue files | `PLAY, SOUND, _SND*` (~30 keywords) |
| `DEP_ZLIB` | data_processing.a; also for audio (:437) | miniz, modp_b64 | `_DEFLATE$/_INFLATE$`, base64, packed `$EMBED` |
| `DEP_EMBED` | `embedded.o` (:473) | - | `$EMBED` data used by `_EMBEDDED$` |

`QBLIB_NAME` (:328-393): `libqb_make_` plus a digit per feature (GL, IMAGE_CODEC, CONSOLE_ONLY, SOCKETS, PRINTER, ICON, SCREENIMAGE, DEVICEINPUT, AUDIO) so a precompiled `internal\c\libqb_make_<bits>.o` of `libqb.cpp` is cached per feature set (rule :477-480; compiled with `$(CXXFLAGS)` only, no explicit `-O`). Objects are not cleaned between builds; a bitness change purges via marker `internal\c\.qb64_target_bits` (`qb64pe.bas:1174-1186`).

### Object/library lists
`libqb/build.mk`: ~29 always-on sources (threading, buffer, command, environ, filesystem, gfs, qbs*, graphics, glut-emu, keyboard, mouse, console, logging...), conditional `window-gui/console`, `main-thread-gui/console`, `qb_http`/`qb_http-stub`, `threading-$(PLATFORM)`, Windows `logging/mingw/{file,pe,pe_symtab,symbol}`, Unix `logging/unix/symbol`; compiled `-O3 -Wall -Wextra`. Static libs via `ar rcs`: core.a (GLFW + glad), audio.a, data_processing.a, game_controller.a, font.a/freetype.a, image.a, clipboard.a, libcurl.a (Windows). tinyfiledialogs + gui.cpp are linked directly always (`gui/build.mk:18`). Optimization: parts `-O3` (gui/gamepad/clipboard `-O2`), third-party `-w`.

### How the compiler picks DEP flags
Each built-in keyword row in `source\subs_functions\subs_functions.bas` can carry `id.Dependency = DEPENDENCY_*`; `SetDependency` (`qb64pe.bas:28195`, called at :11573 and :20245) records it. 12 constants at `:94-105`. Flags are assembled at `:13706-13723`.

### Exact user-program pipeline (`qb64pe.bas:13541-14180`)
1. Compile passes write `internal\temp\*.txt` (flushed by `WriteBuffers`).
2. Windows resources: copy `$EXEICON` to `temp\icon.ico` (:13557); for `$VERSIONINFO` write `<exe>.manifest` (Common-Controls 6.0) and `manifest.h`; write `icon.rc` with `0 ICON "icon.ico"`, manifest resource, `1 VERSIONINFO` (:13560-13642).
3. `$EMBED`: write `embedded.cpp` with byte arrays and `func__embedded`; may add ZLIB (:13647-13689).
4. Make line (:13745-13770): `[internal\c\c_compiler\bin\mingw32-make.exe | make] <DEP flags> EXE=<escaped> "CXXFLAGS_EXTRA=..." "CFLAGS_EXTRA=..." "CXXLIBS_EXTRA=..." -j"<MaxParallelProcesses, default 3>" "BITS=<n>" [STRIP_SYMBOLS=n] [GENERATE_LICENSE=y] [USE_SYSTEM_MINGW=y] OS=<win|lnx|osx>`. Extra flags = user `ExtraCppFlags`/`ExtraLinkerFlags` + library flags; optimisation: `-O2` if `OptimizeCppProgram` (default off, `cfg_methods.bas:630`), with `IncludeDebugInfo` (default off) `-Og -g`, or just `-g`. `StripDebugSymbols` default true.
5. `internal/c/qbx[N].o` is deleted to force rebuild; for `DECLARE LIBRARY` static libs, `nm` output is parsed to emit externs into `global.txt` (:13787-14040).
6. `SHELL _HIDE ... 1>> compilelog 2>&1`. Make builds libqb object, part libs, `qbx.o`, optional `icon.o`/`embedded.o`, then `$(CXX) $(CXXFLAGS) objs -o EXE libs CXXLIBS` (Makefile:516-517); unless `STRIP_SYMBOLS=n` (non-mac): `objcopy --only-keep-debug EXE temp/EXE.sym` then `objcopy --strip-unneeded EXE` (:518-523). `.sym` supports runtime stack-trace symbolization.
7. Success is "output file exists" (:14150-14156); else "ERROR: C++ compilation failed. Check compilelog". Helper scripts `debug_win.bat` (lldb), `debug_lnx.sh`/`debug_osx.command` (gdb), `recompile_*` written into temp; macOS `<exe>_start.command` launcher (:14083-14098).

### Compiler CLI (`qb64pe.bas:14395-14700`)
`-c`, `-x` (console progress), `-y` format (needs `-o`), `-z` generate C++ only, `-p` purge, `-e` force OPTION _EXPLICIT, `-s[:DebugInfo|ExeWithSource|ExeDefaultDir]` persistent, `-f:` temporary (OptimizeCppProgram, StripDebugSymbols, AbsoluteDebugPaths, ExtraCppFlags, ExtraLinkerFlags, MaxCompilerProcesses, TargetBits, GenerateLicenseFile, UseSystemCompiler, layout settings), `-w`, `-q`, `-m`, `-u` (help update, CI), `-l:n`, `-v`, `-?`. Compiled test programs honor env `QB64PE_NOPROMPT`, `QB64PE_LOG_HANDLERS/SCOPES/FILE_PATH`.

## 3. Vendored third-party libraries
License files in `licenses\` (30 files; index `licenses\README.md`). Versions read from headers where present.

| Library | Version | Location (`internal\c\`) | License | Purpose |
|---|---|---|---|---|
| GLFW | 3.5.1 | parts/core/glfw | zlib | window/GL context/input (always unless console-only) |
| glad | 2.0.8 (GL 4.6 compat, gen. 2026-02-14) | parts/core/glad | WTFPL/CC0 + Apache-2.0 (header SPDX) | GL loader |
| gl.h parse copy | - | parts/core/gl_header_for_parsing | - | parsed by compiler at start for `_GL` constants/functions |
| miniaudio | 0.11.25 | parts/audio/miniaudio | MIT/PD | audio engine |
| stb_vorbis | 1.22 | parts/audio/extras/stb | MIT/PD | OGG decode |
| libxmp-lite | 4.6.3 | .../libxmp-lite | MIT | MOD/S3M/XM/IT |
| HivelyTracker replay | n/a | .../hivelytracker | BSD-3 | HVL/AHX |
| QOA | n/a | .../qoa | MIT | QOA audio |
| RADv2 (opal) | n/a | .../radv2 | public domain | Adlib RAD |
| foo_midi + libmidi | n/a | .../foo_midi, libmidi | MIT | MIDI |
| primesynth | n/a (modified by a740g) | .../primesynth | MIT | SF2 synth |
| TinySoundFont | 0.9 | .../tinysoundfont | MIT | SF2 synth |
| ymfmidi | n/a | .../ymfmidi | BSD-3 | OPL MIDI |
| stb_image / stb_image_write | 2.30 / n/a | parts/video/image/stb | MIT/PD | image decode/encode |
| nanosvg | n/a | .../nanosvg | zlib | SVG raster |
| tiny_webp | n/a | .../tiny_webp | BSD-3 | WebP |
| QOI | n/a | .../qoi | MIT | QOI |
| HQx, MMPX, Super-xBR | n/a | .../pixelscalers | Apache-2 / MIT / MIT | pixel scalers |
| jo_gif, sg_curico, sg_pcx | n/a | .../jo_gif, sg_curico, sg_pcx | not in `licenses\` list (sg_* are in-house loaders by a740g) | GIF write, ICO/CUR, PCX |
| FreeType | 2.14.1 | parts/video/font/freetype | FTL | `_LOADFONT` (flattened subset build) |
| libcurl | 8.17.0 | parts/network/http/curl (Windows only) | curl | HTTP (system lib on unix) |
| miniz | 11.3.1 | parts/data | MIT | deflate/inflate |
| modp_b64 | n/a | parts/data | MIT | base64 |
| libstem_gamepad | n/a | parts/input/game_controller | MIT | gamepads |
| tinyfiledialogs | n/a | parts/gui | zlib | dialogs/alerts |
| Clip | n/a (2015-2024) | parts/os/clipboard/clip | MIT | clipboard text/image |
| LLVM-MinGW | latest at download | internal/c/c_compiler (downloaded) | various; libstdc++ GPLv3+exception | Windows toolchain/runtime, shipped in Windows dist |

`license_qb64.txt` covers libqb itself (MIT).

## 4. CI and release
Workflows: `ci.yml` (push to main), `pr.yml` (PR to main), `release.yml` (tags `v*`; wraps the build with Cloudflare Bot-Fight-Mode off/on jobs), all calling reusable `build-process.yml`.
Matrix (`build-process.yml:10-34`, fail-fast off): ubuntu-24.04 x64; ubuntu-24.04 x86 (real 32-bit multilib); macos-15-intel x64 (Xcode 16.4; no Apple Silicon leg); windows-2022 x64; windows-2022 x86; windows-11-arm arm64.
Steps: checkout (full depth); Linux deps incl. i386 multilib with a `--force-overwrite` workaround for libcurl (:78-99); PulseAudio dummy device (:101-105); `calculate_version.sh`; `read-version.sh`; bootstrap; compile; `tests/run_c_tests.sh`; `./qb64pe -?`; on tags, Cloudflare IP allowlisting (help download); `tests/run_tests.sh` (`CI_TESTING=y`, `CI_OS`, ALSA null config, timeout 105 min); `make-dist.sh`; `tests/run_dist_tests.sh`; internal/source push; artifact upload (`tests/results/` + archives, always); on tag, draft release via `softprops/action-gh-release`.
Bootstrap details: Windows always builds the bootstrap as 64-bit (`bootstrap.bat:6-11`); for x86 it then swaps in the 32-bit toolchain and cross-compiles the real compiler with `-f:TargetBits=32` (`bootstrap.bat:19-26`, `compile.bat:4-9`). Linux x86 passes `-f:TargetBits=32` and `BITS=32` (`compile.sh:5-11`). Windows arm64 uses the aarch64 toolchain; `windres` target flag is omitted for aarch64 (`Makefile:130-136`). `compile.*` also runs `make build-tests` and `make clean`.
Versioning: SemVer `X.Y.Z` in `source\global\version.bas` (`Version$` + two `$VERSIONINFO` lines; also a manual user-agent string in `internal\c\libqb\src\qb_http.cpp`, `version.bas:4-9`). Non-release builds append a tag in `internal\version.txt`: `-<commits since last tag>-<short hash>` plus `-dirty` (`calculate_version.sh:16-26`); an exact-tag build removes the file. A local checkout has `-UNKNOWN` (tracked file; `version.bas:18-30`; `IsCiVersion` only for other labels). `read-version.sh` builds the artifact version (including suffix like `-GLFW`).
Release process (`docs\build-system.md`): bump version.bas, wait for main CI + internal/source update, tag `vX.Y.Z`, wait for draft release, add notes, publish.
Packaging (`make-dist.sh`): release builds download `https://qb64phoenix.com/qb64_files/help.zip` into `internal/help` and run `./qb64pe -u`; `make clean`; assemble `dist/qb64pe/` from `source/ licenses/ COPYING.txt README.md qb64pe.1 Makefile internal/{source,help,support,temp,config.ini,version.txt} internal/c/{libqb,parts,*}`. Windows: `qb64pe_win-<platform>-<version>.7z` with prebuilt `qb64pe.exe` and `internal/c/c_compiler`. Linux: `qb64pe_lnx-<platform>-<version>.tar.gz` + `setup_lnx.sh` (no binary). macOS: `qb64pe_osx-<version>.tar.gz` + `setup_osx.command`, `qb64pe_start.command`, man page.

## 5. Tests

Runner chain: `tests/run_tests.sh` runs four suites through `tests/assert.sh <script> ./qb64pe`; `run_dist_tests.sh`, `run_c_tests.sh` separately. `assert.sh` provides `assert_success_named`, counts, colored PASS/FAILURE/IGNORED (`colors.sh`), and sums failures into the exit code. Bash-only (process substitution, `pushd`, `dd`, `od`, `xvfb-run` on Linux). Output in `tests/results/` (gitignored).

### 5.1 compile_tests (`tests/compile_tests.sh`)
- Layout: `tests/compile_tests/<category>/<name>.bas` plus `<name>.output` (expected program stdout) or `<name>.err` (expected compiler error text). 75 category dirs; 404 `.bas`, 331 `.output`, 56 `.err`, 17 compile-only (no expectation: icon, icon_relative, versioninfo 1-3, winresource, sockets, opengl, filename, audio_out/audio_test, basic/test...). Sidecars: `.compile-from-base` (4), `.noprompt` (2), `<name>.<os>.license` (18; checks generated license file with `-f:GenerateLicenseFile=true`), `.flags` (supported by runner, 0 files used). Assets: bmp 82, ico 27, pcx 20, png, webp, svg, ttf, sf2, mid, and .so/.dll/.dylib/.a/.h/.c for DECLARE LIBRARY.
- Run per test: `rm -fr internal/temp/*`; compile from the test's directory with `-f:OptimizeCppProgram=true -f:StripDebugSymbols=false [flags] -q -m -x name.bas -o <exe>`; success tests check compile OK, exe exists, license, then run with `QB64PE_NOPROMPT` and log env vars, args `<resultsdir> <category-name>`, under `xvfb-run` on Linux, merged stdout+stderr compared with `.output` ignoring trailing newlines (custom CRLF-aware byte logic); error tests require compile failure, no exe, and `diff -y` of compiler stdout vs `.err` (exact error text including "Caused by (or after): / LINE n:"). Usage `compile_tests.sh <qb64> [category] [glob]`. Roughly 1,500 asserts in total (estimate).
- Content: 357 of 404 `.bas` use `$CONSOLE:ONLY` (+10 `$CONSOLE`); the 37 without are mostly the compile-only and `.err` tests. 148 expected outputs use `PASS <name>` self-checking lines. Biggest categories: arrays 211 (172 output + 36 err; array/UDT/REDIM/OPTION BASE), const 20 (12 err), types 16 (8 err), glut 19 (window functions; on Windows/macOS degrade to console-only because runners cannot create GL contexts, e.g. `glut/title.bas`), qb64pe 10 (compiler-internals unit tests that `$INCLUDE` `source/utilities/*.bas`), precomp-flags 9, console_only 9, paint 6, auto_include 6, func_tostr 6, source_ordering 6, image 5, timer 4, declare_library* 9, include_* 5, midi 3, http 3, noprompt 3, rest 1-2 each (audio_*, clipboard, colors, data, embed, environ, filesystem, font, hash, iif, keyboard, libz, logging, mem, offset, on_error, operators_test, overloaded, print, putimage, rotations, screenimage, view, cast, deviceinput, plus odd directory names `name with spaces`, `single'quote'test`, `dollar$sign$test`, `parens()` that test filename escaping).
- Graphics tests are off-screen: `utilities/imageassert.bm` `AssertImage` converts an image to 32-bit, saves a BMP to results, compares pixels with an expected PNG (12 tests); `utilities/assert.bm` has `AssertString/AssertBool` printing `PASS:`/`FAIL:`.
- Not covered: the IDE entirely; real windowed rendering/GL output (opengl is compile-only); audible audio (dummy device, state/handles only); interactive keyboard/mouse; sockets (compile-only; http needs internet); printer; broad QBasic semantics (PRINT USING, file I/O modes, string/math functions, DATA/READ etc. only lightly covered); no performance tests.

### 5.2 qbasic_tests (`tests/qbasic_tests.sh`)
Compile-only smoke test over `tests/qbasic_testcases/{n54 (3), open_gl (2), pete (68), qb45com (5), thebob (19), misc (46)}` = 143 `.bas` (plus data files: spr, lev, txt, etc.). One assert each ("Compile"); no run, no output check. `docs\testing.md` calls the folder `qb64_testcases` (stale).

### 5.3 format_tests (`tests/format_tests.sh`)
Tests the formatter (`-y`). 5 categories (general, lineup, lbound_ubound, meta_format, dynamic_member_redim), 5 `.bas`, each with a `.flagmap` listing `<expected-file> -f:flags...` lines; about 36 expected `.out` variants, 3 asserts each. Output compared with `\r` stripped. Tied to QB64pe layout rules.

### 5.4 converter_tests (`tests/add_prefix_test.sh`)
One test: build `internal/support/converter/AddPREFIX.bas` (adds `_` prefix to `$NOPREFIX` code), run it on `converter_tests/addprefix.bas`, diff against `addprefix.output` (3 asserts).

### 5.5 dist tests (`tests/dist_tests.sh`, `tests/dist/`)
Inside the unpacked dist: `internal/temp` holds exactly one file; Windows: `llvm-objdump -s -j .rsrc qb64pe.exe` proves resources; Linux/macOS: run setup script (rebuild from internal/source); then compile `tests/dist/console.bas` and compare to `console.result` ("This is a Dist test").

### 5.6 C++ unit tests (`tests/c`, `tests/build.mk`, `tests/run_c_tests.sh`)
`test.h/test.cpp` (186 lines): `struct unit_test {fn,name}`, `run_tests(module, tests, n)`, `test_assert*` macros (bool/int/buffer compare), colored output, failure via exit code. `buffer.cpp` (7 tests of `libqb/src/buffer.cpp`: single/multiple/partial/full read-write, read-past-end, interleaved) and `http.cpp` (1 test: 7 URL forms against `www.example.com` through `libqb_http_open/get/get_length/get_content_length/close`; needs network and a hard-coded page body, so brittle). `tests/build.mk` (included at Makefile end) defines `build-tests` producing `tests/exes/cpp/<name>_test[.exe]` with `-g -std=gnu++11` and per-test source lists; `run_c_tests.sh` hardcodes `buffer http`. Only libqb parts linkable in isolation are tested; the rest of `libqb.cpp` has no C++-level tests.

### 5.7 Reusability as a conformance suite for a rewrite
Black-box and reusable as-is: the ~331 `.output` tests (BASIC in, stdout out), especially the 357 `$CONSOLE:ONLY` ones; arrays (172), const, types, source_ordering, func_tostr, overloaded, iif, operators_test, on_error, timer, environ, filesystem, data, hash, libz, cast, include_*/auto_include, precomp-flags; and the 143 qbasic_testcases as a "must compile" corpus without oracles. The `PASS name` self-check convention is implementation-neutral.
Coupled to the current implementation: 56 `.err` tests (exact QB64pe error strings and "Caused by (or after)/LINE n:" format); `qb64pe/*` tests (include the compiler's own source utilities); format_tests and converter test; `.license` tests (generated license text, which libs get linked); `noprompt` tests (exact "Runtime error: Line: 5 (in main module) / Unprintable error" text, `QB64PE_NOPROMPT`); logging tests (`QB64PE_LOG_*`); CLI flags (`-f:`, `-x`, `-m`, `-q`, `-o`); `DECLARE LIBRARY` tests needing prebuilt native artifacts and a C toolchain; C++ tests (libqb internals); dist tests.
Environment-dependent: http (internet), audio (dummy device), glut (xvfb on Linux), clipboard/screenimage (display), timers (wall clock), image tests (need `_LOADIMAGE/_MEMIMAGE/_PUTIMAGE` in the runtime).
Harness needed on Windows: the shell scripts need bash (Git Bash/MSYS) plus find/diff/cmp/od/dd/head/wc. For a rewrite, write a small harness (Python/PowerShell/Go) that: enumerates `*.bas`; classifies by sibling (`.output` / `.err` / none); compiles with the new compiler into a temp dir (cwd = test directory unless `.compile-from-base`); runs with args `<results dir> <category-name>` and `QB64PE_NOPROMPT=y`; merges stdout+stderr; compares after stripping trailing newlines and `\r`; loosens `.err` (fail + first line) early on; supports skip/known-fail tags per category (glut, audio, http, sockets, clipboard, screenimage, opengl, license, noprompt, qb64pe, logging); handles odd directory names (spaces, quotes, `$`, parentheses); appends `.exe` on Windows.

## 6. Platforms, toolchains, hacks
- README: Windows 7+, Linux, macOS Catalina+ (`README.md:5`). CI builds Linux x64/x86, macOS x64 (Intel), Windows x64/x86/arm64. ARM macros exist (`libqb-common.h:54-58`); no ARM Linux/macOS CI.
- Minimum toolchain: C++20 compiler (g++ or clang), GNU make (mingw32-make from llvm-mingw on Windows), binutils (`objcopy`, `nm`, `ar`); Windows adds `windres`, `llvm-objdump`, `lldb`; Linux dev packages: OpenGL/GLU, X11 + Xcursor/Xrandr/Xinerama/Xi, ALSA, libpng, libcurl, xcb; macOS Xcode CLT (CI uses Xcode 16.4).
- Hacks: parse-time `$(shell cp ...)` of internal/source; `SHELL := cmd` and path macros on Windows; `-fdebug-prefix-map` vs `-fdebug-compilation-dir` for relative debug paths; `-no-pie` on Linux; `-rdynamic` only when symbols kept; objcopy symbol split except macOS; `libqb_make_<flags>.o` caching with `.qb64_target_bits` marker; `DEPENDENCY_CONSOLE_ONLY` defined for some parts' .cpp to avoid GLFW headers via `common.h`; GLFW compiled with X11 and Wayland sources on Linux; `_CRT_glob=-1` (`qbx.cpp`) for llvm-mingw globbing; 32-bit Windows `nm` leading-underscore matching (`qb64pe.bas:13844, 13905`); macOS `nm` lacks demangling (:14000); libcurl multilib bug workaround; Cloudflare allowlisting for help download; `chmod +x *.command` on mac; PulseAudio/ALSA null device and `xvfb-run` in CI; clip needs `-DHAVE_XCB_XLIB_H -DHAVE_PNG_H` on Linux; FreeType hand-flattened subset with manual header edits (`video/font/build.mk:1-13`); GLFW/glad/curl update procedures documented as comments in their build.mk.
- Support dirs: `internal\support\include\{beforefirstline.bi, aftermain.bas, afterlastline.bm}` (auto-includes), `color\{color0,color32}.bi`, `vwatch\{vwatch.bi,.bm,vwatch_stub.bm}` ($DEBUG variable watcher), `converter\{AddPREFIX.bas, QB45BIN.bas, qbjs-build.bas}`. `internal\temp\temp.bin` is the lock file for the temp-dir slot (`qb64pe.bas:330-338`). `.gitignore` ignores `internal/temp*`, `*.o *.a *.exe`, `internal/c/c_compiler`, `internal/config*`, `internal/help`; `source/.gitignore` and `tests/.gitignore` re-allow `*.bas`/`*.output`/`*.err`. `.gitattributes`: `text=auto`, linguist overrides for `.bas/.bi/.bm`. `qb64pe.1` (Aug 2022, stale), `.clang-format` (C++), `SAMPLES.txt` (pointer to forum).

## 7. docs\ - conventions and architecture intent worth preserving
- `contributing.md` (8 lines): US-English in code/comments; BASIC edited via the IDE with Code Layout on (line indent, SUB/FUNC indent, single spacing), indent 4, UPPER keywords; follow surrounding style so diffs stay clean.
- `auto-including.md`: auto-include model (since v4.0.0; `$USELIBRARY` since 4.3.0): AtTop (`.bi`, `firstLine`), AfterMain (`.bas`, `mainEndLine`), AtBottom (`.bm`, `lastLine`); control variables have states 0 inactive, 1 triggered, 2 in progress, 3 done. Order: beforefirstline.bi, color0/32.bi, AtTop libs (reverse `$USELIBRARY` order for dependencies), vwatch.bi, user code, aftermain.bas (implicit END), AfterMain libs, vwatch.bm/stub, AtBottom libs, afterlastline.bm. Each `$USELIBRARY` triggers a recompile. Since 4.4.0 main and SUB/FUNCTION code may interleave. Libraries come from `libraries/includes/<author>/<libname>`.
- `build-system.md`: 9-step CI process, repo layout, Makefile parameter tables with the warning that only setup scripts and QB64-PE should call the Makefile, SemVer policy, release steps.
- `testing.md`: no trailing spaces or trailing blank lines in expected output; `QB64PE_NOPROMPT=y`; compile from the test dir; C++ tests via `test.h` + `tests/build.mk`.
Intent worth keeping: a committed, CI-verified bootstrap; per-feature dependency pruning (small programs do not link audio/GL; license output follows what is linked); tests as BASIC + expected-output pairs; relative `#line`/debug info for portable debugging; CI version labels; quiet/monochrome CLI modes for automation.

## 8. Not determined
- Versions of several vendored libs (QOA, RADv2, HivelyTracker, ymfmidi, libmidi/foo_midi, primesynth, clip, libstem_gamepad, tinyfiledialogs, nanosvg, QOI, modp_b64, HQx/MMPX/sxBR, jo_gif).
- Whether the fixed `BUILD_QB64` DEP set mirrors what the compiler would compute for itself (it omits GL/image/audio/screenimage/deviceinput, which the IDE seems to need); `internal/source/compilelog.txt` was only partly inspected.
- Which `-O` level `libqb.cpp`/`qbx.cpp` get without `OptimizeCppProgram` (the Makefile adds none for them).
- No tests were executed; counts come from file listings and assertion totals are estimates.
