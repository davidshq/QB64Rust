$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): a member of a member (r.n.v) as the variable of FOR.
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
FOR ro.n.v = 1 TO 3: PRINT ro.n.v;: NEXT: PRINT ro.n.v
FOR ro.n.a = 7 TO 5 STEP -1: PRINT ro.n.a;: NEXT: PRINT ro.n.a
SYSTEM

FUNCTION idx (k AS INTEGER)
    c = c + 1
    idx = k
END FUNCTION
