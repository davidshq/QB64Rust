$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): a single-line IF with ELSE and colons in both branches.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 1
IF a THEN x = 1: y = 2 ELSE x = 3: y = 4
PRINT x; y
NEXT
SYSTEM
