# PCOPY

`PCOPY from, to` copies one page of the screen onto another, as a program draws on a hidden page and then shows it.

## Syntax

```text
PCOPY sourcepage, destinationpage
```

## Behavior

After text is printed on page 1, `PCOPY 1, 0` puts it on page 0 too.
<!-- src: language/screen-output "Drawing statements"; tests\reference\examples\pcopy_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `PCOPY 1`: "`PCOPY` takes 2 arguments".
  <!-- src: language/screen-output "Drawing statements"; verification\v27_x34_pcopy_one -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:pcopy_1 -->
```qb64
img& = _NEWIMAGE(80, 25, 0)
SCREEN img&, , 1, 0
PRINT "page 1";
PCOPY 1, 0
SCREEN , , 0, 0
_SOURCE img&
s$ = CHR$(SCREEN(1, 1)) + CHR$(SCREEN(1, 2)) + CHR$(SCREEN(1, 3)) + CHR$(SCREEN(1, 4))
_DEST _CONSOLE
PRINT "Page 0 after PCOPY 1, 0 starts with: "; s$
END
```

Output:

```text
Page 0 after PCOPY 1, 0 starts with: page

Press any key to continue
```
<!-- end generated -->

## See also

`SCREEN`, `_DISPLAY`.
