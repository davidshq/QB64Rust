$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): member access on a variable of a TYPE that has no such member is an error
' (measured in v16_m8); here: a dotted name next to an array of a TYPE, and a member of a non-TYPE array.
TYPE t
    b AS LONG
END TYPE
DIM a(2) AS t
a.b = 3
PRINT a.b; a(0).b
SYSTEM
