# PRINT USING

`PRINT USING` prints values in a format: `PRINT USING "###.##"; 3.14159` prints `  3.14`. `#` stands for a digit, `&`
for a whole string, and other characters are printed as they are.

## Syntax

```text
PRINT [items] USING format$; value [{;|,} value]... [;|,]
PRINT #filenumber, USING format$; value...
```

## Arguments

- `format$`: any string expression, up to the first `;` outside parentheses.
- `value`: numbers or strings, at least one, separated by `;` or `,`.

## Behavior

Number fields: `#` is a digit, `.` the decimal point, `+` a sign, `**` fills with stars and `$` puts a dollar sign:
`PRINT USING "**$###.##"; 12.5` prints `***$12.50`. A number too wide for its field is printed with a `%` before it.
<!-- src: language/print-using "Formats, as the runtime prints them"; tests\reference\examples\print-using_1 -->

String fields: `!` is the first character, `&` the whole string and `\  \` as many characters as its width.
<!-- src: language/print-using "Formats, as the runtime prints them" -->

The format is evaluated once, and used again from its start when values are left: `PRINT USING "##x"; 1, 2, 3` prints
` 1x 2x 3x`. A `,` between values does what a `;` does.
<!-- src: language/print-using "Values by type"; tests\reference\examples\print-using_1 -->

Items before `USING` are printed first, as `PRINT` prints them.
<!-- src: language/print-using "TAB, SPC and items before USING" -->

The line ends unless the statement's last element is `;` or `,`.
<!-- src: language/print-using "The end of the line" -->

An exponent field `^^^^` writes `E` for a SINGLE, `D` for a DOUBLE and `F` for a `_FLOAT`.
<!-- src: language/print-using "Values by type" -->

## Errors

- A format with no field raises run-time error 5, and a string given to a number field or the reverse error 13. What
  was printed before the error stays, with no line end.
  <!-- src: language/print-using "Errors when it runs" -->
- No value after the format, a number as the format, or two values with nothing between them, is a compile error.
  <!-- src: language/print-using "The form of PRINT USING" -->

## Availability

<!-- begin generated: availability -->
Supported. All 53 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:print-using_1 -->
```qb64
PRINT USING "###.##"; 3.14159
PRINT USING "Men: #####"; 26000
PRINT USING "**$###.##"; 12.5
PRINT USING "& has ## guns"; "Davout"; 48
PRINT USING "!"; "hello"
PRINT USING "##x"; 1, 2, 3
END
```

Output:

```text
  3.14
Men: 26000
***$12.50
Davout has 48 guns
h
 1x 2x 3x

Press any key to continue
```
<!-- end generated -->

## See also

`PRINT`, `STR$`, `_TOSTR$`, `LPRINT`.
