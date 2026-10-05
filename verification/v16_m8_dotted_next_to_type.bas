$CONSOLE:ONLY
' Verification (m2-parser-breadth, M8): a.c as a plain variable next to a TYPE variable a (c is no member).
TYPE t
    b AS LONG
END TYPE
DIM a AS t
a.b = 5
a.c = 6
PRINT a.b; a.c
SYSTEM
