$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): DECLARE SUB s ALIAS "t" () outside a library.
DECLARE SUB s ALIAS "t" ()
s
SYSTEM
SUB s
    PRINT "in s"
END SUB
