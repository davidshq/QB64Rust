$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m4-names-arrays, review): a TYPE block in a SUB whose _DYNAMIC member array is bounded by a CONST
' of the SUB alone, with the type used in the main module above the SUB.
DIM x AS t
PRINT UBOUND(x.v)
s
SYSTEM

SUB s
    CONST k = 6
    TYPE t
        v(k) _DYNAMIC AS LONG
    END TYPE
    DIM y AS t
    PRINT UBOUND(y.v)
END SUB
