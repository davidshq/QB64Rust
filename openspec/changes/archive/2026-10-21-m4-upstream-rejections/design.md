# Design

## Context

For the motivation, the 13 programs and where each mark is raised, see `proposal.md`. For the behaviour, see the
delta specs. How the parts stand in this compiler (2026-10-21, `6fcaab0`):

- **Marks and errors.** `self.unsupported(span, …)` gives a "not supported yet" mark, and `self.error(span, …)` a
  real error (`sema` `check\mod.rs`). The runner counts a rejection only when it has a real error
  (`new_compiler_rejection`, `tools\legacy_tests`). The note after an error is looked up in
  `crates\base\src\old_wording.tsv`. That table is written by `tools\diagnostics\old_wording.py`, which pairs each
  of our messages with the old compiler's recorded message on the same line.
- **Parser marks.** `syntax_error` (`syntax` `parser\mod.rs` 265) turns a syntax error at a known operator or
  keyword into a mark (`unhandled_token`). `p.in_const` is set while a `CONST` is parsed (`parser\decl.rs` 227–239).
- **Types.** Pass 1 declares every `TYPE` block of the program (`m4-names-arrays` D4), so `types_by_name` holds all
  of them when a member or a `DIM` is typed. `user_type_of` (`check\places.rs` 355) also gives `_MEM`.
  `numeric_type` (`places.rs` 1982) is the old compiler's word list (`typname2typ`, QB64pe
  `source\utilities\type.bas` 559) apart from `STRING`, `_UNSIGNED` and `* n`, which `type_of_in`
  (`check\decl.rs`) reads. The old compiler rejects `$NOPREFIX` (`parser\decl.rs` 30), so no word without its
  underscore is a type.
- **Member array bounds.** `member_bounds` (`places.rs` 259) runs in pass 1, before any `CONST`. A bound that is not
  written with numbers alone is marked, because it may name a `CONST` (`v24_x196`, accepted by the old compiler).
- **CONST names.** The last arm of a name in `check\constants.rs` (267) marks every name that is no constant, no
  variable (`is_variable_name`) and no built-in function.
- **Bound functions.** The array argument of `LBOUND`/`UBOUND` (`places.rs` 1838–1864) is a member array
  (`member_array`), a name or a dotted name. Every other expression, an indexed member array among them, is marked.
- **The guard against false errors.** Tier 1 runs the front end over the corpus, the upstream copy, the snippets,
  `qbasic_testcases` and the old compiler's own sources (`crates\driver\tests`). An accepted program with a real
  error fails it unless it is on `tests\known_false_errors.list`, which is empty.

## Goals / Non-Goals

**Goals:**

- The 12 marks become the old compiler's errors, and the 13 upstream programs join `pass.list`.
- Each rule turns into an error only the cases that the old compiler is known (recorded) to reject. A case
  that may be valid stays marked.

**Non-Goals:**

- A `CONST` in a member array's bound, compiled (`SOMEDAY.md`'s row stays).
- An operator where an operand is expected outside a `CONST`, and every other mark of `unhandled_token`.
- Our message texts copying the old compiler's. The note carries its wording, as for every error.

## Decisions

### D1. The measurement family `v51` comes first

`verification\v51_*` records the forms next to the 12 programs that no family has measured yet. The code then
follows the recordings, not guesses:

- **Bounds:** a bound naming a variable declared below the block; a name used nowhere else; a `CONST` defined below
  the block; a `CONST` defined in a SUB; a function call (`LEN(x)`).
- **Member types:** `AS nosuch` on a single member; a `TYPE` defined in a later block; `AS _UNSIGNED nosuch`.
- **Bound functions:** `LBOUND(a(5))` of a plain array, of an element of an array of a type (`LBOUND(f(1))`), of a
  member array of a member (`f(1).g.a(2)`), of a literal, of a parenthesised array name, and `LBOUND(f(10).a(5))`
  without a dimension.
- **CONST:** `CONST c = AND 1`, `= MOD`, `= NOT`, `= 1 OR`; an undefined name in a `CONST` in a SUB; a `CONST` that
  names a constant defined below it; a name of a SUB or of a label.
- **Type names:** `AS BYTE`-like words in `REDIM`, `SHARED`, `COMMON` and a parameter; `AS INTEGER * 2`; `AS
  _BYTE * 2`; `AS STRING2 * 2`; `AS _UNSIGNED nosuch`; a misspelt type in `DECLARE LIBRARY`.

Where a recording disagrees with a delta spec, the spec is changed in the same task (as `m4-builtins-param-types`
1.1 did). *Alternative:* build from the 12 programs alone and leave the neighbours marked. Rejected: each rule
reads a class of names, and unrecorded members of the class are where a false error would come from.

### D2. Member array bounds: the program's `CONST` names before pass 1

`check\mod.rs` collects the name of every `CONST` of the program from the syntax tree before pass 1 (the main
module, every procedure, the included files; with `$COLOR` the colour names and the auto-included constants, which
`sema` already knows by name). `member_bounds` keeps its mark when the bound names one of them. Otherwise, a bound
not written with numbers alone is the error "a bound of a member array must be a constant" with the old compiler's
note. Names inside a function call stay marked until `v51` shows the old compiler's result. Code: `check\mod.rs`
(the collection, beside `collect_block_settings`), `check\places.rs` `member_bounds`.

*Alternative:* evaluate the program's constants in pass 1 and compile the `CONST` bound too. Not this change: it
moves the whole constant evaluation earlier (the `SOMEDAY.md` row).

### D3. Member types and `AS` names: the old compiler's word list, closed

In `member_type` and `type_of_in` the mark becomes an error when the words are none of: a word of `numeric_type`,
`STRING`, `_UNSIGNED` before one of them, `_MEM`, or a `TYPE` of the program (pass 1 has them all). A member gives
"Undefined type", a declaration "Unknown type". Before the general test, a first word that starts with `STRING`
and is followed by `* n` gives "Expected STRING * number/constant". A `* n` after any other word gives the error
`v51` records for it. Code: `check\places.rs` `member_type`, `check\decl.rs` `type_of_in` and its `no_type`, which
chooses the error by place and kind (no shared `is_type_word` helper came of it: `numeric_type`, `STRING` and the
user types are tested inline, as before). The `STRING` prefix is tested only when no `TYPE` of the program has
the name: the old compiler looks the user types up first (`dim2` 17860, `typname2typ`), so `TYPE StringList` is
a type everywhere (review of the staged change, 2026-10-21; `tests\frontend\type_string_prefix_ok.bas`).

*Alternative:* keep the mark for any word that starts with a type word (`_BYTEF`). Rejected: `typname2typ`
compares whole words, so there is no near-type that the old compiler accepts.

### D4. Bound functions: the element of a member array

The arm that marks "`LBOUND` of this expression" gets a case before it. A `Call` whose callee is a member access
to a member array of the program's types, with arguments, is "Incorrect number of arguments". The other forms of
D1 become errors in the same place as `v51` records them. A form `v51` does not measure keeps the mark. Code:
`check\places.rs`, the bound functions' argument match (1838–1864).

### D5. `CONST`: an operator where an operand is expected, and an unknown name

- **Parser.** In `syntax_error`, when `p.in_const` is set and `unhandled_token` names a binary operator word
  (`OR`, `AND`, `XOR`, `EQV`, `IMP`, `MOD`), the diagnostic is an error, not a mark. `NOT` followed by one is the
  same path, since the operand after `NOT` is parsed as an expression. Outside a `CONST`, nothing changes. Code:
  `syntax` `parser\mod.rs` `syntax_error`.
- **Checker.** The last arm of a name in `check\constants.rs` becomes an error. The only cases left there are a
  name that is no constant defined so far, no variable and no built-in function. `v51` decides two of its
  neighbours: a constant defined below (the old compiler reads `CONST` in order, so most likely an error), and a
  label's or SUB's name. If either is accepted by the old compiler, that case keeps the mark.

*Alternative:* make every syntax error at an operator an error. Rejected: outside a `CONST` the old compiler's
grammar has forms we do not parse, and the mark is what keeps those from being false errors.

### D6. The lists

`tests\upstream\pass.list` gains the 13 programs, `arrays/45_err_solved_bug_Incorrect_nr_of_args` among them
(already rejected with a real error since `m4-builtins-param-types`). `tests\known_unsupported_rejections.list`
loses the 12 programs that now get real errors. In `tests\verification\known.list`, every program that now
passes moves to `pass.list`, found by one run of `known.list` with this compiler: `v24_x195` and `v24_x232` at least.

### As measured (task 1.1, `verification\v51_*`)

Where the recordings changed D2–D6 (the specs were changed to match):

- **D2.** Only a `CONST` *above* the block counts: one below it, or in a SUB above or below a block of the main
  module, is the error (`x03`, `x04`, `x13`). So the collection keeps each name's place in file order and whether
  it stands in a procedure (`Checker::const_names`, `consts_above`). `LEN`, an array's element and a FUNCTION of
  the program in a bound are the error (`x05`, `x06`, `x12`, `x15`); `_PI` and `ABS(…)` are accepted (`x10`,
  `x11`, `x14`) and keep the mark, as a function of the constant evaluator.
- **D3.** A member type defined in a *later* block is "Undefined type" too (`x21`), so the member's test is the
  types declared so far in pass 1. A member's `INTEGER * 2` and `Stringfoobar * 2` are "Undefined type" as well
  (`x25`, `x27`); an `AS type` list before the names has its own error (`x26`). In a declaration, the glued word
  is the STRING error with or without `* n` and after `_UNSIGNED` (`x70`, `x83`); `* n` after any other word is
  "Unknown type" (`x65`, `x66`, `x76`–`x78`), "Type cannot be _UNSIGNED" after `_UNSIGNED` (`x84`). The error
  depends on the place: `SHARED` and `_DEFINE` give "Cannot convert type (…) to symbol" (`x61`, `x82`), every
  kind of parameter "Illegal SUB/FUNCTION parameter" (`x63`, `x69`, `x74`, `x87`, `x89`). So `type_of_in`
  takes an `AsPlace`, and a form not measured in a place keeps the mark. A name of a `TYPE` block that was not
  entered keeps the mark (`Checker::types_failed`).
- **D4.** `LBOUND(a(5))` of an array of any other type, of a FUNCTION's call, of a literal and of `a + 1` are
  "Syntax error" (`x28`–`x30`, `x33`, `x39`); of an element of an array of a `TYPE` "Expected ." (`x31`); of a
  member array's element, also through a member or of a single value, "Incorrect number of arguments" (`x32`,
  `x35`–`x37`). `LBOUND((a))` and `LBOUND(n)` of a scalar are accepted (`x34`, `x38`); the first keeps its mark.
- **D5.** Both neighbours are errors: a constant defined below and the name of a SUB, a label, a FUNCTION or an
  array (`x45`–`x49`, `x52`). The arm keeps its mark only for the name of a `CONST` above the one checked
  (`Checker::consts_before`), which this compiler may not have taken. The parser's two messages (an operator
  alone, after `NOT`) are distinct so that each pairs with one old wording.
- **D6.** `arrays/45_err_solved_bug_Incorrect_nr_of_args` was on `pass.list` already: the list gains 12.
- **Notes.** `tools\diagnostics\old_wording.py` left out every old message that holds a `/`, reading it as a
  file's path, so `const/not_bad_identifier` ("Expected variable/value after 'NOT'") and
  `types/*_bad_typename_string` ("Expected STRING * number/constant") got no note. Every recorded path also holds
  `<verification>`, a backslash or a BASIC file's extension, so a `/` alone no longer counts (user, 2026-10-21:
  option 1 of task 6.1; `names_file`, tested by `tools\diagnostics\test_old_wording.py`, run by CI). The table
  gained 45 patterns, among them "Illegal SUB/FUNCTION parameter" and "LSET/RSET expects …"; `--check`: 1075
  notes, all the recorded message.

## Risks / Trade-offs

- [A real error where the old compiler accepts the program] → Tier 1's front-end runs over every accepted input
  set catch it as a new false error. D1 measures the neighbours before each rule is written.
- [The `CONST` names of an included file not seen before pass 1] → The collection walks the included files that
  the parser already joins. A program of `tests\frontend\inc\` with a `CONST` in an included file is used as a
  bound.
- [The wording table pairs a new message with the wrong old message] → `old_wording.py --check` holds each note
  to the recorded message. It runs at the close.
