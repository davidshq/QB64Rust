$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a CONST in the =2 branch only, used after.
PRINT "before"
$IF _CONSOLE_ = 2 THEN
    CONST k = 2
$ELSEIF _CONSOLE_ = 1 THEN
$ELSE
$END IF
$CONSOLE
PRINT "after"
PRINT "k ="; k
SYSTEM
