$CONSOLE:ONLY
' Verification (m3-constant-evaluation, task 1.2): functions of the evaluator in constants without a suffix (their
' type by their text)
CONST d1 = SIN(.2)
PRINT "SIN(.2): "; _TOSTR$(d1); d1
CONST d2 = LOG(10)
PRINT "LOG(10): "; _TOSTR$(d2); d2
CONST d3 = EXP(1)
PRINT "EXP(1): "; _TOSTR$(d3); d3
CONST d4 = 20 ROOT 3
PRINT "20 ROOT 3: "; _TOSTR$(d4); d4
CONST d5 = SQR(2)
PRINT "SQR(2): "; _TOSTR$(d5); d5
CONST d6 = _PI(3)
PRINT "_PI(3): "; _TOSTR$(d6); d6
CONST d7 = _ACOS(.2)
PRINT "_ACOS(.2): "; _TOSTR$(d7); d7
CONST d8 = ATN(1E+10)
PRINT "ATN(1E+10): "; _TOSTR$(d8); d8
SYSTEM
