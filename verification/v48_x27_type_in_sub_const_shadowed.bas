$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m4-names-arrays, review): a TYPE block in a SUB whose _DYNAMIC member array is bounded by a CONST
' that the main module and the SUB both define, with other values: which one gives the bound?
CONST n = 4
DIM x AS t
PRINT UBOUND(x.v)
s
SYSTEM

SUB s
    CONST n = 9
    TYPE t
        v(n) _DYNAMIC AS LONG
    END TYPE
    DIM y AS t
    PRINT UBOUND(y.v)
END SUB
