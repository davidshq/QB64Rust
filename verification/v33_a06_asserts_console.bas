$ASSERTS:CONSOLE
' Verification (m3-asserts-debug-zlib, task 1.2): $ASSERTS:CONSOLE gives a console and a window; a failing _ASSERT with a message (error 315).
_DEST _CONSOLE
$IF _CONSOLE_ = 1 THEN
    PRINT "console 1"
$END IF
$IF _ASSERTS_ = 1 THEN
    PRINT "asserts 1"
$END IF
PRINT "dest"; _DEST; "console"; _CONSOLE
_ASSERT 0, "message here"
PRINT "after"
SYSTEM
