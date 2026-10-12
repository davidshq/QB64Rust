$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): one TYPE variable assigned to another, and to a scalar.
TYPE pt
    x AS LONG
END TYPE
DIM p AS pt, q AS pt
p.x = 4
q = p
PRINT q.x
SYSTEM
