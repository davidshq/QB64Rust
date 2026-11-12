# _TOSTR$

`_TOSTR$` gives the text of a number without the blank `STR$` puts before it: `_TOSTR$(5)` is `"5"`. A second
argument sets the number of significant digits.

## Syntax

<!-- begin generated: syntax -->
```text
_TOSTR$(number[,digits])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The number, of any numeric type.
- The number of significant digits, optional.

## Result

A STRING.

## Behavior

`_TOSTR$(n)` is the number's text with no blank before it: `_TOSTR$(-5)` is `"-5"`.
<!-- src: verification\v20_c_string_edges; tests\reference\examples\_tostr-str_1 -->

A `_FLOAT` value shows all its digits: `_TOSTR$(1 / 3)` is `"0.3333333333333333333"`, with a 0 before the point.
<!-- src: verification\v20_c_string_edges; language/constants "CONST values" -->

With a second argument, the value is given with that many significant digits: `_TOSTR$(1 / 3, 5)` is `"0.33333"`.
<!-- src: verification\v20_c_string_edges; tests\reference\examples\_tostr-str_1 -->

## Errors

- A negative number of digits raises run-time error 5.
  <!-- src: verification\v20_c_string_edges -->
- A string as the number is a compile error: "argument 1 of `_TOSTR$` needs a number".
  <!-- src: verification\v20_x29_tostr_string -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:_tostr-str_1 -->
```qb64
PRINT "["; _TOSTR$(5); "] ["; STR$(5); "]"
PRINT _TOSTR$(1 / 3)
PRINT _TOSTR$(1 / 3, 5)
END
```

Output:

```text
[5] [ 5]
0.3333333333333333333
0.33333

Press any key to continue
```
<!-- end generated -->

## See also

`STR$`, `PRINT USING`.
