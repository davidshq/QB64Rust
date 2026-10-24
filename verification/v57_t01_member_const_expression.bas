$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): STRING * n member with n a CONST expression.
CONST n = 2 + 1
TYPE r
    f AS STRING * n
END TYPE
DIM x AS r
PRINT LEN(x)
SYSTEM
