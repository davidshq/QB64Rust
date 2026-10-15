$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.2): _ASSERT without $ASSERTS: does nothing; are the condition and the message evaluated?
$IF _ASSERTS_ THEN
    PRINT "asserts on"
$ELSE
    PRINT "asserts off"
$END IF
_ASSERT f(0), g$("never")
_ASSERT 0
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
