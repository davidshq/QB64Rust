# PRINT

`PRINT` writes text and numbers to the screen, the console or, with `#n`, a file. `?` is short for `PRINT`.

## Syntax

```text
PRINT [items] [;|,]
PRINT #filenumber, [items] [;|,]
? [items]
```

## Arguments

- `items`: strings and numbers, separated by `;` or `,`.

## Behavior

A number is printed as `STR$` gives it, with a blank before a value that is not negative, and then one space: `PRINT
x; -x` with `x = 7` prints ` 7 -7 `.
<!-- src: language/numeric-semantics "Printing numbers"; tests\corpus\slice\s06_print_items -->

A `;` between items prints nothing between them, and two items written side by side do the same. A string item is
printed as it is.
<!-- src: tests\corpus\slice\s06_print_items; tests\reference\examples\print_1 -->

A `,` moves to the next print zone. In a file and in a window a zone is 14 columns: `PRINT #1, "a", "b", 5` writes
`a`, `b` and ` 5 ` at columns 1, 15 and 29.
<!-- src: verification\v22_b_print; tests\reference\examples\print_1; Q-006 -->

`PRINT` ends the line, unless its last element is a `;` or a `,`. `PRINT` alone prints an empty line.
<!-- src: tests\corpus\slice\s06_print_items; verification\v22_b_print -->

`TAB(n)` and `SPC(n)` among the items move to column `n` or print `n` blanks.
<!-- src: language/screen-output "TAB and SPC in text output" -->

`PRINT #n` writes the same text to file `n`.
<!-- src: language/file-io "PRINT to a file"; verification\v22_b_print -->

## Differences

On the console, a `,` moves to 10-column zones on Windows, and on Linux it prints one space, as in QB64pe. In a file
and in a window the zones are 14 columns, as in QuickBASIC 4.5 (`DIVERGENCES-QB45.md` Q-006).
<!-- src: D-011; Q-006 -->

From QB64pe: with the console's output redirected, its `PRINT` with a `,` can hang. QB64Rust counts the console's
column itself (`DIVERGENCES-QB64PE.md` D-011).
<!-- src: D-011 -->

## Availability

<!-- begin generated: availability -->
Supported. All 131 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:print_1 -->
```qb64
x = 7: s$ = "men"
PRINT "Ney has"; 3000; s$
PRINT "[" + "joined" + "]"
PRINT x; -x; 1.5
PRINT "no line end ";
PRINT "here"
? "A question mark is PRINT."
OPEN "print_1.tmp" FOR OUTPUT AS #1
PRINT #1, "a", "b", 5
CLOSE #1
OPEN "print_1.tmp" FOR INPUT AS #1
LINE INPUT #1, l$
CLOSE #1
KILL "print_1.tmp"
PRINT "[" + l$ + "]"
END
```

Output:

```text
Ney has 3000 men
[joined]
 7 -7  1.5 
no line end here
A question mark is PRINT.
[a             b              5 ]

Press any key to continue
```
<!-- end generated -->

## See also

[`PRINT USING`](print-using.md), [`WRITE`](write.md), [`LPRINT`](lprint.md), [`TAB`](tab.md), [`SPC`](spc.md), [`_ECHO`](_echo.md).
