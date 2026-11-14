# LEFT$

`LEFT$` gives the first characters of a string: `LEFT$("Wellington", 4)` is `"Well"`.

## Syntax

<!-- begin generated: syntax -->
```text
LEFT$(stringValue$, numberOfCharacters%)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `stringValue$`: any string, a fixed-length string (`STRING * n`) included.
- `numberOfCharacters%`: how many characters to take. It may be any numeric type.

## Result

A `STRING`. It is never longer than `stringValue$`.

## Behavior

The result is the first `numberOfCharacters%` bytes of `stringValue$`. A count of 0 gives `""`.
<!-- src: tests\differential\str\funcs; tests\corpus\runtime_comparison\171_left_right_zero -->

A count larger than the string's length gives the whole string.
<!-- src: tests\differential\str\funcs; verification\v20_c_string_edges -->

A negative count gives `""` and raises no error.
<!-- src: verification\v20_c_string_edges; tests\differential\str\funcs -->

A count that is not a whole number is rounded half to even: `LEFT$(s$, 2.5)` takes 2 characters and
`LEFT$(s$, 3.5)` takes 4. A count is then cut to 32 bits, so a count of 3000000000 gives `""`.
<!-- src: language/builtin-functions "Argument conversion"; verification\v20_a_slots -->

## Errors

These are compile errors:

- One argument, or three: "`LEFT$` takes 2 arguments".
  <!-- src: language/builtin-functions "Names and arity"; verification\v20_x01_left_arity -->
- A number as the first argument: "argument 1 of `LEFT$` needs a string".
  <!-- src: language/builtin-functions "Names and arity"; verification\v20_x20_left_number -->
- An argument left out, as in `LEFT$(, 2)` or `LEFT$(a$, )`.
  <!-- src: language/builtin-functions "An omitted argument of a built-in function"; verification\v57_o13_left_first; verification\v57_o14_left_last -->

`LEFT$` raises no run-time error.
<!-- src: verification\v20_c_string_edges -->

## Differences

From QuickBASIC 4.5 and BASIC PDS 7.1: they raise error 5, "Illegal function call", for a negative count. QB64Rust
gives `""`, as QB64pe does (`DIVERGENCES-QB45.md` Q-037).

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:left-str_1 -->
```qb64
city$ = "Wellington"
PRINT LEFT$(city$, 4)
FOR i = 1 TO 4
    PRINT LEFT$("QB64", i)
NEXT
IF LEFT$(city$, 1) = "W" THEN PRINT "It starts with W."
PRINT "["; LEFT$(city$, 0); "]"; LEN(LEFT$(city$, 99))
END
```

Output:

```text
Well
Q
QB
QB6
QB64
It starts with W.
[] 10 

Press any key to continue
```
<!-- end generated -->

## See also

[`RIGHT$`](right-str.md), [`MID$`](mid-str.md), [`LEN`](len.md), [`INSTR`](instr.md).
