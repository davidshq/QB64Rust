$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a TYPE whose name is also a variable's name, and a member named like a
' keyword's prefix.
TYPE pt
    x AS LONG
END TYPE
DIM pt AS pt
pt.x = 4
pt2 = 5
PRINT pt.x; pt2
SYSTEM
