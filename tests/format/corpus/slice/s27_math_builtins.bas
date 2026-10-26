$Console:Only
' Slice program (m2-core-builtins, D9): the math built-ins, their result types (shown by printing) and their edge
' values and errors (verification\v20_b_result_types, v20_f_math_edges). Each raising result is stored into a
' variable that held 7 first, so the placeholder shows. Errors are trapped: the handler prints ERR and resumes
' next. No PRINT comma.
On Error GoTo h
Dim i As Integer, l As Long, q As _Integer64, f As Single, d As Double, x As _Float
' ABS, INT, FIX keep the argument's type.
Print Abs(-7%) / 2; Abs(-7&) / 2; Abs(-2.5!); Abs(-2.5#); Abs(-7&&)
i = -32768: l = -2147483648: q = -9223372036854775807 - 1
Print Abs(i); Abs(l); Abs(q)
f = -2.5: d = -2.5: x = -2.5
Print Int(f); Int(d); Int(x); Fix(f); Fix(d); Fix(x); Int(-7%); Fix(-7&)
Print Int(-0.5); Int(-1.5); Int(0.5); Int(1.5); Fix(-0.5); Fix(-1.5); Fix(0.5); Fix(1.5)
f = 16777217: d = 9007199254740993#
Print Int(f) + 1 / 3; Int(d) / 3
' SGN.
f = 0: d = 0: d = -d
Print Sgn(0); Sgn(f); Sgn(d); Sgn(-5); Sgn(5); Sgn(-0.5); Sgn(-7&&); Sgn(2.5#)
' SIN … LOG, SQR, EXP: typed by the argument.
i = 2: l = 2: q = 2: f = 2: d = 2: x = 2
Print Sqr(i); Sqr(l); Sqr(q); Sqr(f); Sqr(d); Sqr(x); Sqr(2%); Sqr(2#)
Print Sin(i); Sin(l); Sin(q); Sin(f); Sin(d); Sin(x)
Print Cos(i); Cos(l); Cos(f); Cos(d); Tan(i); Tan(l); Tan(f); Tan(d)
Print Atn(i); Atn(l); Atn(f); Atn(d); Log(i); Log(l); Log(q); Log(f); Log(d); Log(x)
Print Exp(i); Exp(l); Exp(q); Exp(f); Exp(d); Exp(x)
Print Sqr(i) * 3; Sqr(l) * 3; Exp(f) / 7; Sin(q) / 3
' Conversions.
Print CInt(2.5); CInt(3.5); CInt(0.5); CInt(1.5); CInt(-0.5); CInt(-1.5); CInt(-1.2) / 3
Print CLng(-1.2) / 3; CLng(70000.5); CLng(70001.5); CLng(0.5); CLng(2.5); CLng(-2.5)
l = 16777217
Print CSng(l) - 16777216; CSng(l); CSng(2# / 3); CDbl(2! / 3); CDbl(2 / 3); CDbl(l) / 3
Print _Round(0.5); _Round(1.5); _Round(2.5); _Round(-0.5); _Round(-1.5); _Round(7); _Round(2.5) / 3
Print _Pi; _Pi(2); _Pi / 3; _Pi(2%); _Pi(1.5!); _Pi(0); _Pi(-1)
Print _Atan2(1, 1); _Hypot(1, 2); _Atan2(1#, 1#); _Hypot(1#, 2#); _Atan2(1%, 2%); _Hypot(3, 4)
Print _Atan2(0, 0); _Atan2(0, -1); Tan(_Pi / 2); Atn(1E+300); Sin(1E+30)
' Edge values and errors.
d = 7: d = Sqr(-1): Print "SQR(-1):"; d
d = 7: d = Sqr(0): Print "SQR(0):"; d
d = 7: d = Log(0): Print "LOG(0):"; d
d = 7: d = Log(-1): Print "LOG(-1):"; d
d = 7: d = Exp(1000): Print "EXP(1000):"; d
d = 7: d = Exp(709): Print "EXP(709):"; d
f = 7: f = Exp(100!): Print "EXP(100!):"; f
f = 7: f = Exp(88!): Print "EXP(88!):"; f
d = 7: d = Exp(-1000): Print "EXP(-1000):"; d
i = 7: i = CInt(32767.4): Print "CINT(32767.4):"; i
i = 7: i = CInt(32767.5): Print "CINT(32767.5):"; i
i = 7: i = CInt(-32768.5): Print "CINT(-32768.5):"; i
i = 7: i = CInt(-32768.6): Print "CINT(-32768.6):"; i
i = 7: i = CInt(40000): Print "CINT(40000):"; i
l = 70000: i = 7: i = CInt(l): Print "CINT(LONG 70000):"; i
q = 70000: i = 7: i = CInt(q): Print "CINT(_INTEGER64 70000):"; i
l = 7: l = CLng(2147483647.4#): Print "CLNG(2147483647.4):"; l
l = 7: l = CLng(2147483647.5#): Print "CLNG(2147483647.5):"; l
l = 7: l = CLng(-2147483648.5#): Print "CLNG(-2147483648.5):"; l
q = 3000000000: l = 7: l = CLng(q): Print "CLNG(_INTEGER64 3E9):"; l
f = 7: f = CSng(1D+300): Print "CSNG(1D300):"; f
x = 1D+300: x = x * x: d = 7: d = CDbl(x): Print "CDBL(1E600):"; d
q = 7: q = _Round(1E+30): Print "_ROUND(1E30):"; q
Print "after"; Sqr(-4) + 1; "next"
System

h:
Print "[handler"; Err; "]"
Resume Next
