$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): an unknown member of an element of a TYPE array.
TYPE t
    b AS LONG
END TYPE
DIM a(2) AS t
a(1).c = 3
PRINT a(1).b
SYSTEM
