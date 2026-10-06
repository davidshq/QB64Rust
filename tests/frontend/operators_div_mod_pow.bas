' TEST: typed
$CONSOLE:ONLY
' numeric-semantics: integer division, MOD and power (m2-control-flow-slice D4). \ and MOD round float operands to
' _INTEGER64 and compute in the promoted width; they fold except by 0. ^ computes in _FLOAT and is never folded; an
' INTEGER operand is believed SINGLE, a LONG DOUBLE, an _INTEGER64 _FLOAT, the widest wins
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE
PRINT i \ 2; l MOD i; q \ l; s MOD 2
PRINT 7 \ 2; -7 MOD 3; 1 \ 0; 5 MOD 0
PRINT i ^ 2; l ^ 2; q ^ 2; i ^ d; 2 ^ 0.5
END
