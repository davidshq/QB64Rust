$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): a label in each branch of a $IF between two settings, and a GOTO to it.
PRINT "before"
GOTO here
$IF _CONSOLE_ = 2 THEN
here:
    PRINT "2"
$ELSEIF _CONSOLE_ = 1 THEN
here:
    PRINT "1"
$ELSE
here:
    PRINT "0"
$END IF
$CONSOLE
PRINT "after"
SYSTEM
