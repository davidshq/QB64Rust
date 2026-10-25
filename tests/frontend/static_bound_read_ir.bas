' TEST: ir
$CONSOLE:ONLY
' STATIC arrays whose bounds read a shared variable or call a FUNCTION (m4-left-marked task 2.1, design D1): the
' arrays are allocated at the start with the bounds read then; no statement of the SUB allocates them.
DIM SHARED n AS LONG
s
SYSTEM

FUNCTION f (j AS LONG)
    f = j + 1
END FUNCTION

SUB s
    STATIC v(n) AS LONG
    STATIC w(f(3)) AS STRING
    PRINT UBOUND(v); UBOUND(w)
END SUB
