$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM _PRESERVE with another number of dimensions.
REDIM m(2, 3) AS LONG
REDIM _PRESERVE m(4) AS LONG
PRINT UBOUND(m)
SYSTEM
