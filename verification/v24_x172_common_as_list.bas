$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON with the type before the names, and COMMON SHARED
' likewise.
COMMON AS LONG a, b()
COMMON SHARED AS LONG c
DIM b(3) AS LONG
a = 100000: b(1) = 2: c = 3: PRINT a; b(1); c
SYSTEM
