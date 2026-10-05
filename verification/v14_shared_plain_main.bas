$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 3.2): after SHARED h AS LONG in an earlier SUB, does plain h in main mean h&?
SUB show2
    SHARED h AS LONG
    PRINT "h in show2:"; h
    h = 5
END SUB
h = 9.5
show2
PRINT "h in main:"; h; "h& in main:"; h&
SYSTEM
