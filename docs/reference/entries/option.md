# OPTION

`OPTION BASE` sets the lowest index of arrays declared without `TO`. `OPTION _EXPLICIT` makes every variable need
a declaration, so that a misspelled name is a compile error. `OPTION _EXPLICITARRAY` does the same for arrays
only.

## Syntax

```text
OPTION BASE 0
OPTION BASE 1
OPTION _EXPLICIT
OPTION _EXPLICITARRAY
```

## Behavior

`OPTION BASE 1` makes 1 the lower bound of each dimension declared below it without `TO`: after it, `DIM a(3)` runs
from 1 to 3. `OPTION BASE 0` makes it 0 again, as it is without either. Bounds written with `TO` do not change.
<!-- src: language/arrays-and-types "OPTION BASE"; verification\v17_f_option_base; verification\v24_e_option_base -->

`OPTION BASE` applies by its place in the file, not by whether it runs, and to every kind of array below it:
declared, used without `DIM`, static, dynamic and member arrays. It may stand anywhere, any number of times. One in
a SUB applies to the lines below the SUB too. `01` is read as 1.
<!-- src: language/arrays-and-types "OPTION BASE"; verification\v24_e_option_base; verification\v24_x239_option_base_01 -->

Under `OPTION BASE 1`, a dimension written as the number `0` stays `0 TO 0`: `DIM z(0)` has one element.
<!-- src: language/arrays-and-types "OPTION BASE"; Q-016 -->

`OPTION _EXPLICIT` applies to the whole program, wherever it stands: also inside a SUB, and after the lines it
concerns. A variable used without a declaration is then a compile error. A declaration is a `DIM`, a `DIM SHARED`,
a `SHARED` of a declared main-module variable, a `STATIC`, a `CONST`, a parameter or a FUNCTION's own result. A
`FOR` variable needs one too.
<!-- src: language/constants "OPTION _EXPLICIT"; verification\v17_f_explicit_late_option; verification\v17_f_explicit_for; verification\v17_f_explicit_kinds -->

`OPTION _EXPLICITARRAY` makes an array used without `DIM` a compile error, and allows variables without a
declaration. `OPTION _EXPLICIT` covers arrays too.
<!-- src: language/constants "OPTION _EXPLICIT"; language/arrays-and-types "Implicit arrays"; verification\v17_f_explicitarray_array; verification\v17_f_explicitarray_scalar -->

The `$IF` flags `_EXPLICIT_` and `_EXPLICITARRAY_` tell whether the program has these options.
<!-- src: compiler/pipeline "Precompiler flags"; verification\v26_x28_flag_explicit -->

## Errors

These are compile errors:

- Under `OPTION _EXPLICIT`, a variable with no declaration: "variable `x` (SINGLE) is not declared
  (`OPTION _EXPLICIT`)".
  <!-- src: language/constants "OPTION _EXPLICIT"; verification\v17_f_explicit_undeclared -->
- Under either option, an array with no `DIM`: "array `a` (SINGLE) is not declared (`OPTION _EXPLICIT` or
  `OPTION _EXPLICITARRAY`)".
  <!-- src: language/arrays-and-types "Implicit arrays"; verification\v17_f_explicitarray_array -->
- `OPTION BASE` with another number, such as 2: "`OPTION BASE` takes `0` or `1`". With nothing or a name after it:
  "expected `0` or `1` after `OPTION BASE`".
  <!-- src: language/arrays-and-types "OPTION BASE"; verification\v24_x64_option_base_2; verification\v24_x65_option_base_alone; verification\v24_x66_option_base_variable -->
- `OPTION EXPLICIT` without the underscore: "expected `OPTION BASE`, `OPTION _EXPLICIT` or `OPTION _EXPLICITARRAY`".
  <!-- src: language/constants "OPTION _EXPLICIT"; verification\v17_f_explicit_no_underscore -->

## Differences

From QuickBASIC 4.5: under `OPTION BASE 1`, `DIM x(0)` is the error "Lower bound exceeds upper bound" there.
QB64Rust gives the array the one element 0, as QB64pe does (`DIVERGENCES-QB45.md` Q-016).
<!-- src: Q-016 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:option_1 -->
```qb64
OPTION _EXPLICIT
OPTION BASE 1
DIM days(7) AS STRING
DIM i AS INTEGER
PRINT "days runs from"; LBOUND(days); "to"; UBOUND(days)
DIM hours(0 TO 23) AS INTEGER
PRINT "hours runs from"; LBOUND(hours); "to"; UBOUND(hours)
FOR i = 1 TO 3
    days(i) = "day" + STR$(i)
NEXT i
PRINT days(1); days(3)
END
```

Output:

```text
days runs from 1 to 7 
hours runs from 0 to 23 
day 1day 3

Press any key to continue
```
<!-- end generated -->

## See also

`DIM`, `LBOUND`, `UBOUND`, `$IF`.
