# CHR$

`CHR$` gives the one-byte string of a code: `CHR$(65)` is `"A"`. `CHR$(34)` is the double quote, which a string
literal cannot hold.

## Syntax

<!-- begin generated: syntax -->
```text
CHR$(code%)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `code%`: a code from 0 to 255.

## Result

A STRING of one byte.

## Behavior

`CHR$(n)` is the string of the one byte `n`. Every code from 0 to 255 gives one byte: `LEN(CHR$(0))` is 1.
<!-- src: language/builtin-functions "Supported built-in functions"; tests\reference\examples\chr-str_1 -->

`CHR$` may stand in a `CONST`, with one code from 0 to 255.
<!-- src: language/constants "CONST values" -->

## Errors

- A code outside 0 to 255 raises run-time error 5, and the result is `""`.
  <!-- src: language/builtin-functions "Runtime errors of built-ins"; verification\v20_c_string_edges -->
- A string as the code is a compile error: "`CHR$` needs a number".
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:chr-str_1 -->
```qb64
PRINT CHR$(72) + CHR$(105)
q$ = CHR$(34)
PRINT q$; "quoted"; q$
PRINT LEN(CHR$(0))
END
```

Output:

```text
Hi
"quoted"
 1 

Press any key to continue
```
<!-- end generated -->

## See also

[`ASC`](asc.md), [`STRING$`](string-str.md).
