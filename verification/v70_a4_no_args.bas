$CONSOLE:ONLY
' Verification (study 44, section 2 item 1): a SUB and a FUNCTION of the same name with no parameters. The name
' alone as a statement, and the name alone in an expression.
h
x = h
PRINT x
SYSTEM

SUB h
    PRINT "sub"
END SUB

FUNCTION h
    PRINT "function"
    h = 7
END FUNCTION
