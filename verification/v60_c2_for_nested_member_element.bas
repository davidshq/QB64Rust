$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): FOR over ro.n.m(i), an element of a member array of a nested member,
' with an index that changes and one that counts its evaluations, with a step.
TYPE t
    a AS LONG
    m(3) AS LONG
END TYPE
TYPE o
    z AS INTEGER
    n AS t
END TYPE
DIM SHARED c AS INTEGER
DIM ro AS o, i AS INTEGER
i = 1
FOR ro.n.m(i) = 1 TO 3: PRINT ro.n.m(1); ro.n.m(2);: i = 2: NEXT: PRINT ro.n.m(1); ro.n.m(2)
c = 0
FOR ro.n.m(idx(3)) = 1 TO 6 STEP 2: PRINT ro.n.m(3);: NEXT: PRINT ro.n.m(3); "calls"; c
SYSTEM

FUNCTION idx (k AS INTEGER)
    c = c + 1
    idx = k
END FUNCTION
