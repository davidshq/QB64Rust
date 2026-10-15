$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.2): $ASSERTS in an included file turns assertions on.
'$INCLUDE:'v33_inc/asserts.bi'
PRINT "before"
_ASSERT 0
PRINT "after"
SYSTEM
