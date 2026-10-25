$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): FOR over r.m(i) with a step, with a negative step, and EXIT FOR after
' the index changed.
TYPE t
    m(3) AS INTEGER
END TYPE
DIM r AS t, i AS INTEGER
i = 1
FOR r.m(i) = 1 TO 9 STEP 2: PRINT r.m(1); r.m(2);: i = 2: NEXT: PRINT r.m(1); r.m(2)
r.m(1) = 0: r.m(2) = 0: i = 1
FOR r.m(i) = 10 TO 1 STEP -3: PRINT r.m(i);: NEXT: PRINT r.m(1)
r.m(1) = 0: r.m(2) = 0: r.m(3) = 0: i = 1
FOR r.m(i) = 1 TO 5: PRINT r.m(i);: IF r.m(i) = 2 THEN i = 3: EXIT FOR
NEXT: PRINT r.m(1); r.m(2); r.m(3); i
SYSTEM
