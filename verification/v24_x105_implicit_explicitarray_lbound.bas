$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): LBOUND of a name that is no array under OPTION _EXPLICITARRAY.
OPTION _EXPLICITARRAY
PRINT LBOUND(q)
SYSTEM
