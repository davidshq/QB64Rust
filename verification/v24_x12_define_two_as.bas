$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): two AS clauses in one _DEFINE.
_DEFINE a AS INTEGER, b AS LONG
a1 = 2.5: b1 = 2.5: PRINT LEN(a1); LEN(b1)
SYSTEM
