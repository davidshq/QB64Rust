# _COMPILERVERSION$

`_COMPILERVERSION$` gives the version of the compiler, as QB64pe's program gives it.

## Syntax

<!-- begin generated: syntax -->
```text
_COMPILERVERSION$
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A STRING, starting with `QB64-PE`.

## Behavior

It gives the version string of the QB64pe release this compiler targets, so a program built with qb64rust gives what
one built with QB64pe gives.
<!-- src: language/builtin-functions "System information functions"; tests\corpus\slice\s159_builtins_last; tests\reference\examples\_compilerversion-str_1 -->

## Errors

- An argument, use in a `CONST`, and the name as a statement are compile errors.
  <!-- src: language/builtin-functions "System information functions"; verification\v58_x65_compilerversion_arg; verification\v58_x110_compilerversion_const; verification\v58_x95_compilerversion_statement -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_compilerversion-str_1 -->
```qb64
PRINT "Starts with: "; LEFT$(_COMPILERVERSION$, 7)
END
```

Output:

```text
Starts with: QB64-PE

Press any key to continue
```
<!-- end generated -->

## See also

`_COMPILEDATE$`, `_OS$`.
