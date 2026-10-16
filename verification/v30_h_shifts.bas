$CONSOLE:ONLY
' Verification (step 4 of M3, measured before its design): _SHL, _SHR, _ROL and _ROR. For each: every integer
' type of the first argument, a negative value, a count of 0, of the type's width, beyond it and negative, a
' count that is a float, a first argument that is a float or a literal, and its use in an expression.
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER, l AS LONG, ul AS _UNSIGNED LONG
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64, s AS SINGLE, d AS DOUBLE, o AS _OFFSET, n AS LONG
PRINT "_SHL"
PRINT _SHL(1, 4); _SHL(3, 1); _SHL(1, 0); _SHL(0, 5); _SHL(1, 31); _SHL(1, 32); _SHL(1, 63); _SHL(1, 64); _SHL(1, 65)
PRINT _SHL(-1, 1); _SHL(1, -1); _SHL(1.5, 1); _SHL(2.5, 1); _SHL(1, 1.5); _SHL(1, 2.5)
b = 1: ub = 1: i = 1: ui = 1: l = 1: ul = 1: q = 1: uq = 1
PRINT _SHL(b, 7); _SHL(b, 8); _SHL(ub, 7); _SHL(ub, 8); _SHL(i, 15); _SHL(i, 16); _SHL(ui, 15); _SHL(ui, 16)
PRINT _SHL(l, 31); _SHL(l, 32); _SHL(ul, 31); _SHL(ul, 32); _SHL(q, 63); _SHL(uq, 63); _SHL(q, 64)
b = -1: i = -1: l = -1: q = -1: PRINT _SHL(b, 1); _SHL(i, 1); _SHL(l, 1); _SHL(q, 1)
b = _SHL(b, 1): PRINT b;: i = _SHL(1, 15): PRINT i;: l = _SHL(1, 31): PRINT l
PRINT "_SHR"
PRINT _SHR(16, 4); _SHR(7, 1); _SHR(1, 0); _SHR(1, 1); _SHR(-1, 1); _SHR(-16, 2); _SHR(16, -1); _SHR(16, 64); _SHR(16, 65)
PRINT _SHR(5.5, 1); _SHR(16, 1.5); _SHR(4294967296, 1); _SHR(18446744073709551615, 63)
b = -128: ub = 128: i = -32768: ui = 32768: l = -2147483648: ul = 2147483648
PRINT _SHR(b, 1); _SHR(ub, 1); _SHR(i, 1); _SHR(ui, 1); _SHR(l, 1); _SHR(ul, 1)
q = -8: uq = 18446744073709551608: PRINT _SHR(q, 1); _SHR(uq, 1); _SHR(q, 63); _SHR(q, 64)
PRINT "_ROL"
b = 1: ub = 129: i = 1: ui = 32769: l = 1: ul = 2147483649: q = 1: uq = 9223372036854775809
PRINT _ROL(b, 1); _ROL(b, 7); _ROL(b, 8); _ROL(b, 9); _ROL(ub, 1); _ROL(ub, 0)
PRINT _ROL(i, 15); _ROL(i, 16); _ROL(ui, 1); _ROL(l, 31); _ROL(l, 32); _ROL(ul, 1); _ROL(q, 63); _ROL(q, 64); _ROL(uq, 1)
PRINT _ROL(1, 1); _ROL(1, 31); _ROL(1, 32); _ROL(-1, 1); _ROL(2147483648, 1); _ROL(4294967296, 1); _ROL(1, -1)
o = 1: PRINT _ROL(o, 1)
PRINT "_ROR"
b = 1: ub = 129: i = 1: ui = 32769: l = 1: ul = 2147483649: q = 1: uq = 9223372036854775809
PRINT _ROR(b, 1); _ROR(b, 8); _ROR(b, 9); _ROR(ub, 1); _ROR(ub, 0); _ROR(i, 1); _ROR(ui, 1)
PRINT _ROR(l, 1); _ROR(l, 32); _ROR(ul, 1); _ROR(q, 1); _ROR(uq, 1); _ROR(q, 64)
PRINT _ROR(1, 1); _ROR(2, 1); _ROR(-1, 1); _ROR(1, -1); _ROR(1, 33)
PRINT "in expressions"
n = 3: PRINT _SHL(1, n) + _SHR(64, n); _SHL(_SHL(1, 2), 2); _ROL(_ROR(l, 3), 3); HEX$(_SHL(255, 8)); _SHL(1, n) OR 1
SYSTEM
