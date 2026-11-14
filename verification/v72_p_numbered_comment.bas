$CONSOLE:ONLY
' Verification (OpenSpec change format-statement-lines, design D9): a comment on a numbered line that a jump names.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
FOR a = 0 TO 1
IF a THEN 20
PRINT "zero"
20 x = a: y = a * 2 ' the target of the jump
PRINT x; y
NEXT
SYSTEM
