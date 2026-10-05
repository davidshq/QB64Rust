$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, review of task 3.2): DIM g AS STRING in a SUB after DIM SHARED g AS LONG
' in main (same name, another type).
DIM SHARED g AS LONG
g = 7
s
PRINT "g in main:"; g
SYSTEM
SUB s
    DIM g AS STRING
    g = "x"
    PRINT "g in s: "; g; " g& in s:"; g&
END SUB
