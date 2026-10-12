$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): LEN of a variable of a type with a STRING member, and of the
' member.
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM p AS ts
p.n = "abc"
PRINT LEN(p.n)
PRINT LEN(p)
SYSTEM
