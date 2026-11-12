# TAB

`TAB(n)` in a `PRINT` moves to column `n`, so that text lines up in columns.

## Syntax

<!-- begin generated: syntax -->
```text
TAB(column%)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `column%`: the column, 1 for the first.

## Result

A string, used as an item of `PRINT`.

## Behavior

`PRINT "Name"; TAB(12); "Men"` prints `Men` from column 12. When the cursor is already past column `n`, the text goes
on the next line, at column `n`.
<!-- src: language/screen-output "TAB and SPC in text output"; tests\reference\examples\tab_1 -->

A string item may follow `TAB` with no separator.
<!-- src: language/screen-output "TAB and SPC in text output" -->

`TAB` works in a window, on the console and in a file, as in QB64pe.
<!-- src: language/screen-output "TAB and SPC in text output"; verification\v22_b_print -->

## Errors

- A number or another `TAB` right after `TAB` with no separator, or a string as the column, is a compile error.
  <!-- src: language/screen-output "TAB and SPC in text output"; language/builtin-functions "Supported built-in functions" -->

## Availability

<!-- begin generated: availability -->
Supported. All 24 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:tab_1 -->
```qb64
img& = _NEWIMAGE(80, 25, 0)
_DEST img&: _SOURCE img&
PRINT "Name"; TAB(12); "Men"
PRINT "Ney"; TAB(12); 3000
_DEST _CONSOLE
PRINT "["; RowText$(1); "]"
PRINT "["; RowText$(2); "]"
END

FUNCTION RowText$ (r)
    t$ = ""
    FOR c = 1 TO 30
        t$ = t$ + CHR$(SCREEN(r, c))
    NEXT c
    RowText$ = RTRIM$(t$)
END FUNCTION
```

Output:

```text
[Name       Men]
[Ney         3000]

Press any key to continue
```
<!-- end generated -->

## See also

`SPC`, `PRINT`, `LOCATE`.
