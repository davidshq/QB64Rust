# Design

## Context

See `proposal.md` for motivation and the spec deltas for requirements. Background: `study\27` §3 (what the corpus
waits for, and the shape of the operation), `study\28` §3.2 and §9 (the call-site check), `study\02` (the old
compiler's statement code, its data segment and `passed` masks). The designs of the earlier changes hold, in
particular D8 of `m2-control-flow-slice` (flat bodies, the pending-error rule), D5 and D10 of `m2-arrays-and-types`
(places and their store rules; the IR shares `sema`'s value tree) and D2 to D5 of `m2-core-builtins` (one
table-driven checker with a closed rule set; the table is never edited).

Where the code stands (2026-10-09):

- The parser already gives every statement of this change a node: `BuiltinStmt` for a statement read by its
  `specialformat` template (`OPEN`, `SEEK`, `NAME`, `SHELL`, `RANDOMIZE`: the name, `FormWord` nodes, punctuation,
  `FormArg` nodes), `CallStmt` for a plain one (`KILL`, `MKDIR`, `RMDIR`, `CHDIR`, `ENVIRON`), and `PrintStmt` (with
  `FileNumber`), `WriteStmt`, `InputStmt`, `LineInputStmt`, `CloseStmt`, `SwapStmt`, `DataStmt`, `ReadStmt`,
  `RestoreStmt`. `ast.rs` has accessors for `PrintStmt`, `DataStmt` (items split by the old compiler's rule,
  `syntax\src\data.rs`), `ReadStmt` and `RestoreStmt` only; the others can only be cast.
- `sema` marks all of them in `check\mod.rs` (`statement`); `PRINT #` in `print`.
- The built-in table has the SUB entries: `Open` twice (`sub_open`, `sub_open_gwbasic`), `Shell` three times
  (`sub_shell`, `sub_shell2`, `sub_shell3`, by word order), `Seek`, `Name`, `Randomize`, `Kill`, `MkDir`, `RmDir`,
  `ChDir`, `Environ`; and `sub_stub` entries without slots for `Print`, `Write`, `Input`, `Read`, `Close`, `Swap`,
  `Mid$`, which the old compiler writes itself.
- The IR has no statement call of a built-in (`ir\src\lib.rs`: `Op`). The emitter writes a function call from the
  table (`table_call`: a placeholder `0` for an absent slot, then the mask).

Read from the old compiler's C++ on 2026-10-09 (`qb64pe -z` on one sample; to be pinned in group 1):

```text
OPEN "t.txt" FOR OUTPUT AS #1    sub_open(qbs_new_txt_len("t.txt",5), 4 ,NULL,NULL, 1 ,NULL,0);
OPEN s FOR INPUT AS a            sub_open(__STRING_S, 3 ,NULL,NULL,*__LONG_A,NULL,0);
PRINT #1, "x"; a, d              tab_spc_cr_size=2; tab_fileno=tmp_fileno= 1 ; if (is_error_pending()) goto skip1;
                                 sub_file_print(tmp_fileno,qbs_new_txt_len("x",1), 0, 0, 0); if (…) goto skip1;
                                 sub_file_print(tmp_fileno,qbs_str((int32)(*__LONG_A)), 1, 1, 0); …
                                 skip1: qbs_cleanup(qbs_tmp_base,0); tab_spc_cr_size=1;
WRITE #1, a, s                   sub_file_print(tmp_fileno,qbs_add(qbs_ltrim(qbs_str((int32)(*__LONG_A))),",")),0,0,0); …
CLOSE #1 / CLOSE                 sub_close( 1 ,1);  /  sub_close(NULL,0);
INPUT #1, a, s                   tmp_fileno= 1 ; if (…) goto skip3; *__LONG_A=func_file_input_float(tmp_fileno,138412064);
                                 if (…) goto skip3; sub_file_input_string(tmp_fileno,__STRING_S); …
LINE INPUT #1, s                 sub_file_line_input_string(tmp_fileno,__STRING_S);
READ a, s, d                     *__LONG_A=func_read_float(data,&data_offset,data_size,138412064);
                                 sub_read_string(data,&data_offset,data_size,__STRING_S); …
RESTORE / RESTORE lab            data_offset=0;  /  data_offset=data_at_LABEL_LAB;
DATA 1,"two",3.5                 bytes 1,"two",3.5, in inline_data[] (global.txt), data_size, data_at_LABEL_LAB
SWAP a, a2 / SWAP s, s           swap_32(&*__LONG_A,&*__LONG_A2);  /  swap_string(__STRING_S,__STRING_S);
RANDOMIZE 5 / RANDOMIZE TIMER    sub_randomize( 5 ,1);  /  sub_randomize(func_timer(NULL,0),1);
d = RND / RND(1) / TIMER         func_rnd(NULL,0)  /  func_rnd( 1 ,0|1)  /  func_timer(NULL,0)
INPUT "p"; a, s                  qbs_print("p",0); qbs_print("? ",0); qbs_input_variabletypes[1]=32;
                                 qbs_input_variableoffsets[1]=&(*__LONG_A); … qbs_input(2,1); if (stop_program) end();
LINE INPUT s                     qbs_input_variabletypes[1]=ISSTRING+512; … qbs_input(1,1);
COMMAND$ / COMMAND$(1)           func_command(NULL,0)  /  func_command( 1 ,0|1)
MID$(s, 2, 1) = "q"              sub_mid(__STRING_S, 2 , 1 ,qbs_new_txt_len("q",1),1);
```

Run the same day: `WRITE #1, 1, "a", 2.5` writes `1,"a",2.5`; `LINE INPUT #` past the end raises 62; `OPEN` of a
missing file for input 53; `PRINT #` to a closed number 52; `KILL` of a missing file 53; `CLOSE #5` with nothing
open no error; `SWAP a&, d#` is the compile error "Type mismatch"; `MID$(s, 1, 1) = "ZZ"` on `"q"` gives `Z`; a
`$CONSOLE:ONLY` program reads `INPUT` and `LINE INPUT` from a redirected standard input, and ends without an error
when the input runs out.

## Goals / Non-Goals

**Goals:**
- One operation and one checking path for every built-in statement whose slots the table gives; adding the next
  plain statement is a row and tests.
- Statements with a shape of their own (items, targets, data) stated in the IR with their error rule, so the
  emitter decides no semantics.
- Every rule measured with `qb64pe.exe` before it is coded; anything measured as wrong code in the old compiler
  put to the user first (as D-004 was), anything merely questionable implemented QB64pe's way and listed in
  `SOMEDAY.md` (`DECISIONS.md` 2026-10-08).
- A verdict on the call-site check from real use.

**Non-Goals:**
- A runtime of our own: every statement calls libqb as the old compiler does.
- Random and binary file access, formatted printing, the screen: see `proposal.md`, "Not in this change".
- Matching the old compiler's C++ beyond the calls (`codegen-cpp`'s design: spelling and layout are free).
- Generated call-site programs for the roughly 250 remaining built-ins: that follows a "kept" verdict, in a later
  change.

## Decisions

### D1. Measure first
Before code, `verification\v22_*` programs (run with `verification\run.sh`; file-writing programs use names
starting `v22_` and delete what they create), the C++ read with `qb64pe -z`, findings in `study\00` §5. One set of
programs per group of tasks, each written when its group starts, so that a session measures what it then builds:

- **Statement calls (`v22_a_*`)**: for `KILL`, `MKDIR`, `RMDIR`, `CHDIR`, `NAME`, `SEEK`, `ENVIRON`: is the call made
  with a placeholder after a raising argument, or skipped (the C++ shows no test before `sub_open`)? Which error
  wins when the argument and the call both raise? `RESUME` re-running the statement. Each argument kind wrong
  (number for string and the reverse), too few and too many arguments. The statement used in a procedure.
- **Files (`v22_b_*`)**: every `OPEN` mode, access and lock word, `LEN =`, with and without `#`, the old form with
  each mode letter and a bad letter; a file number that is a float, 0, negative, 256, already open (error 55),
  beyond LONG; `CLOSE` with several numbers, a float, one not open; `PRINT #` with every `PRINT` item kind, `,`
  zones in a file (14 columns? the console's 10?), a trailing `;` and `,`, an empty `PRINT #1,`; `WRITE` of every
  numeric type, negative numbers, floats, strings holding quotes, no items; `INPUT #` into every numeric type,
  quoted and unquoted fields, blanks, an empty field, a field that is no number, a number too large for its target,
  fewer fields than targets, CR LF against LF; `LINE INPUT #` of an empty line, a last line without a line end, a
  numeric target (compile error?); `EOF` on an empty file, at each position, on an output file, on a closed number;
  `LOF`, `LOC`, `SEEK` function and statement; `FREEFILE` twice without opening; `_FILEEXISTS` of a folder,
  `_DIREXISTS` of a file; `NAME` onto an existing file; `RMDIR` of a folder that is not empty; `KILL` with a
  wildcard; the error numbers of each (52, 53, 54, 55, 62, 64, 75, 76 as they come).
- **Data (`v22_c_*`)**: the order of items across the main module, procedures and included files; a `DATA` inside
  a SUB, an `IF` block, after `END`; bare `DATA` (one empty item or none: `ast.rs` says unmeasured); `READ` into
  every numeric type, an element, a member, a fixed-length string; a string item read into a number, a quoted
  number, `&H10`, `1e3`, `1d3`, an empty item, an item too large for its target; out of data (error 4) and the
  state after `RESUME NEXT`; `RESTORE` to a label before, after and between `DATA` lines, to a label in a procedure
  and from a procedure to a main-module label, to a label with no `DATA` after it, to an unknown label.
- **The rest (`v22_d_*`)**: `SWAP` of every type pair that should work (each numeric type, `STRING`, fixed-length
  strings of equal and unequal length, user types, elements, members) and every pair that should not, a literal
  operand, the same place twice; the `MID$` statement with start 0, negative, past the end, length 0, negative,
  longer than the value, a fixed-length target, an element; `RANDOMIZE` with each numeric type, `USING`, and
  without a seed under `$CONSOLE:ONLY` (it prompts: what is read from a redirected input, and from none);
  `RND(-1)`, `RND(0)`, `RND(1)`, the result type of `RND` and `TIMER` (table: SINGLE) and of `TIMER(0.001)`; `SHELL`
  in each word order, without a command, with a failing command, the function's result type; `COMMAND$` with and
  without an index, index 0 and past the count; `ENVIRON$` with a name, an index, an unknown name, a number that
  is a float; the `ENVIRON` statement without `=`.
- **Console input (`v22_e_*`)**: with the input redirected from a file: prompts ended by `;` and `,`, no prompt,
  the leading `;`, several targets, fewer and more fields than targets (the "Redo from start" text?), a field that
  is no number, each numeric type and its overflow, quoted fields, an empty line, the end of the input in the
  middle of a statement (the sample ended the program: exit code and output), `LINE INPUT` of an empty line; how
  `END` behaves when standard input is a file (does "Press any key to continue" still wait?); what the prompt
  looks like in the recorded output.

### D2. The IR: one statement call, and operations of their own where the shape differs
```text
Op::Builtin { id: BuiltinId, args: Vec<StmtArg> }      one entry per template argument or choice, in template order
StmtArg = Value(Expr)       an argument, already converted to its slot
        | Place(Place)      a variable, element or member the statement stores into or exchanges
        | Word(u8)          which alternative of a `{a|b|c}` choice was written (0-based)
        | Absent            an optional argument or choice left out

Op::Print  { to: Option<Expr>, items, newline }        `to`: the file number; `None` is the console (today's Print)
Op::Write  { to: Option<Expr>, items, newline }        items as Print's Str and Num; `newline` false after a
                                                       trailing comma (measured: `WRITE 1,` gives `1,` and no line end)
Op::Input  { from: Source, line: bool, targets: Vec<Place> }
             Source = File(Expr) | Console { prompt: Option<bytes>, question: bool, stay: bool }
Op::Read   (Vec<Place>)
Op::Restore(usize)                                     a position in Program::data
Program::data: Vec<DataItem { text: bytes, quoted: bool }>
```

`id` is the table entry, so it already says which of `OPEN`'s two forms and `SHELL`'s three was matched; the IR
names no `sub_open`, no mode number and no mask (the emitter turns `Word(3)` of `OPEN` into the old compiler's
mode value by its rule). `SWAP`, the `MID$` statement and `CLOSE` are `Op::Builtin` with their `sub_stub` entries'
ids and a rule each (D3); `CLOSE` has one `Value` per file number.

The error rule, stated in the crate documentation and checked by `validate` (measured, `v22_a_calls`, task 1.1:
the handler runs once, with the argument's error; `sub_environ` alone does not test and raises again, which changes
nothing since the first error is the one serviced):
an `Op::Builtin` evaluates its arguments in order and makes the call also after a raising argument, like
`Op::Call` (the C++ has no test between the arguments and `sub_open`); libqb's entry returns at once while an error
is pending. `Print` to a file, `Write`, `Input` and `Read` check before each item or target and skip the rest
(the `goto skipN` above), as console `Print` does today. A target place follows its own store rule
(`m2-arrays-and-types` D6), measured for an element with a bad index.

*Alternatives:* (a) every statement its own operation (`Op::Open`, `Op::Kill`, …): exhaustive matches for forty
statements in `lower`, `validate`, `dump` and the emitter, and step 9's "a row and a test" is lost; (b) everything
through `Op::Builtin`, `PRINT #` included, with items as arguments: the per-item error rule and the item kinds
(`Str`, `Num`, `Zone`) would have to be re-encoded in arguments, and `Print` already states them; (c) lowering
`READ` and `INPUT` to assignments from a hidden "next item" value: the old compiler's file input for a numeric
target is one libqb entry chosen by the target's type, with no intermediate value to assign.

### D3. `sema`: a statement list beside the function list
`sema\src\builtins.rs` gains `STATEMENTS: &[(&str, StmtRule)]`, the twin of `SUPPORTED`, with the same lookups and
the same test that every row is in the table once. `StmtRule` has one variant per kind of special-casing:

```text
StmtRule = Plain        slots, optional mask and choices from the table entry and its template
                        (KILL, MKDIR, RMDIR, CHDIR, NAME, SEEK, ENVIRON, SHELL ×3, RANDOMIZE)
         | Open         the FOR … AS form: three choices, the name, the number, the record length
         | OpenOld      the letter form
         | Close        any number of file numbers
         | Swap         two places of one type
         | MidAssign    a string place, start, optional length, the value
```

`check\builtins.rs` gets the statement side: find the rows for the statement's name, take the node's `FormArg`s
and `FormWord`s (or a `CallStmt`'s arguments) in order, check each argument's kind and convert it by its slot
(`Slot`, as functions: LONG as a store, DOUBLE exactly, `STRING` as it is), and push `StmtKind::Builtin`. The
statements with their own nodes (`PRINT #`, `WRITE`, `INPUT`, `LINE INPUT`, `READ`, `RESTORE`, `DATA`) are checked
in a new `check\io.rs` and become typed statements of their own; `print` loses its `PRINT #` mark and types the
file number as a LONG slot. `ast.rs` gains the accessors these checks need (`BuiltinStmt::name`, `args`, `words`;
`WriteStmt`, `InputStmt`, `LineInputStmt`: `file`, `prompt`, `items` or `targets`; `CloseStmt::numbers`;
`SwapStmt::operands`), every child an `Option` or an iterator as the others. A target is resolved as an assignment
target is (`assign`: an implicit variable is created; the name is a reference in the symbol table).

A statement in a form that is left out (a `RESTORE` to a line number, `READ` into a whole array or a `TYPE`
variable, `LINE INPUT` into something the old compiler accepts but D1 did not measure) is "not supported yet" at
that form, never guessed.

*Alternative:* derive everything for `Plain` statements from the table without a list: the list is what says
"supported" (the coverage check reads it), exactly as for functions.

**Corrected by tasks 1.7 and 3.1 (2026-10-10).** The old compiler turns every template into a call by one rule
(`seperateargs`: which arguments and choices are C arguments, `NULL` for an absent one, the `passed` bits), ported
as `crates\builtins\src\passing.rs` and checked against the C++ of every `OPEN` form. `OPEN` in both forms is
therefore `Plain`; **the variants `Open` and `OpenOld` are not needed** and do not exist. `Close` stays a rule
(any number of file numbers, one `sub_close(n,1)` each, `sub_close(NULL,0)` alone), as do `Swap` and `MidAssign`.
A `Plain` argument's slot type is the table's type of its C argument; `SEEK`'s position is `_INTEGER64` against
the table's LONG (libqb's `sub_seek(int32, int64)`, measured).

### D4. Functions: rows, and two small extensions
`EOF`, `LOF`, `LOC`, `SEEK`, `FREEFILE`, `_FILEEXISTS`, `_DIREXISTS`, `_CWD$`, `SHELL`, `RND`, `TIMER`, `COMMAND$`
are `Rule::Plain` rows if D1 agrees with the table (result types by printing, as `m2-core-builtins` D1). Two
things the existing rules do not cover:

- **Bare names.** `_PI` is resolved without parentheses by a special case. That becomes a property of the row: a
  function whose slots are all optional or that has none is callable bare when D1 measured it so (`RND`, `TIMER`,
  `COMMAND$`, `FREEFILE`, `_CWD$`). What a variable or `DIM` of such a name does is measured, not assumed.
- **`ENVIRON$`** takes a number or a string against the table's one LONG slot: a rule `StrOrIndex`, in the manner
  of `StringFill`, only if D1 shows two libqb entries; otherwise `Plain`.

An absent optional slot is written `NULL` by the old compiler and `0` by ours; both are the same C++. The emitter
changes to `NULL` (a handful of `cpp` snapshots change once), so that the call-site check needs no rule for it.

**Corrected by task 3.1.** `LOF`, `LOC` and `SEEK` are believed LONG as the table says but **held `_INTEGER64`**:
libqb's `func_lof`, `func_loc` and `func_seek` return `int64`, so arithmetic on them is computed in 64 bits (an
override of the held type by the libqb entry, beside the `std::` overloads). `FREEFILE` and `_CWD$` have no slot
and are called bare only (`FREEFILE(1)` is "Incorrect number of arguments", `FREEFILE()` "Expected (...)").

### D5. Emission
`codegen-cpp\src\builtins.rs` gets `stmt_call` for `Op::Builtin`: `Plain` by the table (`callname`, arguments,
`NULL` for absent ones, the mask; a choice's value by the old compiler's numbering for that template, read from
`qb64pe.bas` and pinned by the call-site check), and one function per other rule (`sub_open`'s seven arguments,
`sub_close(n,1)` per number or `sub_close(NULL,0)`, `swap_8/16/32/64/…` by the places' type, `sub_mid`). A new
module `codegen-cpp\src\io.rs` writes file `Print`, `Write`, `Input` and `Read` as above: the `tmp_fileno` store,
one libqb call per item or target chosen by its type (`func_file_input_float` with the old compiler's type code,
`sub_file_input_string`, `func_read_float`, `sub_read_string`, …), the skip label, the `tab_spc_cr_size` lines that
tell libqb's `TAB`/`SPC` they are in a file statement. Console `Input` fills `qbs_input_variabletypes` and
`qbs_input_variableoffsets` and calls `qbs_input`, then tests `stop_program`, as the old code. The type codes are
the emitter's (libqb's ABI): one function from `Ty` to the code, tested against the codes read from the old C++
for every type.

**Read in task 3.1 (the exact lines are in `study\00` §5).** File `Print`: `tab_spc_cr_size=2;
tab_fileno=tmp_fileno=n;`, the pending-error test, one `sub_file_print(tmp_fileno,text,extraspace,tab,newline)`
per item with the test after each, the skip label, `tab_spc_cr_size=1;`; a comma is the `tab` flag of the item
before it (or a call with `nothingstring` when there is none), not a call of its own. `Write` builds each item's
text in C++ (`qbs_ltrim(qbs_str(…))`, quotes added by `qbs_add`, a `,` appended to every item but the last) and
sends it through `sub_file_print` or `qbs_print`. File `Input`: `tmp_fileno=n;`, then per target a store of
`func_file_input_float(tmp_fileno,code)` / `func_file_input_int64` / `func_file_input_uint64` by the place's own
store rule, or `sub_file_input_string(tmp_fileno,place)`; `LINE INPUT` is `sub_file_line_input_string`. The type
code is the old compiler's type value of the target (width, float, unsigned, and the place's kind).

### D6. Data
`sema` collects the items while it checks, in the order D1 measures (expected: file order across the whole
program, procedures included), into `Program::data`, and records for each `LabelDef` the count of items before
it, so `RESTORE label` is a position. The emitter writes the list as the old compiler's bytes (items joined by
`,` with a trailing `,`, a quoted item in its quotes) into `inline_data[]` with `data_size`, and a position as a
byte offset (`data_at_LABEL_…`, or a number where the label is the lowering's business). Whether an unquoted item's
case and inner blanks are kept is already measured (`v16_m2_*`) and in `syntax\src\data.rs`.

*Alternative:* typed data (numbers parsed when compiling): the old runtime parses the text at `READ` time by the
target's type, so `DATA 1` read into a string is `"1"`; the text is the fact.

### D7. Console input and the `.stdin` sidecar
`Op::Input` with `Source::Console` is compiled only under `$CONSOLE:ONLY` (every program is, today). The prompt is
a string literal or absent (the old compiler takes nothing else; expression prompts are in `SOMEDAY.md`).

The corpus runner starts a program on Windows in a console of its own that receives key presses (for `END`) with
no usable standard input. A program with `<name>.stdin` is started with standard input redirected from a copy of
that file instead (spec delta `testing/golden-corpus`). D1's `v22_e` settles whether such a program may still end
with `END` or must use `SYSTEM`; the slice programs follow what is measured, and the runner's README says so.
`verification\run.sh` gets the same sidecar rule (`< "$n.stdin"` when present).

*Alternative:* scripted key presses into the console: `INPUT` under `$CONSOLE:ONLY` reads standard input, not
keys, and a file is reproducible where timing is not.

### D8. The call-site check (trial)
`tools\callsite\callsite.py` and its programs `tests\callsite\<name>.bas`: each a few `DIM` lines and then one
statement under test on its last line before `END`. For each program the tool runs `qb64pe.exe -z` (serially: it
writes the clone's `internal\temp`, emptied first as the corpus runner does) and `qb64rust.exe -z -o <scratch>`,
and takes from each compiler's main-module fragment the lines that belong to that source line (both write `#line`
markers; the fallback is the lines after the last `DIM`'s code). It drops the statement frame the design calls
free (`do{`, `}while(r);`, `S_n:;`, the event test, `qbs_cleanup`, blank lines), splits the rest into tokens, and
normalises: spacing; variable and temporary names to `v1`, `v2`, … in order of first use; `( x )` around a single
token; skip-label numbers. The two token lists must be equal. Output: one line per program (`equal`, `differ` with
both lists, `not compared` with the diagnostic) and a summary; no path in it.

The first programs are the file statements of groups 1 and 3, one per form and argument type (about 40, written by
hand; a generator comes only with a "kept" verdict). The verdict (spec `testing/call-site-check`, "A recorded
verdict") is **kept** when every one of them compares equal with normalisation rules that name no built-in, and
when at least one deliberate mistake per statement kind (a missing rounding call, a wrong mask, a swapped argument,
made by the tool's self-test on the new compiler's extracted lines) is reported. It is **dropped** when rules per built-in are needed or when
the old compiler's statement frames cannot be told from the calls. With "kept", the check runs in tier 2 (one more
command in `crates\README.md` and `rust.yml`) and `tests\callsite\` grows with each later built-in; with
"dropped", the tool, its programs and the capability's spec delta are deleted and only the `DECISIONS.md` row
stays.

*Alternative:* compare inside `cargo test` from recorded old-compiler lines: the recording would go stale with the
reference clone and hides which side moved; the tool is cheap to rerun and the old compiler is at hand locally.

### D9. Tests and corpus
- `tests\frontend\`: an `ir` test per statement rule and per operation (the coverage check reads them), a `typed`
  test per new function, `check-fail` for every real error and every left-out form, `cpp` tests for `Op::Builtin`
  with choices and absent slots, file `Print`, `Write`, `Input`, `Read`, the data fragment and console `Input`.
- `tests\corpus\slice\` (recorded with `qb64pe.exe`; names may change): `s35_file_system`, `s36_files`,
  `s37_file_errors`, `s38_file_functions`, `s39_data`, `s40_swap_mid`, `s41_random` (seeded), `s42_console_input`
  (with `.stdin`), `s43_shell_environ` (with a `.normalize` where the machine shows), `s44_more_functions`. File names inside them start with the program's
  name and are deleted by the program.
- The coverage check (`testing/compiler-tests` delta) reads `STATEMENTS` as it reads `SUPPORTED`: each name in a
  `slice.list` program and in an `ir` front-end test.
- `validate` checks the new operations: a `Restore` position within the data, an `Input` with at least one target,
  a `Word` only where the template has a choice, places of the types the rule allows.
- The shrink-only lists are regenerated after each group; corpus programs that pass join `slice.list`.

## Risks / Trade-offs

- [The change is large: eight groups] → the groups are independent after group 2 and each ends green with its own
  tests, lists and README lines; if the user wants it split, groups 6 and 7 (console input, `SHELL`, the by-demand
  functions) can become a second change without rework.
- [A statement's output depends on the machine: `SHELL`, `ENVIRON$("PATH")`, `TIMER`, `_CWD$`] → slice programs
  print only what is the same everywhere (a variable the program set itself, comparisons, lengths compared with
  0), with `.normalize` as the corpus already does for `212`, `234`, `238`.
- [Programs leave files behind or collide when run in parallel] → the runner already gives each program its own
  scratch folder (spec `testing/golden-corpus`); `verification\` programs delete what they create and use unique
  names.
- [`RANDOMIZE` without a seed prompts and reads input] → measured in `v22_d`; compiled only as measured, else "not
  supported yet" at that form.
- [The call-site check looks stable on forty hand-picked statements and is not in general] → the verdict's second
  condition (seeded mistakes are caught) and the recorded count; a "kept" verdict is revisited by the first later
  change that needs a rule per built-in.
- [`Op::Builtin` becomes a back door for semantics the IR should state] → a statement gets its own operation as
  soon as its error rule or its items differ from "evaluate the arguments, call" (as `Print`, `Input`, `Read` do
  here); `validate` rejects a rule's statement with slots the rule does not allow.
- [The table is wrong for a statement treated as `Plain`] → every supported statement is in a recorded slice
  program (coverage check) and, during the trial, in a call-site program.

## Migration Plan

None: new behaviour behind "not supported yet" marks that go away. The one change to existing output is `NULL`
for an absent optional slot (D4), which is the same C++; its snapshots are updated in the task that makes it, with
tier 2 green before and after.

## Open Questions

None that change the plan. The provisional points (the error rule of `Op::Builtin` in D2, bare names and
`ENVIRON$` in D4, the data order in D6, `END` with redirected input in D7) are settled by the measurements of
group 1 and of each group's first task, and corrected in the task that finds them, as in the earlier changes.
