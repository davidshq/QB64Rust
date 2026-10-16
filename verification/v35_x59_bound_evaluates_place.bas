$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (59_bound_evaluates_place)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
REDIM p(2) AS t
PRINT UBOUND(p(one(1)).v); LBOUND(p(one(2)).v, 1)
FUNCTION one& (k AS LONG)
    PRINT k;
    one& = 1
END FUNCTION
SYSTEM
