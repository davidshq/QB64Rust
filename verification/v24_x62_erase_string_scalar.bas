$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): ERASE of a string variable.
s$ = "abc"
ERASE s$
PRINT s$
SYSTEM
