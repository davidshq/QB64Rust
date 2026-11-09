$CONSOLE:ONLY
' Verification (study 44, section 2 item 1): a string FUNCTION g$ and a SUB g of the same name.
PRINT g$(1)
g 2
SYSTEM

FUNCTION g$ (n AS INTEGER)
    g$ = "f" + LTRIM$(STR$(n))
END FUNCTION

SUB g (n AS INTEGER)
    PRINT "sub"; n
END SUB
