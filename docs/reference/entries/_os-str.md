# _OS$

`_OS$` describes the system the program runs on, in brackets: `[WINDOWS][64BIT]` on 64-bit Windows.

## Syntax

<!-- begin generated: syntax -->
```text
_OS$
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A STRING.

## Behavior

Each word is in brackets, and one names the bits.
<!-- src: language/builtin-functions "The system's name and the clipboard"; tests\corpus\slice\s150_builtins_more; tests\reference\examples\_os-str_1 -->

## Errors

- `_OS$ = x$` and an argument are compile errors.
  <!-- src: language/builtin-functions "The system's name and the clipboard"; verification\v49_x48_os_assign; verification\v49_x49_os_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_os-str_1 -->
```qb64
o$ = _OS$
PRINT "In brackets:"; LEFT$(o$, 1) = "["; RIGHT$(o$, 1) = "]"
PRINT "Names the bits:"; INSTR(o$, "[32BIT]") > 0 OR INSTR(o$, "[64BIT]") > 0
END
```

Output:

```text
In brackets:-1 -1 
Names the bits:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`_COMPILERVERSION$`](_compilerversion-str.md), [`ENVIRON$`](environ.md).
