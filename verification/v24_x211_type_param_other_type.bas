$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a variable of another user type passed to a user-type parameter.
TYPE t1
    x AS LONG
END TYPE
TYPE t2
    x AS LONG
END TYPE
DIM r AS t2
move r
SYSTEM
SUB move (p AS t1)
    p.x = 1
END SUB
