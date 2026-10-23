# Proposal

## Why

The upstream suite stands at 378 of 404 on `tests\upstream\pass.list`. Thirteen of the 26 programs left are
programs the old compiler rejects, and each needs only a real compile error where we now give a "not supported
yet" mark (user, 2026-10-21: item 1 of the order after the merge of `m4`). They are the cheapest upstream programs
left. The marks behind them also hold back `verification` programs that measured the same errors
(`v24_x195`, `v24_x232`, on `tests\verification\known.list`).

Found before this proposal, by running `target\release\qb64rust.exe --dump typed` on each (build of `6fcaab0`):

| Program | Old compiler | Now | Where the mark is raised |
|---|---|---|---|
| `arrays/39_err_member_must_be_const` | "TYPE member array bounds must be numeric constant" | mark: a bound not written with numbers alone | `sema` `check\places.rs` `member_bounds` |
| `arrays/40_err_undefined_type` | "Undefined type" | mark: the member type `NOTDEFINED` | `check\places.rs` `member_type` |
| `arrays/42_…`, `43_err_solved_bug_Incorrect_nr_of_args` | "Incorrect number of arguments" (`UBOUND(f(10).a(5), 1)`, `LBOUND`) | mark: `UBOUND` of this expression | `check\places.rs`, the bound functions' array argument |
| `arrays/45_err_solved_bug_Incorrect_nr_of_args` | "Incorrect number of arguments" (`CLEAR F(1)`) | **a real error already** (`m4-builtins-param-types`); it never joined `pass.list` | none |
| `const/bad_single_arg`, `const/not_bad_identifier` | "Unexpected element 'OR'", "Expected variable/value after 'NOT'" | mark: operator `OR` (expected an expression) | `syntax` `parser\mod.rs` `syntax_error`, `unhandled_token` |
| `const/undefined_argument`, `const/undefined_argument_first` | "Unexpected element 'UNDEFINED'" | mark: `UNDEFINED`, not a constant defined so far | `check\constants.rs`, the last arm of a name |
| `types/dim_bad_typename_byte`, `types/static_bad_typename_byte` | "Unknown type" (`AS _Bytef`) | mark: the type `_BYTEF` | `check\decl.rs` `type_of_in` |
| `types/dim_bad_typename_string`, `types/static_bad_typename_string` | "Expected STRING * number/constant" (`AS Stringfoobar * 2`) | mark: `STRINGFOOBAR * …` | `check\decl.rs` `type_of_in` |

Each mark was put there as a safety net: at the time, the compiler could not tell the wrong name from a form it
did not know yet. That is no longer true. The program's `TYPE` blocks are declared in pass 1 wherever they stand
(`m4-names-arrays`), every `CONST` form and `$COLOR` are read (`m3-constant-evaluation`), and the old compiler's
type reader (`typname2typ`, QB64pe `source\utilities\type.bas` 559) knows a closed set of words. Tier 1's
shrink-only lists already run the front end over every program the old compiler accepts: the corpus, upstream,
the snippets, `qbasic_testcases` and the old compiler's own sources. So a new real error that is false shows up as a
new line in `tests\known_false_errors.list`, which must stay empty.

## What Changes

- **A member array's bound that names no constant** (a variable or an unknown name): the compile error "TYPE member
  array bounds must be numeric constant", as the old compiler gives it. A bound that names a `CONST` stays "not
  supported yet" (`SOMEDAY.md`'s row; `v24_x196`).
- **A member type that is no type** (no built-in type word and no `TYPE` of the program): the compile error
  "Undefined type".
- **`LBOUND` and `UBOUND` of an element** (a member array with its indexes, `f(10).a(5)`; and the other element
  forms the new family measures as the same error): the compile error "Incorrect number of arguments". Forms the
  family does not measure stay marked.
- **`CONST` expressions**: an operator where an operand is expected (`CONST c = OR`, `NOT OR`) and a name that is
  no constant, variable or function (`CONST c = 20 OR UNDEFINED`) are compile errors, with the old compiler's
  wording in the note. Outside a `CONST`, an operator where an operand is expected stays marked.
- **`AS` type names in `DIM`, `REDIM`, `STATIC`, `SHARED`, `COMMON` and parameters**: a name that is no word of the
  old compiler's type reader and no `TYPE` of the program is the compile error "Unknown type". A name glued to
  `STRING` before `* n` (`Stringfoobar * 2`) is "Expected STRING * number/constant". A `* n` after any other
  word gets whichever error the family measures.
- **Tests**: a measurement family `v51` in `verification\` for the forms around these 12 that are not measured
  yet. The 13 upstream programs go onto `tests\upstream\pass.list`, 12 of them off
  `tests\known_unsupported_rejections.list` (45 left it with `m4-builtins-param-types`). Every `verification`
  program on `known.list` that now passes, `v24_x195` and `v24_x232` among them, moves to `pass.list`.

Out of scope: a `CONST` in a member array's bound (needs the constants in pass 1), the ten QuickBASIC snippets of
`known_unsupported_rejections.list`, and `SUB _GL`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/arrays-and-types`: new requirements "Member array bounds that name no constant", "Undefined member
  types", "LBOUND and UBOUND of an element" and "Unknown type names".
- `language/constants`: new requirement "Malformed CONST expressions".

## Impact

- `syntax`: `parser\mod.rs` `syntax_error` (an operator where an operand is expected inside a `CONST` is an error,
  not a mark).
- `sema`: `check\places.rs` (`member_bounds`, `member_type`, the bound functions), `check\decl.rs`
  (`type_of_in`), `check\constants.rs` (the name arm); a list of the program's `CONST` names known in pass 1.
- `crates\base\src\old_wording.tsv` regenerated (`tools\diagnostics\old_wording.py`), so the new errors carry the old
  compiler's wording as their note.
- Lists: `tests\upstream\pass.list`, `tests\known_unsupported_rejections.list`, `tests\verification\pass.list`
  and `known.list`.
- No IR, `codegen-cpp` or libqb change; no new dependency.
