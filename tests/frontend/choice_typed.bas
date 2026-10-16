' TEST: typed
$CONSOLE:ONLY
' _IIF, _MIN, _MAX and _CLAMP (m3-console-files tasks 2.1, 2.2, design D6; verification\v30_g_iif_min_max,
' v30_result_types.txt): the result is typed by what the arguments are believed, with a rule that is not the
' operators': a float wins over any integer, of two integers the wider, of two of one width the unsigned one. The
' arguments stay as they are (the C++ call converts them); the condition of _IIF is an integer, a float rounded
TYPE t
    s AS SINGLE
    q AS _INTEGER64
END TYPE
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER, l AS LONG, ul AS _UNSIGNED LONG
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, o AS _OFFSET
DIM uo AS _UNSIGNED _OFFSET, bt AS _BIT, ub3 AS _UNSIGNED _BIT * 3, x AS STRING * 5, v AS STRING, r AS t
DIM a(3) AS SINGLE
' A float beside an integer, also a wider one; two floats
PRINT _MIN(s, q); _MAX(q, s); _MIN(d, uq); _MAX(s, d); _MIN(f, s); _MIN(a(1), q); _MAX(r.s, r.q)
' Two integers: the wider; of one width the unsigned one, the first of two signed ones, the second of two unsigned
PRINT _MIN(b, i); _MAX(ub, i); _MIN(b, ub); _MIN(ub, b); _MAX(l, ul); _MIN(q, o); _MIN(o, q); _MIN(uq, uo)
PRINT _MIN(uo, uq); _MIN(o, uq); _MAX(uo, o)
' _BIT types count their bits
PRINT _MIN(bt, bt); _MIN(bt, ub3); _MIN(ub3, b)
' Literals and expressions by what they are believed
PRINT _MIN(2, 3); _MIN(70000, i); _MIN(5000000000, i); _MIN(1.5, l); _MIN(1.5#, s); _MIN(i + 1, i); _MIN(i = 2, b)
' _CLAMP: the widest of the three
PRINT _CLAMP(i, 1, 10); _CLAMP(i, 1.5, 10); _CLAMP(b, l, uq); _CLAMP(q, s, d)
' What follows sees the type: an operator, a store, HEX$
l = _MIN(i, b) + 1
s = _MAX(l, s) * 2
v = HEX$(_MIN(i, b)) + STR$(_MIN(o, q))
' _IIF: numbers of two types, strings of two kinds, a float as the condition, a comparison
PRINT _IIF(i, 1, 2); _IIF(i, 1.5, l); _IIF(s, i + 1, b); _IIF(q, i, ub); _IIF(i > 1, o, q)
PRINT _IIF(d, v, "x" + v); _IIF(i, x, "abc"); LEN(_IIF(0, x, v))
s = _IIF(d, s, i)
l = _IIF(1, o, s) + _IIF(i, bt, bt)
v = _IIF(_IIF(i, 0, 1), "a", "b")
PRINT _MIN(_IIF(i, b, l), _MAX(s, 1))
SYSTEM
