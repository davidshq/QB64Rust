$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): SWAP of two variables of a type with a STRING member.
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM p AS ts, q AS ts
p.n = "p": q.n = "q": p.v = 1: q.v = 2
SWAP p, q
PRINT p.n; p.v; q.n; q.v
SYSTEM
