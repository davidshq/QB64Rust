# $STATIC

`'$STATIC` makes the arrays declared below it static again, after a `'$DYNAMIC`. It stands in a comment. Static is
what arrays with constant bounds are anyway without either metacommand.

## Syntax

```text
'$STATIC
REM $STATIC
```

## Behavior

Below `'$STATIC`, a `DIM` with constant bounds makes a static array: it exists from the start of the program and
`REDIM` of it raises error 10.
<!-- src: compiler/pipeline "$STATIC and $DYNAMIC take effect"; tests\reference\examples\meta-static_1 -->

A `DIM` with a bound that is not a constant still makes a dynamic array, and so does a `DIM` in a procedure that is
not `STATIC`.
<!-- src: language/arrays-and-types "Dynamic arrays" -->

## Errors

- `$STATIC` outside a comment is a compile error, with QB64pe's "Syntax error" as a note.
  <!-- src: compiler/pipeline "Metacommands the old compiler does not know" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:meta-static_1 -->
```qb64
'$DYNAMIC
DIM a(5) AS LONG
'$STATIC
DIM b(5) AS LONG
REDIM a(8) AS LONG
PRINT "a is dynamic:"; UBOUND(a)
ON ERROR GOTO handler
REDIM b(8) AS LONG
PRINT "b is static:"; UBOUND(b)
END

handler:
PRINT "Error"; ERR; "for REDIM of b"
RESUME NEXT
```

Output:

```text
a is dynamic: 8 
Error 10 for REDIM of b
b is static: 5 

Press any key to continue
```
<!-- end generated -->

## See also

[`$DYNAMIC`](meta-dynamic.md), [`DIM`](dim.md), [`STATIC`](static.md).
