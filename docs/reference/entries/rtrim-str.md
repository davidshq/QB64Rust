# RTRIM$

`RTRIM$` gives a string without the spaces at its end. It is the usual way to drop the padding of a fixed-length
string: a `STRING * 10` holding `"Ney"` gives `"Ney"`.

## Syntax

<!-- begin generated: syntax -->
```text
RTRIM$(text$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `text$`: any string, a fixed-length string included.

## Result

A STRING.

## Behavior

The spaces at the end are dropped, and nothing else: a tab or a NUL byte at the end stays.
<!-- src: verification\v20_c_string_edges; tests\reference\examples\rtrim-str_1 -->

A fixed-length string is taken with all its bytes, so its padding of spaces is dropped.
<!-- src: language/fixed-length-strings "Fixed-length strings as arguments"; tests\reference\examples\rtrim-str_1 -->

## Errors

- A number as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:rtrim-str_1 -->
```qb64
DIM name1 AS STRING * 10
name1 = "Ney"
PRINT "["; name1; "] ["; RTRIM$(name1); "]"
END
```

Output:

```text
[Ney       ] [Ney]

Press any key to continue
```
<!-- end generated -->

## See also

[`LTRIM$`](ltrim-str.md), [`_TRIM$`](_trim-str.md), [`STRING`](string.md).
