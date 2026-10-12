$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): a float CONST beyond 64-bit integers that is integer-valued.
CONST a = 1E+19, b = 2 ^ 70
PRINT a; b
SYSTEM
