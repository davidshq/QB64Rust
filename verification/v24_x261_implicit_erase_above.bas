$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 5.1): ERASE of a name that is used as an array only below it.
ERASE z
z(1) = 5
PRINT z(1)
SYSTEM
