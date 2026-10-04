$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): SHARED h AS LONG in a SUB names the main-module h& even when main never DIMs it.
SUB show2
    SHARED h AS LONG
    PRINT "h in show2:"; h
    h = 5
END SUB
h& = 9
show2
PRINT "h& in main:"; h&
SYSTEM
