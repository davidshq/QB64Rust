# STRING$

`STRING$` gives a string of one byte repeated: `STRING$(10, "-")` is a line of ten dashes.

## Syntax

<!-- begin generated: syntax -->
```text
STRING$(count&, {character$ | ASCIIcode%})
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The count: how many bytes.
- The byte: a code, or a string whose first byte is taken.

## Result

A STRING.

## Behavior

`STRING$(n, code)` repeats the byte `code`, and `STRING$(n, s$)` the first byte of `s$`: `STRING$(3, "xyz")` is
`"xxx"`. A count of 0 or less gives `""`.
<!-- src: verification\v20_c_string_edges; tests\reference\examples\string-str_1 -->

A code outside 0 to 255 is taken by its low byte, with no error: `STRING$(2, 256)` gives two bytes of code 0.
<!-- src: verification\v20_c_string_edges -->

`STRING$(n, "")` with an empty string raises no error.
<!-- src: Q-007; verification\v20_c_string_edges -->

## Errors

- One argument is a compile error: "`STRING$` takes 2 arguments".
  <!-- src: language/builtin-functions "Names and arity" -->

## Differences

From QuickBASIC 4.5: `STRING$(n, "")` raises error 5 there, also when `n` is 0. QB64Rust raises nothing, as QB64pe
(`DIVERGENCES-QB45.md` Q-007).
<!-- src: Q-007 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:string-str_1 -->
```qb64
PRINT STRING$(10, "-")
PRINT STRING$(3, 65)
PRINT STRING$(4, "xyz")
END
```

Output:

```text
----------
AAA
xxxx

Press any key to continue
```
<!-- end generated -->

## See also

`SPACE$`, `CHR$`.
