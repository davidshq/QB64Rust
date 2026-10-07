$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a large static array (40 MB).
DIM big(10000000) AS LONG
big(10000000) = 7
PRINT big(10000000); big(0); UBOUND(big)
SYSTEM
