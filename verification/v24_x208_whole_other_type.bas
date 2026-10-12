$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): q = r for variables of two user types with the same members.
TYPE t1
    x AS LONG
END TYPE
TYPE t2
    x AS LONG
END TYPE
DIM q AS t1, r AS t2
q = r
SYSTEM
