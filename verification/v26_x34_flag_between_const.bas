$CONSOLE:ONLY
' Verification (step 11): _CONSOLE_ between $CONSOLE:ONLY and $CONSOLE, choosing a CONST.
$IF _CONSOLE_ = 2 THEN
    CONST k = 2
$ELSEIF _CONSOLE_ = 1 THEN
    CONST k = 1
$ELSE
    CONST k = 0
$END IF
$CONSOLE
PRINT "k ="; k
SYSTEM
