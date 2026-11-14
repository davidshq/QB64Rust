# PRINT USING

`PRINT USING format$; values` prints values by a format of number and string fields.

The pages of this area: [PRINT USING](../entries/print-using.md), [PRINT](../entries/print.md),
[LPRINT](../entries/lprint.md).

## The format

The format is any string expression, ending at the first `;` outside parentheses. When values are left at the format's
end, it starts again.
<!-- src: language/print-using "The form of PRINT USING"; language/print-using "Formats, as the runtime prints them"; tests\reference\examples\print-using_1 -->

It ends the line unless the statement ends with `;` or `,`. `PRINT #n, USING` writes the same to a file.
<!-- src: language/print-using "The end of the line"; language/print-using "PRINT USING to a file" -->

A format with no field raises error 5, and a string for a number field error 13. What was built before the error stays
printed.
<!-- src: language/print-using "Errors when it runs" -->
