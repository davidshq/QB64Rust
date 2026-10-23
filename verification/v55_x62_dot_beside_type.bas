$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x62_dot_beside_type)
TYPE t
    b AS LONG
END TYPE
DIM a AS t
DIM a . b AS LONG
a.b = 4
PRINT a.b
SYSTEM
