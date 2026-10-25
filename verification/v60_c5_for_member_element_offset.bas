$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): FOR over an element of an _OFFSET member array.
TYPE t
    o(2) AS _OFFSET
END TYPE
DIM r AS t
FOR r.o(1) = 1 TO 3: PRINT r.o(1);: NEXT: PRINT r.o(1)
SYSTEM
