$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): what a default type gives its type to: DIM, DIM SHARED, an
' array, REDIM, STATIC, SHARED, a parameter, a FOR variable, a CONST. LEN of a variable shows its type: 2
' INTEGER, 4 LONG or SINGLE, 8 DOUBLE.
DEFINT A-Z
DIM dv: dv = 2.5: PRINT "DIM dv:"; dv; LEN(dv)
DIM SHARED gv: gv = 2.5: PRINT "DIM SHARED gv:"; gv; LEN(gv)
DIM da(3): da(1) = 2.5: PRINT "DIM da(3):"; da(1); LEN(da(1))
REDIM ra(3): ra(1) = 2.5: PRINT "REDIM ra(3):"; ra(1); LEN(ra(1))
FOR fi = 1 TO 2: NEXT: PRINT "FOR fi:"; fi; LEN(fi)
CONST ck = 2.5: PRINT "CONST ck = 2.5:"; ck
proc 5
PRINT "the scenario: array and parameter"
DIM t(3): t(1) = 2.5: PRINT t(1)
p 5
SYSTEM
SUB proc (pv)
    STATIC sv
    SHARED dv
    lv = 2.5
    PRINT "parameter, STATIC, SHARED dv, local, DIM SHARED gv:"; LEN(pv); LEN(sv); dv; LEN(lv); gv
END SUB
SUB p (n)
    PRINT n / 2
END SUB
