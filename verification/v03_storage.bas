$CONSOLE:ONLY
' Verification: fixed-length string initial content, REDIM _PRESERVE order.
' Claims: study\02 sections 3.2 and 3.4.

PRINT "== fixed-length string initial bytes =="
DIM f AS STRING * 4
FOR i = 1 TO 4: PRINT ASC(f, i);: NEXT: PRINT
TYPE t
    s AS STRING * 3
END TYPE
DIM u AS t
FOR i = 1 TO 3: PRINT ASC(u.s, i);: NEXT: PRINT
f = "ab"
FOR i = 1 TO 4: PRINT ASC(f, i);: NEXT: PRINT

PRINT "== REDIM _PRESERVE of a 2-D array =="
REDIM a(1 TO 2, 1 TO 3) AS INTEGER
FOR i = 1 TO 2: FOR j = 1 TO 3: a(i, j) = i * 10 + j: NEXT: NEXT
REDIM _PRESERVE a(1 TO 3, 1 TO 3) AS INTEGER
FOR i = 1 TO 3
    FOR j = 1 TO 3: PRINT a(i, j);: NEXT
    PRINT
NEXT
PRINT "-- last dimension grown --"
REDIM b(1 TO 2, 1 TO 2) AS INTEGER
FOR i = 1 TO 2: FOR j = 1 TO 2: b(i, j) = i * 10 + j: NEXT: NEXT
REDIM _PRESERVE b(1 TO 2, 1 TO 3) AS INTEGER
FOR i = 1 TO 2
    FOR j = 1 TO 3: PRINT b(i, j);: NEXT
    PRINT
NEXT
SYSTEM
