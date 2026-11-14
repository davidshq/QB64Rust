$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): a whole SELECT CASE on one line.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 1
SELECT CASE a: CASE 0: b = 5: CASE ELSE: b = 6: END SELECT
PRINT b
NEXT
SYSTEM
