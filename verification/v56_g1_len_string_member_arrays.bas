$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): LEN of a whole member array of strings over other bounds and
' dimensions, beside other members and of an array element; LEN of one of its elements.
TYPE t1
    s(3) AS STRING
END TYPE
TYPE t2
    id AS INTEGER
    s(1 TO 2, 0 TO 2) AS STRING
    v AS LONG
END TYPE
DIM x AS t1, y AS t2, ya(2) AS t2
PRINT LEN(x.s()); LEN(y.s()); LEN(ya(1).s()); LEN(x.s(1))
x.s(1) = "abcdef"
PRINT LEN(x.s()); LEN(x.s(1))
SYSTEM
