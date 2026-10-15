$CONSOLE:ONLY
$ASSERTS
' Verification (m3-asserts-debug-zlib, task 1.2): a failing _ASSERT under ON ERROR (error 314; errors 300 to 315 are not trapped).
ON ERROR GOTO handler
PRINT "before"
_ASSERT -1
_ASSERT 2
_ASSERT 1 = 1, "fine"
PRINT "passed"
_ASSERT 0
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
