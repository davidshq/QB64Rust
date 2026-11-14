# STR$

`STR$` gives the text of a number, as `PRINT` would show it but without the space after: `STR$(5)` is `" 5"`, with
a blank where the sign would be.

## Syntax

<!-- begin generated: syntax -->
```text
STR$(number)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `number`: a number of any numeric type.

## Result

A STRING.

## Behavior

A value that is not negative gets a blank before it, and a negative one its minus: `STR$(5)` is `" 5"` and
`STR$(-5)` is `"-5"`. `LTRIM$(STR$(n))` drops the blank.
<!-- src: language/builtin-functions "String results"; tests\reference\examples\str-str_1 -->

The digits are those of the value's type: 7 significant digits for a SINGLE, 16 for a DOUBLE and a `_FLOAT`.
`STR$(1 / 3)` has 16, since `/` of two integers is computed in `_FLOAT`.
<!-- src: language/numeric-semantics "Printing numbers"; verification\v20_c_string_edges; tests\reference\examples\str-str_1 -->

`STR$` of a negative zero is `" 0"`. A large or small value has an exponent: `" 1E+20"` for a SINGLE, `" 1D+300"` for
a DOUBLE.
<!-- src: verification\v20_c_string_edges -->

## Errors

- A string as the argument is a compile error: "`STR$` needs a number".
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:str-str_1 -->
```qb64
PRINT "["; STR$(5); "] ["; STR$(-5); "]"
PRINT "["; STR$(1 / 3); "]"
n$ = LTRIM$(STR$(42))
PRINT "Without the blank: ["; n$; "]"
END
```

Output:

```text
[ 5] [-5]
[ .3333333333333333]
Without the blank: [42]

Press any key to continue
```
<!-- end generated -->

## See also

[`VAL`](val.md), [`_TOSTR$`](_tostr-str.md), [`PRINT USING`](print-using.md), [`HEX$`](hex-str.md).
