$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a TYPE block inside a SUB.
show
SYSTEM

SUB show
    TYPE pt
        x AS LONG
    END TYPE
    DIM p AS pt
    p.x = 5
    PRINT p.x
END SUB
