' TEST: typed
$CONSOLE:ONLY
' The functions added by demand in the typed tree (m2-builtin-statements task 7.4; measured in
' verification\v22_f_functions and from the C++): _ACOS, _ASIN, _SINH, _COSH, _TANH and _CEIL are believed _FLOAT
' and held as C++ overloading gives for the argument's held type (a SINGLE stays a float, an integer is a double);
' _COT, _CSC, _SEC, _D2R and _R2D are believed _FLOAT and held DOUBLE (libqb takes and returns a double); the
' argument is passed in its own type; _STRCMP and _STRICMP take two strings and are LONG; _STARTDIR$ has no
' argument and is called by its bare name (verification\v22_f_startdir)
DIM i AS INTEGER, l AS LONG, sg AS SINGLE, d AS DOUBLE, f AS _FLOAT, s AS STRING, t AS STRING
f = _ACOS(sg)
f = _ACOS(d)
f = _ACOS(f)
f = _ACOS(i)
f = _ASIN(sg)
f = _ASIN(l)
f = _SINH(d)
f = _COSH(f)
f = _TANH(.5)
f = _CEIL(sg)
f = _CEIL(l)
d = _CEIL(sg) / 3
f = _COT(sg)
f = _CSC(d)
f = _SEC(f)
f = _D2R(i)
f = _R2D(.5)
d = _D2R(sg) / 3
l = _STRCMP(s, t)
l = _STRICMP(s, "a")
PRINT _ACOS(sg); _COT(d); _STRCMP(s, t) * 3000000000
s = _STARTDIR$
CHDIR _STARTDIR$ + t
