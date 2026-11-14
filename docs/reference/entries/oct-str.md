# OCT$

`OCT$` gives the octal text of a whole number: `OCT$(8)` is `"10"`.

## Syntax

<!-- begin generated: syntax -->
```text
OCT$(number)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number of any numeric type. A floating-point value is first rounded to a whole number.

## Result

A STRING of the digits `0` to `7`, with no `&O` and no leading zeros.

## Behavior

`OCT$(n)` gives the octal digits of `n`: `OCT$(255)` is `"377"` and `OCT$(0)` is `"0"`.
<!-- src: tests\reference\examples\oct-str_1; verification\v20_e_radix -->

A negative number gives its bits in the width of its type: `OCT$(-1%)` is `"177777"`. For a 64-bit value that is
not a variable, an element or a member, the width is the shortest one, as QB64pe gives it.
<!-- src: language/builtin-functions "String results"; tests\corpus\slice\s26_val_len_radix -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:oct-str_1 -->
```qb64
PRINT OCT$(8); " "; OCT$(255); " "; OCT$(0)
PRINT OCT$(-1%)
END
```

Output:

```text
10 377 0
177777

Press any key to continue
```
<!-- end generated -->

## See also

[`HEX$`](hex-str.md), [`_BIN$`](_bin-str.md), [`VAL`](val.md).
