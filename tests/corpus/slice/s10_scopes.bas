$CONSOLE:ONLY
' Procedure scopes (spec language/procedures, "Procedure scopes", "DECLARE lines"): locals (DIM and implicit) are
' new on every call; STATIC keeps its value; SHARED binds a main-module variable; DIM SHARED is seen only by
' procedures after it in the file; DECLARE lines change nothing.
DECLARE SUB counter ()
DECLARE FUNCTION getg& ()
SUB early
    PRINT "g in early:"; g
END SUB
DIM SHARED g AS LONG
g = 10
early
x = 99
localx
localx
PRINT "main x:"; x
counter
counter
counter
PRINT "getg:"; getg&
PRINT "main g after getg:"; g
h& = 9
useshared
PRINT "main h& after useshared:"; h&
samename 5
PRINT "main y:"; y
SYSTEM

SUB localx
    PRINT "x in localx:"; x
    x = 5
    DIM k AS LONG
    PRINT "k in localx:"; k
    k = 7
END SUB

SUB counter
    STATIC c AS LONG
    STATIC acc$
    c = c + 1
    acc$ = acc$ + "a"
    PRINT "count:"; c; acc$
END SUB

FUNCTION getg&
    PRINT "g in getg:"; g
    g = 20
    getg& = g * 2
END FUNCTION

SUB useshared
    SHARED h AS LONG
    PRINT "h in useshared:"; h
    h = 5
END SUB

SUB samename (y AS LONG)
    PRINT "y in samename:"; y
END SUB
