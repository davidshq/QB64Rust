' TEST: check-fail
$CONSOLE:ONLY
' _NEGATE right of the power operator is rejected by the old compiler ("Expected variable/value after '^'",
' verification\v24_x238_power_negate); NOT there is accepted
DIM x AS LONG
PRINT 2 ^ _NEGATE x + 3
PRINT 2 ^ NOT x
SYSTEM
