$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): a line number in each branch of a $IF between two settings, and a GOTO to it.
PRINT "before"
GOTO 10
$IF _CONSOLE_ = 2 THEN
10 PRINT "2"
$ELSEIF _CONSOLE_ = 1 THEN
10 PRINT "1"
$ELSE
10 PRINT "0"
$END IF
$CONSOLE
PRINT "after"
SYSTEM
