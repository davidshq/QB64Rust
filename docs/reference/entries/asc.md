# ASC

`ASC` gives the code of a byte of a string: `ASC("A")` is 65. As a statement, `ASC(s$, i) = n` sets that byte.

## Syntax

<!-- begin generated: syntax -->
```text
ASC(text$[, position%])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

```text
ASC(stringVariable$[, position%]) = code%
```

## Arguments

- `text$`: the string.
- `position%`: which byte, 1 for the first. It is 1 when left out.

## Result

An INTEGER from 0 to 255.

## Behavior

`ASC(s$)` gives the code of the first byte and `ASC(s$, i)` that of the byte at `i`: `ASC("abc", 2)` is 98. A byte
above 127 gives its code, 128 to 255.
<!-- src: verification\v20_c_string_edges; tests\reference\examples\asc_1 -->

As a statement, `ASC(s$) = n` or `ASC(s$, i) = n` sets that byte of the string variable to the low byte of `n`:
`ASC(a$) = 321` stores 65. The string's length does not change.
<!-- src: language/builtin-statements "ASC statement"; verification\v52_d_asc -->

## Errors

- An empty string, or a position below 1 or past the end, raises run-time error 5. The function then gives 0, and the
  statement changes nothing.
  <!-- src: language/builtin-functions "Runtime errors of built-ins"; language/builtin-statements "ASC statement"; verification\v20_c_string_edges -->
- The statement with a target that is no string variable, as `ASC(n%) = 65` or `ASC("ab") = 65`, is a compile
  error: "the `ASC` statement needs a string variable as its first argument".
  <!-- src: language/builtin-statements "ASC statement" -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:asc_1 -->
```qb64
PRINT ASC("A"); ASC("abc", 2)
s$ = "a/b"
ASC(s$, 2) = 92
PRINT s$
ON ERROR GOTO handler
n = ASC("")
END

handler:
PRINT "ASC of an empty string: error"; ERR
RESUME NEXT
```

Output:

```text
 65  98 
a\b
ASC of an empty string: error 5 

Press any key to continue
```
<!-- end generated -->

## See also

`CHR$`, `MID$`, `LEN`.
