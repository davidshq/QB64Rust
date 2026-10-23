$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): an element of a member array as the variable of FOR, with an index that changes and one that counts its evaluations.
TYPE t
    a AS LONG
    v AS INTEGER
    m(3) AS INTEGER
END TYPE
TYPE o
    n AS t
END TYPE
DIM SHARED c AS INTEGER
DIM r AS t, ar(5) AS t, ro AS o, i AS INTEGER
i = 1
FOR r.m(i) = 1 TO 3: PRINT r.m(1); r.m(2);: i = 2: NEXT: PRINT r.m(1); r.m(2)
c = 0
FOR r.m(idx(3)) = 1 TO 3: PRINT r.m(3);: NEXT: PRINT r.m(3); "calls"; c
SYSTEM

FUNCTION idx (k AS INTEGER)
    c = c + 1
    idx = k
END FUNCTION
