' TEST: check-fail
$CONSOLE:ONLY
' The functions of the CONST evaluator rejected (m3-constant-evaluation task 3.2), one form of each verification
' program v45_x01-x34, x62, x63, x66, where the old compiler rejects it too: by its evaluator (SQR of a negative,
' _ARCSEC and _ARCCSC below 1, ASC of an empty string or a number or with a string position, a wrong argument
' count, a string to a number function, an invalid screen mode, a suffix of the other kind on a constant, _ACOS
' giving NAN) or by its own run time inside qb64pe.exe, which stops it (LOG of 0 or a negative, _SECH beyond its
' range, ASC beyond the string, EXP beyond range, _COT, _CSC and _CSCH of 0, a negative to a fractional power, a
' palette index outside the mode's palette).
CONST n = 5, s = "ab"
CONST x01 = SQR(-1)
CONST x02 = _ARCSEC(.5)
CONST x03 = _ARCCSC(-.5)
CONST x04 = LOG(0)
CONST x05 = LOG(-1)
CONST x06 = _SECH(100)
CONST x07 = _ACOS(2)
CONST x08 = ASC("")
CONST x09 = ASC("ab", 5)
CONST x10 = ASC("ab", 0)
CONST x11 = SIN()
CONST x12 = SIN(1, 2)
CONST x13 = SIN("a")
CONST x14 = _RGB32()
CONST x15 = _RGB32(1, 2, 3, 4, 5)
CONST x16 = _RGBA32(1, 2, 3)
CONST x17 = _RGB(1, 2, 3, 5)
CONST x18 = _RED(1, 3)
CONST x19 = ASC(65)
CONST x20 = ASC("a", "b")
CONST x21 = n$ + ""
CONST x22 = s% + 1
CONST x23 = s% + "x"
CONST x24 = _PI(1, 2)
CONST x28 = EXP(20000)
CONST x29 = EXP(1000)
CONST x30 = _COT(0)
CONST x31 = _CSC(0)
CONST x33 = _CSCH(0)
CONST x34 = (-8) ^ 0.5
CONST x62 = _RED(16, 12)
CONST x63 = _RED(&HFF112233, 0)
CONST x66 = _RED(-1, 256)
SYSTEM
