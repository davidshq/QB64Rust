$CONSOLE:ONLY
$DEBUG
' Verification (m3-asserts-debug-zlib, task 1.1): an untrapped error under $DEBUG: the message and the line it names.
PRINT "before"
x = 1
ERROR 7
PRINT "after"
SYSTEM
