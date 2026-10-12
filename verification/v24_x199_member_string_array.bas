$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a member array of STRING: first values, stores, a copy.
TYPE t1
    s(2) AS STRING
END TYPE
DIM p AS t1, q AS t1
p.s(1) = "abc"
q = p
p.s(1) = "changed"
PRINT "["; q.s(0); "] "; q.s(1); " "; p.s(1); UBOUND(p.s)
SYSTEM
