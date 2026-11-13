$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): statements packed with colons, and a comment.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 1
x = a: y = a + 1: PRINT x; y ' three statements
NEXT
SYSTEM
