' TEST: typed
$CONSOLE:ONLY
' numeric-semantics: logical operators (m2-control-flow-slice D4). In the promoted width of the operands (32 bits for
' INTEGER and LONG, 64 with an _INTEGER64); a float operand rounded half to even to _INTEGER64 first, also for
' _ANDALSO, _ORELSE and _NEGATE; _ANDALSO, _ORELSE and _NEGATE give a LONG; integer constants fold
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE
PRINT NOT i; i AND i; l OR i; l XOR q
PRINT i EQV l; i IMP q
PRINT NOT s; s AND 3; 1.5 OR i
PRINT i _ANDALSO l; q _ORELSE i; _NEGATE i; _NEGATE s; s _ANDALSO 1
PRINT 6 AND 3; NOT 5; 5 EQV 3; 5 IMP 3; 1 _ANDALSO 2; 0 _ORELSE 0; _NEGATE 0
END
