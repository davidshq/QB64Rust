' TEST: check-fail
$CONSOLE:ONLY
' The CONST rulings of 2026-10-18 that reject a program the old compiler accepts (const-rulings), one error a line:
' a literal beyond _UNSIGNED _INTEGER64 (D-027; the old evaluator clamps it to 9223372036854775807), a value beyond
' _FLOAT's range (D-029; the old evaluator gives 0, verification\v45_x25, x38), _COTH(0) (D-031; the old compiler's
' is 1, v45_x32) and 0 to a negative power (D-028: a float, so a division by zero; the old evaluator's qbr gives
' -9223372036854775808, v45_x40).
CONST a = 18446744073709551616
CONST b = 1F+4000 * 1F+4000
CONST c = 2 ROOT 0
CONST d = _COTH(0)
CONST e = 0 ^ -1
CONST f = -(1F+4000 * 1F+4000)
SYSTEM
