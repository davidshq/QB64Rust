$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): TYPE variables in SUBs: local (new on each call), STATIC, SHARED.
TYPE pt
    x AS LONG
END TYPE
DIM SHARED g AS pt
DIM m AS pt
m.x = 9
loc
loc
st
st
g.x = 3
sh
PRINT "g.x:"; g.x
SYSTEM

SUB loc
    DIM p AS pt
    PRINT "local p.x:"; p.x
    p.x = 5
END SUB

SUB st
    STATIC q AS pt
    PRINT "static q.x:"; q.x
    q.x = q.x + 5
END SUB

SUB sh
    SHARED m AS pt
    PRINT "shared m.x:"; m.x; " g.x:"; g.x
    g.x = g.x + 1
END SUB
