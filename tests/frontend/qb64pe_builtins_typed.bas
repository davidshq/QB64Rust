' TEST: typed
$CONSOLE:ONLY
' The built-ins of the upstream programs qb64pe/* in the typed tree (m4-qb64pe-sources task 2.1; measured in
' verification\v52_a_instrrev, v52_b_math, v52_c_files): _INSTRREV with and without a start (a LONG, the start a
' float converted as the LONG slot), _READFILE$ and _FULLPATH$ (strings), _WRITEFILE (two string slots), and
' _ARCSEC, _ARCCSC, _ARCCOT, _SECH, _CSCH of an integer, a SINGLE and a DOUBLE (believed _FLOAT)
DIM a AS STRING, n AS LONG, s AS SINGLE, d AS DOUBLE, i AS INTEGER, f AS _FLOAT
DIM fx AS STRING * 4
n = _INSTRREV(a, ".")
n = _INSTRREV(3, a, fx)
n = _INSTRREV(s, "a.b", ".")
a = _READFILE$(a)
a = _FULLPATH$(fx + ".")
_WRITEFILE a, _READFILE$("x")
_WRITEFILE fx, a + "b"
f = _ARCSEC(i)
f = _ARCCSC(s)
d = _ARCCOT(d)
f = _SECH(n)
f = _CSCH(f)
