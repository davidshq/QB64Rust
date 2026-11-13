$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): a single-line IF with an empty THEN branch.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 1
x = 0
IF a THEN ELSE x = 4
PRINT x
NEXT
SYSTEM
