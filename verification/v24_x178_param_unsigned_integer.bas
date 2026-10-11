$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an _UNSIGNED INTEGER array passed to an INTEGER array parameter,
' and an _UNSIGNED _BYTE array to a _BYTE one.
DIM q(3) AS _UNSIGNED INTEGER, b(3) AS _UNSIGNED _BYTE
fill q(), b()
PRINT q(1); b(1)
SYSTEM
SUB fill (a() AS INTEGER, c() AS _BYTE)
    a(1) = -1: c(1) = -1
END SUB
