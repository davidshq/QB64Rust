$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, review of group 4): REDIM PRESERVE, the word without its underscore.
REDIM x(3) AS LONG
x(1) = 5
REDIM PRESERVE x(5) AS LONG
PRINT UBOUND(x); x(1)
SYSTEM
