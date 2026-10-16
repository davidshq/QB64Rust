$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.1): $DEBUG in a $IF branch not taken.
$IF 0 THEN
    $DEBUG
$END IF
$IF _DEBUG_ THEN
    PRINT "debug on"
$ELSE
    PRINT "debug off"
$END IF
SYSTEM
