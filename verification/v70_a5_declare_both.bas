$CONSOLE:ONLY
' Verification (study 44, section 2 item 1): a SUB and a FUNCTION of the same name, each with a DECLARE.
DECLARE SUB f (n AS INTEGER)
DECLARE FUNCTION f% (n AS INTEGER)
f 1
PRINT f(2)
SYSTEM

SUB f (n AS INTEGER)
    PRINT "sub"; n
END SUB

FUNCTION f% (n AS INTEGER)
    f = n + 100
END FUNCTION
