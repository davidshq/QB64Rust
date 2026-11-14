# PALETTE

`PALETTE entry, color` sets one entry of a palette screen's colors. `PALETTE` alone restores the default palette, and
`PALETTE USING` sets many from an array.

## Syntax

```text
PALETTE [entry, color]
PALETTE USING array[(index)]
```

## Behavior

`PALETTE 5, 63` sets entry 5 to the VGA color 63, pure red: `_PALETTECOLOR(5)` is then `&HFFFF0000`.
<!-- src: language/screen-output "Drawing statements"; tests\reference\examples\palette_1 -->

`PALETTE USING` takes the colors from the elements of an INTEGER or LONG array, from the element named or the first.
<!-- src: language/screen-output "PALETTE USING" -->

## Errors

- The forms QB64pe rejects are compile errors; `PALETTE USING` raises its run-time errors. For example:
  - `PALETTE 1`: "this `PALETTE` statement matches none of its forms".
  - `PALETTE 1, 2, 3`: "this `PALETTE` statement matches none of its forms".
  - `PALETTE USING`: "`PALETTE USING` needs an array after `USING`".
  <!-- src: language/screen-output "PALETTE USING"; verification\v27_x06_palette_one; verification\v27_x43_palette_three; verification\v57_u01_palette_using_nothing -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:palette_1 -->
```qb64
img& = _NEWIMAGE(40, 30, 256)
_DEST img&: _SOURCE img&
PALETTE 5, 63
PSET (1, 1), 5
_DEST _CONSOLE
PRINT HEX$(_PALETTECOLOR(5, img&))
END
```

Output:

```text
FFFF0000

Press any key to continue
```
<!-- end generated -->

## See also

[`_PALETTECOLOR`](_palettecolor.md), [`COLOR`](color.md), [`SCREEN`](screen.md).
