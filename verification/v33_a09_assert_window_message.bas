$ASSERTS
' Verification (m3-asserts-debug-zlib, task 1.2): a program with a window only, a failing _ASSERT with a message (error 314: no console).
PRINT "before"
_ASSERT 0, "message"
PRINT "after"
SYSTEM
