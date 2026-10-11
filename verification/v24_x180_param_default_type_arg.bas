$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an array typed by DEFINT passed to a parameter typed by DEFINT,
' and to one written with AS INTEGER.
DEFINT A-Z
DIM q(3)
one q()
two q()
PRINT q(1); q(2)
SYSTEM
SUB one (a())
    a(1) = 2.5
END SUB
SUB two (a() AS INTEGER)
    a(2) = 3.5
END SUB
