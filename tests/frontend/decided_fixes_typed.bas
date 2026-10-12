' TEST: typed
$CONSOLE:ONLY
' The decided fixes of m2-numeric-types design D8 that change typing: an integer power beyond _INTEGER64 in a CONST
' is a DOUBLE (DIVERGENCES.md D-007; the old evaluator wraps 2 ^ 70 to -9223372036854775808), and an IMP whose
' left operand is an IMP is computed as written, (a IMP b) IMP c (D-005; the old compiler computes a OR b OR c).
CONST c = 2 ^ 70, m = 10 ^ 20
PRINT c; m
a = 5: b = 3: d = 0
PRINT a IMP b IMP d
SYSTEM
