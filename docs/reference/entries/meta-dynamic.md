# $DYNAMIC

`'$DYNAMIC` makes the arrays declared below it dynamic, so that `REDIM` can give them new bounds. It stands in a
comment. `'$STATIC` goes back to static arrays.

## Syntax

```text
'$DYNAMIC
REM $DYNAMIC
```

## Behavior

Below `'$DYNAMIC`, a `DIM` with constant bounds makes a dynamic array: its elements are made when the `DIM` runs,
and `REDIM` can change its bounds. Without it, `REDIM` of such an array raises error 10.
<!-- src: compiler/pipeline "$STATIC and $DYNAMIC take effect"; language/arrays-and-types "Dynamic arrays"; tests\reference\examples\meta-dynamic_1 -->

`'$STATIC` below it makes the following `DIM`s static again.
<!-- src: compiler/pipeline "$STATIC and $DYNAMIC take effect"; tests\reference\examples\meta-static_1 -->

Below `'$DYNAMIC`, an array used without `DIM` is dynamic too. The first statement that names it makes it, and when
that statement runs again it raises error 10.
<!-- src: language/arrays-and-types "Dynamic arrays" -->

A comment that holds `$DYNAMIC` after other words, such as `' note $DYNAMIC`, is an ordinary comment.
<!-- src: compiler/pipeline "$STATIC and $DYNAMIC take effect" -->

## Errors

- `$DYNAMIC` outside a comment is a compile error: "unknown metacommand `$DYNAMIC`".
  <!-- src: compiler/pipeline "Metacommands the old compiler does not know"; verification\v24_x38_dynamic_bare_metacommand -->

## Differences

From QuickBASIC 4.5: an array used without `DIM` below `'$DYNAMIC` is made once there, and works in a loop. In
QB64Rust, as in QB64pe, a second run of the statement that first names it raises error 10 (`DIVERGENCES-QB45.md`
Q-030).
<!-- src: Q-030 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:meta-dynamic_1 -->
```qb64
'$DYNAMIC
DIM big(5) AS LONG
big(5) = 1
REDIM big(10) AS LONG
PRINT "big can be given new bounds:"; UBOUND(big)
END
```

Output:

```text
big can be given new bounds: 10 

Press any key to continue
```
<!-- end generated -->

## See also

`$STATIC`, `REDIM`, `DIM`.
