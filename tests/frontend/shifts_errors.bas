' TEST: check-fail
$CONSOLE:ONLY
' The forms of _SHL, _SHR, _ROL and _ROR the old compiler rejects (m3-console-files tasks 2.3 and 2.4;
' verification\v30_x82-x87, x91, x92, and the _BIT rows of v30_result_types.txt), one line each: each is an error
' that is not "not supported yet"
DIM l AS LONG, s AS SINGLE, d AS DOUBLE, bt AS _BIT, ub AS _UNSIGNED _BIT, b4 AS _BIT * 4
PRINT _SHL("a", 1)
PRINT _SHL(1)
PRINT _ROL(1.5, 1)
PRINT _ROR(2.5, 1)
PRINT _ROL(s, 1)
PRINT _ROR(d, 1)
PRINT _ROL("a", 1)
PRINT _ROL(l, 1.5)
PRINT _ROR(l, s)
PRINT _ROL(bt, 1)
PRINT _ROR(ub, 1)
PRINT _ROL(b4, 1)
PRINT _ROL(l)
PRINT _ROR(l, 1, 2)
PRINT _SHR(1, "a")
PRINT _SHL
CONST c1 = _SHL(1, 4), c2 = _SHR(16, 2)
CONST c3 = _ROL(1, 4), c4 = _ROR(1, 1)
