$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): FOR over an element of a member array of two dimensions, the second
' index changed in the body.
TYPE t
    m(2, 2) AS INTEGER
END TYPE
DIM r AS t, j AS INTEGER
j = 0
FOR r.m(1, j) = 1 TO 3: PRINT r.m(1, 0); r.m(1, 1);: j = 1: NEXT: PRINT r.m(1, 0); r.m(1, 1)
SYSTEM
