# DRAW

`DRAW` draws with a string of short commands: `"C3 R10 D10"` sets color 3, then draws 10 pixels right and 10 down.

## Syntax

```text
DRAW commands$
```

## Behavior

The commands move from the last point drawn and draw as they go: `C` sets the color, `R`, `L`, `U` and `D` move right,
left, up and down.
<!-- src: language/screen-output "Drawing statements"; tests\reference\examples\draw_1 -->

The pixels are those QB64pe's program draws.
<!-- src: language/screen-output "Drawing statements" -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/screen-output "Drawing statements" -->

## Differences

From QuickBASIC 4.5: at scale 6 (`S6`) some lines end one pixel elsewhere there, so an outline QB 4.5 closes can be
open by one pixel row in QB64Rust, as in QB64pe (`DIVERGENCES-QB45.md` Q-019).
<!-- src: Q-019 -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:draw_1 -->
```qb64
img& = _NEWIMAGE(40, 30, 256)
_DEST img&: _SOURCE img&
PSET (5, 5), 0
DRAW "C3 R10 D10"
_DEST _CONSOLE
PRINT POINT(10, 5); POINT(15, 5); POINT(15, 10); POINT(15, 15); POINT(5, 15)
END
```

Output:

```text
 3  3  3  3  0 

Press any key to continue
```
<!-- end generated -->

## See also

`LINE`, `PSET`.
