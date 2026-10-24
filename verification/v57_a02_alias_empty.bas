$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): DECLARE SUB s ALIAS "" () outside a library.
DECLARE SUB s ALIAS "" ()
s
SYSTEM
SUB s
    PRINT "in s"
END SUB
