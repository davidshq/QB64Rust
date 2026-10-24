$CONSOLE:ONLY
$ASSERTS
' Verification (m4-statements-metacommands, task 1.1): _ASSERTS_ between $ASSERTS and $ASSERTS:CONSOLE, a PRINT in each branch.
PRINT "before"
$IF _ASSERTS_ = 2 THEN
    PRINT "2"
$ELSEIF _ASSERTS_ = 1 THEN
    PRINT "1"
$ELSE
    PRINT "0"
$END IF
$ASSERTS:CONSOLE
PRINT "after"
SYSTEM
