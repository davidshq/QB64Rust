$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): a single-line IF inside the branch of another, the ELSE the inner one's.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 1
FOR b = 0 TO 1
x = 0
IF a THEN IF b THEN x = 1 ELSE x = 2
PRINT a; b; x
NEXT
NEXT
SYSTEM
