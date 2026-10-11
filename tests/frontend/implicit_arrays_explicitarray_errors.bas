' TEST: check-fail
$CONSOLE:ONLY
' OPTION _EXPLICITARRAY (m2-defxxx-common-arrays task 5.1; verification\v24_x105, x257): an array used without
' DIM is "Array 'z' (SINGLE) not defined", above and below the OPTION line, for LBOUND and in a SUB too; a scalar
' used without DIM and a declared array are fine.
z(1) = 5
OPTION _EXPLICITARRAY
x = 1
DIM d(3) AS LONG
d(1) = x
PRINT LBOUND(q)
i%(2) = 1
SYSTEM
SUB p
    a(1) = 2
END SUB
