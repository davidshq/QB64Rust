$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM y&(5), then REDIM y(7) AS LONG.
REDIM y&(5)
REDIM y(7) AS LONG
PRINT UBOUND(y); UBOUND(y&)
SYSTEM
