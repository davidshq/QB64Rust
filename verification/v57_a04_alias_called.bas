$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): DECLARE SUB s ALIAS t (): s called, and t?
DECLARE SUB s ALIAS t ()
s
t
SYSTEM
SUB s
    PRINT "in s"
END SUB
