$CONSOLE:ONLY
' Verification (m2-parser-breadth, M8): member access after an index, with blanks around the dot.
TYPE t
    b AS LONG
END TYPE
DIM a(2) AS t
a(1).b = 8
a(2) .b = 9
PRINT a(1).b; a(2). b
SYSTEM
