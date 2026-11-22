# Proposal

## Why

The wasm target builds for `wasm32`, where a pointer is 4 bytes. The user accepted a 4-byte `_OFFSET` there on
2026-11-19 (`DECISIONS.md`). The front end still takes every pointer as 8 bytes: `_OFFSET`, the members of
`_MEM`, a `STRING` member's slot, a dynamic member array's slot and alignment. libqb on `wasm32` uses 4. A
program built with both would read and write the wrong bytes. So W0 (`wasm-console`) refused such programs:
270 verification programs are on `tests\verification\known.wasm.list` with the reason `offset`, and no program
that uses `_MEM` builds for the web.

This change is step W0.5 of milestone M9 (`ROADMAP.md`, `study\51` §5.3 and §8). It makes the pointer's size a
property of the target, so these programs build. It also makes libqb's flags between threads atomic. A browser
runs on ARM phones too, and there plain flags are not safe (`study\03` §1.5).

## What Changes

- **A pointer's size comes from the target.** `sema` and `codegen-cpp` take it from the platform: 8 bytes on
  Windows and Linux, as now, and 4 on wasm. It sets `LEN` of an `_OFFSET`, the layout of a `TYPE` that holds an
  `_OFFSET` or a `STRING`, the slot and alignment of a dynamic member array, the `TYPE` and `ELEMENTSIZE` that
  `_MEM` gives for an `_OFFSET`, and `GET` and `PUT` of an `_OFFSET`.
- **`_MEM` is 40 bytes on wasm.** That is libqb's `mem_block` on `wasm32`: five 4-byte pointers, an 8-byte lock
  id at offset 8 and two `LONG`s, rounded up to 8. QB64pe's 32-bit build says 36 (`qb64pe.bas` line 1594), which
  is smaller than the C struct it is copied into. That gets a `DIVERGENCES-QB64PE.md` row.
- **A value of `_OFFSET` holds 32 bits on wasm.** `HEX$`, `OCT$` or `BIN$` of an `_OFFSET` gives the 32-bit
  value. Arithmetic keeps the operator rules measured on QB64pe's 64-bit build.
  It is computed in 64 bits and cut to 32 when it is stored.
- **The generated C++ checks the sizes.** It asserts at compile time that `ptrszint` and `mem_block` have the
  sizes the front end used. A mismatch then fails the C++ build. It never becomes a program that runs wrongly.
- **Removed: the wasm target's "not available yet" errors for `_OFFSET`, `_UNSIGNED _OFFSET`, `_MEM`, the
  built-ins that take or give one, and a member array of `STRING`.** The other errors of `check_target` stay.
- **libqb's flags between threads become `std::atomic`.** These are `lock_display`, `lock_display_required`,
  `lock_mainloop`, `autodisplay`, `ontimerthread_lock`, `exit_ok`, and the polled flags `suspend_program`,
  `stop_program` and `close_program`. The change is edit files, as in W0. It runs on every target, so the
  desktop suites run too. The generated C++ does not change. `qbevent` and the three display lock counters stay
  plain: an atomic `qbevent` made a tight loop three times slower (design D7).
- **The 270 `offset` lines of `known.wasm.list` are judged.** Each one passes and leaves the list, or gets one of
  the other reasons. A program whose output shows an 8-byte pointer is listed as `width`. The reason `offset` is
  removed from the list's set.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `compiler/wasm-target`: "Pointer-sized types wait for their width" is removed. A new requirement gives the
  pointer's size on this target and lists the sizes it changes. "The C++ is the same as for Linux" allows the
  differences that come from the pointer's size.
- `language/numeric-semantics`: "Numeric types" says `_OFFSET` is 64 bits wide. It becomes the target's pointer
  width, and `LEN` of a `_MEM` is 56 on Windows and Linux and 40 on wasm.
- `testing/golden-corpus`: "The wasm known list" drops the reason `offset`.

## Impact

- `crates\base` (`Platform`'s pointer size), `crates\sema` (the program's pointer size, `size_of`, the `_MEM`
  type, `layout_bytes`, literals and folding of `_OFFSET`, `target.rs`), `crates\codegen-cpp` (`decl.rs` layout,
  `mem.rs`, `io.rs`, the size checks), and `crates\ir` where it calls `size_of`.
- `libqb`: new edit files under `tools\libqb\edits\` for the atomic flags.
- `tests\verification\known.wasm.list`, and a few new verification programs whose output is the same on 64 and
  32 bits.
- Documents: `DIVERGENCES-QB64PE.md`, `DECISIONS.md`, the reference chapter `wasm-target.md`, and the pages
  `_offset.md`, `_mem.md` and `len.md`.
- Runs: the wasm suite in the VM, and the desktop suites for the libqb edits.
