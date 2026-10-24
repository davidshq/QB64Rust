$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a $IF and a $ELSE, each with a PRINT.
PRINT "before"
$IF _CONSOLE_ = 2 THEN
    PRINT "first"
$ELSE
    PRINT "else"
$END IF
$CONSOLE
PRINT "after"
SYSTEM
