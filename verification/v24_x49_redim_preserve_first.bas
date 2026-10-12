$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM _PRESERVE as the first statement that names the array.
REDIM _PRESERVE nw(3) AS LONG
nw(3) = 5: PRINT UBOUND(nw); nw(3)
SYSTEM
