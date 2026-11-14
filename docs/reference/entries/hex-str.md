# HEX$

`HEX$` gives the hexadecimal text of a whole number: `HEX$(255)` is `"FF"`.

## Syntax

<!-- begin generated: syntax -->
```text
HEX$(decimalNumber)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A number of any numeric type. A floating-point value is first rounded to a whole number.

## Result

A STRING of the digits `0` to `9` and `A` to `F`, with no `&H` and no leading zeros.

## Behavior

`HEX$(n)` gives the hexadecimal digits of `n`: `HEX$(4096)` is `"1000"` and `HEX$(0)` is `"0"`. A floating-point
value is rounded: `HEX$(10.6)` is `"B"`.
<!-- src: tests\reference\examples\hex-str_1; verification\v20_e_radix -->

A negative number gives its bits in the width of its type: `HEX$(-1%)` is `"FFFF"` and `HEX$(-1&)` is
`"FFFFFFFF"`. For a 64-bit value that is not a variable, an element or a member, the width is the shortest one, as
QB64pe gives it.
<!-- src: language/builtin-functions "String results"; tests\corpus\slice\s26_val_len_radix -->

`HEX$` of an unsigned value gives its digits: with `u~& = 4294967295`, `HEX$(u~&)` is `"FFFFFFFF"`.
<!-- src: language/builtin-functions "Supported built-in functions" -->

## Errors

- A string as the argument is a compile error: "`HEX$` needs a number".
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 42 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:hex-str_1 -->
```qb64
PRINT HEX$(255); " "; HEX$(4096); " "; HEX$(0)
PRINT HEX$(-1%); " "; HEX$(-1&)
PRINT HEX$(10.6)
END
```

Output:

```text
FF 1000 0
FFFF FFFFFFFF
B

Press any key to continue
```
<!-- end generated -->

## See also

[`OCT$`](oct-str.md), [`_BIN$`](_bin-str.md), [`VAL`](val.md), [`STR$`](str-str.md).
