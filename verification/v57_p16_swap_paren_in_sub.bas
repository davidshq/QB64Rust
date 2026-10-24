$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): SWAP (a), b of SUB parameters.
a& = 1: b& = 2
s a&, b&
PRINT a&; b&
SYSTEM
SUB s (a AS LONG, b AS LONG)
    SWAP (a), b
END SUB
