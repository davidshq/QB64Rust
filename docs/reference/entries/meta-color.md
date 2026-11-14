# $COLOR

`$COLOR:0` and `$COLOR:32` give a program named constants for colors: `Red`, `Yellow`, `LightBlue` and the rest.
`$COLOR:0` gives the 16 text colors as numbers 0 to 15; `$COLOR:32` gives 32-bit colors.

## Syntax

```text
$COLOR:0
$COLOR:32
```

## Behavior

The constants work as if they were defined above the program's first line, wherever the `$COLOR` line stands:
also when it is inside a SUB, and when a statement above it already uses a color's name.
<!-- src: language/constants "Colour constants"; verification\v45_x41; verification\v45_x42 -->

With `$COLOR:0`, `Black` is 0, `Blue` 1, `Green` 2, `Cyan` 3, `Red` 4, `Magenta` 5, `Brown` 6, `White` 7,
`Gray` 8, `LightBlue` 9, `LightGreen` 10, `LightCyan` 11, `LightRed` 12, `LightMagenta` 13, `Yellow` 14 and
`BrightWhite` 15. A `CONST` may use them, and a SUB sees them.
<!-- src: language/constants "Colour constants"; verification\v45_b_color0 -->

With `$COLOR:32`, each constant is an `_UNSIGNED LONG` color whose alpha is 255: `HEX$(Red)` is `FFFF0000`.
<!-- src: language/constants "Colour constants"; verification\v45_b_color32 -->

The metacommand may be written in lower case and may stand twice. In a `$IF` branch that is not compiled it gives
nothing.
<!-- src: verification\v45_x50; verification\v45_x44; verification\v45_x54 -->

## Errors

These are compile errors:

- Both `$COLOR:0` and `$COLOR:32` in one program: "`$COLOR:0` and `$COLOR:32` cannot be used in one program".
  <!-- src: language/constants "Colour constants"; verification\v45_x45 -->
- Any other form, such as `$COLOR:16`, `$COLOR` alone, `$COLOR: 0` or a word after it: "`$COLOR` takes `:0` or
  `:32`, alone on its line".
  <!-- src: verification\v45_x46; verification\v45_x51; verification\v45_x52; verification\v45_x53 -->
- A variable of a color's name: "name already in use: `Red`". A store into a color: "cannot assign to the constant
  `Red`".
  <!-- src: language/constants "Colour constants"; verification\v45_x47; verification\v45_x55 -->
- A `CONST` above the `$COLOR` line that names a color: "`Black` is a `$COLOR` constant, which a `CONST` above the
  `$COLOR` line cannot name".
  <!-- src: D-030; verification\v45_x59 -->

## Differences

From QB64pe: a `CONST` above the `$COLOR` line that names a color is an error there too, but its message is
"Unexpected element". QB64Rust says that the name is a `$COLOR` constant (`DIVERGENCES-QB64PE.md` D-030).
<!-- src: D-030; verification\v45_x59 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:meta-color_1 -->
```qb64
$COLOR:0
PRINT "Red is"; Red; "and Yellow is"; Yellow
CONST Warning = LightRed
PRINT "Warning is"; Warning
END
```

Output:

```text
Red is 4 and Yellow is 14 
Warning is 12 

Press any key to continue
```
<!-- end generated -->

## See also

[`COLOR`](color.md), [`CONST`](const.md), [`_RGB32`](_rgb32.md).
