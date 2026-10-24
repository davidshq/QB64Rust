$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a CONST in each branch, never used.
PRINT "before"
$IF _CONSOLE_ = 2 THEN
    CONST k = 2
$ELSEIF _CONSOLE_ = 1 THEN
    CONST k = 1
$ELSE
    CONST k = 0
$END IF
$CONSOLE
PRINT "after"
PRINT "no use of k"
SYSTEM
