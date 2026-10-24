$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): STRING * n member with a CONST defined below the TYPE.
TYPE r
    f AS STRING * n
END TYPE
CONST n = 2
DIM x AS r
PRINT LEN(x)
SYSTEM
