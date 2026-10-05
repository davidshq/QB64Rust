$CONSOLE:ONLY
' Verification (m2-parser-breadth, M1): ' $DYNAMIC (blank before the $) makes DIM a(5) dynamic, so REDIM is allowed.
' $DYNAMIC
DIM a(5)
REDIM a(10)
PRINT UBOUND(a)
SYSTEM
