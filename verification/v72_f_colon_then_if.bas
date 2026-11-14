$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): a statement, a colon and a single-line IF with two statements.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 1
b = 0
x = 1: IF a THEN b = 1: x = 2
PRINT x; b
NEXT
SYSTEM
