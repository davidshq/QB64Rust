# COMMON

`COMMON` declares variables and arrays of the main module, as `DIM` does. `COMMON SHARED` also makes them visible
in every procedure. In QuickBASIC it passed values to a program started with `CHAIN`, which QB64Rust does not
support yet.

## Syntax

```text
COMMON [SHARED] name [AS type] [, name() [AS type]]...
```

## Behavior

`COMMON name AS type` declares the variable as `DIM` does, and `COMMON SHARED` as `DIM SHARED` does, wherever the
line stands in the main module. Without `SHARED`, a procedure does not see it.
<!-- src: language/arrays-and-types "COMMON"; verification\v24_a_common; tests\reference\examples\common_1 -->

`COMMON name()` declares a dynamic array with no elements yet. What stands between its parentheses is ignored. A
`DIM` of the name and type below gives it its bounds, and `REDIM` gives it new ones. Using it before either raises
error 9.
<!-- src: language/arrays-and-types "COMMON"; tests\reference\examples\common_1 -->

The array has the number of dimensions of the first index, `DIM` or `REDIM` that names it.
<!-- src: language/arrays-and-types "COMMON" -->

## Errors

These are compile errors:

- `COMMON` in a SUB or FUNCTION: "`COMMON` cannot stand inside a SUB or FUNCTION".
  <!-- src: language/arrays-and-types "COMMON" -->
- A second declaration of a `COMMON` variable: "name already in use: `a`".
  <!-- src: language/arrays-and-types "COMMON" -->
- A block name, as in `COMMON /name/ a`: "a `COMMON` block name (`/name/`) is not accepted".
  <!-- src: language/arrays-and-types "COMMON" -->
- A `DIM` without `AS` of an array that `COMMON name() AS type` lists: "`DIM g` needs an `AS` clause: `g` is listed
  by `COMMON … AS`".
  <!-- src: language/arrays-and-types "COMMON"; D-063 -->

## Differences

From QB64pe: `COMMON SHARED g() AS DOUBLE` followed by `DIM g(3)` compiles there, and makes a second array `g` of the
default type. QB64Rust rejects the `DIM`, as QuickBASIC 4.5 and BASIC PDS 7.1 do (`DIVERGENCES-QB64PE.md` D-063).
<!-- src: D-063 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:common_1 -->
```qb64
COMMON SHARED level AS INTEGER, names() AS STRING
DIM names(1 TO 2) AS STRING
level = 3
names(1) = "Lannes": names(2) = "Soult"
Show
END

SUB Show
    PRINT "level"; level; names(1); " and "; names(2)
END SUB
```

Output:

```text
level 3 Lannes and Soult

Press any key to continue
```
<!-- end generated -->

## See also

`DIM`, `SHARED`, `CHAIN`.
