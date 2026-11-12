# Fixes

A fix is a named change to a QB64pe behavior that QB64Rust keeps by default. Every fix is off unless a program's
settings file or the command line turns it on, so a program builds and runs as with QB64pe until it asks for a fix.

The pages of this area: [RESUME](../entries/resume.md), [$CHECKING](../entries/meta-checking.md),
[DIM](../entries/dim.md), [REDIM](../entries/redim.md), [STATIC](../entries/static.md), [RUN](../entries/run.md),
[_SETBIT](../entries/_setbit.md).

## The fixes

`qb64rust --list-fixes` prints each fix with what it changes. The fixes of this release:
<!-- src: compiler/fixes "Named fixes"; compiler/fixes "Listing the fixes" -->

| Fix | With the fix on |
|---|---|
| `resume-after-block` | `RESUME NEXT` after an error in the condition of a `WHILE`, `DO`, `LOOP`, `IF` or `ELSEIF` goes on after the whole block |
| `dim-empty-error` | A dimension with no element raises error 5 and leaves the array not allocated, and a statement whose bound raises an error allocates nothing |
| `static-local-dynamic` | `STATIC a(n)` in a procedure beside a shared dynamic array of that name declares an array of the procedure's own |
| `whole-array-value` | A whole array `a()` used as a value in an expression is a compile error, as in QuickBASIC 4.5 |
| `resume-in-procedure` | `RESUME` in a SUB or FUNCTION is a compile error, as in QuickBASIC 4.5 |
| `run-in-procedure` | `RUN` in a SUB or FUNCTION is a compile error, as in QuickBASIC 4.5 |
| `checking-on-service` | An error raised below `$CHECKING:OFF` is handled before the first checked statement after it |
| `bit-number-check` | A bit number outside 0 to 63 in `_READBIT`, `_SETBIT`, `_RESETBIT` and `_TOGGLEBIT` raises error 5 |
<!-- src: compiler/fixes "Named fixes"; language/error-handling "Fix resume-after-block"; language/arrays-and-types "Dynamic arrays"; language/arrays-and-types "Arrays in procedures"; language/arrays-and-types "A whole array as a value"; language/error-handling "Fix resume-in-procedure"; language/control-flow "Fix run-in-procedure"; language/run-time-checks "Fix checking-on-service"; language/builtin-functions "Fix bit-number-check" -->

A fix that is off changes nothing: the program compiles and runs as with no fix named at all.
<!-- src: compiler/fixes "Named fixes"; compiler/fixes "Default off" -->

## Turning fixes on

The compiler reads the fixes from the first `qb64rust.toml` it finds in the main `.bas` file's folder or a folder
above it. Its `[fixes]` table holds `all = true` or `false` for every fix, and `<name> = true` or `false` for one,
which overrides `all`:
<!-- src: compiler/fixes "The settings file" -->

```toml
[fixes]
all = true
run-in-procedure = false
```

On the command line, `--fixes on` or `--fixes off` replaces the file's whole set, and each `--fix <name>=on` or
`--fix <name>=off` then sets one fix. `--no-settings` leaves the file unread.
<!-- src: compiler/fixes "Command-line fixes" -->

An unknown fix name, a value that is not `true` or `false`, or a file that is not valid TOML is an error that names
the file and the line, and the program is not built. An unknown name on the command line is an error too.
<!-- src: compiler/fixes "A bad settings file"; compiler/fixes "Command-line fixes" -->

A build is reused from the build cache only when it was made with the same fixes.
<!-- src: compiler/fixes "Fixes and the build cache" -->
