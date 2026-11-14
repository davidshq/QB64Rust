$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): a whole DO ... LOOP UNTIL on one line.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 1
n = 0
DO: n = n + 1: LOOP UNTIL n >= 3 + a
PRINT n
NEXT
SYSTEM
