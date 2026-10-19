# Design

## Context

`build_with_make` (`crates\driver\src\build.rs`) runs `make -C <copy>` with `CXXFLAGS_EXTRA=[-O2 ]-fwrapv
-I<copy>/internal/c`. QB64pe's `Makefile` (line 30) adds `CXXFLAGS_EXTRA` to `CXXFLAGS`, which every compile rule
uses: `qbx.o`, `embedded.o` and each libqb object (`%.o: %.cpp`, `$(QBLIB)`). `qb64pe.exe` does the same with its
own `-f:` settings (`qb64pe.bas` 13724–13761: `ExtraCppFlags`, `-O2` for `OptimizeCppProgram`), so in QB64pe too
the objects take the flags of the build that makes them first. libqb's own `build.mk` files give each part its
flags (`libqb\src\qbs.cpp` is built with `-O3 -Wall -Wextra`), before `CXXFLAGS_EXTRA`.

`make` looks at file times only. An object built under other flags, or by another toolchain, is never rebuilt.
The extension prepares one copy per version and `libqb.id` (`vscode\src\compiler\libqbStore.ts`) and builds with
the user's QB64pe toolchain (`--qb64pe-root`).

## Goals / Non-Goals

**Goals:**

- libqb's objects are built the same way whatever build makes them.
- A copy whose objects were built otherwise (an older driver, another toolchain) gets them built again.

**Non-Goals:**

- Building libqb with `-O2` for `OptimizeCppProgram`. libqb's parts set their own optimisation; `qb64pe.exe`
  without `-f:` settings gives none, and that is what the recordings were made with.
- Hashing the binary into the extension's `libqb.id` (`STATUS.md`: fix it in the driver).
- The flaky `s75_sound_wait` (`libqb-copy`'s "Risks"): whether the flags played a part is unknown. With libqb
  built without `-fwrapv`, the next runs show whether the crash was related.

## Decisions

### D1. The program's flags go to its own objects only

The driver passes no `CXXFLAGS_EXTRA`. It writes `program.mk` in the build folder and runs `make -C <copy> … -f
Makefile -f <build>/program.mk exe`:

```make
$(QB_QBX_OBJ) $(PATH_INTERNAL_TEMP)/embedded.o: private CXXFLAGS += [-O2 ]-fwrapv -I<copy>/internal/c
```

A target-specific variable reaches that target and the prerequisites `make` builds for it; `private` (GNU make
3.82, the clone's is 4.4.1) keeps it from the prerequisites. `qbx.o` depends on `qbx.cpp` and the fragments,
`embedded.o` on `embedded.cpp`, none of them built, so no libqb object takes the flags either way; `private` keeps it
so if a prerequisite is added (found in review).
`-I` stays with them: `qbx.cpp` and `embedded.cpp` lie in the build folder and include `common.h` from the copy;
libqb's objects find their headers by their own paths, as in QB64pe. The makefile is read after the `Makefile`, so
`QB_QBX_OBJ` is defined. Checked with `make -n -B` on a kept build folder: `qbx.cpp` is compiled with `-fwrapv`,
`libqb\src\qbs.cpp` with `-O3 … -Wall -Wextra` and no flag of the driver.

Alternative: edit the copy's `Makefile` to give `CXXFLAGS_EXTRA` to `qbx.o` only. Rejected: a second makefile does
it without an edit of the copy.

Code: `crates\driver\src\build.rs` (`program_makefile`, `build_with_make`).

### D2. A stamp names what the copy's objects were built with

`<copy>\objects.stamp` holds `qb64rust libqb objects 1: no extra flags` and one line per executable of the clone's
`internal\c\c_compiler\bin` (relative path, size, time), the stamps the build key already takes. Before `make`, a
build compares the file with the stamp it would write. If they differ (no stamp: objects from before this change;
another toolchain), it removes every file under the copy's `internal\c` that builds write (`written_by_builds`:
`.o`, `.a`, `.tmp`) and writes the stamp. `make` then builds what the program needs.

With `QB64RUST_OBJECTS_LOCK` set (the runner, the CLI tests, the extension), this happens holding the lock alone:
`lock_objects` takes it shared, finds the stamp stale, takes it alone, checks again (another build may have done
it meanwhile) and keeps it alone for the build, which then builds objects. Without the lock (a single build from
the command line), it is done without one.

The stamp lies at the top of the copy, outside `internal\c`, so the build key's stamps of the copy and
`copy_libqb.py --check`, which look at `Makefile`, `internal\c`, `licenses` and `COPYING.txt`, do not see it.
`.gitignore` names it. The extension's copies get it from the driver.

Alternative: put the flags into the objects' names. Rejected: the names come from the `Makefile`.

Code: `crates\driver\src\build.rs` (`objects_stamp`, `objects_current`, `refresh_objects`, `remove_built`,
`lock_objects`), `.gitignore`.

### D3. The build cache's key

The key's version goes to 5: `build_with_make`'s fixed arguments changed. An entry made before has `qbx.o` built
the same way, but the version is what says that the arguments changed.

## Risks / Trade-offs

- **libqb's objects lose `-fwrapv`.** `qb64pe.exe` built them without it, and the recordings come from those
  objects, so this moves libqb back to how QB64pe builds it. A libqb function that relied on wrapping would be
  undefined in both. → The slice list, the upstream pass list and the CLI tests that build check behaviour.
- **One rebuild of libqb's objects** in every existing copy (a few minutes for all of them, once).
- **A toolchain changed in place with the same sizes and times** is not seen. The build key has the same limit.
