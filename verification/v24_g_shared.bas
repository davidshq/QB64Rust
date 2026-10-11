$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): SHARED a() in a procedure, for a static and a dynamic main
' array, with AS and without; a main array that is not shared, named in a SUB; a local DIM of a name the main
' module has.
ON ERROR GOTO h
DIM g(3) AS LONG
DIM h(3)
REDIM d(3) AS LONG
DIM ns(3) AS LONG
g(2) = 8: h(2) = 1.5: d(2) = 4: ns(1) = 5
show
PRINT "main after the SUB: g(2), h(2), UBOUND(d), d(5), ns(1), ns(2)"; g(2); h(2); UBOUND(d); d(5); ns(1); ns(2)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB show
    SHARED g() AS LONG
    SHARED h()
    SHARED d() AS LONG
    PRINT "SUB: g(2), h(2), d(2)"; g(2); h(2); d(2)
    g(2) = 9: h(2) = 2.5
    REDIM d(5) AS LONG
    d(5) = 6
    PRINT "SUB, ns(1) of a main array that is not shared:";
    PRINT ns(1)
    DIM ns(8) AS LONG
    ns(2) = 3
    PRINT "SUB, after a local DIM ns(8): UBOUND, ns(1), ns(2)"; UBOUND(ns); ns(1); ns(2)
END SUB
