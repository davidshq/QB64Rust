# Design

## Context

What W0 left is in `proposal.md`, "Why". These facts about the code shape the design:

- **The front end has no pointer width.** `Ty::Off` and `Ty::UOff` are 64 bits in `Ty::int_bits`
  (`crates\sema\src\lib.rs`), 8 bytes in `size_of` (same file), and `_MEM` is a `UserType` of five `Ty::Off`, an
  `I64` and two `I32` (`mem_type`, same file). `codegen-cpp` writes 8 in its own places: `Layout::ty_size` gives a
  `STRING` member 8, `member_size` gives a dynamic member array 8, and `layout` aligns owner members to 8
  (`crates\codegen-cpp\src\decl.rs`). `sema`'s `layout_bytes` (`check\places.rs`) does the same for the size limit
  of a `TYPE`.
- **The C++ already has the target's width.** The generated code declares `_OFFSET` places as `ptrszint`
  (`names.rs`), which libqb makes 4 bytes on `wasm32` (`QB64_32`, `libqb-common.h`). So variables are the right
  size today. The faults are where the front end writes a number: offsets inside a `TYPE`, strides of arrays of
  user types, `LEN`, the type codes handed to libqb, and constants.
- **`int_bits` of an `_OFFSET` means two things.** In the operator rules (`check\ops.rs`) and conversions
  (`conversion`, `lib.rs`), it is the width the value is computed in: the C++ computes in `int64`. In
  `input_type_code` (`codegen-cpp\src\io.rs`), `mem.rs`'s type codes, `qb_bits` and `radix_width`
  (`sema\src\builtins.rs`) and the literal ranges (`literal.rs`, `consteval.rs`), it is the width the value is
  stored in. On the desktop both are 64, so nothing told them apart. There are 49 calls of these helpers in 12
  files.
- **`codegen-cpp` reads `ir::Program`, not `sema`'s.** `ir::lower` copies `sema::Program` into it
  (`crates\ir\src\lower.rs`), and `ir::validate` calls `size_of`.
- **libqb's handshakes are plain `int32` flags** that one thread sets and another spins on (`study\03` §1.5).
  This works on x86 because it orders memory strongly. WebAssembly's threads and ARM do not guarantee that order.
- **QB64pe has a 32-bit build.** Its `_OFFSET` is 32 bits there (`source\utilities\type.bi` line 88), its `_MEM`
  is `ptrsz * 5 + 4 * 2 + 8` bytes (`qb64pe.bas` line 1594), and `BuildUDTDynLayoutM` (`utilities\type.bas`, about
  line 820) aligns to `ptrbits`. We have no 32-bit `qb64pe.exe` to record from, so none of it is measured.

## Goals / Non-Goals

**Goals:**

- One number, the pointer's size, set once from the platform and read by every pass that writes a size.
- A wrong size fails the C++ build, not the program.
- No change in the C++ written for Windows and Linux, apart from the size checks of D4.

**Non-Goals:**

- 64-bit WebAssembly (`-sMEMORY64`). Safari does not run it (`study\51` §5.3).
- The operator rules of QB64pe's 32-bit build for `_OFFSET`. They are not measured, and D2 keeps the 64-bit
  ones.
- A 32-bit Windows or Linux target. The width is a property of the platform, so one would be one more row in
  `Platform::pointer_bytes`. But building and testing it is not part of this change.
- The display's triple buffer (`display_frame[3].state`) and other graphics state between threads. That is W1.
- A one-thread build (`study\51` §5.1, to decide after W1).

## Decisions

### D1. The width lives on the program, set from the platform

`Platform::pointer_bytes()` in `crates\base\src\platform.rs` gives 8 for Windows and Linux and 4 for Wasm.
`sema`'s checker takes the platform from `parsed.flags.platform()` and stores it in a new field,
`sema::Program::platform`; `Program::pointer_bytes()` reads the size from it. `ir::lower` copies the field to
`ir::Program`, which has the same method. The field is the platform and not the number because `Program` derives
`Default`: a default platform is Windows, with 8 bytes, where a default number would be 0. Every function that writes a size reads
it from there. `size_of(types, t)` becomes `size_of(types, pointer_bytes, t)`. It has 8 callers, in `sema`,
`ir::validate` and `codegen-cpp::decl`.

`sema` still holds no rule about a platform: `check_target` (`sema\src\target.rs`) keeps what each target lacks.
The width is a number, like the other settings `sema` already reads from `parsed.flags`.

*Alternatives.* A width in a thread-local or a global, set before the check: it is smaller, but the language
server checks files on several threads, and a pass that forgets to set it would get the desktop's width without
any error. A width inside `Ty` (`Ty::Off(4)`): every `match` on `Ty` would change, about 150 places, for a value
that is the same for the whole program.

### D2. The width a value is computed in, and the width it is stored in

`Ty::int_bits` keeps its meaning: the width a value of the type is computed in. For `_OFFSET` it stays 64 on
every target. The C++ computes in `int64` and stores into a `ptrszint`, which cuts the result to 32 bits on
`wasm32`. A new `Ty::stored_bits(self, pointer_bytes)` gives the bits a place of the type holds: for
`_OFFSET` it is `8 * pointer_bytes`, and for every other type it is `int_bits`. Each of the 49 calls is read and
moved to `stored_bits` where it describes a stored value:

- `input_type_code` (`codegen-cpp\src\io.rs`): the bits libqb writes for console `INPUT`. With 64, libqb
  would write 8 bytes into a 4-byte variable.
- the type codes of `mem.rs` (`int(8, …) + 8192` becomes `int(pointer_bytes, …)`);
- `qb_bits` (`sema\src\builtins.rs`) and through it `radix_width` (`HEX$`, `OCT$`, `BIN$`), `int_entry` (`CINT`,
  `CLNG`), the float result of `SQR` and the like, and `PRINT USING`'s unsigned 64-bit case;
- `same_storage` (`check\proc.rs`): an integer variable is passed by reference to a parameter of another integer
  type when both are one width. With 64, an `_OFFSET` variable of 4 bytes would be handed to an `_INTEGER64`
  parameter, and the procedure's store would write 8 bytes;
- `varptr_type` (`codegen-cpp\src\cmem.rs`): the type byte of `VARPTR$`, which the old compiler takes from the
  type's bits (`qb64pe.bas` 21375–21385): 20 for 32 bits, 5 for 64;
- `LEN` of a whole member array of `STRING` (`check\builtins.rs`) and the stride of `clear_member_array`
  (`codegen-cpp\src\stmt.rs`), which both had a literal 8.

Found while applying, and so not changed:

- There is no `_OFFSET` literal: `5%&` is a compile error (`literal.rs`), and a `CONST` of the type is used as
  an `_INTEGER64` literal (`constant_literal`). So nothing is cut in `literal.rs` or `consteval.rs`.
- `_MK$` and `_CV` of `_OFFSET` call libqb's `o2string` and `string2o`, which take `intptr_t`. They are 4 bytes
  on `wasm32` already.
- `READ` and `INPUT #` into an `_OFFSET` read an `int64` value and store it into the `ptrszint`, which cuts it.
- `trace.rs` reads an `_OFFSET` through its C type.
- `debug.rs` has `POINTER_SIZE = 8` for the debugger's layouts. `$DEBUG` is rejected on the wasm target
  (`target.rs`), so it stays.

The calls in `check\ops.rs`, `conversion` and `is_wider_than` stay on `int_bits`, and so do the tests for 16
bits or fewer (`check\expr.rs` 438, `check\mem.rs` 628, `exp_single`) and the `SELECT CASE` selector
(`check\blocks.rs` 640), where 32 and 64 take the same branch or the value is stored. The doc comment of
`int_bits` and of `stored_bits` says which width each means.

*Why not cut `int_bits` to 32 on wasm?* Then the operator rules would change on wasm. `_OFFSET + LONG` would be
computed in 32 bits, and a `_OFFSET` compared with an `_UNSIGNED LONG` would take other result types. Those
rules were measured on the 64-bit build. What the 32-bit build does is not measured. Computing in 64 bits and
cutting at the store gives the same value whenever the result fits in 32 bits, which a real address does.

### D3. `_MEM` is libqb's block, 40 bytes

`mem_type` takes the width. With 4-byte pointers, its packed members land at the offsets of libqb's
`mem_block` on `wasm32` (`libqb\internal\c\libqb\include\memblock.h`). The 8-byte `lock_id` is at offset 8, which
is already aligned. Only the end differs: C rounds the struct up to 40, a multiple of the `int64`'s alignment,
and the packed members end at 36. So `_MEM`'s size on wasm is 40. A hidden tail member of 4 bytes,
`$_PAD`, makes it so. Like `$_LOCK_ID`, its name starts with `$`, so a program cannot name it.

QB64pe's 32-bit build gives 36 (`qb64pe.bas` line 1594). With that size, an array of `_MEM` would have elements
36 bytes apart, and the generated code copies a `mem_block` of 40 into each. The last element would overrun the
array. That is a fault in QB64pe that we do not copy. It gets a row in `DIVERGENCES-QB64PE.md`, marked
unmeasured, since we cannot run the 32-bit build.

### D4. The C++ checks the sizes it was given

`codegen-cpp` writes two lines into `global.txt`, the first fragment that every program includes:

```cpp
static_assert(sizeof(ptrszint) == 4, "qb64rust: _OFFSET is 4 bytes on this target");
static_assert(sizeof(mem_block) == 40, "qb64rust: _MEM is 40 bytes on this target");
```

The numbers are the program's: 8 and 56 on the desktop. They are written for every target. If a pass and libqb
ever disagree, `clang` or `em++` stops with that message. The desktop's C++ gains these two lines and nothing
else. The snapshot tests are updated once.

*Alternative:* checks in libqb. They would know libqb's sizes, but not the sizes the front end used.

### D5. Owner layouts align to the pointer

In a type that owns dynamic member arrays, `Layout::layout` (`codegen-cpp\src\decl.rs`) aligns each dynamic
member and each owner member to `pointer_bytes`, and rounds the size up to it. This is what QB64pe's
`BuildUDTDynLayoutM` does with `ptrbits`. The slot of a dynamic member array is `pointer_bytes`, and so is a
`STRING` member in `ty_size`, `member_size` and `sema`'s `layout_bytes`. On the desktop all of these stay 8.

### D6. `target.rs` loses its pointer rules

`check_target` no longer reports `_OFFSET`, `_UNSIGNED _OFFSET`, `_MEM`, the built-ins that take or give one,
`_MEMGET`, `_MEMPUT`, `_MEMFILL`, `_MEM(…)`, `_MEMELEMENT`, `_OFFSET(…)` or a member array of `STRING`.
`pointer_word`, `pointer_member`, `result`, the `reported` set and the pointer arms of `builtin` and `expr` are
removed. `not_yet` stays, because `part_not_yet` uses it. The module's doc comment loses item 3.

A built-in can still be rejected for its part of libqb. The audio words that take an `_OFFSET`, for example
`_MEMSOUND`, keep their "yet" error for `DEPENDENCY_MINIAUDIO`.

### D7. Atomic flags in libqb, as edit files

The edit files `tools\libqb\edits\atomic-thread-flags-*.edit` declare these flags as `std::atomic` of their
current type, with the same names. They are flags of `study\03` §1.5 that are shared by `QBMAIN`, `MAIN_LOOP`
and the timer thread:

- in `libqb.cpp`: `lock_mainloop`, `autodisplay`, `exit_ok`, `lock_display` and `lock_display_required`;
- in `qbx.cpp`: `ontimerthread_lock`;
- the polled flags `suspend_program`, `stop_program` and `close_program`, with their `extern` declarations in
  `libqb.cpp`, `keyboard.cpp` and `error_handle.cpp` (the last two also get `#include <atomic>`).

An edit file holds one block of one file, so there are 16 files: one per place a flag is declared, and two for
the includes.

The default ordering (sequentially consistent) is kept. The generated code's `if (stop_program)` works the same
on a `std::atomic`, so no fragment changes.

The edits are not inside a test for Emscripten: the same race is on ARM Linux, and one form of libqb is simpler
to test. So the desktop suites test them.

**Two flags of the first plan stay plain** (found while applying, 2026-11-20):

- **`qbevent` stays a plain `uint32`.** The generated code tests it between all statements. As a `std::atomic`
  it made a loop of 400 million statements take about 4.4 s, where it takes 1.38 s plain (task 5.2: the
  Windows host, four interleaved runs each). The program's C++ is built without optimization, so each test
  became a call of `std::atomic`'s conversion. With `qbevent` plain and the other flags atomic the loop takes
  1.38 s. `qbevent` is a one-way signal: another thread sets it, and `QBMAIN` reads it again at the next
  statement, so a late read only delays `evnt` by one statement. The handshakes, where a thread waits for
  another's answer, are the flags that need the ordering, and they are atomic.
- **The counters `display_lock_request`, `display_lock_confirmed` and `display_lock_released` stay plain.** They
  are assigned to each other in five places, which a `std::atomic` does not allow without an edit at each. They
  serve only `SUB _GL`, which the web target will not have (`study\51` §5.2).

*Alternative:* `volatile`. It stops the compiler from caching a flag, but it does not order memory between
threads. On ARM and in WebAssembly that ordering is the whole problem.

### D2a. `STR$` and `PRINT` of an `_OFFSET` on wasm32

Found by the first wasm build of a program with an `_OFFSET` (2026-11-20). On `wasm32`, `intptr_t` is `long`, a
type of its own beside `int32_t` (`int`) and `int64_t` (`long long`). So `qbs_str` of a `ptrszint` matched no
overload best, and the C++ did not compile. The edit file `tools\libqb\edits\wasm-qbs-str-pointer.edit` adds two
overloads to `qbs.h`, inside a test for Emscripten, that give the value as its 32 bits.

### D8. How the 270 programs are judged

The `offset` lines are removed from `tests\verification\known.wasm.list`, and those 270 programs run in the VM.
They are about 270 programs and so a large run (`CLAUDE.md` working rule 6). The run is given only those
programs, by list, if `check wasm-verification` can take one. Otherwise it is the whole step, with its time
said first. Each program
that fails then gets a line with one of the remaining reasons. A program whose output shows an 8-byte pointer,
such as `LEN` of an `_OFFSET`, `LEN` of a `_MEM` or the `SIZE` of a block of a `TYPE` with a `STRING`, gets
`width`. A program that crashes or gives a wrong value is a fault and is fixed, not listed.

New programs, family `v73` (`verification\README.md`), test the 4-byte layout. Each prints its sizes compared
with a constant from `$IF 32BIT`: for example, `PRINT LEN(p) = W`, where `W` is 4 under `$IF 32BIT` and 8
otherwise. QB64pe's recording on 64 bits then holds for the wasm run, so no file for the wasm platform is
needed (spec `testing/golden-corpus`, "What a wasm run is judged by"). The programs cover:

- a `TYPE` of a `LONG`, an `_OFFSET` and a `STRING * 2`;
- an array of `_MEM` stored and read back;
- `_MEMGET` and `_MEMPUT` through a block of a `TYPE`;
- a member array of `STRING`, fixed and `_DYNAMIC`;
- an owner type with two dynamic member arrays and an `_OFFSET` between them;
- `HEX$` of an `_UNSIGNED _OFFSET` of -1;
- `INPUT #` into an `_OFFSET`, and `PUT` and `GET` of one;
- `_MK$` and `_CV` of `_OFFSET`.

The `-z` comparison of fragments in `crates\driver\tests\it\wasm_target.rs` allows the two size checks of D4.
Its program with an `_OFFSET` shows that the fragments differ only in sizes.

## Risks / Trade-offs

- **A call left on `int_bits` that should be on `stored_bits`** → it gives a 64-bit size on wasm, and D4 does not
  catch it, because the variable itself is the right size. Mitigation: every one of the 49 calls is read and
  classified in one task, and the result is listed in the task's note. The `v73` programs cover each stored
  use: `INPUT`, `_MEM` type codes, radix, literals, `_MK$`. A program that gives a wrong value on wasm is
  rebuilt once with `-fsanitize=address`, by hand, to find the bad write.
- **The `std::atomic` flags change the desktop runtime** → a deadlock or a slowdown would show in every program.
  Mitigation: the desktop suites run in the VM (`check full`). The CLI tests that time a window close, such as
  `trace_record_when_the_window_is_closed`, run as well. The tight-loop timing of task 5.2 found the slowdown of
  an atomic `qbevent`, which is why it stays plain (D7).
- **The size checks of D4 change every snapshot** → one large but mechanical diff for the user to read. It is
  two lines per snapshot, and the task that adds them changes nothing else in the snapshots.
- **QB64pe's 32-bit behavior is not measured** → the wasm sizes rest on libqb's C struct and on reading
  `qb64pe.bas`. Mitigation: D4 checks the C struct. D3's row says that the value is unmeasured.

## Migration Plan

None. The desktop keeps its sizes and its C++, apart from the two checks of D4. A program that W0 refused for
the wasm target now builds.

## Open Questions

- Whether some of the 196 `_MEM` programs also need a part of libqb that is not built yet (audio, images). The
  run of D8 shows it, and each one moves to the reason `target`. It changes neither the design nor the tasks.
