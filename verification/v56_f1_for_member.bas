$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): a member of a TYPE variable as the variable of FOR: nested, with
' EXIT FOR, a STEP, the member changed in the body, the other members kept, and members of each numeric type.
TYPE t
    a AS LONG
    v AS INTEGER
    s AS SINGLE
    b AS _BYTE
    q AS _INTEGER64
    w AS _UNSIGNED INTEGER
END TYPE
DIM r AS t, u AS t
r.a = 99
FOR r.v = 1 TO 3: PRINT r.v;: NEXT: PRINT r.v; r.a
FOR r.v = 1 TO 2
    FOR u.v = 5 TO 6: PRINT r.v; u.v; "/";: NEXT
NEXT r.v
PRINT
FOR r.v = 1 TO 10: IF r.v = 4 THEN EXIT FOR
NEXT: PRINT r.v
FOR r.s = 0 TO 1 STEP 0.25: PRINT r.s;: NEXT: PRINT r.s
FOR r.b = 125 TO 127: PRINT r.b;: NEXT: PRINT r.b
FOR r.q = 10 TO 1 STEP -3: PRINT r.q;: NEXT: PRINT r.q
FOR r.w = 2 TO 0 STEP -1: PRINT r.w;: NEXT: PRINT r.w
FOR r.v = 1 TO 5: r.v = r.v + 1: PRINT r.v;: NEXT: PRINT
FOR r.v = r.a TO r.a + 1: PRINT r.v;: NEXT: PRINT
PRINT r.a; u.a
SYSTEM
