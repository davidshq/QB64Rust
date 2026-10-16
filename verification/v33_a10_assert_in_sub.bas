$CONSOLE:ONLY
$ASSERTS
' Verification (m3-asserts-debug-zlib, task 1.2): a failing _ASSERT in a SUB, and the line the error names.
PRINT "before"
s
PRINT "after"
SYSTEM
SUB s
    PRINT "in s"
    _ASSERT 0
END SUB
