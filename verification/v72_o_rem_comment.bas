$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): a REM comment after a single-line IF.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 1
x = 0
IF a THEN x = 1: REM first turn
PRINT x
NEXT
SYSTEM
