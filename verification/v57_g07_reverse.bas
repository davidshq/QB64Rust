$CONSOLE
' Verification (m4-statements-metacommands, task 1.1): the reverse order, $CONSOLE then $CONSOLE:ONLY, a PRINT in each branch.
PRINT "before"
$IF _CONSOLE_ = 2 THEN
    PRINT "2"
$ELSEIF _CONSOLE_ = 1 THEN
    PRINT "1"
$ELSE
    PRINT "0"
$END IF
$CONSOLE:ONLY
PRINT "after"
SYSTEM
