$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM with another number of dimensions.
REDIM m(2, 3)
REDIM m(4)
PRINT UBOUND(m)
SYSTEM
