# WRITE

`WRITE` writes values separated by commas, strings in double quotes: a line that `INPUT #` reads back field by field.
`WRITE #1, "Davout", 26000` writes `"Davout",26000`.

## Syntax

```text
WRITE [items]
WRITE #filenumber, [items]
```

## Behavior

The items are written with commas between them, strings in double quotes and numbers without blanks, and then a line
end: `WRITE #1, "Davout", 26000, 0.9` writes `"Davout",26000,.9`.
<!-- src: language/file-io "WRITE"; tests\reference\examples\write_1 -->

`INPUT #` reads such a line back into the same variables.
<!-- src: language/file-io "INPUT and LINE INPUT from a file"; tests\reference\examples\write_1 -->

`WRITE` without a file number writes to the console or screen. A `,` after the last item leaves out the line end.
<!-- src: language/file-io "WRITE" -->

## Errors

- A `;` between the items or after the last one, or after the file number, is a compile error.
  <!-- src: language/file-io "WRITE with a semicolon" -->

## Differences

From QuickBASIC 4.5:

- A string holding quotes, as `say "hi" now`, is written with its quotes as they are, there and in QB64Rust, but
  `INPUT #` reads the line back into other fields (`DIVERGENCES-QB45.md` Q-027).
  <!-- src: Q-027 -->
- `WRITE #` of `.01 * s&` with `s& = 69` gives `.6899999845772982` there and `.69` in QB64Rust, as the product is a
  DOUBLE there and a SINGLE here (Q-021).
  <!-- src: Q-021 -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:write_1 -->
```qb64
OPEN "write_1.tmp" FOR OUTPUT AS #1
WRITE #1, "Davout", 26000, 0.9
CLOSE #1
OPEN "write_1.tmp" FOR INPUT AS #1
LINE INPUT #1, l$
CLOSE #1
PRINT l$
OPEN "write_1.tmp" FOR INPUT AS #1
INPUT #1, n$, men, morale
CLOSE #1
PRINT n$; men; morale
KILL "write_1.tmp"
END
```

Output:

```text
"Davout",26000,.9
Davout 26000  .9 

Press any key to continue
```
<!-- end generated -->

## See also

[`PRINT`](print.md), [`INPUT`](input.md), [`OPEN`](open.md).
