$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a PRINT in each branch, and lines after the $END IF and a loop.
PRINT "before"
$IF _CONSOLE_ = 2 THEN
    PRINT "2"
$ELSEIF _CONSOLE_ = 1 THEN
    PRINT "1"
$ELSE
    PRINT "0"
$END IF
$CONSOLE
PRINT "after"
FOR i = 1 TO 2
    PRINT "loop"; i
NEXT
SYSTEM
