$Console:Only
' The functions added by demand (spec language/builtin-functions "Supported functions"): _ACOS, _ASIN, _SINH,
' _COSH, _TANH, _COT, _CSC, _SEC, _D2R, _R2D, _CEIL with an argument of every kind (a SINGLE is computed as a
' float by the std:: functions, an integer as a double), in arithmetic, at the values where _COT, _CSC and _SEC
' raise error 5, at large values; _STRCMP and _STRICMP. _COTH only as above 1: the old compiler's is 2 * EXP(x) - 1,
' the hyperbolic cotangent here (DIVERGENCES.md D-031; its values in the CLI test const_rulings_run).
On Error GoTo h
Dim i As Integer, l As Long, q As _Integer64, s As Single, d As Double, f As _Float, a As String, b As String
i = 1: l = 1: q = 1: s = .5: d = .5: f = .5
Print "_ACOS"; _Acos(s); _Acos(d); _Acos(f); _Acos(i); _Acos(l); _Acos(q); _Acos(.5); _Acos(1)
Print "_ASIN"; _Asin(s); _Asin(d); _Asin(f); _Asin(i); _Asin(l); _Asin(q); _Asin(.5); _Asin(1)
Print "_SINH"; _Sinh(s); _Sinh(d); _Sinh(f); _Sinh(i); _Sinh(l); _Sinh(q); _Sinh(.5); _Sinh(1)
Print "_COSH"; _Cosh(s); _Cosh(d); _Cosh(f); _Cosh(i); _Cosh(l); _Cosh(q); _Cosh(.5); _Cosh(1)
Print "_TANH"; _Tanh(s); _Tanh(d); _Tanh(f); _Tanh(i); _Tanh(l); _Tanh(q); _Tanh(.5); _Tanh(1)
Print "_COT"; _Cot(s); _Cot(d); _Cot(f); _Cot(i); _Cot(l); _Cot(q); _Cot(.5); _Cot(1)
Print "_CSC"; _Csc(s); _Csc(d); _Csc(f); _Csc(i); _Csc(l); _Csc(q); _Csc(.5); _Csc(1)
Print "_SEC"; _Sec(s); _Sec(d); _Sec(f); _Sec(i); _Sec(l); _Sec(q); _Sec(.5); _Sec(1)
Print "_D2R"; _D2R(s); _D2R(d); _D2R(f); _D2R(i); _D2R(l); _D2R(q); _D2R(.5); _D2R(180)
Print "_R2D"; _R2D(s); _R2D(d); _R2D(f); _R2D(i); _R2D(l); _R2D(q); _R2D(.5); _R2D(1)
Print "_CEIL"; _Ceil(s); _Ceil(d); _Ceil(f); _Ceil(i); _Ceil(l); _Ceil(q); _Ceil(.5); _Ceil(-1.5); _Ceil(2.000001)
Print "a third of each:"; _Acos(s) / 3; _Sinh(d) / 3; _D2R(i) / 3; _Ceil(s) / 3; _Ceil(l) / 3
Print "a third of each:"; _Asin(s) / 3; _Cosh(s) / 3; _Tanh(s) / 3; _Cot(s) / 3; _Csc(s) / 3; _Sec(s) / 3; _R2D(s) / 3
f = 1 / 3: Print "of a _FLOAT third:"; _Asin(f) * 3; _Cot(f) * 3; _D2R(f) * 3; _Ceil(f) * 3
d = _Acos(s): Print "stored in a DOUBLE:"; d
d = .5: s = _Acos(d): Print "stored in a SINGLE:"; s: s = .5
l = _Ceil(2.5): Print "stored in a LONG:"; l
l = _R2D(1): Print "_R2D(1) in a LONG:"; l
Print "_COT(0)": d = _Cot(0): Print d
Print "_CSC(0)": d = _Csc(0): Print d
Print "_SEC(0)": d = _Sec(0): Print d
Print "_COTH above 1:"; _Coth(s) > 1; _Coth(d + .5) > 1; _Coth(i + 1) > 1
Print "_SINH(1000)"; _Sinh(1000); "_COSH(1000)"; _Cosh(1000); "_TANH(1000)"; _Tanh(1000)
Print "_CEIL of large values:"; _Ceil(1E+20); _Ceil(123456789.5#); _Ceil(-.5)
Print "nested:"; _R2D(_Acos(0)); _Ceil(_D2R(180)); _Asin(_Sinh(0))
Print "_STRCMP:"; _StrCmp("a", "b"); _StrCmp("b", "a"); _StrCmp("a", "a"); _StrCmp("a", "A"); _StrCmp("", "a"); _StrCmp("ab", "a")
Print "_STRICMP:"; _StriCmp("a", "b"); _StriCmp("b", "a"); _StriCmp("a", "A"); _StriCmp("", ""); _StriCmp("AB", "a")
a = "Hello": b = "hello"
Print "of variables:"; _StrCmp(a, b); _StriCmp(a, b); _StrCmp(a + "x", b); _StriCmp(a + "x", b)
Print "_STRCMP in arithmetic:"; _StrCmp("a", "b") * 3000000000
If _StriCmp(a, b) = 0 Then Print "equal but for case"
Print "done"
System
h:
Print "  error"; Err
Resume Next
