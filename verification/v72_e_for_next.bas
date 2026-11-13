$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): a whole FOR ... NEXT on one line.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 1 TO 2
FOR i = 1 TO 3: PRINT i * a;: NEXT: PRINT
NEXT
SYSTEM
