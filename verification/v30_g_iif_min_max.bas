$CONSOLE:ONLY
' Verification (step 4 of M3, measured before its design): _IIF, _MIN, _MAX and _CLAMP. For _IIF: which part is
' evaluated (functions that print), the condition's type and a condition that is a float or a string compare,
' the value of two numeric parts of different types, strings of both kinds, nesting, its use as an argument and
' in an expression. For _MIN, _MAX and _CLAMP: signed against unsigned, floats, a value that is not a number, a
' _CLAMP whose bounds are the wrong way round. (LEN of a numeric expression is rejected, so the type of a result
' is not shown here; the C++ written for each pair of types shows it.)
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER, l AS LONG, ul AS _UNSIGNED LONG
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, x AS STRING * 3, v AS STRING
PRINT "_IIF: the part evaluated"
PRINT _IIF(1, t(1), t(2))
PRINT _IIF(0, t(1), t(2))
PRINT _IIF(t(0), t(1), t(2))
PRINT _IIF(1, ts$("a"), ts$("b"))
PRINT "the condition"
PRINT _IIF(-1, 1, 2); _IIF(2, 1, 2); _IIF(0.5, 1, 2); _IIF(0.4, 1, 2); _IIF(-0.5, 1, 2); _IIF(0#, 1, 2)
PRINT _IIF(1E-30, 1, 2); _IIF(4294967296, 1, 2); _IIF("a" < "b", 1, 2); _IIF(NOT 0, 1, 2)
d = 0.25: PRINT _IIF(d, 1, 2);: s = 0: PRINT _IIF(s, 1, 2);: q = 4294967296: PRINT _IIF(q, 1, 2)
b = 1: i = 2: l = 3: q = 4: s = 5: d = 6: f = 7: ub = 8: ui = 9: ul = 10: uq = 11
PRINT "values across types"
b = -1: ub = 255: PRINT _IIF(1, b, ub); _IIF(0, b, ub)
i = -1: ui = 65535: PRINT _IIF(1, i, ui); _IIF(0, i, ui)
l = -1: ul = 4294967295: PRINT _IIF(1, l, ul); _IIF(0, l, ul)
q = -1: uq = 18446744073709551615: PRINT _IIF(1, q, uq); _IIF(0, q, uq)
PRINT _IIF(1, 1, 2.5); _IIF(0, 1, 2.5); _IIF(1, 7, 2) / 2; _IIF(1, 7, 2) \ 2
i = _IIF(1, 2.5, 0): PRINT i;: i = _IIF(1, 3.5, 0): PRINT i
PRINT _IIF(1, 16777217, 1!); _IIF(1, 1 / 3, 1#)
PRINT "strings"
x = "ab": v = "var"
PRINT "["; _IIF(1, x, v); "]"; "["; _IIF(0, x, v); "]"; LEN(_IIF(1, x, v)); LEN(_IIF(0, x, "longer"))
PRINT _IIF(1, "a", "b") + _IIF(0, "a", "b"); LEN(_IIF(1, "", "b"))
v = _IIF(LEN(v) > 2, UCASE$(v), v): PRINT v
PRINT "nesting and use"
PRINT _IIF(1, _IIF(0, 1, 2), 3); _IIF(_IIF(1, 0, 1), 4, 5); _IIF(1, 2, 3) + _IIF(0, 2, 3)
PRINT ABS(_IIF(1, -4, 4)); STR$(_IIF(0, 1, 2)); MID$("abcdef", _IIF(1, 2, 3), 2)
l = 5: PRINT _IIF(l > 3, "big", "small"); _IIF(l > 3 AND l < 5, 1, 0); _IIF(l = 5, l * 2, l)
PRINT "_MIN and _MAX"
PRINT _MIN(1, 2); _MAX(1, 2); _MIN(-1, 1); _MAX(-1, 1); _MIN(2.5, 2); _MAX(2.5, 2); _MIN(1, 1)
b = -1: ub = 255: PRINT _MIN(b, ub); _MAX(b, ub)
i = -1: ui = 65535: PRINT _MIN(i, ui); _MAX(i, ui)
l = -1: ul = 4294967295: PRINT _MIN(l, ul); _MAX(l, ul)
q = -1: uq = 18446744073709551615: PRINT _MIN(q, uq); _MAX(q, uq)
b = 1: i = 2: l = 3: q = 4: s = 5: d = 6: f = 7
PRINT _MIN(1 / 3, 1#); _MAX(16777217, 1!); _MIN(t(1), t(2)); _MAX(_MIN(5, 9), 7)
d = 0: d = d / d: PRINT _MIN(d, 1); _MIN(1, d); _MAX(d, 1); _MAX(1, d)
PRINT "_CLAMP"
PRINT _CLAMP(5, 1, 10); _CLAMP(0, 1, 10); _CLAMP(11, 1, 10); _CLAMP(1, 1, 10); _CLAMP(10, 1, 10)
PRINT _CLAMP(2.5, 1, 2); _CLAMP(1, 1.5, 2); _CLAMP(-5, -3, -1); _CLAMP(5, 3, 3)
PRINT "bounds the wrong way round:"; _CLAMP(5, 10, 1); _CLAMP(0, 10, 1); _CLAMP(11, 10, 1)
b = 1: i = 2: l = 3: q = 4: s = 5: d = 6: f = 7
l = -1: ul = 4294967295: PRINT _CLAMP(l, 0, ul); _CLAMP(ul, l, 5); _CLAMP(t(5), t(1), t(9))
SYSTEM
FUNCTION t (n AS LONG)
    PRINT "<t"; n; ">";
    t = n
END FUNCTION
FUNCTION ts$ (a AS STRING)
    PRINT "<ts "; a; ">";
    ts$ = a
END FUNCTION
