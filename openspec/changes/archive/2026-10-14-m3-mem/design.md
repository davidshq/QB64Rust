# Design

## Context

Motivation and scope: `proposal.md`. What the old compiler writes, read in `qb64pe.bas` and in the output of
`qb64pe -z` for a program of each kind of form (2026-10-14, outside the repository):

- **The type.** `_MEM` is user type 1, defined before the program's first line (`qb64pe.bas` 1588): 56 bytes,
  `OFFSET` 0, `SIZE` 8, `$_LOCK_ID` 16 (`_INTEGER64`), `$_LOCK_OFFSET` 24, `TYPE` 32, `ELEMENTSIZE` 40 (all
  `_OFFSET` but the lock id), `IMAGE` 48 and `SOUND` 52 (LONG). The two `$` names cannot be written in BASIC. A
  variable is a `void*` of 56 bytes like any user type (`__UDT_M`); a member is read as any member is
  (`*(ptrszint*)(((char*)__UDT_M)+((0+8)))`); a store to one is "Cannot set read-only element of _MEM TYPE"
  (`qb64pe.bas` 26849). `LEN(m)` is 56.
- **A function that gives a `_MEM`** is written into a global temporary of type `mem_block`, one per use
  (`mem_block func1;` in `global.txt`), and the value is copied with `memcpy(dest, &(func1=func__mem(…)), 56)`.
  This compiler has no function with a user-type result yet; this is the first.
- **`_MEM(reference)`** is `func__mem(address, size, type value, element size, lock)` (the "-7 helper",
  `qb64pe.bas` 22730–23038). Measured: a LONG `a` → `func__mem((ptrszint)__LONG_A,4,132,4,sf_mem_lock)`; a whole
  array → the address of element 0, the bytes of all elements, `65536 +` the element's type value, the array's
  lock (`(mem_lock*)((ptrszint*)__ARRAY_INTEGER_B)[8]`); an element → its address and the bytes from it to the
  end; a `STRING * 6` → `->chr`, 6, 512, 6, `sf_mem_lock`. `_MEMELEMENT` (the "-8 helper", 22406) gives the one
  element. Type values (the wiki's table, measured for LONG 132, an INTEGER array 65666, `STRING * n` 512, an
  image of 32 bits 3204, new memory 16384): bytes of the element, +128 an integer, +256 a float, +512 a string,
  +1024 unsigned, +2048 a pixel, +4096 a `_MEM`, +8192 an `_OFFSET`, +16384 no type, +32768 a user type, +65536
  an array.
- **`_MEM(offset, size)`** is `func__mem_at_offset`; an argument that is not an `_OFFSET` is an error (20931).
- **Locks.** Every SUB and FUNCTION opens `sf_mem_lock` and frees it on exit, the main module has its own, and an
  array descriptor carries a lock in the slot after its dimensions, renewed by `REDIM` and `ERASE`. This
  compiler writes all of that already (`codegen-cpp\procs.rs` 22–25 and 187, `decl.rs` 301, `arrays.rs` 79), to
  match the old compiler's output.
- **`_MEMGET` (function)** is `*(int32*)(func__memget(blk, offset, size))` (20573–20630); for a string type
  `qbs_new_txt_len`, for a user type a `void*`.
- **`_MEMPUT`, `_MEMGET` (statement) and `_MEMFILL`** are written inline (10168–10500): a check of the lock and
  the region (`error(308)`, `error(300)`, `error(309)`), then a `memcpy` from a temporary of the value's type, or
  `sub__memfill_2(blk, offset, bytes, value)` for a typed fill, `sub__memfill(blk, offset, bytes, address,
  size)` for a variable. `_MEMCOPY` is `sub__memcopy(src, soffset, bytes, dst, doffset)`; `_MEMNEW`,
  `_MEMFREE`, `_MEMEXISTS`, `_MEMIMAGE` and `_MEMSOUND` are calls of their table rows, with the `_MEM` argument
  passed as `(void*)` of the variable.
- **`MKL$(x)`** is `l2string(x)`, **`CVL(s)`** `string2l(s)`, one pair for each numeric type (`i`, `l`, `s`,
  `d`, `b`, `ub`, `ui`, `ul`, `i64`, `ui64`, `f`, `o`, `uo`, `bit`, `ubit` with a width); `_MK$(type, x)` and
  `_CV(type, s)` take the type as their first argument (21217–21296). The parser reads a type as an argument
  already (`TypeArg`, `VAL(s, type)`).
- **Runtime errors 300 to 315 are not trapped**: `fix_error` (`libqb\src\error_handle.cpp` 368) ends the program
  with the message even under `ON ERROR` (measured: a `_MEMPUT` into freed memory stops with "Memory has been
  freed").
- In this compiler, `_MEM` is "not supported yet" as a type (`check\decl.rs`), `places.rs` names it among the
  type words a `TYPE` may not take, and five front-end tests use it as their example of an unsupported
  declaration (`mem_follow_on`, `unsupported`, `explicit_after_unsupported`, `deftype_errors`,
  `template_forms`).

Constraints: nothing copied from other projects; libqb and the reference clone are not changed; no divergence
without a decision; a behaviour that looks questionable is implemented QB64pe's way and listed in `SOMEDAY.md`.

## Goals / Non-Goals

**Goals:**

- Every form of the proposal compiles to the old compiler's C++ (compared by the call-site check where it is a
  call) and is rejected where the old compiler rejects it.
- Each is run by a console slice program recorded with `qb64pe.exe`, the runtime errors among them.
- The upstream programs that need nothing else pass and join `pass.list`.

**Non-Goals:**

- `$CHECKING:OFF` and the forms written without checks (`sub__memfill_nochecks`, the inline checks left out).
- `_MEM` over a `_DYNAMIC` or `_STATIC` member (deferred with those members, `deferred.list`).
- `_INFLATE$`, `_DEFLATE$`, `_BASE64ENCODE$`, `_BASE64DECODE$`; `_SNDOPEN` and the sound handles.
- Running `_MEMSOUND`.

## Decisions

### D1. `_MEM` is a predefined user type

`sema` declares `_MEM` as user type 0 before pass 1, with its eight members in the old compiler's order and
sizes; the two lock members get names a program cannot write (`$_LOCK_ID`, `$_LOCK_OFFSET`, as the old compiler
names them), so a lookup of a written member never finds them. Everything a user type does then works for it:
`DIM`, arrays, members of other types, parameters, whole-value copies, `LEN`. A store whose place ends in a
member of `_MEM` is the old compiler's error. The emitter lays it out as any user type; the layout gives 56 bytes
with no special case, since the members are 8, 8, 8, 8, 8, 8, 4, 4.

*Alternative:* a type kind of its own (`Ty::Mem`). Rejected: it would repeat what user types do (arrays,
parameters, copies) for one type, and the old compiler treats it as a user type everywhere but the store check.

### D2. A function with a `_MEM` result

`ExprKind::Builtin` with a user-type result is new. The emitter declares one `mem_block` temporary per use in the
global part (`mem_block func<n>;`, numbered as other temporaries are) and writes a store of such a value as the
old compiler does, a `memcpy` of 56 bytes from the temporary. Measured by task 1.1 (`v32_x02`, `x03`, `x93`–`x96`,
`x115`): **such a value is accepted only as the value stored into a place of type `_MEM`**; as an argument of a
built-in, a SUB or a FUNCTION it is "TYPE '_MEM' required for function", with a member read "Invalid
expression", and stored into a number (`k = _MEMNEW(4)`) the old compiler's C++ fails to build, so it is a
compile error here (the rule of `study\00` §6 for a form QB64pe fails to build). Only `_MEM`, `_MEMELEMENT`,
`_MEMNEW`, `_MEMIMAGE` and `_MEMSOUND` have such a result; a user FUNCTION of a user type stays as it is today.

### D3. The block of a reference

A new expression node, `MemOf { place, whole_array, element_only }`, built by `sema` from the argument of `_MEM`
or `_MEMELEMENT`, which is a place (not evaluated as a value) as the argument of `VARPTR` is. The emitter writes
the five arguments of `func__mem` from it: the address of the place, the size (of the value; of the array, or
from the element to the end; of the element alone for `_MEMELEMENT`), the type value (D1's table, computed from
the place's type and kind), the element size, and the lock (`sf_mem_lock` of the procedure the variable lives in,
or the main module's; the array's descriptor slot for an array or an element). A kind the old compiler accepts
and this compiler cannot place yet stays "not supported yet" and is put to the user (task 1.3).

Measured by task 1.1 (`qb64pe -z`, `verification\v32_x01_accepted`, values in `v32_a_blocks`), the five
arguments of `func__mem` for each kind of place. `L` is `sf_mem_lock`, the lock of the body where `_MEM` stands
(also for a `SHARED`, `STATIC` or main-module variable named in a SUB, a parameter, a local); `K(a)` is the
array's own lock, `(mem_lock*)((ptrszint*)__ARRAY_LONG_A)[4*d+4]` for `d` dimensions (8 for one, 12 for two);
`N(a)` is the bytes of all elements, `(4*(__ARRAY_LONG_A[2]&1)*__ARRAY_LONG_A[5]*…)` with one `[4k+5]` per
dimension; `i` an element's flat index with its `array_check`s; `U` the size of the variable's user type; `o`
the offset of a member in it.

- A number variable: `(ptrszint)__LONG_A,4,132,4,L` (its bytes, the type value of its type).
- A `STRING * n` variable: `(ptrszint)__STRING6_F->chr,6,512,6,L`.
- A `_MEM` variable: `(ptrszint)(((char*)__UDT_M2)+((0+0))),56-((0+0)),4096,56,L`.
- A user-type variable: `(ptrszint)(((char*)__UDT_U)+((0+0))),U-((0+0)),32768,U,L`.
- A member of a user-type variable (also a member's member): `(ptrszint)(((char*)__UDT_U)+((0+o))),U-((0+o)),32768,1,L`.
- An element of a member array of a variable (`u.v(2)`): as a member, the offset `((((0+o)+((array_check(( 2 )-(0),4))*4))+0))`.
- A whole member array of a variable (`u.v()`): `(ptrszint)(((char*)__UDT_U)+((0+o))),16,65668,4,L` (its own bytes, an array of its elements).
- A whole array of numbers: `(ptrszint)(&(((int32*)(__ARRAY_LONG_A[0]))[0])),N(a)-(4*(0)),65668,4,K(a)`; of a
  `_MEM` array type 69632.
- An element of an array of numbers: `(ptrszint)(&(((int32*)(__ARRAY_LONG_A[0]))[i])),N(a)-(4*(i)),65668,4,K(a)`.
- A whole array of `STRING * n`: `(ptrszint)((qbs_new_fixed(&((uint8*)(__ARRAY_STRING3_FS[0]))[(0)*3],3,1))->chr),N(fs)-(3*(0)),66048,3,K(fs)`; an element the same with its index.
- A whole user-type array: `(ptrszint)(((char*)__ARRAY_UDT_UT[0])+(0)),N(ut)-(0),98304,U,L` (the variable's
  lock, not the array's).
- An element of a user-type array: `(ptrszint)(((char*)__ARRAY_UDT_UT[0])+(((i)*U+0))),N(ut)-(((i)*U+0)),98304,U,L`.
- A member of an element (`ut(1).d`): `…+(((i)*U+o)),N(ut)-(((i)*U+o)),98304,1,L`.
- A whole member array of an element (`ut(1).v()`): `…+(((i)*U+o)),16,65668,4,L`.
- An element of a member array of an element (`ut(1).v(2)`): its address, the whole member array's bytes, `98304`,
  the elements' size, `L`.
- A whole array parameter: as a whole array, with the parameter's descriptor; **the old compiler's lock slot
  is `4*d+4` with `d` the number of dimensions known where `_MEM` stands, -1 (slot 0) when nothing above has
  given it** (`v32_a`, SUB `y`). Here always the array's own slot (user, task 1.3: `DIVERGENCES.md` D-016).
- `_MEMELEMENT`: a variable or a whole array as `_MEM`; an element `…,4,65668,4,K(a)` (its own bytes); a whole
  user-type value `…,U,32768,U,L`; an element of a user-type array `…,U,32768,U,L`; a member its own type
  (`u.a`: `(ptrszint)(((char*)__UDT_U)+((0+0))),2,130,2,L`); an element of a member array the whole member array
  (`u.v(1)`: its address, `16,132,16,L`); an element of a `STRING * n` array
  `(ptrszint)qbs_new_fixed(…)->chr,3,66048,3,K(fs)`; a member of an element of a user-type array its own type
  (`ut(1).d`: `…,8,264,8,L`), an element of a member array of such an element the same as `_MEM`
  (`ut(1).v(2)`: `…,16,98304,4,L`; the values measured by the review of the change, `v32_x119`, the lock read in
  `qb64pe.bas` 22498 and 22576).

Not supported yet, as before: a member array marked `_DYNAMIC` or `_STATIC` (deferred). Every other kind the
compiler supports is placed.

### D4. The statements written inline

`_MEMGET`, `_MEMPUT` and `_MEMFILL` become `StmtKind`s of their own, as `FIELD` and `LSET` did, since what the
old compiler writes is a checked copy, not a call: the block (a place of type `_MEM`), the offset (an `_OFFSET`
expression), and the value with its type (`AS type` or the variable's). The emitter writes the old compiler's
check and copy text, taken line by line from task 1.1's output. `_MEMCOPY` and `_MEMFREE` are rows with a
template (`?,?,?{To}?,?`), whose `_MEM` slots are passed as the address of the variable (a new slot kind,
`MemRef`), as are the `_MEM` arguments of `_MEMEXISTS` and `_MEMGET`.

### D5. `MKI$` … `_CV`

`MKI$`, `MKL$`, `MKS$`, `MKD$`, `CVI`, `CVL`, `CVS`, `CVD` are rows with a fixed slot type each (the argument
converted to that type as an assignment would) and the call `<t>2string` or `string2<t>`. `_MK$` and `_CV`
take a `TypeArg` first and pick the pair from it, as `VAL(s, type)` reads its type today; a `STRING` type is the
old compiler's error. Measured by task 1.1 (`v32_x97`–`x99`): `_BIT` and `_UNSIGNED _BIT` are width 1
(`bit2string(1,x)`, `string2bit(s,1)`); **a width cannot be written** (`_BIT * 4` is "Cannot convert type (_BIT
* 4%) to symbol" in the old compiler), so `_BIT * n` is an error here too. `MKS$` and `MKD$` pass their number
as it is (`s2string(*__LONG_K)`), `MKI$` and `MKL$` as a store into their type.

### D6. `_OFFSET(reference)`

`ExprKind::AddressOf(place)`, written `((uptrszint)(address))` as the old compiler writes it; the address is the
one `VARPTR` and `_MEM` take, a whole array the address of its first element. A value that is not a place, and a
`_BIT` array, are the old compiler's errors.

### D7. What no test runs

`_MEMSOUND` goes on `NEVER_RUN` with a call-site program for each form, and is held in a slice program as code
that is not reached; it names the audio part, which the build links already.

### D8. Tests

- `verification\v32_*`: the forms measured by task 1.1, accepted and rejected (`v32_x*` one form each), and
  programs of values (`v32_a` the members of each kind of block, `v32_b` get, put, fill and copy by each type,
  `v32_c` `MKI$` … `_CV`, `v32_d` each runtime error and its message, one program each since none can be
  trapped). Measured by task 1.2: **every error from 300 to 313 is reachable** (`v32_d01`–`d32`): 300 a region
  out of range, 301 a negative size (`_MEM(o, n)`, `_MEMFILL`, `_MEMCOPY`), 302–304 `_MEMCOPY` beyond its source,
  destination or both, 305, 306, 313 `_MEMCOPY` from or into freed memory, 307 `_MEMFREE` twice, 308 a freed block
  (freed, its SUB returned, its array redimensioned, its image freed), 309 a block never set, 310–312 `_MEMCOPY`
  with one or both never set. 314 and 315 are `_ASSERT`'s.
- Console slice programs on `slice.list`, recorded with `qb64pe.exe`: the forms of D3 to D6, an image read
  through `_MEMIMAGE` (a `$CONSOLE:ONLY` program can make an image, as `mem/mem_checking_off` does), and one
  program per runtime error the runner can compare (the message and exit code are in the recorded output).
- Call-site programs for every call form and argument type, `typed` and `ir` front-end tests.
- The five front-end tests that use `_MEM` as an unsupported declaration take `CONST c$ = CHR$(65)` instead
  (still "not supported yet", and accepted by the old compiler), as the pipeline spec's scenarios now do.

### D9. Upstream

Task 6.1 runs the 14 upstream programs that name `_MEM` and are not deferred, the image tests that include
`imageassert.bm`, and the six `paint/tile_*` tests; those that compile cleanly and pass join `pass.list`. What
still holds the others is named in `tests\upstream\README.md` (`_INFLATE$`, `_BASE64DECODE$`, `$CHECKING`,
`SYSTEM` with an exit code).

### D10. Order of work

Group 1 measures. Group 2: the type (D1), the `_MEM` result (D2), `_MEMNEW`, `_MEMFREE`, `_MEMEXISTS`,
`_MEMIMAGE`, `_MEMCOPY`, `_MEMSOUND`. Group 3: the block of a reference (D3) and `_OFFSET` (D6). Group 4: the
inline statements and `_MEMGET` (D4) with the runtime errors. Group 5: `MKI$` … `_CV` (D5), independent of the
others. Group 6: upstream, lists, the closing runs. Each of groups 2 to 5 ends with fmt, clippy and tier 1 green
and its tests in place.

## Risks / Trade-offs

- **The type values and sizes are many small rules** (D3): a wrong one is a wrong `TYPE` or `SIZE`, seen only
  when printed. Mitigation: the call-site check compares the five arguments of `func__mem` for every kind of
  place, and `v32_a` prints them.
- **The inline statements are long C++ text** (D4): a mistake shows as a failed build or a wrong check.
  Mitigation: the call-site check compares the calls inside them; slice programs hit each error branch.
- **A runtime error ends the program**, so each error needs a program of its own; the runner compares the
  output and exit code of such a program as it does for other runtime errors.
- **Taking `_MEM` out of the five front-end tests** changes what they pin; the replacement keeps their point (an
  unsupported declaration), checked by running them.

## Open Questions

- Which kinds of place `_MEM` must cover beyond what upstream and the wiki use (D3): answered by task 1.1, put to
  the user in task 1.3 if one cannot be placed. Answered: every kind the compiler supports is placed; the one
  question put to the user was the lock slot of an array parameter (D-016, fixed).

## As built (2026-10-14)

- **D1.** `_MEM` is added to the program's types the first time a program names it or a built-in that gives a
  block (`Checker::mem_ty`), not before pass 1: a type in every program would have put a `TYPE _MEM` block into
  every `typed` dump. It is the last type then, not type 0; nothing reads the number. Everything a user type
  does worked for it unchanged.
- **D2.** A block-valued call stands only as the value of an `Assign` into a `_MEM` place (the checker's
  `mem_store`, set by `assign`); its C++ is a `mem_block funcN` temporary appended to `global.txt` at the end of
  `emit` and a `memcpy` of 56 bytes (`store_block`). The same path stores a user-type value `_MEMGET` reads,
  without a temporary.
- **D3.** `ExprKind::MemOf { of: Reference, element }`, `MemAt` and `AddressOf`, with `sema::Reference` (a place
  or a whole array) and `Rule::Own` for built-ins checked before the table (`sema\src\check\mem.rs`); the emitter's
  rules by kind are in `codegen-cpp\src\mem.rs`. Measured on the way: a member that is itself a `_MEM` or user
  type has that type's size as its element size, and `_MEMELEMENT` of any whole user-type value is typed 32768.
  Not supported yet, not measured: a `TYPE` that holds a `STRING`, a member array that is not the last step of
  the place or whose elements are of a user type, a name with `()` that names no array.
- **D4.** `StmtKind::MemGet`, `MemPut`, `MemFill` with `sema::MemValue` (a typed value, the bytes of a reference,
  a string), and `ExprKind::MemRead` for the function. The parser's type argument takes `STRING * n` and `_BIT *
  n` (`type_arg`); a user type's name as `_MEMGET`'s third argument stays an expression, read as a type by the
  checker. A value `AS type` is written without the conversions C++ makes on the store (`stored`), as the old
  compiler writes it. `_MEMCOPY` and `_MEMFREE` are plain rows with `Slot::Mem`, as planned.
- **D5.** `Rule::Mk` and `Rule::Cv`; `_MK$` and `_CV` keep their type as a 0 of it in their first slot. A
  `_BIT` width cannot be written in the old compiler (measured), so `_BIT` is one bit wide here too.
- **D6, D7.** As planned.
- **Beyond the design:** `LEN` of a whole member array (`LEN(p(0).a())`), a false error that the marked `DIM …
  AS _MEM` had hidden, is "not supported yet"; a store of a block-valued call into a number (`k = _MEMNEW(4)`),
  accepted by the old compiler whose C++ then fails to build, is a compile error (`study\00` §6).
