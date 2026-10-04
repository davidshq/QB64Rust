$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): SHARED h AS LONG in a SUB, then DIM h AS LONG in main (the old compiler fails in C++).
SUB show2
    SHARED h AS LONG
    PRINT "h in show2:"; h
END SUB
DIM h AS LONG
h = 9
show2
SYSTEM
