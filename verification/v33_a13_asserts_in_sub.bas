$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.2): $ASSERTS inside a SUB that is never called.
PRINT "before"
_ASSERT 0
PRINT "after"
SYSTEM
SUB s
    $ASSERTS
END SUB
