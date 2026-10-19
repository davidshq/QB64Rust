' TEST: typed
$CONSOLE:ONLY
' The CONST rulings of 2026-10-18 that change a value (const-rulings). D-027: an unsuffixed literal beyond
' _INTEGER64 is read as a statement reads it, a float result beyond it stays a _FLOAT, and the four limits hold
' their types' limits (the old evaluator clamps all of them to 9223372036854775807 or -9223372036854775807).
' D-028: ^ gives a float in every lane, read back by its text (the old evaluator's 2 ^ -1 is 0; 2 ^ 3 is still 8).
' D-031: _COTH is the hyperbolic cotangent (the old compiler's is 2 * EXP(x) - 1).
CONST big2~&& = 18446744073709551615
CONST plain = 18446744073709551615
CONST lowest = -9223372036854775808
CONST g = 9.3E+18 / 1, g3 = -9.9E+18 / 1
PRINT big2~&&; plain; lowest; g; g3
PRINT _INTEGER64_MIN; _UINTEGER64_MAX; _OFFSET_MIN; _UOFFSET_MAX
CONST half = 2 ^ -1, quarter = 2 ^ -2, eight = 2 ^ 3, wide = 2 ^ 62 * 4
PRINT half; quarter; eight; wide
CONST ct## = _COTH(.2), ct1 = _COTH(45)
PRINT ct##; ct1
SYSTEM
