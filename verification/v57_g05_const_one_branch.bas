$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a CONST in the =1 branch only, used after.
PRINT "before"
$IF _CONSOLE_ = 2 THEN
$ELSEIF _CONSOLE_ = 1 THEN
    CONST k = 1
$ELSE
$END IF
$CONSOLE
PRINT "after"
PRINT "k ="; k
SYSTEM
