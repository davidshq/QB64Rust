# WIDTH

`WIDTH` sets the number of text columns and rows of the screen, as `WIDTH 40` or `WIDTH 80, 25`. With a file number it
sets the line width of a file.

## Syntax

<!-- begin generated: syntax -->
```text
WIDTH [columns%][, rows%]
WIDTH {file_number|device}, columnwidth%
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

`WIDTH` is compiled in every form QB64pe accepts, with its arguments, and changes the screen as its program does.
<!-- src: language/screen-output "Text-screen statements"; tests\reference\examples\width_1 -->

`WIDTH(n) = v` stores into an array named `WIDTH`, as QB64pe does.
<!-- src: language/arrays-and-types "Implicit arrays" -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/screen-output "Text-screen statements" -->

## Differences

From QuickBASIC 4.5: whether `WIDTH , 60` in `SCREEN 12` keeps a color set by `PALETTE` differs (`DIVERGENCES-QB45.md`
Q-035).
<!-- src: Q-035 -->

## Availability

<!-- begin generated: availability -->
Supported. All 20 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:width_1 -->
```qb64
WIDTH 40
PRINT "WIDTH 40 sets 40 columns."
END
```

Output:

```text
WIDTH 40 sets 40 columns.

Press any key to continue
```
<!-- end generated -->

## See also

`SCREEN`, `LOCATE`.
