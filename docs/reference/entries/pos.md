# POS

`POS(0)` gives the column of the cursor, 1 for the first. `CSRLIN` gives its row.

## Syntax

<!-- begin generated: syntax -->
```text
POS(0)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number, which is not used. `POS(0)` is the usual form.

## Result

An INTEGER.

## Behavior

After `PRINT "abc";` on a new line, `POS(0)` is 4.
<!-- src: language/screen-output "Functions that read the screen and the keyboard"; tests\reference\examples\pos_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:pos_1 -->
```qb64
img& = _NEWIMAGE(80, 25, 0)
_DEST img&: _SOURCE img&
PRINT "abc";
c = POS(0)
_DEST _CONSOLE
PRINT "After three characters POS(0) is"; c
END
```

Output:

```text
After three characters POS(0) is 4 

Press any key to continue
```
<!-- end generated -->

## See also

`CSRLIN`, `LOCATE`, `TAB`.
