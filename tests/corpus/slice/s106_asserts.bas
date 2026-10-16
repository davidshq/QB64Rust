$CONSOLE:ONLY
' Slice (m3-asserts-debug-zlib task 2.2): $ASSERTS on the last line turns on every _ASSERT above it. Passing
' assertions of each form, the condition and the message evaluated, then a failing one without a message under
' ON ERROR: error 314 is not trapped, the program stops (verification\v33_a01, a04, a05, a08, a14).
ON ERROR GOTO handler
DIM k AS LONG, d AS DOUBLE, i64 AS _INTEGER64, s AS STRING
$IF _ASSERTS_ THEN
    PRINT "asserts on"
$END IF
k = 1: d = .6: i64 = 4294967297: s = "never"
_ASSERT k
_ASSERT d, s
_ASSERT i64
_ASSERT k = 1, "m"
_ASSERT (k)
CALL _ASSERT(k)
CALL _ASSERT(k, s)
_ASSERT f(1), g$("message")
PRINT "passed"
_ASSERT k = 2
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
FUNCTION f (x)
    PRINT "f called"
    f = x
END FUNCTION
FUNCTION g$ (s$)
    PRINT "g called"
    g$ = s$
END FUNCTION
$ASSERTS
