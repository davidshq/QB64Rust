$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a variable set and printed inside each branch.
PRINT "before"
$IF _CONSOLE_ = 2 THEN
    q = 2
    PRINT "q"; q
$ELSEIF _CONSOLE_ = 1 THEN
    q = 1
    PRINT "q"; q
$ELSE
    q = 0
    PRINT "q"; q
$END IF
$CONSOLE
PRINT "after"
PRINT "q after"; q
SYSTEM
