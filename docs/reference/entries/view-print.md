# VIEW PRINT

`VIEW PRINT top TO bottom` limits text output to a range of rows, which then scroll on their own. `VIEW PRINT` alone
uses the whole screen again.

## Syntax

```text
VIEW PRINT [top TO bottom]
```

## Behavior

After `VIEW PRINT 5 TO 10`, `PRINT` starts at row 5.
<!-- src: language/screen-output "Text-screen statements"; tests\reference\examples\view-print_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `VIEW PRINT 2`: "this `VIEW` statement matches none of its forms".
  <!-- src: language/screen-output "Text-screen statements"; verification\v27_x05_view_print_one -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:view-print_1 -->
```qb64
img& = _NEWIMAGE(80, 25, 0)
_DEST img&
VIEW PRINT 5 TO 10
PRINT "inside"
r = CSRLIN
_DEST _CONSOLE
PRINT "After VIEW PRINT 5 TO 10, PRINT starts at row"; r - 1
END
```

Output:

```text
After VIEW PRINT 5 TO 10, PRINT starts at row 5 

Press any key to continue
```
<!-- end generated -->

## See also

[`VIEW`](view.md), [`LOCATE`](locate.md), [`CLS`](cls.md).
