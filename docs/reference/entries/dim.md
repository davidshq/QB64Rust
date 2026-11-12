# DIM

`DIM` declares variables and arrays and gives them a type: `DIM total AS LONG`, `DIM scores(1 TO 5) AS INTEGER`.
`DIM SHARED` in the main module makes them visible in the procedures below it.

## Syntax

```text
DIM [SHARED] name[(bounds)] [AS type] [, name[(bounds)] [AS type]]...
DIM [SHARED] AS type name[(bounds)] [, name[(bounds)]]...
```

## Arguments

- `name`: a variable or array name, with a type suffix or an `AS` clause, not both.
- `bounds`: for each dimension, `upper` or `lower TO upper`, separated by commas.
- `type`: a numeric type, `STRING`, `STRING * n`, a user type or `_MEM`.

## Behavior

`DIM name AS type` declares a variable of that type. With `AS type` before the names, every name of the list gets
that type: `DIM AS DOUBLE x, y`.
<!-- src: language/arrays-and-types "DIM AS type lists"; tests\reference\examples\dim_1 -->

A dimension written `upper` runs from 0, or from 1 under `OPTION BASE 1`. Bounds that are not whole numbers are
rounded half to even: `DIM a(2.5)` runs from 0 to 2.
<!-- src: language/arrays-and-types "Static arrays"; language/arrays-and-types "OPTION BASE" -->

Every element starts at 0, as an empty string, or as NUL bytes for a fixed-length string. A variable declared by
`DIM` starts the same way.
<!-- src: language/arrays-and-types "Static arrays"; tests\reference\examples\dim_1 -->

In the main module, an array with constant bounds is static: it exists from the start of the program, and its `DIM`
line does nothing when it runs, even when it runs twice. A bound with an operator, such as `DIM a(k + 1)`, or a
variable, or `$DYNAMIC` in effect, makes a dynamic array, which `REDIM` can change.
<!-- src: language/arrays-and-types "Static arrays" -->

An array is a name plus a type, in a name space of its own: `DIM a(3)` and a variable `a` are two things.
`DIM a()` with nothing in the parentheses declares the variable `a`, not an array.
<!-- src: language/arrays-and-types "Static arrays"; language/arrays-and-types "A scalar and an array of one name" -->

`DIM` in a SUB or FUNCTION declares a local, new on each call. An array there is dynamic unless the procedure is
`STATIC`.
<!-- src: language/procedures "Procedure scopes"; language/arrays-and-types "Static arrays" -->

`DIM SHARED` in the main module makes the variable visible in every procedure that comes later in the file. A
procedure above the `DIM SHARED` line does not see it: its name there is a local.
<!-- src: language/procedures "Procedure scopes" -->

## Errors

These are compile errors:

- A name with a type suffix and an `AS` clause, as in `DIM a%% AS _BYTE`: "a name with a type suffix cannot have an
  `AS` clause".
  <!-- src: language/numeric-semantics "Numeric types"; verification\v21_x09_suffix_and_as -->
- A suffix on a name of a `DIM AS type` list: "a name in a list with the type before the names cannot have a type
  suffix: `b%`".
  <!-- src: language/arrays-and-types "DIM AS type lists" -->
- A second `DIM` of a variable: "name already in use: `x`". A second `DIM` of a static array: "the static array `a`
  is already declared".
  <!-- src: language/arrays-and-types "Static arrays" -->
- An upper bound below the lower bound, as in `DIM a(3 TO 1)`: "invalid bounds for `a`: 3 TO 1".
  <!-- src: language/arrays-and-types "Static arrays" -->
- A `DIM` of a constant's name: "name already in use".
  <!-- src: language/constants "CONST visibility" -->

## Differences

From QuickBASIC 4.5: an array used without `DIM` in a SUB is an array 0 to 10 there, kept between calls. In
QB64Rust, as in QB64pe, every use raises error 9 until a `DIM` or `REDIM` of it in the SUB runs
(`DIVERGENCES-QB45.md` Q-009).
<!-- src: Q-009; language/arrays-and-types "Implicit arrays" -->

## Availability

<!-- begin generated: availability -->
Supported. All 75 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:dim_1 -->
```qb64
DIM total AS LONG, name1 AS STRING
DIM AS DOUBLE x, y
DIM scores(1 TO 5) AS INTEGER
DIM grid(3, 3)
total = 100000: name1 = "Davout": x = 1 / 3
scores(1) = 90: scores(5) = 70
grid(3, 3) = 2.5
PRINT name1; total; x; y
PRINT "scores runs from"; LBOUND(scores); "to"; UBOUND(scores); "and scores(5) is"; scores(5)
PRINT "grid(3, 3) is"; grid(3, 3); "and grid(0, 0) is"; grid(0, 0)
END
```

Output:

```text
Davout 100000  .3333333333333333  0 
scores runs from 1 to 5 and scores(5) is 70 
grid(3, 3) is 2.5 and grid(0, 0) is 0 

Press any key to continue
```
<!-- end generated -->

## See also

`REDIM`, `SHARED`, `STATIC`, `COMMON`, `OPTION`, `TYPE`, `ERASE`.
