$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): a statement, a colon and the header of a block IF.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 1
b = 0
x = 1: IF a THEN
b = 1
END IF
PRINT x; b
NEXT
SYSTEM
