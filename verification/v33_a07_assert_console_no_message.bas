$CONSOLE
$ASSERTS
' Verification (m3-asserts-debug-zlib, task 1.2): a program with a console and a window, a failing _ASSERT without a message (error 314).
_DEST _CONSOLE
PRINT "before"
_ASSERT 0
PRINT "after"
SYSTEM
