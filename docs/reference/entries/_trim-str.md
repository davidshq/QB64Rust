# _TRIM$

`_TRIM$` gives a string without the spaces at both ends: `_TRIM$("   both   ")` is `"both"`. It is `LTRIM$` and
`RTRIM$` in one.

## Syntax

<!-- begin generated: syntax -->
```text
_TRIM$(text$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `text$`: any string.

## Result

A STRING.

## Behavior

The spaces at the start and at the end are dropped, and nothing else: tabs and NUL bytes stay.
<!-- src: verification\v20_c_string_edges; tests\reference\examples\_trim-str_1 -->

## Errors

- A number as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:_trim-str_1 -->
```qb64
s$ = "   both sides   "
PRINT "["; _TRIM$(s$); "]"
t$ = CHR$(9) + "tab" + CHR$(9)
PRINT LEN(_TRIM$(t$))
END
```

Output:

```text
[both sides]
 5 

Press any key to continue
```
<!-- end generated -->

## See also

[`LTRIM$`](ltrim-str.md), [`RTRIM$`](rtrim-str.md).
