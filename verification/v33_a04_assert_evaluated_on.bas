$CONSOLE:ONLY
$ASSERTS
' Verification (m3-asserts-debug-zlib, task 1.2): with $ASSERTS, a passing _ASSERT: are the condition and the message evaluated?
$IF _ASSERTS_ THEN
    PRINT "asserts on"
$END IF
_ASSERT f(1), g$("never")
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
