$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 3.2): SHARED g without a type in a SUB before DIM g AS LONG in main.
SUB show
    SHARED g
    PRINT "g in show:"; g
    g = 2.5
END SUB
DIM g AS LONG
g = 7
show
PRINT "g in main:"; g
SYSTEM
