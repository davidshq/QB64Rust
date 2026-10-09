# Tasks

Order of the groups: 1 (measure before designing), 2 (the tester, recorded before any typing rule is written), 3
(the `Ty` refactor without behaviour change), then 4 to 9 in order, then 10. Corrections that a measurement makes to
the design or the spec deltas are listed in the task that found them. Each group regenerates the shrink-only lists
(`QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test inputs`, entries may only go away), adds the
programs it makes pass to `slice.list`, `tests\upstream\pass.list` and `tests\differential\pass.list`, and keeps
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`, tier 1 and tier 2 green.

## 1. Measurements before code (design D1)

- [x] 1.1 Write and run `verification\v21_a_*` (declarations, `AS` spellings, suffixes on variables, FUNCTION names
  and literals, out-of-range literal suffixes, the `_BIT * n` limit, implicit variables with the new suffixes, `LEN`
  of each type) with `qb64pe.exe` (`verification\run.sh`). Verify: output recorded beside each program; findings in
  `study\00` §5.
  *Done 2026-10-08:* `v21_a_decls`, `_suffixes`, `_literals`, `_functions`; rejections `v21_x01`–`x19`. First runs
  showed the D-009 overlap in lines with two wide `_BIT` scalars; those are now printed one at a time.
- [x] 1.2 `verification\v21_b_*`: `_BIT` stores (mask, sign extension, float rounding into 17 to 31 bits), `_BIT`
  members and parameters, passing by reference across signedness and across `_BIT` widths. Verify: outputs
  recorded; findings in `study\00` §5.
  *Done 2026-10-08:* `v21_b_bit_stores`, `_float_stores`, `_passing`, `_bit_scopes`, `_bit_overlap`; rejections
  `v21_x20`–`x30`. Across `_BIT` widths cannot be measured: every `_BIT` parameter fails the old compiler's C++
  build (`x20`–`x24`).
- [x] 1.3 `verification\v21_c_*`: fixed-length strings (`STRING * 0`, the length limit, any suffix form, assignment,
  comparison, `LEN`, `ASC`/`MID$`/`INSTR`, members, arrays, a fixed string passed to a `STRING` parameter, a parameter
  declared `STRING * n`). Verify: outputs recorded; findings in `study\00` §5.
  *Done 2026-10-08:* `v21_c_fixed_basics`, `_fixed_args`, `_fixed_len_2147483647`, `_2147483648`, `_4294967297`;
  rejections `v21_x31`–`x34`.
- [x] 1.4 `verification\v21_d_*`: `FOR` with each new type as variable, `SELECT CASE` with each as selector, `CONST`
  with each new suffix, and the special-cased built-ins (`LEN`, `STR$`, `HEX$`, `OCT$`, `_BIN$`, `ABS`, `SGN`,
  `VAL(…, type)`, `CINT`, `CLNG`, `_ROUND`, `INT`, `FIX`) with each new type. Also read the C++ (`qb64pe -z`) for the
  types of the generated temporaries. Verify: outputs recorded; findings in `study\00` §5.
  *Done 2026-10-08:* `v21_d_for`, `_select`, `_const`, `_builtins`; rejections `v21_x35`, `x36`; C++ read for the
  `FOR` copies, the `SELECT` copies and items, `SQR`/`EXP`/`VAL`/`_ROUND`.
- [x] 1.5 Correct the spec deltas and D3–D7 where 1.1–1.4 disagree with them, and list each correction here. Put any
  measured behaviour that looks like wrong code in the old compiler to the user before it is designed (record the
  answer in `DECISIONS.md`). Verify: `openspec validate m2-numeric-types --strict` passes;
  `python tools\repo_check\check_repo.py --untracked` clean.
  *Done 2026-10-08.* Put to the user and decided (`DECISIONS.md`): out-of-range suffixed literals and `CONST`s,
  `_BIT`-suffixed literals and constants, and `STRING * n` parameters are "not supported yet"; `_BIT` parameters, a
  `STRING * n` whose n becomes 0 or negative in 32 bits and a literal beyond 64 bits are compile errors, as QB64pe
  fails to build them; n wraps to 32 bits as in QB64pe (later improvements in `SOMEDAY.md`). Every scenario of the
  deltas that the old compiler can run is in `v21_e_scenarios` and gives the stated output. Corrections:
  - `numeric-semantics`: a `_BIT` store from a float is rounded by `qbr` from `_FLOAT` for every width (was:
    SINGLE up to 16 bits, DOUBLE for 17 to 31; D5 likewise); unsigned targets of 16 bits or fewer round from SINGLE
    (scenario added). `_BIT * n` n is 1 to 64, a literal; one name per suffix is its own variable; `_UNSIGNED
    STRING` means `STRING`; `LEN` of a `_BIT` an error; a `_BIT` value reads as `_INTEGER64` (an `_UNSIGNED _BIT *
    64` above 2^63 prints negative); a `_BIT * n` up to 32 counts as its 32-bit storage in arithmetic. Literals:
    the decision above replaces "converted as the old compiler converts it"; radix literals read with the type's
    signedness; `%&`, `~%&` after a number and an integer suffix on a float literal are errors.
  - `constants`: a suffixed `CONST` out of its type's range, and a `_BIT` suffix, are "not supported yet"; the
    scenario `CONST u~% = -1` becomes `CONST u~% = 65535` plus an out-of-range scenario.
  - `arrays-and-types`: a `_BIT` member is a compile error (was "not supported yet").
  - `procedures`: a `_BIT` parameter is a compile error (was "as measured"); `STRING * n` parameters "not supported
    yet"; `f$n` FUNCTIONs return n bytes, as measured (first "not supported yet", changed by the user's rule of
    2026-10-08 to follow QB64pe, `DECISIONS.md`); a bare `` ` `` FUNCTION cannot be called; by reference also between the 64-bit
    integer and `_OFFSET` types; a `_BIT` variable is always passed as a copy.
  - `control-flow`: a `_BIT` `FOR` variable is a compile error (was: widened to `_INTEGER64`); `SELECT CASE`
    modified: an integer item is not converted to the selector's type (C++ compares them), shown by unsigned
    selectors; a count-down through 0 scenario for an unsigned `FOR`.
  - `fixed-length-strings`: n is a literal or a constant name, read as 32 bits; suffix form `name$n`; `""` stores
    n blanks; `MID$` statement; a fixed string passed to a `STRING` parameter is by reference (in parentheses, as
    member and element too), cut and padded (scenario pinned to `longe`, was "equals the old compiler's").
  - `builtin-functions`: `VAL` with an unsigned type is libqb's unsigned parse (minus dropped); with `_BIT` an error.
  - `differential-tests`: a pad before each wide `_BIT` (D-009), no `^` with `_OFFSET`, no unsupported literals.
  - Design: D3 and D4 (`_BIT` believed `_INTEGER64`, held as its storage), D5 (rounding), D6 (arguments, n), D7
    (literals), D2 (generator rules); `proposal.md` (`SELECT CASE` now has a delta, the generator is
    `crates\difftest`, the new "not supported yet" forms). `DIVERGENCES.md` D-009 corrected: the victim is the `_BIT`
    scalar allocated before the wide one, of any width.
- [x] 1.6 Bring the forms 1.5 left "not supported yet" into the change, as QB64pe does them (user, 2026-10-08,
  `DECISIONS.md`): measure where QB64pe converts an out-of-range or `_BIT`-suffixed literal or a suffixed `CONST`,
  how a wide radix literal behaves, and what a `STRING * n` parameter does beyond `LEN`. Verify: outputs recorded;
  findings in `study\00` §5; deltas and design corrected; `openspec validate m2-numeric-types --strict` passes.
  *Done 2026-10-08:* `v21_f_literal_uses`, `v21_f_radix`, `v21_f_fixed_param`; rejections `v21_x37`–`x41`.
  Corrections: `numeric-semantics` (a suffixed literal is held as written, in the C++ type of its text, and believed
  the suffix's type; converted by `PRINT`, `STR$`, `ABS`, nowhere else; `HEX$` sees the held value; a signed radix
  literal wider than its type is a compile error, an unsigned one holds its whole value; scenarios replaced),
  `constants` (an integer or `_BIT` suffix makes the constant such a literal), `procedures` and
  `fixed-length-strings` (a `STRING * n` parameter is a `STRING` parameter whose `LEN` is n), `differential-tests`
  (only the wide signed radix literal is left out), design D2, D6, D7 (also in range: a `U32`-held `4294967295~&`
  would make `+ 1` wrap, measured 4294967296), `proposal.md`.

## 2. The differential tester (design D2)

- [x] 2.1 Crate `crates\difftest` (binary `qb64rust-difftest`, `gen [--check]`), seeded xorshift, the type list and
  boundary values per type from `sema::Ty`; documented in `crates\README.md`. Verify: `cargo run -p
  qb64rust-difftest -- gen` twice writes identical files; `cargo clippy` clean.
  *Done 2026-10-08.* `sema::Ty` has only the six old numeric types until group 3, so the type list is the crate's
  own (`TYPES`, 17 types: four `_BIT` widths, 1, 7 unsigned, 24, 40 unsigned, stand for `_BIT * n`) and
  `for_sema` maps every `Ty` variant with an exhaustive match: a new variant fails the crate's build until it is
  mapped (group 3 adds the mapping). A second `gen` writes nothing; `gen --check` reports a missing, changed or
  extra program. Added for task 2.6: `gen --out <dir> --types <KEY,...>` writes the programs for some types only,
  into another folder (never `tests\differential`).
- [x] 2.2 The program groups of D2 (`ops`, `unary`, `store`, `fold`, `print`): labelled lines, the `ON ERROR`
  handler, the D-006 and division-by-zero exclusions named in each header, no `PRINT` comma;
  `tests\differential\README.md` (what each group covers, how to regenerate, record and run). Verify: a unit test in the crate checks
  that every ordered pair of numeric types appears in each `ops` program and every pair source/target in `store`.
  *Done 2026-10-08:* 59 programs (20 `ops`, 20 `fold`, 17 `store`, `unary`, `print`; 5 MB; about 2,300 `PRINT`
  lines in an `ops` program), more than D2's estimate of 45 because `fold` has a program per operator and `store`
  one per target. Unit tests: determinism, every pair (`^` has none with an `_OFFSET` operand, named in its header),
  safety (no `PRINT` comma, a pad per wide `_BIT`, the exclusions named), literals. Each line's value pair is a pair
  of slot variables DIMmed once (D2's "one `DIM` per numeric type and value slot"); `fold` adds two pairs one step
  beyond the range. `SYSTEM` ends each program, so no key press is needed.
- [x] 2.3 Record with the old compiler: `run_legacy_tests.py --suite corpus --corpus-root tests\differential
  --record` against `qb64pe.exe`. Any program that does not compile with the old compiler, crashes or is not
  deterministic is fixed in the generator: the case is left out and named in the program's header (a crash or failed
  build is fixed by `study\00` §6's rule and gets a `DIVERGENCES.md` row). Verify: every program has a
  `.output`; a second `--record` changes no file; `check_repo.py --untracked` clean
  (no local paths in the outputs).
  *Done 2026-10-08:* all 59 compile, run to their last line (checked for each program) and agree between the two
  runs: 59 `.output`, no `.err`; 211 s. No exclusion beyond D2's was needed. A second `--record` wrote nothing;
  `check_repo.py --untracked` clean. Found: a `_FLOAT` literal is a C++ double in the old compiler, so the first
  `_FLOAT` extremes (`±1.18973149535723176F+4932`) were infinite; they are now `±1.797693134862315F+308` and the
  infinite literal is `_FLOAT`'s step beyond its range in `fold` and `print` (recorded again, twice). The outputs
  hold D-010's infinite text with NUL bytes (`tests\differential\README.md`); `.gitattributes` marks them `-text`.
  Facts in `study\00` §5.
- [x] 2.4 Tier 1: the crate's freshness test (regenerate in memory, compare with the files, require a `.output` per
  program) and `crates\driver\tests\inputs.rs` walking `tests\differential` (front end, round trip; the "clean
  programs are listed" check accepting `tests\differential\pass.list`). Verify: `cargo test` passes; editing one
  generated line by hand makes it fail and name the file (tried by hand, not committed).
  *Done 2026-10-08:* `crates\difftest\tests\fresh.rs`; `inputs.rs` gains the set `differential/` (also checked by
  the false-error and parse-gap lists: no entry, every error is "not supported yet"). `cargo test` passes. Tried by
  hand: an edited line in `ops\add.bas` fails with "differs from the generator: ops/add.bas"; a moved
  `unary.output` fails with "no recording … unary/unary.bas"; both restored.
- [x] 2.5 CI: a step in `rust.yml`'s `tier2` job running `tests\differential\pass.list` with `qb64rust`; time per
  program measured and the total noted in `study\19` §6. Verify: the step runs locally with the CI command line
  against the QB64pe 4.7.0 release; it passes with an empty or partial list.
  *Done 2026-10-08:* step "Differential pass list". The runner now passes a `--list` that names no program
  (it stopped with "no tests selected" before). Run locally with the CI command line against the downloaded
  `v4.7.0-GLFW` release: the empty list runs nothing and exits 0; the six-type subset of 2.6 against the same
  release: 27 of 48 pass, as against the clone. About 2.1 s per program (`study\19` §6).
- [x] 2.6 Run every differential program once with today's `qb64rust` (expected: those touching only the six old
  types pass, the rest are "not supported yet"); start `pass.list` with the passing ones. Answer the design's open
  question (are eight value pairs enough?) from the old compiler's recorded output. Verify: the run's counts and
  the answer are recorded here.
  *Done 2026-10-08.* **0 of 59 compile**: every program declares all 17 types, so each stops at the first new-type
  suffix, with "not supported yet" errors only (no false error, no wrong code); `pass.list` starts empty. The
  expectation above assumed programs split by type; to check today's rules anyway, the six-type subset (`gen --out
  <scratch> --types INTEGER,LONG,INT64,SINGLE,DOUBLE,FLOAT`, 48 programs) was recorded with `qb64pe.exe` and run
  (not committed): **27 of 48 pass** (all 20 `ops`, `unary`, the 6 `store`); the 20 `fold` and `print` stop at
  "overflow" for the 48 literals beyond INTEGER and LONG (`32768%`, `-2147483649&`; design D7 removes that error in
  group 4). With those lines dropped, `print` and 17 `fold` programs agree; **14 lines differ** in `fold\add`,
  `sub`, `mul`: `-2147483648&` is held as a 64-bit C++ literal by the old compiler, so `(-2147483648&) *
  (-32768%)` is 70368744177664 where today's compiler wraps to 0 (D7's held type, group 4). **Answer: eight pairs
  are enough for now.** Each C++ regime with mixed signedness shows in at least two of the eight pairs of every
  type pair (`< BYTE UBYTE 5` is true by `int` promotion, `< LONG ULONG 5` false by unsigned conversion, as min/min
  and the random negatives), and every difference found came from the extreme pairs; the random pairs found nothing
  the fixed ones did not. Revisit if group 5 finds a rule the pairs miss.
- [x] 2.7 (added after review, `DECISIONS.md` 2026-10-08) The six-type subset as a recorded group, so tier 2 guards
  today's types while the full programs cannot compile: `generate` also writes `old6\<group>_<name>.bas` for the
  six old types, recorded with `qb64pe.exe`; the passing ones on `pass.list`. Verify: tier 1 fresh and recorded;
  the pass list passes with the CI command line.
  *Done 2026-10-08:* 48 programs, recorded in 132 s, all run to their last line. **27 of 48 pass** (as in 2.6) and
  are on `pass.list`: 52 s in tier 2. The 20 `fold` programs and `print` get "overflow" for `32768%` and the like,
  which the old compiler accepts: in `tests\known_false_errors.list` until group 4 (D7). Also from the review:
  the runner passes only a list that names nothing (not one whose programs `--glob` or `--category` leave out);
  `gen --types` refuses to write into `tests\differential`; the D-006 rule leaves out floats from ±2147483647.5
  (rounded half to even they may be the smallest LONG; no program changed).

## 3. `Ty` without a derived order (design D3)

- [x] 3.1 Add the new variants and the methods `int_bits`, `is_signed`, `is_unsigned`, `float_rank`, `storage`; drop
  `PartialOrd`/`Ord`; rewrite the uses of the order (`check\ops.rs`, `check\blocks.rs` `FOR` temp, `ir\src\dump.rs`)
  with the methods; every `match` lists the new variants, which nothing produces yet (`unreachable!` with a reason,
  or the declaration's "not supported yet"). Verify: no `typed`, `ir` or `cpp` snapshot changes
  (`cargo insta test` reports nothing); tier 2 slice, upstream and differential pass lists unchanged; unit tests
  for each method.
  *Done 2026-10-09 (session 29).* The variants as design D3; `is_int`, `is_float`, `is_numeric` now derive from
  `int_bits`/`float_rank`. One more method, `is_wider_than`, replaces `>` on the order everywhere. The order had
  more uses than listed: besides `ops.rs` (now `wider`, the old `max`, and `is_wider_than` for the comparison's
  narrower float) and `ir\src\dump.rs` (`is_int`), the `CASE` item type in `blocks.rs` (not the `FOR` temp, which
  never used it; now `ops::promote` and `wider`), `convert_exact` in `check\expr.rs` and `convert` in
  `ir\src\lower.rs` (by `is_wider_than`). `wider` asserts on two integer
  types of one width (that is design D4's `held`, task 5.1). The arms for the new variants are one pattern macro,
  `sema::unproduced_types!()` (re-exported by `ir`), with `unreachable!(NEW_TYPE_UNREACHABLE)`; a variant leaves
  the macro when its declaration is supported, which makes every `match` decide (groups 4–5). `difftest`'s
  `for_sema` maps every variant (its test now covers all 17 generator types); the typed-tree dump names them; the codegen
  `INT`/`FIX` arm no longer ends in `(_, _)`. Verified: `cargo test` (`cargo-insta` not installed; no `.snap.new`,
  no snapshot changed), clippy `-D warnings`, unit tests per method and for `wider`; tier 2 against the reference
  clone: slice 192 of 192, upstream 42 of 42, differential 27 of 27.

## 4. Declarations and literals (design D7)

- [x] 4.1 `AS` spellings and type suffixes of every new numeric type for variables, implicit variables, `DIM`,
  `DIM SHARED`, `STATIC`, `SHARED`, FUNCTION names and parameters, as measured in 1.1; `LEN` of each (`size_of`).
  Verify: `parse-ok`/`check-ok`/`check-fail` front-end tests per form; `typed` snapshot of a declaration of each
  type; the `_MEM` declaration still "not supported yet" (pipeline delta scenarios as front-end tests).
  *Done 2026-10-09 (session 30).* **The gate:** declaring a variable of a new type does not make its values
  computable (that is group 5), so the checker lets every declaration through but reports "not supported yet" for a
  *value* of a new type: a load, a store, a literal or constant held or believed in one, a FUNCTION result, a `FOR`
  variable (`Checker::gate`, `Ty::is_gated`). `LEN` of such a variable is its size, not a value, and passes. So the
  IR and the emitter never meet a value of these types. The design's `unproduced_types!()` is split: it keeps
  `FixedStr` only; the new numeric types form `gated_types!()`, the arm of every `match` on a value's type; the
  `match`es on a declaration's type (`size_of`, `type_name`, the C types and names, declare, allocate, clear) handle
  them now. A type leaves `gated_types!()` when its rules exist (groups 5, 6). The suffix table is
  `literal::suffix_type` (names and numbers), the `AS` words `check\decl.rs` `type_of` (`_UNSIGNED` before an
  integer type, `_OFFSET`, `_BIT`, ignored before `STRING`; `_BIT * n` with n a number from 1 to 64). Rejected as
  measured, each an error: `_UNSIGNED SINGLE`, `AS _UNSIGNED`, `_UNSIGNED` before a user type (`v21_x46`), `_BIT *
  0`/`65`/a constant, `` k`65 ``, `` k`0 ``, `LEN` of a `_BIT`, a `_BIT` member, parameter or `FOR` variable, `VAL(…,
  _BIT)`, any call of a `_BIT` FUNCTION, with arguments or bare ("Name already in use"; declared and never called it
  compiles: `v21_x16`, `x17`, `x47`, `x48`) (the last five ahead of groups 6 and 8, being errors). Still "not
  supported yet": arrays and members of the new types (task 8.1), `VAL` with them (8.4).
  Emitted names are the old compiler's (`__UBYTE_B`, `__BIT7_J`, `qb64pe.bas` 25797–25819); a `_BIT * n` scalar
  has storage of its own of its `storage()` size (D-009). Tests: `numeric_decls` (`typed`; the declarations stand in
  a SUB because the dump lists a procedure's variables with their types, not the main module's), `numeric_decls_cpp`,
  `numeric_decl_errors`, `numeric_gate` (`check-fail`), `mem_follow_on` (the pipeline delta's four scenarios);
  `explicit_after_unsupported` now uses `_MEM` as its unsupported declaration. `numeric_decls` and
  `numeric_decls_cpp` built and run with both compilers give the same output.
- [x] 4.2 Literal suffixes of the new types in `sema\src\literal.rs` (held and believed types as design D7, for
  every integer suffix, in range or not, `_BIT` suffixes included; radix literals; today's "overflow" for `40000%`
  removed) and suffixed `CONST`s in `check\constants.rs`. Verify: unit tests per suffix, in range and beyond;
  `typed` snapshot showing `ty` and `qb` of `300~%%`, `-1~&&`, `` 9`3 ``; `check-fail` test for `&H1FF%%`; the
  `print` and `fold` differential programs' literal lines pass when the programs compile (group 5).
  *Done 2026-10-09 (session 30).* `NumLit::Int` carries `ty` (held) and `qb` (believed); `literal::held_type` is
  D7's rule, with one simplification: an INTEGER literal in INTEGER's range stays held as INTEGER (C++ widens it in
  every operation, so nothing changes; no snapshot moved). A suffixed `CONST` with an integer or `_BIT` suffix keeps
  its value unchecked (`consteval::settle`) and is used as `literal::constant_literal` of it (`&&` and plain
  constants stay `_INTEGER64` literals as before). Unit tests: `literal.rs` (every suffix, held and believed types in
  range and beyond, radix literals, constants, ranges), `consteval.rs`. **The `typed` snapshot of `300~%%`, `-1~&&`
  and `` 9`3 `` moves to task 5.1:** their believed types are new types, so the gate stops them; the unit tests pin
  their two types, and `const_suffixed` (`typed`) shows the same split for the old types (`32768%` held LONG,
  believed INTEGER; `-2147483648&` held `_INTEGER64`) with output equal to `qb64pe.exe`'s. `check-fail` for
  `&H1FF%%`, `5%&`, `18446744073709551616~&&`: `numeric_decl_errors`. **The `old6` group's 20 `fold` programs and
  `print` now pass** (all 48 of `old6` on `pass.list`), the 14 wrong lines of task 2.6 included; upstream
  `const/expression` passes (43 of 279) and `const/offset` gets its real error. One literal moved the other way:
  `-9223372036854775808&&` is held `_UNSIGNED _INTEGER64` (C++ types `9223372036854775808ll` so), which the gate
  stops until group 5 (`literal_overflow`; brought back by task 5.1). The lexer now reads `name$n` as one token (needed by 4.3; `$` followed
  by digits after a number stays as it was).
- [x] 4.3 `STRING * n` and `t$n` parameters (design D6): a `Str` parameter whose `LEN` folds to n. Verify: `typed`
  snapshot of `LEN(t)` in such a SUB; the procedures delta's scenario as a line of `s34_types_procs`.
  *Done 2026-10-09 (session 30)* except the `s34` line, which comes with the program in task 8.5. `t$n` is a name of
  its own: in the procedure it names the parameter (`Checker::fixed_params`); anywhere else `name$n` is a
  fixed-length string, "not supported yet" until group 7. n is read in 32 bits; a constant's name as n is "not
  supported yet" (parameters are read in pass 1, before any `CONST`). Measured on the way (`verification\v21_x42`–
  `x45`): `_UNSIGNED STRING`, with a length or not, is "Illegal SUB/FUNCTION parameter" (though `DIM` takes it);
  `t$0` and `STRING * 4294967296` (0 in 32 bits) are errors on a parameter too. `fixed_param` (`typed`, the
  measured program's forms) gives the same output with both compilers.

## 5. Typing, conversions and emission (design D4, D5)

- [ ] 5.1 `held` and `believed` in `check\ops.rs` and the `_OFFSET` arm of `op_typing`; comparisons in the held
  type. Verify: unit tests for each rule in `ops.rs`; `typed` snapshots for a mixed-signedness example of each
  operator family; `-9223372036854775808&&` compiles again (held `_UNSIGNED _INTEGER64` since 4.2, gated until
  here): `literal_overflow` loses its "not supported yet" line and a `typed` test shows the literal's two types.
- [ ] 5.2 `conversion(from, to)` for every pair, the rounding helpers by target width, and the emitter's C types and
  names for the new types (`names.rs`, `decl.rs`, `value.rs`). Verify: `cpp` snapshots of stores and operations
  per type family; the `store` and `print` differential programs pass in tier 2 and join `pass.list`.
- [ ] 5.3 Iterate until the `ops`, `unary` and `fold` programs pass: each difference is fixed in the rules (5.1,
  5.2); an old-compiler accident is reproduced and listed in `SOMEDAY.md` "QB64pe behaviours to review". Record each rule correction against
  `study\02` here and in `study\00` §5. Verify: all `ops`, `unary`, `fold` programs in `pass.list` and passing in
  tier 2; slice program `s31_unsigned_ops` (the corrected cases as readable examples) recorded with `qb64pe.exe`
  and passing; `tests\corpus\slice\SOURCE.md` updated.
- [ ] 5.4 `s30_new_types` (declarations, suffixes, literals, `LEN`, the numeric-semantics delta's scenarios)
  recorded and passing. Verify: tier 2 slice run; the scenarios of `language/numeric-semantics` that the old
  compiler agrees with are lines of `s30` or `s31`.

## 6. `_BIT` (design D5)

- [ ] 6.1 `_BIT` and `_BIT * n` scalars: storage by `storage()` (D-009), the store mask and sign extension in the
  emitter, `_BIT` members, parameters and arrays as measured in 1.2 (arrays "not supported yet"). Verify: `cpp`
  snapshot of the masked stores; slice program `s32_bit` recorded and passing; CLI build-and-run test for D-009
  (two `_BIT * 33` scalars, `#[ignore]`d without the clone); `DIVERGENCES.md` D-009's "Pinned by" filled in.

## 7. Fixed-length strings (design D6)

- [ ] 7.1 `Ty::FixedStr(n)` for variables (every storage class), with NUL-filled allocation, assignment through
  `qbs_set`, loads as `Str`, `LEN`, and the forms of 1.3. Verify: `typed` and `cpp` snapshots; front-end tests for
  `STRING * 0` and the other measured rejections; the fixed-length-strings delta's declaration and assignment
  scenarios as lines of `s33_fixed_strings`.
- [ ] 7.2 Fixed-length string members and array elements (temporary descriptors over the bytes, offsets with
  `size_of`), and fixed strings as built-in and procedure arguments as measured in 1.3 (unmeasured forms "not
  supported yet"). Verify: `s33_fixed_strings` recorded with `qb64pe.exe` and passing in tier 2;
  `DIVERGENCES-QB45.md` Q-005's "Pinned by" filled in; the follow-on scenarios still hold with `_MEM`.

## 8. The new types everywhere else

- [ ] 8.1 Static arrays of the new numeric types and of fixed-length strings; `TYPE` members of the new numeric
  types (not `_BIT`). Verify: `typed`/`cpp` snapshots; the arrays-and-types delta's scenarios as lines of
  `s34_types_procs`.
- [ ] 8.2 Parameters and FUNCTION results of the new types, including `f$n` FUNCTIONs (a `FixedStr(n)` result,
  design D6); by reference across signedness, as measured in 1.2. Verify: the procedures delta's scenarios as lines
  of `s34_types_procs`; `typed` snapshot of a cross-signedness
  `Arg::Ref`.
- [ ] 8.3 `FOR` with each new type (the widened hidden type by width), `SELECT CASE` selectors of each new type (the
  hidden copy's type as measured in 1.4), `CONST` with each new suffix. Verify: the control-flow and constants
  deltas' scenarios as lines of `s34_types_procs`; `typed` snapshots.
- [ ] 8.4 Arguments of the new types to the 43 supported built-ins (slot conversion; the special cases of 1.4).
  Verify: the builtin-functions delta's scenario as a line of `s34_types_procs`; unit tests for the special
  cases; the built-in coverage check still passes.
- [ ] 8.5 Record `s34_types_procs` with `qb64pe.exe`. Verify: it passes in tier 2; `SOURCE.md` lists `s30`–`s34`.

## 9. The decided fixes and the warning (design D8)

- [ ] 9.1 D-005: drop the `IMP` chain mark. Verify: `cpp` snapshot shows `(a IMP b) IMP c`; CLI build-and-run test
  prints ` 4 ` for `5 IMP 3 IMP 0`; `DIVERGENCES.md` D-005 pinned.
- [ ] 9.2 D-006: `qb_safe_idiv`/`qb_safe_mod` with the signed -1 test. Verify: CLI build-and-run test (LONG and
  `_INTEGER64`, `\` raises 6 under a handler, `MOD` gives 0); every other `cpp` snapshot that contains the
  templates updated once; tier 2 unchanged; the numeric-semantics spec's "unspecified" sentence gone at archive;
  `DIVERGENCES.md` D-006 pinned.
- [ ] 9.3 D-007 and D-008 in `consteval` and `check\constants.rs`. Verify: `check-fail` test for `CONST c = 1 / 0`;
  `typed` snapshot and CLI build-and-run test for `CONST c = 2 ^ 70` (prints ` 1.180591620717411D+21 `); `check-ok`
  test and CLI build-and-run test for `lbl1: CONST k = 4`; `DIVERGENCES.md` D-007, D-008 pinned.
- [ ] 9.4 The chained-`^` warning in `consteval` and warnings in the driver (printed only with `-w`, not counted,
  exit status unchanged). Verify: CLI tests for the cli delta's two warning scenarios and the constants delta's
  "Parenthesised chain"; the corpus runner's compile line (`-q -m -x`) prints nothing new for `s18_const`.

## 10. Integration

- [ ] 10.1 Full runs with the release build: full corpus, upstream (`--suite compile`), slice, differential. Add
  every newly passing program to its list; regenerate the shrink-only lists. Verify: counts recorded here and in
  `STATUS.md` ("Numbers at the last full runs"); no program prints wrong output; `tests\differential\pass.list`
  names every differential program.
- [ ] 10.2 The CI `tier2` steps against the QB64pe 4.7.0 release (slice list, upstream pass list, differential pass
  list), locally with the CI command lines, then on GitHub after the push. Verify: all three pass; the GitHub run's
  result recorded here.
- [ ] 10.3 Docs: `crates\README.md` (the type set, `difftest`), `study\00` §5 (all measured facts of this change)
  and §6 (pinned-by), `DIVERGENCES.md`/`DIVERGENCES-QB45.md` "Pinned by" columns, `GLOSSARY.md` (differential test,
  held/believed type if missing), `STATUS.md` step 8 done. Verify: `check_repo.py --untracked` clean;
  `openspec validate m2-numeric-types --strict` passes.
