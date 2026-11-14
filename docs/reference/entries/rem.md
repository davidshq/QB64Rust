# REM

`REM` starts a comment: the rest of the line is not compiled. A `'` does the same.

## Syntax

```text
REM text
' text
statement: REM text
statement ' text
```

## Behavior

`REM` stands where a statement may stand: at the start of a line, or after a colon. Everything after it, to the
end of the line, is a comment.
<!-- src: tests\reference\examples\rem_1; verification\v41_x74_rem_after; verification\v16_m1_rem_after_colon -->

A `'` may also follow a statement with no colon between: `PRINT "one" ' a comment`.
<!-- src: tests\reference\examples\rem_1 -->

`IF condition THEN REM text` is a single-line `IF` with an empty body, not the start of an `IF` block.
<!-- src: verification\v16_m4_line_if_block -->

In an unquoted `DATA` item, `REM` is part of the item: `DATA apples REM and pears, plums` holds the two items
`apples REM and pears` and `plums`.
<!-- src: verification\v16_m2_data_rem; tests\reference\examples\rem_1 -->

`REM` starts a comment also in a `TYPE` block. A member line `rem AS INTEGER` is a comment, so the type has no
member `rem`.
<!-- src: verification\v61_m_rem -->

A comment may hold a metacommand: `REM $INCLUDE: 'file'` and `' $INCLUDE: 'file'` include the file, and
`$STATIC` and `$DYNAMIC` work in a comment too. Any other comment is ignored, also one that starts with `$`.
<!-- src: compiler/pipeline "Comment metacommands are never ignored"; verification\v16_m1_rem_after_colon -->

## Errors

- A use of a `TYPE` member named `rem`: "`REM` is not a member of `TYPE T` (`o.rem`)", since its line was a comment.
  <!-- src: verification\v61_m_rem -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:rem_1 -->
```qb64
REM This whole line is a comment.
' So is this one.
PRINT "one" ' a comment after a statement
PRINT "two": REM REM after a colon
DATA apples REM and pears, plums
READ a$, b$
PRINT a$; " / "; b$
END
```

Output:

```text
one
two
apples REM and pears / plums

Press any key to continue
```
<!-- end generated -->

## See also

[`$INCLUDE`](meta-include.md), [`$DYNAMIC`](meta-dynamic.md), [`DATA`](data.md).
