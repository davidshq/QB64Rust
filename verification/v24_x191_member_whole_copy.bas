$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): q = p for a type with a member array: every element copied, the
' copy its own.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
END TYPE
DIM p AS t1, q AS t1
p.id = 1: p.v(0) = 10: p.v(3) = 13
q = p
p.v(3) = 0
PRINT q.id; q.v(0); q.v(3); p.v(3)
SYSTEM
