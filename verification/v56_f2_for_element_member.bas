$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): a member of an array element as the variable of FOR, with an index that changes in the body and one that counts its evaluations.
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
FOR ar(i).v = 1 TO 3: PRINT ar(1).v; ar(2).v; i;: i = 2: NEXT: PRINT ar(1).v; ar(2).v
c = 0
FOR ar(idx(3)).v = 1 TO 3: PRINT ar(3).v;: NEXT: PRINT ar(3).v; "calls"; c
SYSTEM

FUNCTION idx (k AS INTEGER)
    c = c + 1
    idx = k
END FUNCTION
