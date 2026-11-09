$CONSOLE:ONLY
' Verification (study 44, section 2 item 1): a SUB and a FUNCTION of the same name, the SUB first.
f 3
PRINT f(2)
SYSTEM

SUB f (n AS INTEGER)
    PRINT "sub"; n
END SUB

FUNCTION f& (n AS INTEGER)
    f = n * 10
END FUNCTION
