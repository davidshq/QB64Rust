# RIGHT$

`RIGHT$` gives the last characters of a string: `RIGHT$("Wellington", 3)` is `"ton"`.

## Syntax

<!-- begin generated: syntax -->
```text
RIGHT$(stringValue$, numberOfCharacters%)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `stringValue$`: any string, a fixed-length string included.
- `numberOfCharacters%`: how many characters to take.

## Result

A STRING. It is never longer than `stringValue$`.

## Behavior

The result is the last `numberOfCharacters%` bytes. A count of 0 gives `""`, and a count larger than the string's
length the whole string.
<!-- src: verification\v20_c_string_edges; tests\reference\examples\right-str_1 -->

A negative count gives `""` and raises no error.
<!-- src: language/builtin-functions "String results"; verification\v20_c_string_edges -->

A count that is not a whole number is rounded half to even.
<!-- src: language/builtin-functions "Argument conversion" -->

## Errors

- One argument, or three, is a compile error: "`RIGHT$` takes 2 arguments".
  <!-- src: language/builtin-functions "Names and arity" -->

## Differences

From QuickBASIC 4.5 and BASIC PDS 7.1: they raise error 5 for a negative count. QB64Rust gives `""`, as QB64pe does
(`DIVERGENCES-QB45.md` Q-037).
<!-- src: Q-037 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:right-str_1 -->
```qb64
PRINT RIGHT$("Wellington", 3)
PRINT "["; RIGHT$("abc", 0); "] ["; RIGHT$("abc", 5); "]"
file$ = "report.txt"
PRINT "Extension: "; RIGHT$(file$, 3)
END
```

Output:

```text
ton
[] [abc]
Extension: txt

Press any key to continue
```
<!-- end generated -->

## See also

`LEFT$`, `MID$`, `LEN`.
