# Design

## Context

`m3-window-output` made a program with a window compile, with the text-screen, drawing and pacing statements.
Civil War Strategy (`tests\programs\civil-war-strategy\CWSTRAT.BAS`, 5,075 lines) then has 135 marked errors left;
this change takes all of them but sound and `$EXEICON` (`proposal.md`).

What was measured before this design (`verification\v28_*`, 83 programs; the C++ of seven programs read with
`qb64pe -z` outside the repository; `qb64pe.bas` read for the label rule and for `cmemlist`) is in `study\00` §5,
"What Civil War Strategy needs to build". In short:

- Ten of the built-ins are plain calls of their rows: `TAB`, `SPC`, `DEF SEG`, `BLOAD`, `BSAVE`, `POKE`, `PEEK`,
  `_FULLSCREEN`, `_RGB32`, `_RGBA32`.
- Graphics `GET` and `PUT` are plain calls with one argument of a kind the compiler does not have yet.
- `VARPTR` and `VARSEG` have no runtime entry; the old compiler writes an expression, and allocates the variable
  they name in another place.
- A line `name:` with a SUB's name is a call or a label, decided by the whole program.

How the game uses them: `DEF SEG = VARSEG(a(1))`, `BLOAD f$, VARPTR(a(1))`, `DEF SEG` on three lines in a row, four
times, always with element 1 of a static INTEGER array of the main module (`mtn`, `graphic`, `graft`, each `1 TO
1564`, shared); `PUT (x, y), a, PSET` 19 times and `GET (x1, y1)-(x2, y2), a` 8 times, with whole INTEGER arrays,
static and shared but for `g2` (`DIM g2(1 TO 8000)` in `SUB image2`); `TAB(n)` in 22 `PRINT` lines, one of them `PRINT #`;
`topbar:` as a line of the main module, `ships:` and `cannon:` as lines of `SUB navy` with `GOTO ships`, `GOTO
cannon`, `CALL ships`, `CALL cannon`.

## Goals / Non-Goals

**Goals**

- The game's marks drop to sound (`SOUND`, `PLAY`, `BEEP`) and `$EXEICON`.
- Every form of each built-in the old compiler accepts is either compiled with the old compiler's call or marked
  "not supported yet" with a row in `SOMEDAY.md`; a form it rejects is an error of the program.
- What the new statements draw and load is checked by the screen state of programs recorded with `qb64pe.exe`,
  among them programs cut from the game that load its pictures.

**Non-Goals**

- Sound; `$EXEICON`; the game built and run as a whole.
- Conventional memory for anything but a static numeric array of the main module (D4).
- File `GET` and `PUT` (step 4 of M3's order).
- A change to libqb.

## Decisions

### D1. `name:` with the name of a SUB: a call unless a jump of the program names the word

The old compiler's rule (`validlabel`, `qb64pe.bas` 27121; the label check at 13027; measured `v28_c_labels`,
`x71`–`x81`): where a label is being made and the word is a SUB that can be called with no argument, the line is
taken as a call and the word is remembered; when the program ends with a jump to a label that was never made and
whose word was so remembered, the compiler starts again with the word as a label everywhere.

`sema` does the same without a second pass over the code: before the labels are entered (`declare_labels`) it
collects, over every body, the words named as a jump's target. Then, for a line `name:` where `name` is a SUB:

| The SUB | A jump names the word | The line is |
|---|---|---|
| needs an argument | either | a label |
| takes none, or all its parameters are optional | no | `CALL name` (the statement after the colon follows it) |
| the same | yes, in any body | a label, in every body |

A FUNCTION's name is always a label (`x75`). A jump to the word from a body that has no such line is the error it
is for any label (`x71`, `x72`), and two such lines in a body are a duplicate label only when they are labels
(`x73`, `x74`). No new statement kind is needed: the line becomes the `Label` or the `Call` that exists.

**The set of jump targets must not miss a jump**, or a label would be compiled as a call. It is collected from the
syntax tree, from every node that holds a label as its target, whether or not `sema` compiles that statement:
`GOTO`, `GOSUB`, `RETURN label`, `RESTORE`, `RESUME label`, `ON ERROR GOTO`, `ON n GOTO/GOSUB` and the `ON
<event> GOSUB` family. Task 1.2 read every place of `qb64pe.bas` that makes a reference to a label and measured
a program per statement (`v28_x82`–`x92`): those are all of them. `IF c THEN name` and `CALL name` are no
jumps (the line stays a call); `RUN name` with a SUB's name is an error there ("Name already in use") and a
`RUN` is not compiled here. One safeguard: a line `name:` of a zero-argument SUB keeps today's mark when a line
the parser could not read has the word in it.

The mark for a label with the name of a built-in that is not compiled (`BEEP: x = 1`) stays as it is.

*Alternative considered:* compile such a line always as a label unless the game's exact shape is seen. Rejected:
`topbar:` in the game's main module is a call; as a label the bar it draws would be missing, with no error.

### D2. The ten plain built-ins are rows

`TAB` and `SPC` are `Rule::Plain` function rows giving a string; `PRINT`, `PRINT #` and `WRITE` take them as the
string items they are, so `check\io.rs` and `codegen-cpp\io.rs` do not change. The mention of `TAB` and `SPC` as
unsupported in `crates\README.md` goes. That a number may not follow `TAB(5)` without a separator is the parser's
rule for any item already; task 3.1 checks the message is an error and not a mark.

`DEF SEG`, `BLOAD`, `BSAVE`, `POKE`, `_FULLSCREEN` are `StmtRule::Plain` rows; `PEEK`, `_FULLSCREEN`, `_RGB32`,
`_RGBA32` `Rule::Plain` function rows. `DEF SEG` is a statement of two words whose second must be `SEG` (the table:
`secondargmustbe`), as `VIEW PRINT` is. `DEF FN` is not touched (the old compiler rejects it and `sema` says so),
and `DEF` alone is an error. The parser already reads `DEF SEG` as a statement of its own and not as `DEF FN`
(`crates\syntax\tests\parser.rs`); task 3.2 checks every form of it.

`_RGB32` has one to four slots and one entry of libqb per count (C++ overloads): the call is written with the
arguments given. `_FULLSCREEN` joins `WINDOW_BUILTINS`: compiled, compared by the call-site check, never run (a test
that takes the whole desktop is not wanted).

### D3. `VARPTR` and `VARSEG` are two expressions of the IR, written as the old compiler writes them

The table has no call name for them ("the qb64 compiler handles this function"). `sema` gives them a rule of their
own that takes a place and not a value, as `LEN` does for a variable, and the IR two expression kinds:

| Form | C++ (INTEGER array `a`) | Believed type |
|---|---|---|
| `VARPTR(a(i))` | `((array_check((i)-A[4],A[5]))*2)` | `_INTEGER64` |
| `VARPTR(a(i, j))` | the combined index of the element, times the size | `_INTEGER64` |
| `VARSEG(a(i))` | `( ( ((ptrszint)(A[0])) - ((ptrszint)(&cmem[0])) ) /16)` | `_INTEGER64` |

The index inside `VARSEG` is checked and not written: measured, the old compiler does not evaluate it (a FUNCTION
in it is not called, an index outside the bounds raises nothing; `v28_d_varptr_index`; `SOMEDAY.md`, "QB64pe
behaviours to review"). The index of `VARPTR` is the element's index as an array read writes it, bounds check
included, and is evaluated once. Stored, in `STR$` and in an operation both are `_INTEGER64` values. As the
direct argument of `HEX$`, `OCT$` or `_BIN$` the old compiler passes a width of 0, not an `_INTEGER64`'s:
marked.

### D4. An array named in `VARPTR` or `VARSEG` lives in conventional memory; only numeric arrays of the main module

An array of the IR gets a flag, set by `sema` when any `VARPTR` or `VARSEG` of the program names it (before code
is written, so the order of statements does not matter, as with the old compiler's second pass). The emitter then
writes the old compiler's allocation for it:

```
A[0]=(ptrszint)cmem_static_pointer;
if ((cmem_static_pointer+=((A[5]*2)+15)&-16)>cmem_dynamic_base) error(257);
if ((A[5]*2)>65536) error(257);
memset((void*)(A[0]),0,A[5]*2);
A[2]=1+2+4;
```

in place of `mem_static_malloc` and `1+2`. Arrays are allocated in the order they are declared, which they
already are, so a segment is the value the old compiler's program has (`v28_b_bload`: the differences between
arrays are printed and match).

**A dynamic array too (found by task 1.2; the decision of 2026-10-13 said static ones only).** An array listed in
`COMMON` is dynamic to the old compiler, and the game's `mtn` is one (`COMMON SHARED … mtn()` with `DIM SHARED
mtn(1 TO 1564)`). In conventional memory such an array differs in one thing: its descriptor starts with flag 4
in place of 0. Everything else is in the code every dynamic array of numbers has: each allocation makes its
block with `cmem_dynamic_malloc` when the descriptor has flag 4 and with `calloc` when not, and each release
frees it the matching way. The test is made when the program runs, since the array may be a parameter that
stands for either kind. The emitter now writes that test wherever it allocates or releases a dynamic array of
numbers (`new_block`, `free_block`), as the old compiler does, so `DIM` with a variable bound, `REDIM`, `REDIM
_PRESERVE`, `ERASE` and a SUB that erases or redims its parameter work on a conventional array (measured and
run: `v28_e_cmem_arrays`, slice `s73`). So every array of numbers of the main module may be named, static or
dynamic.

**Supported:** an element of an array of numbers declared in the main module (`DIM`, `DIM SHARED`, `COMMON` with
its `DIM`, `REDIM`), named from any body. That is every use in the game.

**Marked "not supported yet", each with its own message and a row in `SOMEDAY.md`:** `VARPTR`/`VARSEG` of a scalar,
a string, a fixed-length string, an array of a procedure (`DIM` or `STATIC` there), an array parameter, an array
of a user type, of strings or of `_BIT`, a member; `SADD`; `VARPTR$`. Each needs another allocation
(`dblock+cmem_sp`, `qbs_new_cmem`, `cmem_dynamic_malloc`, a parameter's callers), no program in hand uses one,
and each is its own piece of the emitter; the old compiler's code for each is recorded by task 1.2 so a later
change starts from it.

*Alternative considered:* all kinds now. Not taken: it is scope the user has not asked for, the parameter case
needs the flag carried through calls (and the old compiler gave an internal error for one such program while
this was measured), and nothing would run it.

`DEF SEG`, `POKE`, `PEEK`, `BLOAD` and `BSAVE` themselves take any number: with a segment and an offset that do not
come from an array they read and write libqb's emulated memory, as in the old compiler's program.

### D5. Graphics `GET` and `PUT`: a slot kind for an array's bytes

The table types the array argument "offset+size (largest safe block, restricted; graphics GET/PUT)". It becomes a
slot kind, `Slot::Block`: a place that is an array, whole (`a`, `a()`) or from an element (`a(i)`, `a(i, j)`). The
emitter writes `byte_element(address, bytes to the array's end, temporary)` as measured, the index written once
for the address and once for the size. The old compiler's text has it twice and so runs a FUNCTION in it twice
(task 1.2); an index that calls a SUB or FUNCTION is therefore marked, and any other index reads only, so writing
it twice changes nothing. The emitter declares the temporary (`byte_element_N`, 12 bytes
from static memory) with the variables of the body the statement stands in.

Every other slot is one the compiler has: the coordinates are `_FLOAT` slots passed as they are; `STEP`, `_CLIP`
and a passed colour are the template's flags; the action word is a choice with the values 1 to 5.

**Which form a statement is.** `GET` and `PUT` name four entries of the table: two graphics forms and the file
form each. A row names the entries it stands for, so `GET` and `PUT` stand for the graphics entries only; a
statement that is not of that form (`GET #1, , x`) keeps a mark ("`GET` on a file"). A statement that starts with
`(` or `STEP` is the graphics form to the old compiler and a syntax error when it does not fit (`GET (1), , x`,
`v28_x93`, `x94`): an error here too.

**Supported arrays:** numeric, static or dynamic, of the main module or a procedure, a parameter (all are the same
text with the array's name). **Marked:** an array of a user type (another text: `((char*)A[0])+…`), of strings,
fixed-length strings or `_BIT`, a member, and a name that is no array (a plain variable's bytes), each with a row
in `SOMEDAY.md`.
`GET (1, 2)-(3, 4), 5`, the old compiler's internal error, is an error here.

### D6. Data files beside a corpus program: `<name>.files\`

A folder `<name>.files\` beside `<name>.bas` is copied into the program's run folder (files and folders, as they
are) before it is compiled, with either compiler, for a run and for `--record`. Its files, by relative path and
bytes, are part of what the pass cache compares (`pass_key`). A `.files` folder beside a program with an `.err` or
`.norun` file is an error in the test, as `.keys` without `.screen` is. `.gitattributes` marks `*.files/**`
`-text`, so the copies stay byte for byte. The runner's own tests gain the cases (the copy, the pass cache seeing a
changed file, the misuse).

The programs cut from the game get copies of the pictures they load (ten files of 3,135 bytes, MIT, from
`tests\programs\civil-war-strategy`), written by `tools\screen\cut_game.py` so a copy is made again with the
program.

*Alternative considered:* a list file naming files elsewhere in the repository. Not taken: a corpus program would
stop being complete in its own folder, and the saving is about 30 KB.

### D7. What is checked, and by which program

| What | Checked by |
|---|---|
| The call of every form | a program in `tests\callsite` per form and per argument type of one slot of each kind. The numbered temporary `byte_element_N` joins the prefixes rule 2 of the check renumbers (generic: no rule names a built-in) |
| `TAB`, `SPC` on the console and in a file | a console slice program (from `v28_a_tab_spc`), no comma after a console item (`study\00` §5: it never ends with redirected output) |
| `TAB`, `SPC`, `POS` in a window | a screen program `d17_tab_window` (text mode and `SCREEN 12`; a zone after `TAB`) |
| `DEF SEG`, `VARSEG`, `VARPTR`, `POKE`, `PEEK`, `BSAVE`, `BLOAD` | a console slice program (from `v28_b_bload`, without its write past an array's end) |
| Labels and calls | a console slice program (from `v28_c_labels` and `x73`–`x81`); `check-fail` tests for `x71`, `x72`, `x74` |
| Graphics `GET` and `PUT` | screen programs: `d18_get_put` (each action, `_CLIP`, a colour, `STEP`, two dimensions, a dynamic array, an array parameter, from an element), `g06_message_box` |
| `BLOAD` of a real picture, then `PUT` | programs cut from the game with `.files\`: its mountains (`MTN.VGA`, eight `PUT`s), its icon and the capital marker taken from it by `GET` (`CWSICON.VGA`), a face and a fort; its `GET`/`PUT` animation of an army; a report screen laid out with `TAB`. Which procedures can be cut whole is settled in task 6.2 |
| `_RGB32`, `_RGBA32` | `m32_newimage_32`, and a line of a console slice program (they are arithmetic) |
| `_FULLSCREEN` | the call-site check only; a slice program has it as code that is not reached |
| Rejected forms | `check-fail` front-end tests from `v28_x03`–`x64` |

### D8. Order inside the change

1. The rest of the measurements (task group 1): what this design leaves to them.
2. Labels (the game's report loses 7 marks; no dependency).
3. The ten plain rows (`TAB` and `SPC` first: 22 marks).
4. `VARPTR`, `VARSEG` and conventional memory (with 3, `DEF SEG` and `BLOAD` lines compile: 12 marks).
5. Graphics `GET` and `PUT` (27 marks).
6. The runner's data files, then the cut programs.
7. Closing runs and documents, once (`CLAUDE.md` working rule 9).

Each group ends with tier 1, fmt and clippy green and names the files to stage; tier 2 runs at the close.

### As built (2026-10-13)

- **D1.** `collect_jump_words` in `sema\src\check\flow.rs` walks every tree for the statements above and keeps
  the words of lines with a parse error apart; `declare_label` then makes a label, records a call
  (`sub_call_lines`) or keeps the mark. Of the 23 measured label programs 20 behave as recorded; the three with
  `ON <event> GOSUB` stop at that statement's own mark (event handlers are not compiled), and a front-end test
  shows that their word is taken for a label.
- **D2.** Rows as planned. `_RGB32` needed a rule (`Rule::GivenArgs(1)`): its table entry has four slots and no
  template, and the call is written with the arguments given. Two rows beyond the plan: `DATE$` and `TIME$`
  (plain, no slot). The game's one `DATE$` stands on a line `TAB` kept marked, so it showed only when `TAB`
  compiled; `TIME$` is the same entry beside it.
- **D3, D4.** `ExprKind::VarPtr`, `ExprKind::VarSeg`, `Var::conventional`; the dynamic case above. Five `cpp`
  snapshots of earlier changes gained the flag test in every allocation and release of a dynamic array of
  numbers, and nothing else.
- **D5.** `StmtRule::Graphics`, `Slot::Block`, `StmtArg::Block`. The product of the element counts is written
  in the old compiler's order (the last dimension first) so the text compares equal.
- **D6.** As planned, and one thing the runner's own tests found: the run folder is emptied before each run, so
  the data files are copied again before each (also right when a run changes one).
- **D7.** The game's map and reports read the whole state of a game and cannot be cut as procedures. Where that
  is so, `cut_game.py` now copies a range of the game's lines among written ones (`g07`, `g08`, `g09`, `g11`);
  `g10` is a whole procedure. The call-site check has 147 programs; 12 written first were taken out because the
  check cannot compare an element's index or operands of mixed width inside an argument, which is why `VARPTR`
  has none and `GET`/`PUT` only whole arrays there (`tools\callsite\README.md`).

## Risks / Trade-offs

- **A jump the collection misses turns a label into a call** → the tree walk takes every node with a label
  target, supported or not; the error-node safeguard; a measured program per jump statement (D1).
- **A segment value differs from the old compiler's** because something else of the program is allocated in
  conventional memory there and not here (a string or scalar named in `VARPTR`: marked here, so such a program does
  not compile) → tests print differences of segments and what `PEEK` and `BLOAD` read, never a bare segment.
- **`BLOAD` and `POKE` write where the program says**, past an array's end too → as in the old compiler's program;
  libqb keeps the write inside its own block. Nothing new to guard.
- **The index of a `GET`/`PUT` array is written twice** → as the old compiler; measured for a side effect, marked
  if it cannot be written the same way.
- **Screen programs with pictures depend on the copied `*.VGA` bytes** → `-text` in `.gitattributes`; the pass
  cache keys on them.
- **Tier 2 grows** by about ten screen programs (0.3 s each with the build cache) and the call-site check by an
  estimated 120 programs (about a minute without its cache). Machine time, accepted as before.

## Open Questions

1. **Conventional memory for static numeric arrays of the main module only** (D4)? Scalars, strings, dynamic and
   local arrays and parameters would stay marked until a program needs one.
2. **`_RGB32`, `_RGBA32` and `_FULLSCREEN` in this change?** They are outside the wording of step 2 (`DECISIONS.md`
   2026-10-12 puts the image built-ins in no step); the first two let `m32_newimage_32` pass, the third is one of
   the game's marks.
3. **Data files as copies in `<name>.files\`** (D6), against a list that names files of `tests\programs`?

**Answered 2026-10-13: yes to all three** (`DECISIONS.md`), as the design assumed.
