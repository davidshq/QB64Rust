# Source of `libqb/`

Commit: `16f629784e` (2026-09-30) of QB64pe (`github.com/QB64-Phoenix-Edition/QB64pe`), the reference clone's commit
(`DECISIONS.md` 2026-10-09).

`libqb/` holds QB64pe's `Makefile`, `internal/c/` (without the toolchain `internal/c/c_compiler`, which no git
tracks; the build takes it from the clone), `licenses/` and `COPYING.txt`: 1232 files, 26889412 bytes,
each the commit's file byte for byte under its relative path, but for the edits below. The new compiler builds
against it with `make -C libqb` (design of the OpenSpec change `libqb-copy`); the libqb objects it builds here are
git-ignored.

Edits of this project, applied by the tool after copying (each file's header says why; `DIVERGENCES-QB64PE.md` has the
row, `UPSTREAM.md` the fix as offered to QB64pe):

- `tools/libqb/edits/atomic-thread-flags-b-libqb-polled.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/atomic-thread-flags-c-libqb-lock-mainloop.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/atomic-thread-flags-d-libqb-autodisplay.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/atomic-thread-flags-e-libqb-exit-ok.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/atomic-thread-flags-f-libqb-lock-display.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/atomic-thread-flags-g-libqb-close-program.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/atomic-thread-flags-h-libqb-suspend-stop.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/atomic-thread-flags-i-qbx-lock-mainloop.edit`: `internal/c/qbx.cpp`
- `tools/libqb/edits/atomic-thread-flags-j-qbx-exit-ok.edit`: `internal/c/qbx.cpp`
- `tools/libqb/edits/atomic-thread-flags-k-qbx-close-program.edit`: `internal/c/qbx.cpp`
- `tools/libqb/edits/atomic-thread-flags-l-qbx-polled.edit`: `internal/c/qbx.cpp`
- `tools/libqb/edits/atomic-thread-flags-m-qbx-ontimerthread-lock.edit`: `internal/c/qbx.cpp`
- `tools/libqb/edits/atomic-thread-flags-p-error-handle-include.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/atomic-thread-flags-q-error-handle-close-program.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/atomic-thread-flags-r-keyboard-polled.edit`: `internal/c/libqb/src/keyboard.cpp`
- `tools/libqb/edits/atomic-thread-flags-s-keyboard-include.edit`: `internal/c/libqb/src/keyboard.cpp`
- `tools/libqb/edits/call-trace-a-error-hook-header.edit`: `internal/c/libqb/include/error_handle.h`
- `tools/libqb/edits/call-trace-b-critical-message.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/call-trace-c-fix-error.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/d012-error-exit-code-no.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/d012-error-exit-code.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/d031-coth.edit`: `internal/c/libqb/include/extended_math.h`
- `tools/libqb/edits/d048-console-eof-end.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d048-console-eof-input-string.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d048-console-eof-input.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d049-bload-past-block.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d049-bsave-past-block.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d050-memnew-zero-lock.edit`: `internal/c/libqb/src/memblock.cpp`
- `tools/libqb/edits/d051-statuscode-missing-handle.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d052-division-error-text.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/d052-division-error-trappable.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/d052-division-fatal-entry.edit`: `internal/c/libqb/src/error_handle.cpp`
- `tools/libqb/edits/d052-division-fatal-header.edit`: `internal/c/libqb/include/error_handle.h`
- `tools/libqb/edits/d052-division-signal-handler.edit`: `internal/c/qbx.cpp`
- `tools/libqb/edits/d053-bit-numbers-modulo-64.edit`: `internal/c/libqb/include/bitops.h`
- `tools/libqb/edits/d056-screenclick-display-scale.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d062-raw-stream-lock.edit`: `internal/c/parts/audio/audio.cpp`
- `tools/libqb/edits/d066-string-pool-growth.edit`: `internal/c/libqb/src/qbs.cpp`
- `tools/libqb/edits/d066-string-pool-size.edit`: `internal/c/libqb/src/qbs.cpp`
- `tools/libqb/edits/d067-readfile-too-long.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/d067-string-add-too-long.edit`: `internal/c/libqb/src/qbs.cpp`
- `tools/libqb/edits/d067-string-new-negative.edit`: `internal/c/libqb/src/qbs.cpp`
- `tools/libqb/edits/wasm-console-lock-keys.edit`: `internal/c/libqb/src/window-console.cpp`
- `tools/libqb/edits/wasm-filesystem-cstdlib.edit`: `internal/c/libqb/src/filesystem.cpp`
- `tools/libqb/edits/wasm-libqb-common.edit`: `internal/c/libqb/include/libqb-common.h`
- `tools/libqb/edits/wasm-makefile-flags.edit`: `Makefile`
- `tools/libqb/edits/wasm-makefile-unix.edit`: `Makefile`
- `tools/libqb/edits/wasm-os-name.edit`: `internal/c/libqb.cpp`
- `tools/libqb/edits/wasm-qbs-str-pointer.edit`: `internal/c/libqb/include/qbs.h`
- `tools/libqb/edits/wasm-stacktrace-none.edit`: `internal/c/libqb/src/logging/stacktrace.cpp`
- `tools/libqb/edits/wasm-window-a-glut-includes.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-b-glut-native-display.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-c-glut-native-context.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-d-glut-native-window.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-e-glut-main-loop.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-f-glut-icon.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-g-webgl-shim.edit`: `internal/c/common.h`
- `tools/libqb/edits/wasm-window-h-glut-fullscreen.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-i-glut-characters.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-j-glut-key-name.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-k-glut-exit-message.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-l-glut-program-exit.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-m-glut-swap-interval.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-n-glut-lock-key-mods.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-o-glut-title.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-p-gui-alert.edit`: `internal/c/parts/gui/gui.cpp`
- `tools/libqb/edits/wasm-window-q-gui-include.edit`: `internal/c/parts/gui/gui.cpp`
- `tools/libqb/edits/wasm-window-r-glut-cursor-mode.edit`: `internal/c/libqb/src/glut-emu.cpp`
- `tools/libqb/edits/wasm-window-s-audio-vsti.edit`: `internal/c/parts/audio/extras/build.mk`
- `tools/libqb/edits/wasm-window-t-audio-main-thread.edit`: `internal/c/parts/audio/miniaudio/miniaudio.c`
- `tools/libqb/edits/wasm-window-u-glut-focus.edit`: `internal/c/libqb/src/glut-emu.cpp`

Licence: MIT for QB64pe (`COPYING.txt`, `licenses/license_qb64.txt`); the parts under `internal/c/parts` keep
their own licences, listed in `licenses/`.

Made by `tools/libqb/copy_libqb.py`; `--check` says whether the copy still equals the commit with its edits. Do not
edit the copied files by hand: add an edit file and run the tool.
