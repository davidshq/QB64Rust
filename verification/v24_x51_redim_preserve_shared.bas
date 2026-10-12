$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): the order of the words after REDIM: _PRESERVE SHARED and SHARED
' _PRESERVE.
REDIM _PRESERVE SHARED a(3) AS LONG
PRINT UBOUND(a)
SYSTEM
