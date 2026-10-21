# Design

## Context

The findings and the reasons for each are in `proposal.md`. What shapes the approach:

- `sema`, `ir` and `codegen-cpp` allow no `_ =>` arm on an enum (`crates\README.md`, "Lints";
  `DECISIONS.md` 2026-10-04). A `match` on a `Ty` lists every variant, so a list of the numeric types is the
  normal way to write "any number". `sema` exports one pattern macro, `place_only_types!` (`crates\sema\src\lib.rs`,
  `#[macro_export]`, written `$crate::Ty::…`); `ir` re-exports it and `codegen-cpp` imports it from `ir`.
- `ir` shares `sema`'s types and value tree where the lowering would only copy them (`DECISIONS.md` 2026-10-07,
  "The IR shares `sema`'s value tree"); its statement side, its variables and the facts of the error rule stay its own.
- `tools\callsite\callsite.py` already imports `clone_lock` and `compiler_copy` from `tools\legacy_tests\`
  (by `sys.path`).
- The C++ the compiler writes is pinned by the `cpp` snapshots of `tests\frontend\` (tier 1). The closing runs
  use `--build-cache`, which gives a program whose C++ did not change a byte-identical executable, and
  `--pass-cache`, which then does not run it ("not run: passed before", counted on the run's last line;
  `tools\legacy_tests\README.md`).

## Goals / Non-Goals

**Goals:**

- Each repeated piece of the proposal written once, with no change in any output: diagnostics, symbol table,
  typed tree, IR, C++, and the two tools' behaviour.
- Every `match` stays exhaustive: adding a `Ty` variant still fails to compile at each site that must decide
  about it.

**Non-Goals:**

- The repeats the proposal leaves alone (`ExprKind`/`BinOp` lists, the `Ty`-to-text tables, the whole-array copy
  emitters, the syntax crate's node lists, the dumps' `ty`, the `Rng`s).
- Partial lists of numeric types (`Ty::I8 | Ty::U8 => …` by width, the unsigned ones): only an arm that names
  every numeric type, with or without `_BIT`, takes a macro.
- Moving functions between modules or crates beyond what each decision names.

## Decisions

### D1. Two pattern macros for the numeric types

In `crates\sema\src\lib.rs`, beside `place_only_types!`, each `#[macro_export]` with a doc comment:

| Macro | Pattern | For |
|---|---|---|
| `numeric_types!()` | the ten integer types, `Ty::Bit { .. }`, `F32`, `F64`, `F80` | an arm on a place's or a declared type, where `_BIT` is a number like the others |
| `numeric_value_types!()` | the same without `Ty::Bit { .. }` | an arm on a value's type, where `_BIT` has its own arm (it is held in its storage type, [`BIT_VALUE_UNREACHABLE`]) or another arm already takes it |

The name of the second says why `_BIT` is missing, in the words of the existing `BIT_VALUE_UNREACHABLE`.
`crates\ir\src\lib.rs` re-exports both beside `place_only_types`; `codegen-cpp` imports them from `ir` and
deletes `numbers!` (`decl.rs`) and `numeric!` (`mem.rs`).

A site takes a macro when one arm names every numeric type (or every one but `_BIT`), in any order, alone or
with other variants (`numeric_value_types!() | Ty::Str => …`). The search for the sites is a script in the
scratchpad: the 21 lists in the order of the enum, found on 2026-10-21, are in `crates\sema\src\lib.rs` (seven,
among them `is_numeric`, `storage`, `held_value`, `size_of`, `owns_desc` and `type_name`), `sema\src\builtins.rs`,
`sema\src\check\{builtins,io,mem,places}.rs`, `ir\src\{validate,dump}.rs` and
`codegen-cpp\src\{arrays,builtins,decl,mem,procs}.rs`. Lists in another order are looked for by hand in the same
files and in the rest of the three crates.

*Alternatives:* one macro taking `_BIT` as a parameter (`numeric!(bit)`): a reader then has to know the
parameter, and the bare name reads the same at each site. A method (`Ty::is_numeric`) at the sites: the lint
sees only `match`, and `crates\README.md` asks for a type predicate to be a `match`; the arms also take other
variants with the numbers, which a predicate cannot.

### D2. `clear_temp` in `clone_lock.py`

`clear_temp(qb_root)` moves from both tools to `tools\legacy_tests\clone_lock.py`, the module whose docstring
describes the tools' use of the clone, "the generated C++ goes to internal/temp, which the tools empty before each
compile". It takes `run_legacy_tests.py`'s comment on `temp.bin` (why it is kept) and docstring. Both tools
import it from there; `run_legacy_tests.py`'s six calls and `callsite.py`'s one are unchanged.

*Alternative:* `compiler_copy.py`, as the review first said: that module is about the copy of the new compiler
a run uses, not the clone.

### D3. The flat index of an element, once

`crates\codegen-cpp\src\place.rs`: `flat_index(array, index)` and `dyn_element_desc` build the same sum of
`array_check((i)-d[s],d[s+1])*d[s+2]` terms, the first with the variable's C name, the second with a pass
variable. A private method `flat_index_in(&mut self, desc: &str, index: &[Expr]) -> String` builds it;
`flat_index` calls it with `self.name(array)`, `dyn_element_desc` with its `pass<n>`.

### D4. `Shape`'s methods

`is_array`, `dim_count` and `bounds` move from `impl Var` to `impl Shape` in `crates\sema\src\lib.rs`. `Var` in
`sema` and in `ir` (`crates\ir\src\lib.rs`) keeps the three methods as one-line delegates (`self.shape.…()`), since
they are called on variables throughout `sema`, `ir` and `codegen-cpp`.

### D5. `ProcKind` and `RuntimePart` are `sema`'s

`crates\ir\src\lib.rs` deletes its `ProcKind` and `RuntimePart` and re-exports `sema`'s, with the reason of
2026-10-07 in the comment (the lowering only copied them). `crates\ir\src\lower.rs` drops the two conversions (the
`ProcKind` match at line 51 and the `RuntimePart` match at line 80 on 2026-10-21) for a copy of the value. This
extends the 2026-10-07 decision to two types of the statement side, so it gets a `DECISIONS.md` row.

`ir::Resume` stays: `Resume::To` holds `ir::LabelId`, which the lowering maps (`self.ids.get(l)`). `Storage`,
`Label`, `Settings` and `Var` differ in their fields and stay too.

### D6. Two helpers in `sema\check`

- `fn as_words(&self, a: ast::AsClause) -> Vec<String>` in `crates\sema\src\check\mod.rs` beside `word`, for the
  six `a.type_words().map(|t| self.word(t)).collect()` (`decl.rs` `type_of_in` and `default_type_of`, `mem.rs`
  `mem_value_type`, `places.rs` `member_type`, `proc.rs` `fixed_string_size` and `unsigned_string`), and
  `fn split_unsigned(words: &[String]) -> (bool, &[String])` beside it for the four that split off a leading
  `_UNSIGNED` (`type_of_in`, and `mem.rs` `mem_value_type`, `mem_type_arg` and `bytes_type`; the last two
  found by the review of the change). `member_type` reads only whether the first word is `_UNSIGNED`
  and keeps doing so.
- `fn new_proc(&mut self, name: String, kind: ProcKind) -> ProcId` in `crates\sema\src\check\proc.rs`: pushes
  the `Proc` record and its parameter scope and enters the name in `procs_by_name`, for `declare_proc` and
  `declare_broken`; each then does what is its own (the name's symbol, or the broken set).

### D7. One helper for the screen-state tests

`crates\driver\tests\cli.rs`: the five tests that link `tools\screen\screen_state.cpp`
(`extra_linker_flags_link_the_screen_state_file`, `screen_state_prompt_that_never_comes`,
`screen_state_wait_no_rule_answers`, `screen_state_mistake_in_a_mouse_step`,
`screen_state_mouse_step_of_a_hidden_window`) write `p.bas` and `p.keys`, build with the linker flag, and run
with `QB64_SCREEN_STATE`, `QB64_SCREEN_KEYS` and `QB64_SCREEN_PROMPT_MS`. Two functions in `cli.rs` (not in
`support\`, which the other test files share and which has no `qb64rust_build`) do that:
`build_with_screen_state(d, program) -> PathBuf` and `run_screen_state(d, exe, keys, prompt_ms) -> Output`,
the keys and the prompt time optional (the first test runs without keys; the mouse-step test reads stderr, not
the state file, so each test reads `state.txt` itself). Each test keeps its program, keys, prompt time and
assertions. The comment on the 5000 ms prompt time stays with its test.

### D8. How "nothing changed" is shown

- Tier 1: no snapshot changes (`typed`, `ir`, `cpp` among them); `git status` shows no `.snap` file.
- Before the first edit, the closing set's three lists (slice list, upstream pass list, `tests\verification\pass.list`)
  are run once on the base build with `--build-cache` and `--pass-cache`, so that every passing program has an
  entry. At the close the same runs must report every program "not run" but those that failed on the base build
  too: a program that runs again had its C++ changed, which is a finding to explain before the change closes.
- The call-site check (it compares the C++ calls, so the codegen edits are covered there as well).
- The two Python tools: `test_run_legacy_tests.py`, and the call-site check's run above.

## Risks / Trade-offs

- [A wrong macro at a site] → It cannot pass silently: `numeric_value_types!()` where `_BIT` was listed leaves
  the `match` non-exhaustive (an error), and `numeric_types!()` beside an existing `Ty::Bit` arm is an
  unreachable pattern, fatal under `clippy -D warnings`.
- [rustfmt does not format inside a macro's body] → The two bodies are a fixed list, written once.
- [Grep and go-to-definition find `numeric_types!()` and not `Ty::F32` at a site] → A search for a variant
  misses those arms. Accepted: the macro's doc comment lists the variants, and the existing
  `place_only_types!` has the same property.
- [Partial lists of the numeric types stay written out] → Arms that name some numeric types and not others
  (SWAP and VAL in `codegen-cpp\src\builtins.rs`, `codegen-cpp\src\io.rs`, `sema\src\lib.rs`) are deliberate
  subsets: a pattern cannot be "a macro minus a variant". A new numeric `Ty` variant would have to be placed in
  each by hand; a `match` that misses it still fails to compile.
- [The base-build cache run costs machine time] → With the build and pass caches warm from the last close, it
  runs only what changed since; it is started in the background while the first edits go on.
- [`m4-handlers-logging-checking` touches the same files] → This change is built and closed before that one
  starts; its plan names files, not lines, so it stays valid.
