# UCASE$

`UCASE$` gives a string with its letters in upper case: `UCASE$("Grande Armee")` is `"GRANDE ARMEE"`. It is the
usual way to compare text without regard to case.

## Syntax

<!-- begin generated: syntax -->
```text
UCASE$(text$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `text$`: any string, a fixed-length string included.

## Result

A STRING of the same length.

## Behavior

The letters `a` to `z` become `A` to `Z`. Every other byte stays as it is, also the accented letters of codes above
127.
<!-- src: verification\v20_c_string_edges; tests\reference\examples\ucase-str_1 -->

A fixed-length string is taken with all its bytes, padding included.
<!-- src: language/fixed-length-strings "Fixed-length strings as arguments" -->

## Errors

- A number as the argument is a compile error: "`UCASE$` needs a string".
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:ucase-str_1 -->
```qb64
PRINT UCASE$("Grande Armee")
answer$ = "yes"
IF UCASE$(answer$) = "YES" THEN PRINT "Confirmed."
END
```

Output:

```text
GRANDE ARMEE
Confirmed.

Press any key to continue
```
<!-- end generated -->

## See also

[`LCASE$`](lcase-str.md), [`_STRICMP`](_strcmp.md).
