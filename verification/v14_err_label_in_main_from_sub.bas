$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): RESUME in a SUB to a main-module label.
SUB t
RESUME back
END SUB
back:
PRINT "x"
SYSTEM
