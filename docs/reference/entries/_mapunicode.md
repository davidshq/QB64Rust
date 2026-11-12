# _MAPUNICODE

`_MAPUNICODE code TO char` sets which Unicode code point an ASCII character shows, and `_MAPUNICODE(char)` gives it.

## Syntax

```text
_MAPUNICODE unicode TO ascii
unicode = _MAPUNICODE(ascii)
```

## Behavior

After `_MAPUNICODE 9731 TO 65`, `_MAPUNICODE(65)` is 9731.
<!-- src: language/fonts "Unicode text and the print mode"; tests\reference\examples\_mapunicode_1 -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `_MAPUNICODE 1, 2`: "this `_MAPUNICODE` statement matches none of its forms".
  - `_MAPUNICODE 1`: "this `_MAPUNICODE` statement matches none of its forms".
  - `PRINT _MAPUNICODE`: "`_MAPUNICODE` takes 1 argument".
  <!-- src: language/fonts "Forms of the font built-ins"; verification\v31_x45_mapunicode_comma; verification\v31_x46_mapunicode_one; verification\v31_x47_mapunicode_fn_bare -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_mapunicode_1 -->
```qb64
_MAPUNICODE 9731 TO 65
PRINT "Character 65 now shows code point"; _MAPUNICODE(65)
END
```

Output:

```text
Character 65 now shows code point 9731 

Press any key to continue
```
<!-- end generated -->

## See also

[`_UPRINTSTRING`](_uprintstring.md), [`CHR$`](chr-str.md).
