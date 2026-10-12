$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): __foo(1) = 2: a name with two leading underscores.
__foo(1) = 2
PRINT __foo(1); UBOUND(__foo)
SYSTEM
