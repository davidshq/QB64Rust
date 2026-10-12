$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): LEN of an element of a member array and of a member array of a
' user type.
TYPE t2
    AS INTEGER q(2), r
END TYPE
TYPE t3
    AS t2 n(1), m
END TYPE
DIM x AS t3
PRINT LEN(x.n(1).q(2)); LEN(x.n(1)); LEN(x.m)
SYSTEM
