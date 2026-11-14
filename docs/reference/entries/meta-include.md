# $INCLUDE

`$INCLUDE` puts the text of another file into the program at the line where it stands, as if it were written
there. It is how a program is split into several files.

## Syntax

```text
'$INCLUDE: 'file'
REM $INCLUDE: 'file'
```

`$INCLUDE` always stands in a comment: after `'` or after `REM`. The name is between single quotes. Blanks are
allowed around the colon.

## Behavior

The file's lines are read after the line that holds the `$INCLUDE`, and they compile as part of the program. A
statement may stand before it on the same line: `PRINT "a": REM $INCLUDE: 'x.bi'`.
<!-- src: compiler/pipeline "Included files"; verification\v16_m7_comment_blank; verification\v16_m1_rem_after_colon -->

The comment may have blanks around the colon, `'$INCLUDE :  'x.bi'`. When one comment holds two `$INCLUDE`s, only
the last is included.
<!-- src: verification\v16_m7_metacommand_form; verification\v16_m1_two_includes -->

A relative name is looked for in the folder of the file that holds the `$INCLUDE`, then relative to the compiler's
root folder. It is never looked for relative to the folder the compiler runs in, nor in the main file's folder
when the `$INCLUDE` stands in another included file.
<!-- src: compiler/pipeline "Included files"; verification\v16_m7_order; verification\v16_m7_main_folder -->

A file may be included more than once. Each time, its text is compiled again. A file that holds `$INCLUDEONCE` is
included only the first time.
<!-- src: compiler/pipeline "Included files"; verification\v16_m7_twice -->

A block may start in one file and end in another: a `FOR` in the main file may end with a `NEXT` in an included
file, and a SUB may be opened in one and closed in the other.
<!-- src: compiler/pipeline "Included files"; verification\v16_m7_block_across; verification\v16_m7_sub_across -->

A `$LET` or `$IF` in an included file works in file order, as if its text stood in the main file. An `$INCLUDE`
inside a `$IF` branch that is not taken is not read, so its file need not exist.
<!-- src: compiler/pipeline "Preprocessor"; verification\v16_m7_let_reaches_main; verification\v16_m7_metacommand_form -->

An error in an included file is reported at its own line in that file. At run time, `_ERRORLINE` gives the line of
the main file's `$INCLUDE`, and `_INCLERRORLINE` and `_INCLERRORFILE$` give the line and name in the included file.
<!-- src: compiler/pipeline "Included files"; language/error-handling "The line and file of an error"; verification\v43_b_include -->

A file holding SUBs and FUNCTIONs is usually included at the end of the program, after the main module's last
statement, and a file of `CONST`, `TYPE` and `DECLARE` lines at the top.
<!-- src: verification\v16_m7_sub_at_end -->

## Errors

These are compile errors:

- A file that cannot be found, and an empty name.
  <!-- src: compiler/pipeline "Included files"; verification\v16_m7_missing -->
- `$INCLUDE` outside a comment, such as `$INCLUDE: 'x.bi'` at the start of a line.
  <!-- src: compiler/pipeline "Included files"; verification\v16_m7_bare; verification\v16_m7_bare_blank -->
- Files included more than 100 deep, as when a file includes itself with nothing to stop it.
  <!-- src: compiler/pipeline "Included files"; verification\v16_m7_self -->
- A file of SUBs included inside a SUB.
  <!-- src: verification\v16_m7_sub_in_sub -->

A file that includes itself is no error when a `$IF` stops the second inclusion.
<!-- src: compiler/pipeline "Preprocessor"; verification\v16_m7_self_guarded -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:meta-include_1 -->
```qb64
'$INCLUDE: 'shapes.bi'
PRINT "A square has"; SIDES; "sides."
PRINT "Its perimeter for a side of 3 is"; Perimeter(3)
END
'$INCLUDE: 'shapes.bm'
```

The file `shapes.bi`:

```qb64
' shapes.bi: the declarations, included at the top of the program.
CONST SIDES = 4
```

The file `shapes.bm`:

```qb64
' shapes.bm: the procedures, included at the end of the program.
FUNCTION Perimeter (side)
    Perimeter = side * SIDES
END FUNCTION
```

Output:

```text
A square has 4 sides.
Its perimeter for a side of 3 is 12 

Press any key to continue
```
<!-- end generated -->

## See also

[`$INCLUDEONCE`](meta-includeonce.md), [`$IF`](meta-if.md), [`$LET`](meta-let.md), [`_INCLERRORLINE`](_inclerrorline.md).
