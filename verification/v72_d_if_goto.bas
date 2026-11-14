$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): IF and GOTO without THEN.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 1
IF a GOTO 100
PRINT "no"
100 PRINT "end"
NEXT
SYSTEM
