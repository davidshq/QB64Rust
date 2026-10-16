$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.2): $ASSERTS on the last line turns on an _ASSERT above it.
PRINT "before"
_ASSERT 0
PRINT "after"
SYSTEM
$ASSERTS
