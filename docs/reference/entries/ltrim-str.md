# LTRIM$

`LTRIM$` gives a string without the spaces at its start: `LTRIM$("   left")` is `"left"`. `LTRIM$(STR$(n))` is the
usual way to drop the blank that `STR$` puts before a number.

## Syntax

<!-- begin generated: syntax -->
```text
LTRIM$(text$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `text$`: any string.

## Result

A STRING.

## Behavior

The spaces at the start are dropped, and nothing else: a tab or a NUL byte at the start stays.
<!-- src: verification\v20_c_string_edges; tests\reference\examples\ltrim-str_1 -->

## Errors

- A number as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:ltrim-str_1 -->
```qb64
s$ = "   left side"
PRINT "["; LTRIM$(s$); "]"
PRINT "["; LTRIM$(STR$(7)); "]"
END
```

Output:

```text
[left side]
[7]

Press any key to continue
```
<!-- end generated -->

## See also

`RTRIM$`, `_TRIM$`, `STR$`.
