' TEST: ir
$CONSOLE:ONLY
' _IIF in the IR (m3-console-files task 2.2, design D6): a choice, one expression with its condition and its two
' parts, so that only the part chosen is evaluated: a FUNCTION or an element in a part stays inside it. The parts
' are as they are; the calls of _MIN, _MAX and _CLAMP take their arguments as they are too
DIM i AS INTEGER, l AS LONG, s AS SINGLE, v AS STRING, a(3) AS LONG
l = _IIF(i > 0, a(i), 0)
l = _IIF(s, f&(1), f&(2))
v = _IIF(i, v + "a", MID$(v, 2))
PRINT _IIF(i, 1, 2.5); _IIF(i, "a", v)
IF _IIF(i, l, s) > 1 THEN l = _MIN(i, l)
s = _MAX(s, a(1)) + _CLAMP(i, f&(0), 10)
SYSTEM
FUNCTION f& (n AS LONG)
    f& = n
END FUNCTION
