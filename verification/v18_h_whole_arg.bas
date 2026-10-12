$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a whole TYPE variable passed to a SUB.
TYPE pt
    x AS LONG
END TYPE
DIM p AS pt
p.x = 4
show p
PRINT p.x
SYSTEM

SUB show (v AS pt)
    v.x = v.x + 1
END SUB
