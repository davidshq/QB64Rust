' TEST: typed
$CONSOLE:ONLY
' STATIC arrays whose bounds read a shared variable, an element of a shared array, a main-module CONST or call a
' FUNCTION (m4-left-marked task 2.1, design D1): allocated by the start code with the bounds read then
' (verification\v55_x74, x75, v60_a1-a7). The bounds are the main module's values. An array the procedure
' shares with SHARED is the main module's too, not one of the procedure (static_bounds_errors.bas).
CONST k = 2
DIM SHARED n AS LONG, sa(3) AS LONG
DIM ma(4) AS LONG
s
t
SYSTEM

SUB t
    SHARED ma() AS LONG
    STATIC x(UBOUND(ma)) AS LONG
    PRINT UBOUND(x)
END SUB

FUNCTION f (j AS LONG)
    f = j + 1
END FUNCTION

SUB s
    STATIC v(n) AS LONG
    STATIC w(1 TO f(3), sa(1) + k) AS INTEGER
    PRINT UBOUND(v); UBOUND(w, 2)
END SUB
