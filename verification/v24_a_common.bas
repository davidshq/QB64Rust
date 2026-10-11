$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON of a name typed by a default. LEN of a variable shows its
' type: 2 INTEGER, 4 LONG or SINGLE, 8 DOUBLE.
DEFINT A-Z
COMMON cv
COMMON SHARED cg
cv = 2.5: cg = 2.5: PRINT "COMMON cv, COMMON SHARED cg:"; cv; LEN(cv); cg; LEN(cg)
show
SYSTEM
SUB show
    PRINT "cg in a SUB:"; cg; LEN(cg)
END SUB
