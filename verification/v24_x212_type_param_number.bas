$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a number passed to a user-type parameter.
TYPE t1
    x AS LONG
END TYPE
move 5
SYSTEM
SUB move (p AS t1)
    p.x = 1
END SUB
