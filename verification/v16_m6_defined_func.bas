$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): DEFINED(name) written as a function.
$LET A = 1
$IF DEFINED(A) THEN
PRINT "DEFINED(A): true"
$ELSE
PRINT "DEFINED(A): false"
$END IF
SYSTEM
