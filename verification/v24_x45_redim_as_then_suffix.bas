$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM x(5) AS LONG, then REDIM x&(7).
REDIM x(5) AS LONG
REDIM x&(7)
PRINT UBOUND(x); UBOUND(x&)
SYSTEM
