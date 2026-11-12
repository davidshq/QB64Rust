# _COMPILEDATE$

`_COMPILEDATE$` gives the date the program was compiled, and `_COMPILETIME$` the time.

## Syntax

```text
date$ = _COMPILEDATE$
time$ = _COMPILETIME$
```

## Result

A STRING: the date of 11 characters, the time of 8, `hh:mm:ss`.

## Behavior

They are fixed when the program is built, so a program checks their form.
<!-- src: language/builtin-functions "The system's folders, commands and build"; tests\corpus\slice\s156_system_builtins; tests\reference\examples\_compiledate-str_1 -->

## Errors

- An argument, and either as a statement, are compile errors.
  <!-- src: language/builtin-functions "The system's folders, commands and build"; verification\v53_x64_compiledate_stmt; verification\v53_x58_compiletime_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_compiledate-str_1 -->
```qb64
d$ = _COMPILEDATE$: t$ = _COMPILETIME$
PRINT "Lengths:"; LEN(d$); LEN(t$)
PRINT "The time's separators: "; MID$(t$, 3, 1); MID$(t$, 6, 1)
END
```

Output:

```text
Lengths: 11  8 
The time's separators: ::

Press any key to continue
```
<!-- end generated -->

## See also

`_COMPILERVERSION$`, `DATE$`.
