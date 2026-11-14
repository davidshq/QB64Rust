# INSTR

`INSTR` finds a string inside another and gives its position: `INSTR("Austerlitz", "l")` is 7. It gives 0 when the
string is not there.

## Syntax

<!-- begin generated: syntax -->
```text
INSTR([start%,] baseString$, searchString$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `start`, optional: the position to search from, 1 for the first.
- `text$`: the string searched.
- `find$`: the string looked for.

## Result

A LONG: the position of the first byte of the match, or 0.

## Behavior

`INSTR(s$, f$)` gives the position of the first `f$` in `s$`, and `INSTR(start, s$, f$)` of the first one at `start`
or after: `INSTR("abcabc", "c")` is 3 and `INSTR(4, "abcabc", "c")` is 6.
<!-- src: verification\v20_c_string_edges; tests\reference\examples\instr_1 -->

An empty `f$` gives 1. A start of 0 searches from the first byte, and a start past the end gives 0.
<!-- src: verification\v20_c_string_edges -->

The start may be left out with an empty place, `INSTR(, a$, b$)`, as QB64pe allows.
<!-- src: language/builtin-functions "An omitted argument of a built-in function"; verification\v58_i01_instr_omitted -->

## Errors

- A wrong number of arguments, or a number where a string is needed, is a compile error.
  <!-- src: language/builtin-functions "Names and arity"; verification\v20_x15_instr_arity -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:instr_1 -->
```qb64
s$ = "Austerlitz, Jena, Wagram"
PRINT INSTR(s$, ",")
PRINT INSTR(12, s$, ",")
PRINT INSTR(s$, "Borodino")
first$ = LEFT$(s$, INSTR(s$, ",") - 1)
PRINT first$
END
```

Output:

```text
 11 
 17 
 0 
Austerlitz

Press any key to continue
```
<!-- end generated -->

## See also

[`_INSTRREV`](_instrrev.md), [`MID$`](mid-str.md), [`LEFT$`](left-str.md).
