' TEST: cpp
$CONSOLE:ONLY
' Member arrays in the C++ (m2-defxxx-common-arrays task 9.2, design D8): the elements lie in the value, from the
' member's offset, first dimension fastest (id 4 bytes, v 4 x 4 from offset 4, g 2 x 3 x 2 from 20, s 2 pointers
' from 32: 48 bytes). A read checks each index with array_check against the member's constant bounds. A store
' through a member with an index on the way evaluates a number before the indexes and a string after them, and
' is skipped when an index was outside. LBOUND and UBOUND are constants, or picked by a dimension evaluated
' once. A Clear finds the place once. A whole member array is passed as a descriptor made for the call. A whole
' value is copied part by part: each element of a STRING member array is a part.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    g(1 TO 2, 0 TO 2) AS INTEGER
    s(0 TO 1) AS STRING
END TYPE
DIM t AS t1, a(2) AS t1, i AS LONG
t.v(i) = 7
a(i).g(2, i) = i + 1
a(i).s(1) = "s" + STR$(i)
i = t.v(3) + a(i).g(1, 2)
PRINT UBOUND(t.v); LBOUND(a(i).g, 2); UBOUND(t.g, i)
ERASE a(i).v
REDIM t.s(0 TO 1)
bump a(i).v(2)
fill a(i).g()
a(1) = t
SYSTEM

SUB bump (n AS LONG)
    n = n + 1
END SUB

SUB fill (b() AS INTEGER)
    b(1, 0) = 1
END SUB
