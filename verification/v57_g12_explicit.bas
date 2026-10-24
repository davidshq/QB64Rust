$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): _EXPLICIT_ between nothing and OPTION _EXPLICIT on the last line? (setting after).
PRINT "before"
$IF _EXPLICIT_ = 2 THEN
    PRINT "2"
$ELSEIF _EXPLICIT_ = 1 THEN
    PRINT "1"
$ELSE
    PRINT "0"
$END IF
'
PRINT "after"
SYSTEM
OPTION _EXPLICIT
