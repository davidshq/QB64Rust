$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): STRING * n member with a CONST n& defined above.
CONST n& = 4
TYPE r
    f AS STRING * n&
END TYPE
DIM x AS r
PRINT LEN(x)
SYSTEM
