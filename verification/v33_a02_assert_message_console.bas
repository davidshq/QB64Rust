$CONSOLE:ONLY
$ASSERTS
' Verification (m3-asserts-debug-zlib, task 1.2): a failing _ASSERT with a message in a console program (echoes it, error 315).
PRINT "before"
_ASSERT 1 > 2, "bad"
PRINT "after"
SYSTEM
