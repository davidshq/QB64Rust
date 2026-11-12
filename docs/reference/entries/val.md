# VAL

`VAL` reads a number from the start of a string: `VAL("12")` is 12 and `VAL("12abc")` is 12. A string that does not
start with a number gives 0.

## Syntax

<!-- begin generated: syntax -->
```text
VAL(stringValue$[, numericalType])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `string$`: the text to read.
- A type, optional: `SINGLE`, `DOUBLE`, `_FLOAT` or an integer type, to read the number in.

## Result

A `_FLOAT`. With a type argument, a SINGLE, DOUBLE or `_FLOAT` as named, and an `_INTEGER64` for every integer type.

## Behavior

`VAL` reads digits, a sign, a decimal point and an exponent from the start of the string, and stops at the first
other character: `VAL("-3.5")` is -3.5, `VAL("1e3")` and `VAL("1d3")` are 1000, and `VAL("abc")` and `VAL("")` are
0.
<!-- src: language/builtin-functions "Result types"; tests\corpus\slice\s26_val_len_radix; tests\reference\examples\val_1 -->

Blanks and tabs inside the number are skipped: `VAL(" 1 2")` is 12. A comma ends it: `VAL("1,5")` is 1.
<!-- src: tests\corpus\slice\s26_val_len_radix -->

`&H`, `&O` and `&B` read a hexadecimal, octal or binary number: `VAL("&HFF")` is 255, and `VAL("&HFFFF")` is 65535,
not -1.
<!-- src: tests\corpus\slice\s26_val_len_radix; tests\reference\examples\val_1 -->

With a type, the number is read in it: `VAL("16777217", SINGLE)` is 16777216. With an integer type the result is not
narrowed to it: `VAL("40000", INTEGER)` is 40000, and a fraction is dropped: `VAL("2.7", INTEGER)` is 2.
<!-- src: language/builtin-functions "Result types"; tests\corpus\slice\s26_val_len_radix -->

With an unsigned type, a minus sign is dropped: `VAL("-1", _UNSIGNED LONG)` is 1.
<!-- src: language/builtin-functions "Supported built-in functions" -->

## Errors

These are compile errors:

- A number as the argument: "`VAL` needs a string".
  <!-- src: language/builtin-functions "Names and arity" -->
- A `_BIT` type as the second argument.
  <!-- src: language/builtin-functions "Supported built-in functions" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:val_1 -->
```qb64
PRINT VAL("12"); VAL("-3.5"); VAL("12abc"); VAL("abc")
PRINT VAL("&HFF"); VAL("&O17"); VAL("&B101")
PRINT VAL(" 1 2"); VAL("1e3")
END
```

Output:

```text
 12 -3.5  12  0 
 255  15  5 
 12  1000 

Press any key to continue
```
<!-- end generated -->

## See also

[`STR$`](str-str.md), [`HEX$`](hex-str.md), [`INPUT`](input.md).
