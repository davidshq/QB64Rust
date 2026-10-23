$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): a whole member array as the variable of FOR.
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
FOR r.m = 1 TO 2: NEXT
SYSTEM

FUNCTION idx (k AS INTEGER)
    c = c + 1
    idx = k
END FUNCTION
