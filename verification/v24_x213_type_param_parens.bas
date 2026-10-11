$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a user-type variable in parentheses passed to a user-type
' parameter.
TYPE t1
    x AS LONG
END TYPE
DIM q AS t1
move (q)
PRINT q.x
SYSTEM
SUB move (p AS t1)
    p.x = p.x + 1
END SUB
