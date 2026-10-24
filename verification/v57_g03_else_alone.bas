$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): only the $ELSE branch holds a line.
PRINT "before"
$IF _CONSOLE_ = 2 THEN
$ELSEIF _CONSOLE_ = 1 THEN
$ELSE
    PRINT "else"
$END IF
$CONSOLE
PRINT "after"
SYSTEM
