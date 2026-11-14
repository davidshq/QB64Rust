# STRING

`STRING` is the type of text: a string of bytes. A plain `STRING` grows and shrinks with what is stored in it; a
`STRING * n` always holds exactly `n` bytes. The suffix is `$`.

## Syntax

```text
DIM name AS STRING
DIM name AS STRING * n
name$
name$n
```

## Arguments

- `n`: the length of a fixed-length string, a number literal or the name of a numeric constant.

## Behavior

A `STRING` variable holds any number of bytes, and starts empty. `LEN` gives its length.
<!-- src: tests\reference\examples\string_1; verification\v24_a_define -->

`DIM name AS STRING * n`, or the suffix form `name$n`, declares a string of exactly `n` bytes. `name$n` is a
variable other than `name$`.
<!-- src: language/fixed-length-strings "Fixed-length string declarations" -->

After `DIM name AS STRING * n`, `name$` is that same fixed-length variable, as a value and as a place, also in a
procedure that shares it: after `DIM SHARED pathsep AS STRING * 1`, `pathsep$ = "\"` in a SUB stores into
`pathsep`. A `DIM name$` beside it declares nothing new.
<!-- src: language/fixed-length-strings "A suffixed name of a fixed-length string variable"; language/fixed-length-strings "Declaring the $ name of a fixed-length string variable"; verification\v52_x39_fixed_scalar_dollar_shared_sub -->

A fixed-length string starts as `n` NUL bytes (0), in QB 4.5 too. A shorter value stored in it is padded with spaces, and a
longer one is cut: with `DIM f AS STRING * 4`, `f = "ab"` holds `"ab  "` and `f = "abcdef"` holds `"abcd"`.
<!-- src: language/fixed-length-strings "Fixed-length string declarations"; language/fixed-length-strings "Assignment to a fixed-length string"; Q-005; tests\reference\examples\string_1 -->

A fixed-length string is read with its padding, also in a comparison: after `f = "ab"`, `f = "ab"` is false and
`f = "ab  "` is true.
<!-- src: language/fixed-length-strings "Assignment to a fixed-length string" -->

The length `n` is read as a 32-bit integer. A constant of a float is rounded half to even.
<!-- src: language/fixed-length-strings "Fixed-length string declarations" -->

Two strings compare byte by byte. A string compared with a number is a compile error.
<!-- src: language/numeric-semantics "Comparisons"; verification\v17_g_ops -->

`_UNSIGNED STRING` is taken as `STRING`.
<!-- src: language/numeric-semantics "Numeric types"; verification\v21_a_decls -->

## Errors

These are compile errors:

- A length of 0: "a fixed-length string's length must be 1 to 2147483647 (it is read in 32 bits)".
  <!-- src: language/fixed-length-strings "Fixed-length string declarations"; verification\v21_x31_fixed_len_0 -->
- An expression as the length, such as `STRING * (2 + 3)`: "expected a size after `*`".
  <!-- src: language/fixed-length-strings "Fixed-length string declarations"; verification\v21_x32_fixed_len_expr -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:string_1 -->
```qb64
DIM name1 AS STRING
name1 = "Wellington"
PRINT name1; " has"; LEN(name1); "characters"
DIM code AS STRING * 4
code = "ab"
PRINT "["; code; "]"; LEN(code)
code = "abcdef"
PRINT "["; code; "]"
title$ = "Duke"
PRINT title$ + " of " + name1
END
```

Output:

```text
Wellington has 10 characters
[ab  ] 4 
[abcd]
Duke of Wellington

Press any key to continue
```
<!-- end generated -->

## See also

[`LEN`](len.md), [`DEFSTR`](defint.md), [`_DEFINE`](_define.md), [`LEFT$`](left-str.md).
