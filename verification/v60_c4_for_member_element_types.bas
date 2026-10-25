$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): FOR over an element of a member array of every numeric type, near the
' end of its range where the type has one.
TYPE t
    b(2) AS _BYTE
    ub(2) AS _UNSIGNED _BYTE
    n(2) AS INTEGER
    un(2) AS _UNSIGNED INTEGER
    l(2) AS LONG
    ul(2) AS _UNSIGNED LONG
    q(2) AS _INTEGER64
    uq(2) AS _UNSIGNED _INTEGER64
    s(2) AS SINGLE
    d(2) AS DOUBLE
    f(2) AS _FLOAT
END TYPE
DIM r AS t
FOR r.b(1) = 120 TO 127 STEP 4: PRINT r.b(1);: NEXT: PRINT r.b(1)
FOR r.ub(1) = 250 TO 255 STEP 3: PRINT r.ub(1);: NEXT: PRINT r.ub(1)
FOR r.n(1) = 32760 TO 32767 STEP 4: PRINT r.n(1);: NEXT: PRINT r.n(1)
FOR r.un(1) = 2 TO 0 STEP -1: PRINT r.un(1);: NEXT: PRINT r.un(1)
FOR r.l(1) = 2147483640 TO 2147483647 STEP 4: PRINT r.l(1);: NEXT: PRINT r.l(1)
FOR r.ul(1) = 1 TO 0 STEP -1: PRINT r.ul(1);: NEXT: PRINT r.ul(1)
FOR r.q(2) = -3 TO 3 STEP 3: PRINT r.q(2);: NEXT: PRINT r.q(2)
FOR r.uq(2) = 5 TO 9 STEP 2: PRINT r.uq(2);: NEXT: PRINT r.uq(2)
FOR r.s(0) = 0 TO 1 STEP 0.25: PRINT r.s(0);: NEXT: PRINT r.s(0)
FOR r.d(0) = 0 TO 0.3 STEP 0.1: PRINT r.d(0);: NEXT: PRINT r.d(0)
FOR r.f(0) = 1 TO 2 STEP 0.5: PRINT r.f(0);: NEXT: PRINT r.f(0)
SYSTEM
