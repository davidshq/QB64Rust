$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): parentheses in a $IF condition.
$LET A = 1
$IF (A = 1) THEN
PRINT "(A = 1): true"
$ELSE
PRINT "(A = 1): false"
$END IF
SYSTEM
