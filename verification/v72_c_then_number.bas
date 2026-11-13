$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): THEN and ELSE followed by a line number.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 1
IF a THEN 100 ELSE 200
100 PRINT "then"
GOTO 300
200 PRINT "else"
300 PRINT "end"
NEXT
SYSTEM
