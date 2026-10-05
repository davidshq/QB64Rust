$CONSOLE:ONLY
' Verification (m2-parser-breadth, M8): a.b used as a plain variable first, then a declared as a TYPE variable.
TYPE t
    b AS LONG
END TYPE
a.b = 3
DIM a AS t
a.b = 5
PRINT a.b
SYSTEM
