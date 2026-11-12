# COLOR

`COLOR foreground, background` sets the colors of the text and drawing that follow.

## Syntax

```text
COLOR [foreground][, background]
```

## Behavior

After `COLOR 12, 1`, `CLS` fills with color 1 and a `PSET` with no color draws in 12.
<!-- src: language/screen-output "Text-screen statements"; tests\reference\examples\color_1 -->

Either color may be left out. The colors are compiled in every form QB64pe accepts.
<!-- src: language/screen-output "Text-screen statements" -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `COLOR t`: "argument 1 of `COLOR` needs a number".
  <!-- src: language/screen-output "Text-screen statements"; verification\v27_x10_color_string -->

## Availability

<!-- begin generated: availability -->
Supported. All 19 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:color_1 -->
```qb64
img& = _NEWIMAGE(40, 30, 256)
_DEST img&: _SOURCE img&
COLOR 12, 1
CLS
PSET (1, 1)
_DEST _CONSOLE
PRINT "After CLS the background is"; POINT(30, 20); "and PSET draws in"; POINT(1, 1)
END
```

Output:

```text
After CLS the background is 1 and PSET draws in 12 

Press any key to continue
```
<!-- end generated -->

## See also

`PALETTE`, `_DEFAULTCOLOR`, `$COLOR`, `CLS`.
