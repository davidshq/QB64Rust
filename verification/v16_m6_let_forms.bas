$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): $LET of a predefined name, $LET without a value, blanks around =.
$LET WIN = 0
$IF WIN THEN
PRINT "WIN still true"
$ELSE
PRINT "WIN overridden to 0"
$END IF
$LET   Q=  two words
$IF Q = two THEN
PRINT "Q = two"
$END IF
SYSTEM
