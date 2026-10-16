$CONSOLE:ONLY
$ASSERTS
' Slice (m3-asserts-debug-zlib task 2.2): a failing _ASSERT with a message in a console program writes the message
' and raises error 315, which ON ERROR does not trap (verification\v33_a02).
ON ERROR GOTO handler
PRINT "before"
_ASSERT 1 > 2, "bad" + " value"
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
