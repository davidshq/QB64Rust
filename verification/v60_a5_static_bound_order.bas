$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): the order in which the start code reads FUNCTION bounds: two STATIC
' arrays in one SUB and one in another SUB above it, each bound recording its call in a shared variable.
DIM SHARED c AS LONG
PRINT "order:"; c
s1
s2
SYSTEM

FUNCTION f (k AS LONG)
    c = c * 10 + k
    f = k + 1
END FUNCTION

SUB s1
    STATIC w(f(3)) AS LONG
    PRINT "s1:"; UBOUND(w)
END SUB

SUB s2
    STATIC v(f(1)) AS LONG
    STATIC u(f(2)) AS INTEGER
    PRINT "s2:"; UBOUND(v); UBOUND(u)
END SUB
