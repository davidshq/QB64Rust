# Design

## Context

For the motivation, scope and what was found before this proposal, see `proposal.md`. For the behaviour, see the
delta specs. Where the parts stand in this compiler (2026-10-21):

- **`ON ERROR GOTO`.** The parser already reads `_NEWHANDLER` or `_LASTHANDLER` before a label
  (`syntax\src\parser\errors.rs` `on_error`; `ast::OnErrorStmt::handler_word`). `sema` marks both forms "not
  supported yet" (`check\flow.rs` `on_error`). `Op::SetHandler(Option<LabelId>)` is written by
  `codegen-cpp\src\stmt.rs` as `error_goto_line=n;`, with `n` numbered per label (`Emitter::handler`). `mainerr.txt`
  maps each number to its label. Its first line already sets `error_handler_history` to an empty string at the
  start of `QBMAIN`, as the old compiler's `ErrTxtBuf` does (`qb64pe.bas` 3286). `ON ERROR GOTO 0` already clears
  the history.
- **Statements of the table.** A statement with a row in `STATEMENTS` (`sema\src\builtins.rs`) is a plain call.
  The log statements need more than that: the statement's file, procedure and line as arguments.
- **`$CHECKING`.** `check\mod.rs` reads `$CHECKING:OFF` and `:ON` in `afterlastline.bm` only and keeps
  `self.at.checking`. A procedure takes that state when its header is met, into `sema::Proc::checked`, then
  `ir::Proc::checked` (design D8 of `m3-network-embed`). `codegen-cpp\src\procs.rs` `body` writes a statement without
  its `do{…}while(r);` and its event check when the body is unchecked, and `labels` leaves out a label's check.
  Everywhere else `$CHECKING` is "not supported yet".
- **The checks that OFF leaves out**, where our emitter writes them:
  - the event check of a statement and of a label (`procs.rs` `body`, `labels`);
  - `array_check` (`place.rs` `flat_index`, `member_flat_index`, `dyn_element_desc`);
  - the checked forms of the memory statements (`mem.rs` `mem_read`, `mem_get_stmt`, `mem_put_stmt`,
    `mem_fill_stmt`, the checks in `mem_checked`).

  The old compiler also leaves out its `UDTRootDataGuard$` below OFF. Our emitter writes no such guard: a member
  array lies in its value (design D8 of `m2-defxxx-common-arrays`), and the guard of a dynamic member array
  (`owner.rs` `dyn_fn(…, "guard")`) stands for `UDTDynArrayDescGuard$`, which the old compiler keeps below OFF.

## Goals / Non-Goals

**Goals:**

- `source_ordering/checking_off`, `mem/mem_checking_off`, `on_error/errtest` and `logging/commands` pass, and
  their four lines leave the upstream demand report.
- The C++ of an unchecked statement makes the same libqb calls as the old compiler's. The call-site check
  compares it (programs under `tests\callsite\` with `$CHECKING:OFF`).
- The log of a verification program is compared, so that the log's file, procedure and line are tested.

**Non-Goals:**

- `$DEBUG`'s own forms (`vWatch`) below OFF. The old compiler's warning, "$DEBUG features won't work in these
  blocks", is given only from the IDE (`NOT NoIDEMode`), so a `-x` build has nothing to match.
- An index outside its bounds, or `_MEMGET` of a region outside its block, below OFF. Neither is defined in the
  old compiler's C++, and no test runs one.
- The other `qb64pe\*` programs (the second change), and `SUB _GL`.

## Decisions

### D1. The handler chain is two operations of the IR

`ir::Op` gains `NewHandler(LabelId)` and `LastHandler` beside `SetHandler`. `sema::StmtKind::OnError` gains the
form (`check\flow.rs` `on_error`: `_NEWHANDLER label` → new, `_LASTHANDLER` alone → last), and the lowering maps
them (`ir\src\lower.rs`). The emitter (`stmt.rs`) writes the old compiler's lines (`qb64pe.bas` 10066–10068,
10106–10110):

- `NewHandler`: `qbs_set(error_handler_history, qbs_add(qbs_add(qbs_str((int32)(error_goto_line)),
  qbs_new_txt_len("|", 1)), error_handler_history));`, then `qbs_cleanup(qbs_tmp_base, 0);`, then
  `error_goto_line=n;`.
- `LastHandler`: `error_goto_line = qbr(qbs_val<uint64_t>(error_handler_history));`, the `func_mid` line that
  drops the first number, then `qbs_cleanup(qbs_tmp_base, 0);`.

The numbers differ from the old compiler's. It numbers every `ON ERROR GOTO` statement, and we number every label.
A number stored in the history goes back through `mainerr.txt` to the same label either way, so the behaviour is
the same, and the call-site check sees the same calls.

The errors are checked in `on_error` before the label. The parser (`errors.rs`) also takes a `Number` after
`_NEWHANDLER` and an `Ident` or a `Number` after `_LASTHANDLER`, so that `_NEWHANDLER 0`, `_LASTHANDLER h` and
`_LASTHANDLER 0` reach the check and get the old compiler's message, not a parse error. `_NEWHANDLER` alone is
"Expected: ON ERROR GOTO [_NEWHANDLER] label". Another word before the label (`ON ERROR GOTO foo h`) gets that
message in the old compiler (`v50_x26`); here it stays a parse error ("expected end of statement"), since taking
any two words would read a keyword after the label (`ON ERROR GOTO h ELSE …` in a one-line `IF`) as the label.
A line number after `_NEWHANDLER` is a label, as after `GOTO` (`v50_x13`). Three words after `_NEWHANDLER` are
some other error in the old compiler ("Name already in use (ERROR)"); here they are a parse error. A word with a
leading underscore that is no label stays as it is today.

`v50_a_chain` uses no error 11: in this libqb `ERROR 11` (division by zero) is a critical error that no handler
catches, whatever the chain holds.

*Alternative:* number per statement as the old compiler does. That would change the numbering of every program's
`mainerr.txt` for no change in behaviour.

### D2. A log statement is an operation with its place

`STATEMENTS` gets the four names with a `StmtRule::Log(level)` (`builtins.rs`). `check\builtins.rs` checks the
one argument as a string, which gives the old compiler's error for a number (the message is measured in `v50`).
The statement becomes `StmtKind::Log { level, msg }` and then `Op::Log { level, msg }`. The emitter writes
`sub__log<level>(file, func, line, msg);` (`stmt.rs`), with arguments from what it already holds:

- *file*: the file as the old compiler names it, as a C string escaped as its `escapeString$` does
  (`names::escape_string`: a backslash doubled, where `c_string` writes `\134`), so that the call-site check sees
  the same literal. `v50_b_log.log.txt` shows a full path: the old
  compiler prefixes the name given on the command line with `_STARTDIR$` when that file exists there
  (`qb64pe.bas` 413), and names an included file by its includer's folder and the path as written
  (`getfilepath$(…) + a$`, 12544, so `<verification>\v50_inc/log.bi`). The shown names our loader gives
  included files already follow that rule from the main file's shown name (`FileLoader::load`: the includer's
  shown folder joined with the path). So the emitter takes one more input, the *log folder*: the working
  directory with a trailing `\`, put before every shown name that is relative. The driver passes it only for a
  build (`main.rs`); `--dump cpp` passes none, so the `cpp` tests do not depend on the working directory. A file
  found in the compiler root is named by its shown name (the old compiler names it by the path as written); no
  test logs from one.
- *func*: the old compiler's C name of the body's procedure (`subfunc$`, `qb64pe.bas` 5416): `SUB_S`, `FUNC_F`
  for `FUNCTION f$`, `SUB_A__ASCII_CHR_046__B` for `SUB a.b`, or `Main QB64 Code` in the main module. It spells a
  `.` as `__ASCII_CHR_046__` (`fix046$`, 476) where our C names write `__046__`, so the emitter writes it with
  `names::old_proc_name`; our functions keep their names. libqb shows a C name as `S (QB64)`.
- *line*: the statement's line in its own file (`Stmt::line`; the old compiler's `inclinenumber` in an included
  file).

The four table rows have no template, so a log statement comes to `sema` written like a SUB call
(`check\proc.rs`), which takes a `StmtRule::Log` row as it takes a plain one; `stmt_call` then pushes
`StmtKind::Log`.

`_LOGMINLEVEL` is a `Rule::Plain` row (`func__logminlevel`); with the runner's log handler set it is 2. QB64pe's
row sets `ideret` where it means `id.ret` (`subs_functions.bas` 4383), so the table gives it no type; `sema`
takes LONG, the type the row means, and the call-site program `func_logminlevel` compares equal. The old
compiler's error for `_LOGINFO 5` is "Illegal string-number conversion", for `_LOGINFO` alone "Syntax error",
and for two arguments "Invalid expression"; `_LOGINFO ("a")` is accepted. `_LOGINFO` as a function and
`_LOGMINLEVEL()` are rejected.

An `_LOGERROR` entry carries stack frames. libqb names each frame by its C symbol (`#1  in S() (QB64)`, `#2  in
Main QB64 code`), so a procedure's C name and signature show in the log. `v50_b_log` logs errors only from the
main module and from a SUB without parameters, whose name and signature `()` are the same in both compilers; an
`_LOGERROR` in a procedure with a dot in its name, or with parameters our signature passes otherwise, would show
our symbol there.

### D3. `$CHECKING` is read everywhere, and the flag belongs to each statement and label

`check\mod.rs` accepts `$CHECKING:OFF` and `:ON` in any file. `line` is already upper case, so mixed case is
taken. `v50` shows which forms the old compiler rejects, each with "Syntax error": `$CHECKING`, `$CHECKING:`,
`$CHECKING:MAYBE`, `$CHECKING: OFF` and `$CHECKING:OFF 'note`; after a colon (`PRINT "a": $CHECKING:OFF`) it is
"Unexpected character on line". It accepts lower case, a trailing blank, OFF or ON twice, and `$CHECKING:OFF`
inside an `IF` block (metacommands are read before blocks). `sema::Stmt` gets `checked: bool`; a label's comes
with its `StmtKind::Label` statement. `push` (`check\mod.rs`) takes it from `self.at.checking`. A block's
statement is pushed after its body, so `block_statement` (`check\blocks.rs`) notes the state at the header and
gives it to what the block pushed at its own level; the statements inside keep their own. `Proc::checked` goes
away in both crates. A procedure's statements carry the state where each stands, which is what the old compiler does,
since it writes bodies in source order (`source_ordering/checking_off`). `afterlastline.bm` keeps working, since its
`$CHECKING:OFF` now reaches its statements one by one.

`ir::Stmt` and `ir::Label` get `checked`. The lowering copies it (as it does `include_line`), and every statement
it makes for a block takes the block's. `procs.rs` `body` and `labels` test the statement's or label's flag in place of the body's.

### D4. What the emitter leaves out below OFF

The emitter holds `checking: bool`, set from each statement in `body` before its operations are written:

- `place.rs`: `flat_index`, `member_flat_index` and `dyn_element_desc` write `(i)-lower` in place of
  `array_check((i)-lower,count)`, with the multiplier unchanged (`qb64pe.bas` 15627–15640, 16138–16150,
  16337–16349). The guard of a dynamic member array stays (Context).
- `mem.rs`: `mem_read` writes the offset itself in place of `func__memget(…)`. `mem_get_stmt` and `mem_put_stmt`
  write the copy through the offset with no `tmp_long` and no `mem_checked`: `*(int32*)addr=*(int32*)(off);`,
  `memmove(addr,(uint8_t*)off,size);`, `*(t*)(off)=v;` (`qb64pe.bas` 10228–10236, 10328–10336, 10376–10378).
  `mem_fill_stmt` calls `sub__memfill_nochecks(off,count,(ptrszint)addr,size)` and
  `sub__memfill_nochecks_<T>(off,count,v)`, without the block (10465–10497). A string value keeps its
  `g_tmp_str=` or `tmp_long=` line before the copy, as there.

- `stmt.rs` `member_path`: a store through a member with an index on the way checks no index (the old
  compiler's `((i)-(lower))*stride`); the descriptor test and the guard of a dynamic member array stay.

The `is_error_pending()` tests of block conditions, the `SUB` entry guard and `_ARRAYCOPY` are written whatever
`CheckingOn` is (`qb64pe.bas` 5822, 6301–6403, 6725, 17320). Below OFF an error stays pending across statements,
which with checking on it never does past a boundary, so the IR's rules for a pending error meet code they did not
meet before. They agree with the old compiler's C++ (`IF`, `WHILE`, `DO WHILE` and `CASE` enter their body, `LOOP
UNTIL` leaves, the lowering's own jumps are plain `goto`s as its `else` and `}while(1)` are), except at the exit
test of `FOR`: the old compiler writes `if (value>final) break;` with no test of a pending error, where the IR's
`Branch` is `Skip` and would never leave the loop. The lowering makes that branch `UseValue` below OFF
(`v50_c_checking`). With checking on no error is pending there, so it stays `Skip`, and no checked program's C++
changes.

### D5. A statement below OFF may not jump across a C++ initialiser

Below OFF a statement's lines are no longer inside `do{…}`, so a C++ declaration with an initialiser among them
would stand in the function's own scope. A `goto` across it (to a label, to `exit_subfunc`) would then be a C++
error, as the note at `qb64pe.bas` 15973 says. Our emitter already moves the declarations it makes into the data
fragment (`pass_decls`, `data{k}.txt`), and an unchecked body already builds for `afterlastline.bm`. Where a
statement still declares something inline, its lines get a plain `{…}` block below OFF. That block only scopes the
declaration and writes no call, so the call-site check is unaffected. The `v50` programs build every kind of
statement below OFF to find any such case (task 3.3). They found none: a store through a member, the one
statement that declares variables of its own, already writes them in braces (`stmt.rs` `store_through_member`),
so no block is added.

### D6. The log in the verification suite

`run_legacy_tests.py`, in the verification suite's run: the corpus and compile suites already set
`QB64PE_LOG_HANDLERS=file` and the log's path; the verification suite did not, and now does in both its recording
and its compare (`log_env`, `results/<name>-log.txt`, removed first: libqb appends). New code reads that file after the run and keeps the lines of
scope `QB64`, with the stack-frame lines that follow an entry. On each entry line it removes the leading
`[seconds]`, on each frame line the `[0x…]`, and it masks the run folder as `<verification>`. When recording, it
writes the result to `<name>.log.txt` if it is not empty, and removes the file if it is. When comparing, it holds
the result to the recorded file, or to nothing. A difference is the stage "log". The pass cache's key gains the
`.log.txt`. `test_run_legacy_tests.py` tests each step with a stand-in program that writes a log.

*Alternative:* a list of the programs whose log counts. That needs a list to keep, and a program that logs by
mistake would go unseen.

### D7. The measurements, `v50`

Recorded in task 1.2, before the code:

- `v50_a_chain`: the chain (two and three deep, `_LASTHANDLER` with nothing kept, `ON ERROR GOTO 0` then
  `_LASTHANDLER`, `_NEWHANDLER` in a SUB).
- `v50_b_log`: the four levels and `_LOGMINLEVEL`, in the main module, a SUB, a FUNCTION and an included file
  (`v50_inc\`).
- `v50_c_checking`: checking off and on around statements, a block, a label, a SUB and an included file, with an
  error raised below OFF, a handler, `RESUME`, `RESUME NEXT`, and an `ON TIMER` that comes due below OFF.
- `v50_d_mem`: `_MEMGET` (both forms), `_MEMPUT` and `_MEMFILL` of every kind of value, below OFF. `_FLOAT` is
  left out: the old compiler's build of `_MEMPUT` and `_MEMGET` of a `_FLOAT` crashed with checking on as well.
- `v50_e_index`: array, member-array and dynamic member-array elements read and written below OFF, within the
  bounds.
- `v50_x01`–`x26`: one form each that the old compiler may reject: the chain's errors and forms, the log
  statements' and `_LOGMINLEVEL`'s, and the forms of `$CHECKING` (D1, D2, D3 name what each shows).

What `v50_c_checking` shows of an error raised below OFF: libqb's own routines do nothing while an error is
pending (`PRINT` prints nothing), the error is serviced at the next checked statement, `RESUME NEXT` goes on after
that statement, and `RESUME` runs that statement again, not the one that raised the error. An `ON TIMER` that
comes due below OFF runs after the first checked statement.

## Risks / Trade-offs

- [The stack frames of an `_LOGERROR` entry may differ between the builds: other inlining, other frames] → the
  frames are compared after the address is removed. If they still differ for a reason of the build and not of the
  program, D6 keeps only the frames' names, with a line in the design that says why.
- [An error raised below OFF is serviced at the next checked statement, and `RESUME` there retries that statement,
  not the one that raised it] → that is the old compiler's behaviour, measured in `v50_c`; the spec points there,
  and `SOMEDAY.md` lists it under "QB64pe behaviours to review".
- [The emitter's `checking` field is left from the last statement when an operation is written outside `body`
  (the epilogue, the data fragment)] → `body` sets it for each statement and restores `true` after the body, and
  the `v50` programs cover a SUB below OFF called from checked code.
