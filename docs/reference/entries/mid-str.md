# MID$

`MID$` gives part of a string: `MID$("Austerlitz", 4, 3)` is `"ter"`. As a statement, `MID$(s$, start) = t$`
overwrites bytes of a string variable in place.

## Syntax

<!-- begin generated: syntax -->
```text
MID$(stringValue$, startPosition%[, bytes%])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

```text
MID$(stringVariable$, start%[, length%]) = replacement$
```

## Arguments

- `stringValue$`: the string.
- `startPosition%`: the first byte taken, 1 for the first.
- `bytes%`: how many bytes. When left out, every byte to the end.

## Result

A STRING.

## Behavior

`MID$(s$, start, n)` gives `n` bytes from `start`. Fewer remain when the string ends first: `MID$("abc", 2, 9)` is
`"bc"`. A start past the end, or a length of 0 or less, gives `""`.
<!-- src: language/builtin-functions "String results"; verification\v20_c_string_edges; tests\reference\examples\mid-str_1 -->

A start of 0 or less still takes from the first byte, but the length counts from the start given:
`MID$("abc", 0)` is `"abc"`, and `MID$("abc", 0, 2)` is `"a"`, the bytes at 0 and 1.
<!-- src: verification\v20_c_string_edges -->

The statement `MID$(s$, start[, n]) = t$` writes the bytes of `t$` over those of `s$` from `start`, never changing
the length of `s$`: what does not fit is dropped. A start below 1 or past the end, and a length below 1, change
nothing.
<!-- src: language/builtin-statements "MID$ statement"; tests\differential\str\mid_stmt; tests\reference\examples\mid-str_1 -->

The target of the statement may be a variable, a `STRING * n`, an element or a member.
<!-- src: language/builtin-statements "MID$ statement" -->

## Errors

These are compile errors:

- One argument, or four: "`MID$` takes 2 or 3 arguments".
  <!-- src: language/builtin-functions "Names and arity" -->
- The statement with a target that is no variable, as `MID$("abc", 1) = "x"`: "`MID$` needs a variable here".
  <!-- src: language/builtin-statements "MID$ statement" -->

`MID$` raises no run-time error.
<!-- src: language/builtin-functions "String results"; verification\v20_c_string_edges -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:mid-str_1 -->
```qb64
s$ = "Austerlitz"
PRINT MID$(s$, 4, 3)
PRINT MID$(s$, 7)
PRINT "["; MID$(s$, 20); "]"
MID$(s$, 1, 3) = "XYZ"
PRINT s$
MID$(s$, 9) = "ABCDEF"
PRINT s$; LEN(s$)
END
```

Output:

```text
ter
litz
[]
XYZterlitz
XYZterliAB 10 

Press any key to continue
```
<!-- end generated -->

## See also

[`LEFT$`](left-str.md), [`RIGHT$`](right-str.md), [`INSTR`](instr.md), [`ASC`](asc.md).
