# CSRLIN

`CSRLIN` gives the row of the cursor, 1 for the top. `POS(0)` gives its column.

## Syntax

<!-- begin generated: syntax -->
```text
CSRLIN
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

An INTEGER.

## Behavior

After `LOCATE 5, 10: PRINT "x";`, `CSRLIN` is 5 and `POS(0)` is 11.
<!-- src: language/screen-output "Functions that read the screen and the keyboard"; tests\reference\examples\csrlin_1 -->

`CSRLIN` is written without parentheses.
<!-- src: language/screen-output "Functions that read the screen and the keyboard" -->

## Errors

- `CSRLIN()` with parentheses is a compile error.
  <!-- src: language/screen-output "Functions that read the screen and the keyboard" -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:csrlin_1 -->
```qb64
img& = _NEWIMAGE(80, 25, 0)
_DEST img&: _SOURCE img&
LOCATE 5, 10
PRINT "x";
r = CSRLIN: c = POS(0)
_DEST _CONSOLE
PRINT "After the x the cursor is at row"; r; "column"; c
END
```

Output:

```text
After the x the cursor is at row 5 column 11 

Press any key to continue
```
<!-- end generated -->

## See also

`POS`, `LOCATE`.
