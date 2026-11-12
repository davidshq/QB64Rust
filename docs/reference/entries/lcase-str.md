# LCASE$

`LCASE$` gives a string with its letters in lower case: `LCASE$("WATERLOO")` is `"waterloo"`.

## Syntax

<!-- begin generated: syntax -->
```text
LCASE$(text$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `text$`: any string, a fixed-length string included.

## Result

A STRING of the same length.

## Behavior

The letters `A` to `Z` become `a` to `z`. Every other byte stays as it is, also the accented letters of codes above
127.
<!-- src: verification\v20_c_string_edges; tests\reference\examples\lcase-str_1 -->

## Errors

- A number as the argument is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:lcase-str_1 -->
```qb64
PRINT LCASE$("WATERLOO 1815")
cmd$ = "QUIT"
IF LCASE$(cmd$) = "quit" THEN PRINT "Leaving."
END
```

Output:

```text
waterloo 1815
Leaving.

Press any key to continue
```
<!-- end generated -->

## See also

`UCASE$`, `_STRICMP`.
