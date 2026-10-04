$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, review of task 3.2): DIM g AS LONG in a SUB after DIM SHARED g AS LONG in
' main (same name and type): a local, the shared one, or an error?
DIM SHARED g AS LONG
g = 7
s
PRINT "g in main:"; g
SYSTEM
SUB s
    DIM g AS LONG
    PRINT "g in s:"; g
    g = 3
END SUB
