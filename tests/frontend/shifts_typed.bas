' TEST: typed
$CONSOLE:ONLY
' _SHL, _SHR, _ROL and _ROR (m3-console-files task 2.3, design D7; verification\v30_h_shifts,
' v30_result_types.txt). _SHL and _SHR are plain: the value an _UNSIGNED _INTEGER64 (a float rounded), the count a
' LONG, the result an _UNSIGNED _INTEGER64. _ROL and _ROR take two integers as they are and turn within the width
' of the first one's type, a literal by its size; the result is unsigned of that width
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER, l AS LONG, ul AS _UNSIGNED LONG
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64, s AS SINGLE, d AS DOUBLE, o AS _OFFSET, uo AS _UNSIGNED _OFFSET
DIM b8 AS _BIT * 8, v AS STRING
PRINT _SHL(i, 2); _SHR(s, d); _SHL(1, 2); _SHR(q, q); _SHL(uq, ub); _SHR(-1, 1)
PRINT _ROL(b, 1); _ROL(ub, 1); _ROL(i, 1); _ROL(ui, 1); _ROL(l, 1); _ROL(ul, 1); _ROL(q, 1); _ROL(uq, 1)
PRINT _ROR(o, 1); _ROR(uo, 1); _ROR(b8, 1)
' Literals of three sizes, a negative one, an expression (believed an _INTEGER64)
PRINT _ROL(1, 2); _ROL(70000, i); _ROR(5000000000, l); _ROL(-1, 1); _ROL(i + 1, 2)
' The count of any integer type
PRINT _ROL(b, q); _ROR(l, ub); _ROL(i, uq)
' What follows sees the type
l = _ROL(i, 1) + 1
uq = _SHL(1, 40) OR _SHR(uq, 1)
v = HEX$(_ROL(b, 1)) + HEX$(_SHL(b, 1))
PRINT _MIN(_ROL(i, 1), b)
SYSTEM
