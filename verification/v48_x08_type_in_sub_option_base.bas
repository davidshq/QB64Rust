$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): OPTION BASE 1 in a SUB above a TYPE block in the SUB, and a member array without TO.
DIM ob AS based
PRINT LBOUND(ob.m); UBOUND(ob.m); LEN(ob)
DIM l(3)
PRINT LBOUND(l)
SYSTEM

SUB sb
    OPTION BASE 1
    TYPE based
        m(3) AS LONG
    END TYPE
END SUB
