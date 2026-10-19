# Design

## Context

The `CONST` evaluator (`crates\sema\src\consteval.rs`, `certify.rs`; design D2–D8 of `m3-constant-evaluation`)
copies the old evaluator: three lanes (float, unsigned, signed), each group, function result and whole value
printed with `_TOSTR$` and read back as its text says. `m3-constant-evaluation` kept five behaviours, ruled on
2026-10-18 (`DECISIONS.md`): D-027 to D-029 and D-031 fixed, D-030 kept with a clearer message. The measurements
are in `verification\v45_*`.

## Goals / Non-Goals

**Goals:**

- Build the rulings D-027, D-028, D-029 and D-031, each with its row's test; mark D-030 built.
- `_COTH` the same function in a `CONST` and at run time.

**Non-Goals:**

- `NAN` as a whole value (`VAL` reads it as 0, as before): no ruling covers it, and only an overflowing
  intermediate makes one (`INF - INF`), which D-029 already rejects in most programs.
- The other hyperbolic and reciprocal functions at run time (`_SECH`, `_CSCH`, `_ARCSEC` …): not asked for.
- An unsuffixed literal beyond `_INTEGER64` in a statement: still "not supported yet" (`literal.rs`).

## Decisions

### D1. Values beyond `_INTEGER64` (D-027)

`consteval::literal` reads plain digits as the signed lane up to 9223372036854775807, the unsigned lane up to
18446744073709551615, and beyond that gives the error "this number is beyond `_UNSIGNED _INTEGER64`'s range". C++
has no wider literal, which is why the statement side calls this an overflow. A minus is the evaluator's unary
operator, so `-9223372036854775808` is the unsigned 2^63 negated in the signed lane, `i64::MIN`. The four limits
come from `beforefirstline.bi`'s literals (`tools\builtins\builtins.json`), so they follow with no change of their
own: `_UINTEGER64_MAX~&&` is the unsigned value, `_INTEGER64_MIN&&` `i64::MIN`.

`read_text` (the read-back of a float) reads plain digits within `_INTEGER64` as before. Beyond it, it reads them
as the `_FLOAT` of that text, the type the float lane gives (`FLOATTYPE`), as `2 ^ 70` already is by its exponent
(D-007).

### D2. `^` gives a float (D-028)

`power` returns the float lane's result for any operands. The read-back where a group or the whole value ends
types it by its text, so an integral power within `_INTEGER64` (`2 ^ 3`) is still an `_INTEGER64`, as `4 / 2` is.
The row's "typed as a float for every integer power" describes the power before the read-back: the evaluator has
no other float type than the one the text gives. Inside an expression the power is now in the float lane, so
`2 ^ 62 * 4` is 2^64 and no longer wraps to 0, as in a statement. 0 to a negative power is an error, "division by
zero". It was `i64::MIN` (qbr of infinity), and as a float it would be infinite, which D-029 rejects; the error
says why.

### D3. Beyond `_FLOAT` (D-029)

`read_text` gives the error "this `CONST` overflows: its value is beyond `_FLOAT`'s range" for `INF` and `-INF`,
inside an expression and as the whole value alike. The old compiler has two behaviours there: "Unexpected element"
inside, 0 as the whole value. `NAN` keeps its two behaviours (Non-Goals).

### D4. `_COTH` (D-031)

In a `CONST` (`certify::function`): the argument is converted to `double`, as libqb's `func_coth(double)` takes it.
The value is `1 / tanh(x)` from the reference (two precisions that must agree), rounded to the nearest `double`
(`f64_nearest`, split out of `ucrt_within`). The result is exact and needs no certification: it is this project's
value, not the old library's. At 0 the error "`_COTH(0)` in a `CONST` is a division by zero". There is no upper
limit (coth(45) is 1).

At run time: the edit `tools\libqb\edits\d031-coth.edit` replaces `func_coth` in
`libqb\internal\c\libqb\include\extended_math.h` with `1 / std::tanh(num)`, error 5 at 0, as `UPSTREAM.md`
proposes. `_COTH` joins the plain built-ins (`builtins.rs`: `Rule::Plain`, held `double` as `func_sec`), with four
call-site programs. `std::tanh` is not correctly rounded, so a run-time value can differ from the `CONST` value in
the last bit, as for every other math function (rule 12 of `CLAUDE.md`).

The evaluator's certification tests drop `_COTH` from the sample check (`v45_sample`'s `_COTH` lines are the old
function). The calibration, an ignored test, keeps measuring UCRT's `exp` through the old helper, since its data
are the old recordings.

### D5. The copy's own edits

`copy_libqb.py` applies `tools\libqb\edits\*.edit` in name order after taking the commit's blobs. It writes the
edited files and lists the edits in `SOURCE.md`, and `--check` compares the copy with the edited files. An edit
file names its target and gives exact old lines, which must occur once, and new lines. The edit takes the target's
line ends, whatever the edit file's own. A target that is missing or an old block that does not fit stops the
tool. The copy stays reproducible from the commit and the edits, and nothing in `libqb\` is edited by hand.
Alternative: unified diffs applied with `git apply`. Rejected: applying a diff in memory for `--check` needs a
temporary tree or a diff parser of its own, and exact blocks say the same with less.

### D6. Tests whose recorded cases change

A corpus program is recorded from `qb64pe.exe`, so a case the rulings change leaves it and is held elsewhere, as
`tests\corpus\README.md` does for D-004 and D-019:

- `s142_const_float` loses its D-027 cases.
- `s143_const_funcs` loses `_COTH(.2)` and `2 ^ -1`.
- `s146_const_edges` loses `2 ROOT 0`, `_COTH(0)`, the product beyond `_FLOAT` and `0 ^ -1`.

Each is re-recorded with `qb64pe.exe` and says in its header where the cases went:
`tests\frontend\const_rulings_typed.bas` (the values), `const_rulings_errors.bas` (the errors), and the CLI test
`const_rulings_run` (built and run, `_COTH` at run time).

`crates\driver\tests\consts_v45.rs` keeps comparing every function of `v45_b_funcs` with the old value, and lists
the two decided differences with the value expected here. `s44_more_functions` calls `_COTH` only as "above 1",
which both compilers print alike, so the slice list covers the built-in.

Upstream `const/math_funcs` tests `_COTH(.2)` against `1.44280551632034` and prints `COTH: -1  0` here. The copy
of upstream is never edited and a `.normalize` rule is for output that varies, so the program leaves `pass.list`
for `tests\known_clean_not_passing.list` with the reason. This goes against `pass.list`'s "never remove one", and
the user is told at the close.

## Risks / Trade-offs

- **Programs the old compiler accepts are rejected** (a literal beyond 64 bits, a value beyond `_FLOAT`,
  `_COTH(0)` and `0 ^ -1` in a `CONST`), as the rulings say. → Each error names what is wrong.
- **`^` in the float lane inside an expression** changes results that used to wrap in 64 bits (`2 ^ 62 * 4`).
  That is the ruling's point. → The slice list and the upstream pass list show any program that relied on the
  wrap; none did.
- **Upstream progress drops by one** (`const/math_funcs`).
