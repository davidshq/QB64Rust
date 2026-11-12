# STATIC

`STATIC` in a SUB or FUNCTION declares variables that keep their values between calls. A local variable made by
`DIM` or by first use starts again on each call.

## Syntax

```text
STATIC name [AS type] [, name [AS type]]...
STATIC name(upper) [AS type]
SUB name (parameters) STATIC
FUNCTION name (parameters) STATIC
```

## Behavior

A `STATIC` variable starts at 0 or as an empty string, and keeps its value from one call of the procedure to the
next: a counter `STATIC n AS LONG` with `n = n + 1` counts 1, 2, 3 over three calls.
<!-- src: language/procedures "Procedure scopes"; tests\reference\examples\static_1 -->

Inside the procedure, a `STATIC` variable comes before a `SHARED` variable of the name and a main-module
`DIM SHARED` one.
<!-- src: language/procedures "Procedure scopes" -->

`STATIC a(5)` declares an array of the procedure's own, from 0 to 5, that keeps its elements between calls.
<!-- src: Q-015; tests\corpus\slice\s56_proc_arrays -->

`STATIC` after a SUB's or FUNCTION's header makes every variable of the procedure keep its value. If such a
procedure also holds a `STATIC` statement, only the variables first named above that statement are kept; below it,
new variables are made on each call.
<!-- src: Q-010; verification\v24_x279_sub_static_scalars -->

## Errors

These are compile errors:

- A second `STATIC` of a name in one procedure: "name already in use: `a`".
  <!-- src: verification\v24_x122_static_twice -->
- An array bound that names a variable of the procedure: "`STATIC v(...)`: a bound names a variable of the
  procedure, which the code that allocates the array when the program starts cannot see (the old compiler's C++
  build fails)".
  <!-- src: verification\v24_x121_static_variable_bound -->

## Differences

From QB64pe: a `STATIC a(5)` in a procedure beside a main-module `DIM SHARED a(3)` of the same type makes QB64pe's
program end at its start with "Duplicate definition". In QB64Rust it is the procedure's own array
(`DIVERGENCES-QB64PE.md` D-041).
<!-- src: D-041 -->

From QuickBASIC 4.5:

- The number in `STATIC a(5)` is the number of dimensions there, and a later `DIM` gives the bounds. In QB64Rust,
  as in QB64pe, it is the upper bound (`DIVERGENCES-QB45.md` Q-015).
  <!-- src: Q-015 -->
- In a `SUB … STATIC` that holds a `STATIC` statement, every variable is kept there (Q-010).
  <!-- src: Q-010 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:static_1 -->
```qb64
FOR i = 1 TO 3
    Visit
NEXT i
END

SUB Visit
    STATIC count AS INTEGER
    DIM fresh AS INTEGER
    count = count + 1
    fresh = fresh + 1
    PRINT "count ="; count; "and fresh ="; fresh
END SUB
```

Output:

```text
count = 1 and fresh = 1 
count = 2 and fresh = 1 
count = 3 and fresh = 1 

Press any key to continue
```
<!-- end generated -->

## See also

`SHARED`, `DIM`, `SUB`, `FUNCTION`, `$STATIC`.
