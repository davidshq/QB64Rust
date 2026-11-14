# _SCREENEXISTS

`_SCREENEXISTS` tells whether the program's screen has been made, so that drawing on it is safe.

## Syntax

<!-- begin generated: syntax -->
```text
_SCREENEXISTS
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

-1 or 0, a LONG.

## Behavior

In a `$CONSOLE:ONLY` program it gives -1.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\reference\examples\_screenexists_1 -->

It may be written without parentheses, and `HEX$(_SCREENEXISTS)` gives QB64pe's string.
<!-- src: language/builtin-functions "Built-ins without parentheses"; language/conventional-memory "VARPTR and VARSEG in HEX$, OCT$ and _BIN$" -->

## Errors

- `_SCREENEXISTS()` and an argument are compile errors.
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x100_screenexists_empty; verification\v26_x75_screenexists_args -->

## Differences

From QB64pe: it is registered there with no type for its result. QB64Rust gives a LONG (`DIVERGENCES-QB64PE.md`
D-013).
<!-- src: D-013 -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_screenexists_1 -->
```qb64
PRINT "_SCREENEXISTS in a console program:"; _SCREENEXISTS
END
```

Output:

```text
_SCREENEXISTS in a console program:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`_SCREENHIDE`](_screenhide.md), [`_WINDOWHANDLE`](_windowhandle.md).
