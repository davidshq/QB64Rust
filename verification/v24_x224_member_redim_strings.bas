$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM and ERASE of a STRING member array, and of a member array
' through a chain.
TYPE t2
    AS INTEGER q(2), r
END TYPE
TYPE t3
    AS t2 n(1), m
    s(0 TO 1) AS STRING
END TYPE
DIM x(0) AS t3
x(0).s(1) = "abc": x(0).n(1).q(2) = 7: x(0).n(0).q(2) = 8: x(0).n(1).r = 9
REDIM x(0).s(0 TO 1)
ERASE x(0).n(1).q
PRINT "["; x(0).s(1); "]"; x(0).n(1).q(2); x(0).n(0).q(2); x(0).n(1).r
ERASE x(0).n
PRINT x(0).n(0).q(2); x(0).n(1).r
SYSTEM
