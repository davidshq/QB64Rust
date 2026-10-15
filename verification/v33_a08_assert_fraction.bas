$CONSOLE:ONLY
$ASSERTS
' Verification (m3-asserts-debug-zlib, task 1.2): the condition is a LONG: .6 and -.4 and .4 after rounding.
_ASSERT .6
PRINT "0.6 passed"
_ASSERT 1E+10
PRINT "1E+10 passed"
_ASSERT 4294967296
PRINT "4294967296 passed"
_ASSERT .4
PRINT "0.4 passed"
SYSTEM
