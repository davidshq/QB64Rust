' TEST: check-fail
$CONSOLE:ONLY
' OPTION _EXPLICIT (m2-defxxx-common-arrays task 5.1; verification\v24_x256): an array used without DIM is an
' error as under OPTION _EXPLICITARRAY ("Array 'z' (SINGLE) not defined"); a declared one is fine.
OPTION _EXPLICIT
DIM d(3) AS LONG
d(1) = 2
z(1) = 5
PRINT UBOUND(u$)
SYSTEM
