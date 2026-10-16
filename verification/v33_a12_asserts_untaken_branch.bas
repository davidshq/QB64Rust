$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.2): $ASSERTS in a $IF branch that is not taken.
$IF 0 THEN
    $ASSERTS
$END IF
PRINT "before"
_ASSERT 0
PRINT "after"
SYSTEM
