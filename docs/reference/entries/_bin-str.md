# _BIN$

`_BIN$` gives the binary text of a whole number: `_BIN$(5)` is `"101"`.

## Syntax

<!-- begin generated: syntax -->
```text
_BIN$(number)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number of any numeric type. A floating-point value is first rounded to a whole number.

## Result

A STRING of the digits `0` and `1`, with no `&B` and no leading zeros.

## Behavior

`_BIN$(n)` gives the binary digits of `n`: `_BIN$(255)` is `"11111111"` and `_BIN$(0)` is `"0"`.
<!-- src: tests\reference\examples\_bin-str_1; verification\v20_e_radix -->

A negative number gives its bits in the width of its type: `_BIN$(-1%%)` is eight ones. For a 64-bit value that is
not a variable, an element or a member, the width is the shortest one, as QB64pe gives it.
<!-- src: language/builtin-functions "String results"; tests\corpus\slice\s26_val_len_radix; tests\reference\examples\_bin-str_1 -->

## Errors

- A string as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_bin-str_1 -->
```qb64
PRINT _BIN$(5); " "; _BIN$(255); " "; _BIN$(0)
PRINT _BIN$(-1%%)
END
```

Output:

```text
101 11111111 0
11111111

Press any key to continue
```
<!-- end generated -->

## See also

[`HEX$`](hex-str.md), [`OCT$`](oct-str.md), [`VAL`](val.md), [`_READBIT`](_setbit.md).
