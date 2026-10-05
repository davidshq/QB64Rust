$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 3.2): SHARED g without a type in a SUB after DIM g AS LONG in main.
DIM g AS LONG
g = 7
show
PRINT "g in main:"; g; "g! in main:"; g!
SYSTEM
SUB show
    SHARED g
    PRINT "g in show:"; g
    g = 2.5
END SUB
