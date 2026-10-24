$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): a label in each branch of a $IF between two settings in a SUB, and a GOSUB to it.
PRINT "before"
s
PRINT "after"
SYSTEM
SUB s
    GOSUB here
    EXIT SUB
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
    RETURN
END SUB
