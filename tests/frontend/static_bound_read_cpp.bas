' TEST: cpp
$CONSOLE:ONLY
' STATIC arrays whose bounds read a shared variable or call a FUNCTION (m4-left-marked task 2.2, design D1): the
' allocations stand in maindata.txt after the main module's temporaries and pass declarations, the bounds in the
' branch for an array that is not allocated (the old compiler's C++ of verification\v60_a2, a5).
DIM SHARED n AS LONG
s
SYSTEM

FUNCTION f (j AS LONG)
    f = j + 1
END FUNCTION

SUB s
    STATIC v(n) AS LONG
    STATIC w(f(3)) AS LONG
    PRINT UBOUND(v); UBOUND(w)
END SUB
