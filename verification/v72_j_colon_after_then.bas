$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): a colon right after THEN and right after ELSE.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 1
x = 0
IF a THEN: x = 4
IF a THEN ELSE: x = 3
PRINT x
NEXT
SYSTEM
