# Design

## Context

See `proposal.md` for why. What the code has now:

- **Conventional memory is for arrays only.** `Checker::var_address` (`sema\src\check\places.rs`) takes
  `VARPTR(a(i))` and `VARSEG(a(i))` of an array of numbers of the main module and sets `Var::conventional` on it
  (`sema\src\lib.rs`). Every other argument is marked. The IR carries the flag (`ir\src\lib.rs`, `Var::conventional`;
  `validate.rs` checks that an address is of such an array). The emitter allocates such an array with flag 4 from
  `cmem_static_pointer` or `cmem_dynamic_malloc` (`codegen-cpp\src\decl.rs`, `alloc_array`, `new_block`,
  `free_block`, `may_be_conventional`). Scalars, strings and `TYPE` variables are allocated with
  `mem_static_malloc` and `qbs_new` (`decl.rs`, `alloc`).
- **The old compiler** (`qb64pe.bas` 21306–21560) marks a variable named by `SADD`, `VARPTR`, `VARPTR$` or `VARSEG`
  anywhere in the program (`cmemlist`) and compiles the program again. On that pass the variable's allocation in
  the data fragment takes its bytes from the top of `dblock` (`cmem_sp-=bytes; V=(T*)(dblock+cmem_sp)`, 18215,
  18300, 18384, 18473, 18559, 18645; a `TYPE` 17790, a fixed-length string 18008, a string `qbs_new_cmem` 18109).
  `dblock` is `&cmem[1280]`, and libqb takes the first 8 bytes for `nothingvalue` (`libqb.cpp` 25968). The texts
  are:
  - `VARPTR` of a number, a `TYPE` value or a member of one: `((unsigned short)(((uint8*)V)-&cmem[1280]))`, plus
    the member's offset;
  - of a variable-length string: `V->cmem_descriptor_offset`; of a fixed-length one: `((unsigned
    short)(V->chr-&cmem[1280]))`;
  - of an element of an array of a `TYPE`: the element's offset in the array plus the member's
    (`(index*size+offset)`); of fixed-length strings `((index)*len)`; of variable-length strings the element's
    `cmem_descriptor_offset` (its elements are made by `qbs_new_cmem`, 15150);
  - `VARSEG`: 80 for every variable and for an array of variable-length strings, the array's segment for any
    other array; `_BIT` arrays are the error "VARPTR cannot reference _BIT type arrays";
  - `SADD(t$)`: `((unsigned short)(T->chr-&cmem[1280]))`; `VARPTR$(v)`: `func_varptr_helper(t, offset)` with
    `t` the type byte (21359–21385).
  The values are `_INTEGER64`, except `VARPTR$`, a string.
- **Graphics `GET` and `PUT`** pass their array as `byte_element` bytes (`Checker::block_arg`,
  `check\builtins.rs`; `StmtArg::Block { array, index }`). A name that is no array, an array that is not of numbers,
  and an index that calls a FUNCTION are marked. **File `GET` and `PUT`** (`bytes_arg`) mark `und()` of a name that
  is no array. So does **`_UCHARPOS`** (`long_array_arg`, which goes through `block_arg`).
- **Member arrays** have their bounds read in pass 1 (`Checker::member_bounds`, `Bounds::Pass1`), before any
  `CONST` is evaluated. A bound that may name a constant is marked (`may_be_constant`). The first bounds of a
  `_DYNAMIC` member array may name a `CONST` already: they are read in pass 2 (`Bounds::Pass2`, the
  `const_bounds` flag), since they do not change the type's layout.
- **`FOR`** takes a scalar variable (`Checker::for_var`, `check\blocks.rs`); a name with a `.` is marked.
- **`ERASE` and `REDIM` of a member array** of an element whose index may raise is one statement per target
  (`clear_member_array`, `places.rs`), so such a target beside others is marked.
- **`LEN` of a whole member array of strings** (`check\builtins.rs`, `len_whole`) and **`_MEM` of a value of a
  `TYPE` with a `STRING` member** (`check\mem.rs`, `block_reference`) are marked.
- **`HEX$`, `OCT$`, `_BIN$`** of `VARPTR`, `VARSEG` or `_SCREENEXISTS` are marked: the old compiler passes the width
  0 there, not the argument type's (`check\builtins.rs`, the `Rule::Radix` arms).
- **Rejected forms that are marked:** `SADD` and `VARPTR$` (`check\expr.rs`, no arm yet), `STRING * n` with a name
  that is no integer `CONST` (`check\decl.rs`), and a name `f$n` outside a FUNCTION (`Checker::split_name`, the
  "fixed-length strings" arm).

## Goals / Non-Goals

**Goals:**

- Every form of the proposal compiles and runs as the old compiler's program does, or is the old compiler's error.
  `VARPTR`, `SADD` and `VARPTR$` give the old compiler's numbers, not just some numbers.
- The 30 programs of `tests\verification\known.list` named in the proposal pass, and the new family `v56` with
  them.
- Nothing else changes: the call-site check shows every existing program's call unchanged. So does the C++ of a
  program that names no variable by `VARPTR`, `VARSEG`, `SADD` or `VARPTR$`.

**Non-Goals:**

- `VARPTR`, `VARSEG`, `SADD` and `VARPTR$` of a variable, array or parameter of a SUB or FUNCTION. The old
  compiler allocates each on every call from `cmem_sp` (5787, 6013) and makes the callers' arguments
  conventional, a fact carried through calls (12081, 12810). This keeps its mark and `SOMEDAY.md` row 61.
- `VARPTR$` in a `PLAY` or `DRAW` string (row 79), and an index that calls a FUNCTION in the array of `GET` or
  `PUT` (row 66).
- `_MEMGET`, `_MEMPUT` and `_MEMFILL` with a value of a `TYPE` with a `STRING` member (row 109): only `_MEM` of it
  is in the demand report.
- Found by the measurements (task 1.1) and left marked: a `TYPE` that holds a `STRING` in conventional memory or
  in a graphics `GET`/`PUT` (the old compiler's program crashes or writes over string pointers, D1, D5), `FOR`
  over an element of a member array (its index is evaluated at every pass, D8), and the whole-array forms the
  family did not measure (D2).

## Decisions

### D1. A variable in conventional memory

`Var::conventional` (sema, IR) applies to any variable of the main module, shared or not. It no longer means only
an array. It is set by `var_address` and the new `SADD`/`VARPTR$` arms, wherever in the program they stand, as now
for arrays: codegen runs after the whole program is checked, so this is the old compiler's second pass. A
variable of a procedure (`STATIC` too) stays marked (Non-Goals).

**A `TYPE` that holds a `STRING`** (a member, or a member of a member) stays marked as a variable or array in
conventional memory: the old compiler's allocation there leaves out the value's strings
(`initialise_udt_varstrings` stands only in the other branch, `qb64pe.bas` 17790–17852), and its program crashes at
the first use of one (measured, `v56_a5`: exit 139). `VARPTR`, `VARSEG` and `VARPTR$` of such a variable, member
or element are "not supported yet", with a `SOMEDAY.md` row.

The emitter's `alloc` (`codegen-cpp\src\decl.rs`) takes a branch for a conventional variable, with the old
compiler's text (read from its C++, `-z` of `v56_a1`, `a3`, `a4`):
- a number: `if(V==NULL){ cmem_sp-=size; V=(T*)(dblock+cmem_sp); if (cmem_sp<qbs_cmem_sp) error(257); *V=0; }`
  (18300–18733; a `_FLOAT` takes 32 bytes);
- a `_BIT` scalar: `cmem_sp-=4; if (cmem_sp<qbs_cmem_sp) error(257); V=(T*)(dblock+cmem_sp); *V=0;` (18215);
- a `TYPE` value: `cmem_sp-=size; if (…) error(257); V=(void*)(dblock+cmem_sp); memset(V,0,size);` (17790);
- a fixed-length string: `cmem_sp-=len; if (…) error(257); V=qbs_new_fixed((uint8*)(dblock+cmem_sp),len,0);
  memset(V->chr,0,len);` (18008; `qbs_new_fixed` then takes 4 bytes more for the descriptor, `qbs_cmem.cpp` 135);
- a variable-length string: `if (!V)V=qbs_new_cmem(0,0);` (18109; 4 bytes for the descriptor).

**Every `_BIT` scalar of the main module lives there in the old compiler, named or not** (18215 has no other
branch; measured, `v56_a3`, `v56_x06`: `VARPTR` of one is accepted). So that the offsets after it are the old
compiler's, the emitter allocates every main-module `_BIT` scalar this way **when the program has any variable in
`dblock`**; otherwise as now (the goal "nothing else changes": no program without `VARPTR` can see where a `_BIT`
lives).

An array of strings, of fixed-length strings or of a `TYPE` named by `VARPTR`, `VARSEG`, `SADD` or `VARPTR$`
becomes conventional as an array of numbers does. Fixed-length strings and a `TYPE` take `cmem_static_pointer` or
`cmem_dynamic_malloc` as numbers do (flag 4; `may_be_conventional` covers them). An array of strings keeps its
pointers in ordinary memory (`[2]=1+2`, a dynamic one `[2]=4`): each element is made by `qbs_new_cmem(0,0)`, last
first, which takes a descriptor from `dblock` (15150; a dynamic one when `REDIM` runs, 15313, 15376, measured
`v56_a3`).

**The offsets equal the old compiler's** because the variables take `dblock` in the order of the old compiler's
data fragment: the order of declaration in the text of the main module (measured, `v56_a1`–`a3`: `DIM` lists,
implicit variables at their first use, `COMMON SHARED` before the rest, `DIM SHARED`, a variable named only in a
SUB, one named only by `VARSEG`, a `CONST` between; no alignment). The emitter writes the allocations in the order
of `per_program_var`. Where `v56_a` shows our order to differ, the emitter follows the old one; where it cannot,
a `DIVERGENCES.md` row is added and the program's recorded output gets a sidecar.

**A string keeps its place** (task 1.3, measured `v56_a4`: `VARPTR` is the same after assignment, growth,
`LEFT$`, `UCASE$`, an assignment from a string outside, `SWAP`, `MID$`, a SUB's parameter, `READ`, `LSET`, `STR$`).
libqb keeps a `cmem` descriptor through `qbs_set` (`qbs.cpp` 305, 334; `qbs_cmem.cpp` `qbs_copy_cmem`,
`qbs_move_cmem`), `swap_string` (`qbx.cpp` 401: two `qbs_set`), `sub_mid`, `sub_read_string`, `sub_lset`. The places
where our emitter makes a string variable's descriptor, and what each writes for a conventional one:
- `decl.rs` `alloc`, a scalar: `qbs_new_cmem(0,0)` (above);
- `decl.rs` `alloc_array`, a static array of strings: `qbs_new_cmem(0,0)` per element;
- `arrays.rs`, a dynamic array of strings allocated (line 392) or kept by `REDIM _PRESERVE`/`_RETAIN` (line 300):
  `if (n[2]&4){… qbs_new_cmem(0,0);}else{… qbs_new(0,0);}`, as the old compiler writes it for every array of strings
  (15313, 15376), the flag tested when the program runs since the array may be a parameter. Written so only when
  the program has a conventional array of strings, so that no other program's C++ changes;
- `procs.rs` 157, a string parameter copied on entry: already `qbs_new_cmem` when the argument has a `cmem`
  descriptor (5746);
- `ERASE` and `CLEAR` of a static array of strings set each length to 0 (`decl.rs` `clear_elements`), and of a
  dynamic one free each string (`release`): the descriptor is not made again, as in the old compiler.
No other place replaces a string variable's `qbs*`.

### D2. Address expressions

`ExprKind::VarPtr { array, index }` and `VarSeg { array }` (sema, IR) become one
`ExprKind::Address { of: AddressOf, word: AddressWord }`. `AddressWord` is `VarPtr`, `VarSeg`, `Sadd` or
`VarPtrStr`. `AddressOf` is one of:
- `Element { array, index }`: an element of an array of numbers, of fixed-length strings or of a `TYPE`;
- `Var(VarId)`: a scalar of a number type (also `_BIT`), or a `TYPE` variable for `VARSEG` and `VARPTR$`;
- `Member { place, offset }`: a member of a `TYPE` variable or of an element of an array of a `TYPE`, through any
  chain of members and member-array elements;
- `Str(VarId)` and `StrElement { array, index }`: a variable-length string;
- `Fixed(VarId)`.

The IR validation checks that the variable or array is conventional. The emitter (`codegen-cpp\src\value.rs`,
where `VarPtr` is written now) writes the old compiler's text for each (21328–21560; its C++, `-z` of `v56_a1`,
`a4`, `a6`, `t01`, `t07`, `t08`):
- a number or `_BIT`: `((unsigned short)(((uint8*)V)-&cmem[1280]))`;
- a member of a variable: `((unsigned short)(((uint8*)R) - &cmem[1280] + (offset) ))`, the offset with the
  bounds checks of the member arrays on the way (`array_check`, which gives 0 after error 9);
- an element of a `TYPE` array, or a member of one: `((flat)*size+offset)`; of fixed-length strings `((flat)*len)`;
- a variable-length string, or an element of an array of them: `V->cmem_descriptor_offset`; a fixed-length string
  `((unsigned short)(V->chr-&cmem[1280]))`;
- `VARSEG`: 80 for a variable, a member of a variable and an element of an array of strings; for any other array
  the segment of its data, as for numbers now.

Measured edges (`v56_t02`, `x06`, `x07`, `a6`): `VARPTR` of a `TYPE` variable is the old compiler's internal
error, so it is a compile error here ("`VARPTR` of a whole `TYPE` variable"); `VARSEG` and `VARPTR$` of one are
accepted. `VARPTR` of a `_BIT` scalar is accepted; of a `_BIT` array it is "VARPTR cannot reference _BIT type
arrays". A name alone that is an array of strings, fixed-length strings or a `TYPE` is the implicit scalar of
that name (`v56_a6`: `VARPTR(s2)` is a new `SINGLE`), which the existing lookup of a name already gives. Not
measured, so still marked: `VARPTR` and `VARSEG` of a whole array `x()` that is not of numbers, and `VARSEG` of a
`_BIT` array.

### D3. `SADD` and `VARPTR$`

New arms in `check\expr.rs` beside `("VARPTR", None)`. `SADD` takes a variable-length string variable or
element, or `sa()` (element 0, `v56_x11`): `((unsigned short)(V->chr-&cmem[1280]))`. Anything else is an error
(the old compiler's wording where it has one): a fixed-length string, an element of an array of them and a
literal are "SADD only accepts variable-length string variables" (`v28_x55`, `x56`, `v56_x05`); a `TYPE` variable
"User defined types in expressions are invalid" (`v56_x09`); a number and a `STRING` member fail its C++ build
(`v56_x03`, `x04`), an error here with no note.

`VARPTR$` takes a variable of every type measured (`v56_b1`–`b5`, `t04`, `t05`): a number, a `_BIT`, a string, a
fixed-length string, a `TYPE` variable, a member, an element of an array of strings, and `sa()` (element 0,
`v56_x10`). An element of any other array is "VARPTR$ only accepts variable-length string arrays" (`v28_x57`,
`v56_x01`, `x02`). The result is `func_varptr_helper(t,offset)` (`AddressWord::VarPtrStr`, a string), with the old
compiler's two quirks: the offset of a fixed-length string is its descriptor's (`cmem_descriptor_offset`), and
that of a member is its variable's, without the member's offset. The type byte `t` (21359–21385): 3 for a
string of either kind; 0 for a `TYPE` variable; for a number 1, 2, 20, 5 for 8, 16, 32 and 64 bits (`_OFFSET` too),
4, 8, 6 for `SINGLE`, `DOUBLE`, `_FLOAT`, plus 128 when unsigned; for `_BIT * n` 64 + n, plus 128 when unsigned.

### D4. The width 0 in `HEX$`, `OCT$`, `_BIN$`

The radix functions take a width argument that the emitter writes from the argument's type. Where the argument
is `VARPTR`, `VARSEG`, `SADD` or `_SCREENEXISTS`, the old compiler's width is 0 (`func_hex(…,0)`, read from the C++
of `v56_c`): their type is 64 bits and they are no reference (`qb64pe.bas` 21052). The slot's argument gets a width
override in sema (`check\builtins.rs`, the `Rule::Radix` arms), carried in the call's IR. For a positive value the
width changes nothing (libqb's `func_hex`, `hexoctbin.cpp` 198); for `_SCREENEXISTS`, -1 in a program without a
window, `HEX$` gives the empty string, `OCT$` `177777` and `_BIN$` sixteen ones (measured, `v56_c`).

### D5. The array's place of graphics `GET` and `PUT`

`StmtArg::Block` gains the kinds the old compiler takes. Each kind's `byte_element` text, read from the old
compiler's C++ (`-z`) of `v28_x01_accepted`, `v56_d` and two programs of every form written for task 1.2:
- a plain variable of a number type: its bytes, `byte_element((uint64)V,size,t)`; a fixed-length string
  `(uint64)V->chr,len`;
- a variable-length string: its characters, `(uint64)V->chr,V->len`;
- a name never declared, and a name alone that is an array of strings or of fixed-length strings: the scalar of
  that name (implicit, its default type) and its bytes (`GET …, sa` is `__SINGLE_SA`);
- a `TYPE` variable: `(((char*)R)+((0+0))),size-((0+0))`; a member of one (also through member arrays):
  `(((char*)R)+((0+offset))),size-((0+offset))`, the bytes from the member to the variable's end;
- an element of an array of variable-length strings, or `sa()` (element 0): that string's characters,
  `(uint64)(E)->chr,(E)->len`;
- an array of fixed-length strings or of a `TYPE`, whole (`fa()`, `ra()`, and `ra` alone, element 0) or from an
  element, and a member of an element (`ra(2).y`): the bytes from there to the array's end, the existing `block`
  texts with the member's offset added for a member.
A value too small for the rectangle is libqb's error 5 (`v16_m5`, `v56_d`), not the compiler's. A `TYPE` that
holds a `STRING`, and arrays of one, stay marked: `GET` would write over its string pointers (`GET …, rs.n` passes
the pointer's bytes). The code lives in `block_arg` (sema) and the emitter of `GET`/`PUT` (`builtins.rs`,
`block`); `StmtArg::Block` gains a member offset, and a new `StmtArg::VarBytes(Place)` takes a plain variable,
string or member.

### D6. An undeclared array in file `GET`/`PUT` and `_UCHARPOS`

`und()`, `unl&()` and `nd&()` of a name that is no array make the implicit array (`Checker::implicit_array`,
step 2's rules for where it is allocated). Then the form goes on as for a declared array of that type (measured,
`v56_e`: `GET #1, 1, und()` reads 44 bytes into `und(0 TO 10)`, `PUT` writes them; `_UCHARPOS` fills `nd&()`).
For file `GET` of `und$()`, an array of variable-length strings, the old compiler takes element 0 as a string
variable (`sub_get2(1,1,(((qbs**)(und[0]))[0]),1)`, read from its C++; it reads nothing, since the string is
empty), where a declared one is rejected (`v30_x38` against `x178`). `PUT` of `und$()` is the old compiler's error
"Cannot pass array of variable-length strings" (`v56_e2`), as for a declared one.

### D7. Member array bounds that name constants

A static member array's bound that names a constant is read in pass 2, as the first bounds of a `_DYNAMIC` member
are now. Unlike those, it changes the type's layout. So a type with such a bound is laid out again at the start of
pass 2, before any body is checked: its members' offsets and its size, then every type that holds it, in file
order. `const_bounds` and `Bounds::Pass2` in `places.rs` take static members. Pass 1 gives such a member 0 TO 0
until then, and nothing in pass 1 reads a type's size. Task 4.1 checks this with a test that names `LEN` of the
type in a `CONST`, which the old compiler rejects anyway. The constants a bound may name are those of
`may_be_constant`: a `CONST` above the block, a constant of the auto-included file, a function of the constant
evaluator.

### D8. A `TYPE` member as a `FOR` variable

`for_var` returns a place, not a `VarId`. A numeric member of a `TYPE` variable, through members only (`r.v`,
`ro.n.v`), is a place with a fixed address, so the loop reads and stores it as a variable (measured, `v56_f1`,
`f4`: nesting, `EXIT FOR`, `STEP`, a body that changes it, the other members kept, every numeric type). The IR's
loop (`ir\src\lower.rs`, `loop_block`) and the emitter take such a place where they take a variable now.
Measured edges: a member of an array element (`ar(i).v`) and a fixed-length string member are the old compiler's
"Unsupported variable used in FOR statement" (`v56_f2`, `x13`); a whole member array (`r.m`) is "Expected array
index" (`x12`); an element of a member array (`r.m(i)`) is accepted but its index is evaluated at every pass
(`v56_f3`: 7 calls of the index's FUNCTION for 4 passes, and a change of `i` in the body moves the loop to
another element), so it stays marked.

### D9. A member array of an element in a list of `ERASE` or `REDIM`

The old compiler goes on with the other targets of the statement before the error of an index is handled
(`v24_x312` lines 9 and 10). The targets of one `ERASE` or `REDIM` become ops of one statement with the error
check after the last (`clear_member_array` loses `alone`). A target whose index raised clears nothing, as alone.

### D10. `LEN` of a whole member array of strings, `_MEM` of a `TYPE` with a `STRING`

In the old compiler a `STRING` member is a pointer, 8 bytes: `LEN(x.s())` of 4 elements is 32 (`v48_x30`), and
`_MEM(a())` of 4 elements of a `STRING` and a LONG is 48 bytes (`v48_x11`). Measured (`v56_g1`): `LEN` of a whole
member array of strings is 8 bytes an element over every bound and dimension, also of an array element's member
and beside other members. `v56_g2`: `_MEM` and `_MEMELEMENT` of a variable, an element, a member and a whole array
of a `TYPE` with a `STRING` member, and of a `TYPE` holding such a `TYPE`, give the `SIZE`, `ELEMENTSIZE` and `TYPE`
of the layout with 8 bytes for a `STRING`, by the rules of a `TYPE` without one. `len_whole` and `block_reference`
give these; `_MEMGET` and the rest keep their mark. `LEN` of such a `TYPE` variable, or of a whole array of one, is
"UDT must have fixed size" (`v56_x24`, `x25`).

### D11. The rejected forms

Each becomes an error at the place that marks it now, with the old compiler's wording as a note through the
wording table:
- `SADD` of anything but a variable-length string variable, and `VARPTR$` of an element of an array not of
  variable-length strings (D3);
- `STRING * n` with a name that is no numeric `CONST`: a variable, a name never assigned, a string `CONST`
  ("Number/Constant expected after *", `v51_x75`, `v56_x15`–`x17`; `check\decl.rs`). A numeric `CONST` of a float
  is accepted, rounded half to even (`v56_x14`, `x18`, `x29`: 2.5 is 2, 2.7 is 3, 3.5 and 4.5 are 4), and is built
  so, not made an error;
- a name `f$n` as a SUB's name: "Type symbols after a SUB name are invalid"; as a `CONST`: "Invalid constant
  type"; as a member, also `AS STRING * n`: "Invalid name" (`v21_x54`, `x55`, `x57`, `v56_x19`–`x22`, with or
  without a FUNCTION of the name). Each is checked where that declaration reads its name (`check\decl.rs`,
  `check\proc.rs`, `places.rs` `member_type`), before `split_name` marks it. (`p$5` as a SUB's parameter is
  accepted, `v56_x23`; it is not part of this change.)

### D12. The measurement family `v56`

Recorded with `qb64pe.exe` in task 1.1, before the code (the catalogue line is in `verification\README.md`):
- `v56_a1`–`a7`: `VARPTR` and `VARSEG` of scalars of every type, strings, fixed-length strings, `_BIT` and
  arrays of each kind in several declaration orders (`DIM`, implicit, `SHARED`, `COMMON`, after `'$DYNAMIC`);
  `PEEK` and `POKE` through them; a string's descriptor after every kind of assignment; `a5` a `TYPE` with a
  `STRING` in conventional memory (crashes);
- `v56_b1`–`b5`: `VARPTR$` of each type, printed as `ASC` of each byte, and `SADD` as the string grows;
- `v56_c`: `HEX$`, `OCT$`, `_BIN$` of `VARPTR`, `VARSEG` and `_SCREENEXISTS`;
- `v56_d` (a hidden screen, `$SCREENHIDE`, as `v16_m5`): graphics `GET` and `PUT` of each kind of D5, printing
  the bytes taken and the `POINT`s each `PUT` drew;
- `v56_e`, `e2`: file `GET`/`PUT` of `und()`, `unl&()`, `und$()` with data in the file, `_UCHARPOS` of `nd&()`;
- `v56_f1`–`f4`: `FOR` over `r.v`, `ar(i).v`, `r.m(i)`, `ro.n.v`;
- `v56_g1`, `g2`: `LEN` and `_MEM` of the D10 forms;
- `v56_t01`–`t09`: `VARPTR`, `VARSEG` and `VARPTR$` of `TYPE` variables, members and elements;
- `v56_x01`–`x30`: one form each, accepted or rejected, at the edges of D2, D3, D8, D10 and D11.

The old compiler's C++ of 26 of them (`-z`, task 1.2) is where the texts of D1–D6 come from.

## Risks / Trade-offs

- **The order of `dblock`** is the risk of this change: one variable out of order shifts every offset after it.
  `v56_a` covers the orders the corpus and Civil War Strategy use. A program whose order differs shows it at once:
  the printed offsets differ. A `_BIT` scalar declared anywhere in the main module is part of that order (D1).
- **Strings in conventional memory** depend on libqb keeping them there. A path that replaces the descriptor with
  `qbs_new` (a temporary, a `SWAP` of descriptors) would put a string outside `cmem`, and its `VARPTR` would be
  wrong. D1's check of every such place is task 2.3's.
- **Laying a type out again in pass 2** (D7) touches every user of a type's layout. The front-end tests of member
  arrays and the whole upstream `types/` set run in the closing set.
- **`dblock` is 64 KB less the strings in conventional memory.** A program with a very large fixed-length string
  named by `VARPTR` gets error 257, as with the old compiler.
