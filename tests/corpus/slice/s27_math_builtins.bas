$CONSOLE:ONLY
' Slice program (m2-core-builtins, D9): the math built-ins, their result types (shown by printing) and their edge
' values and errors (verification\v20_b_result_types, v20_f_math_edges). Each raising result is stored into a
' variable that held 7 first, so the placeholder shows. Errors are trapped: the handler prints ERR and resumes
' next. No PRINT comma.
ON ERROR GOTO h
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, f AS SINGLE, d AS DOUBLE, x AS _FLOAT
' ABS, INT, FIX keep the argument's type.
PRINT ABS(-7%) / 2; ABS(-7&) / 2; ABS(-2.5!); ABS(-2.5#); ABS(-7&&)
i = -32768: l = -2147483648: q = -9223372036854775807 - 1
PRINT ABS(i); ABS(l); ABS(q)
f = -2.5: d = -2.5: x = -2.5
PRINT INT(f); INT(d); INT(x); FIX(f); FIX(d); FIX(x); INT(-7%); FIX(-7&)
PRINT INT(-0.5); INT(-1.5); INT(0.5); INT(1.5); FIX(-0.5); FIX(-1.5); FIX(0.5); FIX(1.5)
f = 16777217: d = 9007199254740993#
PRINT INT(f) + 1 / 3; INT(d) / 3
' SGN.
f = 0: d = 0: d = -d
PRINT SGN(0); SGN(f); SGN(d); SGN(-5); SGN(5); SGN(-0.5); SGN(-7&&); SGN(2.5#)
' SIN … LOG, SQR, EXP: typed by the argument.
i = 2: l = 2: q = 2: f = 2: d = 2: x = 2
PRINT SQR(i); SQR(l); SQR(q); SQR(f); SQR(d); SQR(x); SQR(2%); SQR(2#)
PRINT SIN(i); SIN(l); SIN(q); SIN(f); SIN(d); SIN(x)
PRINT COS(i); COS(l); COS(f); COS(d); TAN(i); TAN(l); TAN(f); TAN(d)
PRINT ATN(i); ATN(l); ATN(f); ATN(d); LOG(i); LOG(l); LOG(q); LOG(f); LOG(d); LOG(x)
PRINT EXP(i); EXP(l); EXP(q); EXP(f); EXP(d); EXP(x)
PRINT SQR(i) * 3; SQR(l) * 3; EXP(f) / 7; SIN(q) / 3
' Conversions.
PRINT CINT(2.5); CINT(3.5); CINT(0.5); CINT(1.5); CINT(-0.5); CINT(-1.5); CINT(-1.2) / 3
PRINT CLNG(-1.2) / 3; CLNG(70000.5); CLNG(70001.5); CLNG(0.5); CLNG(2.5); CLNG(-2.5)
l = 16777217
PRINT CSNG(l) - 16777216; CSNG(l); CSNG(2# / 3); CDBL(2! / 3); CDBL(2 / 3); CDBL(l) / 3
PRINT _ROUND(0.5); _ROUND(1.5); _ROUND(2.5); _ROUND(-0.5); _ROUND(-1.5); _ROUND(7); _ROUND(2.5) / 3
PRINT _PI; _PI(2); _PI / 3; _PI(2%); _PI(1.5!); _PI(0); _PI(-1)
PRINT _ATAN2(1, 1); _HYPOT(1, 2); _ATAN2(1#, 1#); _HYPOT(1#, 2#); _ATAN2(1%, 2%); _HYPOT(3, 4)
PRINT _ATAN2(0, 0); _ATAN2(0, -1); TAN(_PI / 2); ATN(1E+300); SIN(1E+30)
' Edge values and errors.
d = 7: d = SQR(-1): PRINT "SQR(-1):"; d
d = 7: d = SQR(0): PRINT "SQR(0):"; d
d = 7: d = LOG(0): PRINT "LOG(0):"; d
d = 7: d = LOG(-1): PRINT "LOG(-1):"; d
d = 7: d = EXP(1000): PRINT "EXP(1000):"; d
d = 7: d = EXP(709): PRINT "EXP(709):"; d
f = 7: f = EXP(100!): PRINT "EXP(100!):"; f
f = 7: f = EXP(88!): PRINT "EXP(88!):"; f
d = 7: d = EXP(-1000): PRINT "EXP(-1000):"; d
i = 7: i = CINT(32767.4): PRINT "CINT(32767.4):"; i
i = 7: i = CINT(32767.5): PRINT "CINT(32767.5):"; i
i = 7: i = CINT(-32768.5): PRINT "CINT(-32768.5):"; i
i = 7: i = CINT(-32768.6): PRINT "CINT(-32768.6):"; i
i = 7: i = CINT(40000): PRINT "CINT(40000):"; i
l = 70000: i = 7: i = CINT(l): PRINT "CINT(LONG 70000):"; i
q = 70000: i = 7: i = CINT(q): PRINT "CINT(_INTEGER64 70000):"; i
l = 7: l = CLNG(2147483647.4#): PRINT "CLNG(2147483647.4):"; l
l = 7: l = CLNG(2147483647.5#): PRINT "CLNG(2147483647.5):"; l
l = 7: l = CLNG(-2147483648.5#): PRINT "CLNG(-2147483648.5):"; l
q = 3000000000: l = 7: l = CLNG(q): PRINT "CLNG(_INTEGER64 3E9):"; l
f = 7: f = CSNG(1D+300): PRINT "CSNG(1D300):"; f
x = 1D+300: x = x * x: d = 7: d = CDBL(x): PRINT "CDBL(1E600):"; d
q = 7: q = _ROUND(1E+30): PRINT "_ROUND(1E30):"; q
PRINT "after"; SQR(-4) + 1; "next"
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
