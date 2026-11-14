# _INSTRREV

`_INSTRREV` finds the last place a string stands inside another: `_INSTRREV("a.b.c", ".")` is 4. It is the usual way
to find a file name's extension or folder.

## Syntax

<!-- begin generated: syntax -->
```text
_INSTRREV([start%,] baseString$, searchString$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `start`, optional: the last position a match may start at.
- `text$`: the string searched.
- `find$`: the string looked for.

## Result

A LONG: the position of the last match, or 0.

## Behavior

`_INSTRREV(s$, f$)` gives the position of the last `f$` in `s$`. With a start, of the last one that starts at or
before it: `_INSTRREV(3, "a.b.c", ".")` is 2.
<!-- src: language/builtin-functions "Searching from the end with _INSTRREV"; tests\reference\examples\_instrrev_1 -->

When there is no match the result is 0.
<!-- src: language/builtin-functions "Searching from the end with _INSTRREV" -->

## Errors

- A wrong number of arguments, or a number where a string is needed, is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_instrrev_1 -->
```qb64
path$ = "games/napoleon/won1.bas"
p = _INSTRREV(path$, "/")
PRINT p; MID$(path$, p + 1)
PRINT _INSTRREV(10, path$, "/")
PRINT _INSTRREV(path$, "\\")
END
```

Output:

```text
 15 won1.bas
 6 
 0 

Press any key to continue
```
<!-- end generated -->

## See also

[`INSTR`](instr.md), [`MID$`](mid-str.md), [`RIGHT$`](right-str.md).
