$CONSOLE:ONLY
' Verification (m2-builtin-statements task 7.4): the plain functions asked for by the corpus, upstream programs or
' Civil War Strategy that an existing rule may fit: result types (by the printed digits and by arithmetic) and edge
' values of _ACOS, _ASIN, _SINH, _COSH, _TANH, _COT, _CSC, _SEC, _D2R, _R2D, _CEIL, _STRCMP, _STRICMP.
ON ERROR GOTO h
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT
i = 1: l = 1: q = 1: s = .5: d = .5: f = .5
PRINT "_ACOS"; _ACOS(s); _ACOS(d); _ACOS(f); _ACOS(i); _ACOS(l); _ACOS(q); _ACOS(.5); _ACOS(1)
PRINT "_ASIN"; _ASIN(s); _ASIN(d); _ASIN(f); _ASIN(i); _ASIN(l); _ASIN(q); _ASIN(.5); _ASIN(1)
PRINT "_SINH"; _SINH(s); _SINH(d); _SINH(f); _SINH(i); _SINH(l); _SINH(q); _SINH(.5); _SINH(1)
PRINT "_COSH"; _COSH(s); _COSH(d); _COSH(f); _COSH(i); _COSH(l); _COSH(q); _COSH(.5); _COSH(1)
PRINT "_TANH"; _TANH(s); _TANH(d); _TANH(f); _TANH(i); _TANH(l); _TANH(q); _TANH(.5); _TANH(1)
PRINT "_COT"; _COT(s); _COT(d); _COT(f); _COT(i); _COT(l); _COT(q); _COT(.5); _COT(1)
PRINT "_CSC"; _CSC(s); _CSC(d); _CSC(f); _CSC(i); _CSC(l); _CSC(q); _CSC(.5); _CSC(1)
PRINT "_SEC"; _SEC(s); _SEC(d); _SEC(f); _SEC(i); _SEC(l); _SEC(q); _SEC(.5); _SEC(1)
PRINT "_D2R"; _D2R(s); _D2R(d); _D2R(f); _D2R(i); _D2R(l); _D2R(q); _D2R(.5); _D2R(180)
PRINT "_R2D"; _R2D(s); _R2D(d); _R2D(f); _R2D(i); _R2D(l); _R2D(q); _R2D(.5); _R2D(1)
PRINT "_CEIL"; _CEIL(s); _CEIL(d); _CEIL(f); _CEIL(i); _CEIL(l); _CEIL(q); _CEIL(.5); _CEIL(-1.5); _CEIL(2.000001)
PRINT "a third of each:"; _ACOS(s) / 3; _SINH(d) / 3; _D2R(i) / 3; _CEIL(s) / 3; _CEIL(l) / 3
PRINT "out of range: _ACOS(2)"; _ACOS(2); "_ASIN(-2)"; _ASIN(-2)
PRINT "at zero: _COT(0)"; _COT(0); "_CSC(0)"; _CSC(0); "_SEC(0)"; _SEC(0)
PRINT "large: _SINH(1000)"; _SINH(1000); "_COSH(1000)"; _COSH(1000); "_TANH(1000)"; _TANH(1000)
PRINT "_CEIL of large values:"; _CEIL(1E+20); _CEIL(123456789.5#); _CEIL(-.5)
PRINT "_STRCMP:"; _STRCMP("a", "b"); _STRCMP("b", "a"); _STRCMP("a", "a"); _STRCMP("a", "A"); _STRCMP("", "a"); _STRCMP("ab", "a")
PRINT "_STRICMP:"; _STRICMP("a", "b"); _STRICMP("b", "a"); _STRICMP("a", "A"); _STRICMP("", ""); _STRICMP("AB", "a")
PRINT "_STRCMP in arithmetic:"; _STRCMP("a", "b") * 3000000000
PRINT "done"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
