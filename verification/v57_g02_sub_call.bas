$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a SUB call in each branch.
PRINT "before"
$IF _CONSOLE_ = 2 THEN
    s2
$ELSEIF _CONSOLE_ = 1 THEN
    s1
$ELSE
    s0
$END IF
$CONSOLE
PRINT "after"
SYSTEM
SUB s2
    PRINT "s2"
END SUB
SUB s1
    PRINT "s1"
END SUB
SUB s0
    PRINT "s0"
END SUB
