$CONSOLE:ONLY
$ASSERTS
' Verification (m3-asserts-debug-zlib, task 1.2): CALL _ASSERT(...) and _ASSERT (x) in parentheses.
CALL _ASSERT(1)
CALL _ASSERT(1, "m")
_ASSERT (1)
PRINT "passed"
CALL _ASSERT(0, "bad")
PRINT "after"
SYSTEM
