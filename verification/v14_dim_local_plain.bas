$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, review of task 3.2): DIM g (no type) in a SUB after DIM SHARED g AS LONG
' in main. Which g do the plain names in the SUB mean afterwards?
DIM SHARED g AS LONG
g = 7
s
PRINT "g in main:"; g
SYSTEM
SUB s
    DIM g
    g = 2.5
    PRINT "g in s:"; g; "g! in s:"; g!; "g& in s:"; g&
END SUB
