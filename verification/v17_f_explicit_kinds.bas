$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): OPTION _EXPLICIT with every declaration kind (DIM, CONST, DIM SHARED, SHARED, STATIC, parameter, FUNCTION name).
OPTION _EXPLICIT
DIM x AS LONG
CONST k = 2
DIM SHARED g AS LONG
x = k
g = 5
s 7
PRINT x; g; f
SYSTEM
SUB s (p)
    SHARED x AS LONG
    STATIC st
    st = st + 1
    p = p + 1
    PRINT "s:"; p; st; x; g; k
END SUB
FUNCTION f
    f = 3
END FUNCTION
