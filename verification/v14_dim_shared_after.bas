$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): DIM SHARED after a SUB in the file: does the SUB see the variable?
SUB show
    PRINT "g in show:"; g
    g = 20
END SUB
DIM SHARED g AS LONG
g = 7
show
PRINT "g in main:"; g
SYSTEM
