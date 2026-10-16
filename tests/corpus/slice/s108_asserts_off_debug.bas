$CONSOLE:ONLY
' Slice (m3-asserts-debug-zlib task 2.2): without $ASSERTS an _ASSERT checks nothing, but its condition and message
' are evaluated (verification\v33_a03). $DEBUG on the last line sets _DEBUG_ and _SOCKETS_ above it and changes
' nothing else outside the IDE (v33_d01, d11).
$IF _ASSERTS_ THEN
    PRINT "asserts on"
$ELSE
    PRINT "asserts off"
$END IF
$IF _DEBUG_ THEN
    PRINT "debug on"
$END IF
$IF _SOCKETS_ THEN
    PRINT "sockets on"
$END IF
_ASSERT f(0), g$("never")
_ASSERT 0
_ASSERT 0, "never"
PRINT "after"
SYSTEM
FUNCTION f (x)
    PRINT "f called"
    f = x
END FUNCTION
FUNCTION g$ (s$)
    PRINT "g called"
    g$ = s$
END FUNCTION
$DEBUG
