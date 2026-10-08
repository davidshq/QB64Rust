$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): math built-ins at their edge values, and the errors they raise. Each result
' is stored into a variable that held 7 first, so a raising call shows the value it leaves.
ON ERROR GOTO h
DIM d AS DOUBLE, f AS SINGLE, i AS INTEGER, l AS LONG, q AS _INTEGER64
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
PRINT "CINT halves:"; CINT(0.5); CINT(1.5); CINT(-0.5); CINT(-1.5)
l = 7: l = CLNG(2147483647.4#): PRINT "CLNG(2147483647.4):"; l
l = 7: l = CLNG(2147483647.5#): PRINT "CLNG(2147483647.5):"; l
l = 7: l = CLNG(-2147483648.5#): PRINT "CLNG(-2147483648.5):"; l
l = 7: l = CLNG(-2147483648.6#): PRINT "CLNG(-2147483648.6):"; l
q = 3000000000: l = 7: l = CLNG(q): PRINT "CLNG(_INTEGER64 3E9):"; l
PRINT "CLNG halves:"; CLNG(0.5); CLNG(1.5); CLNG(2.5); CLNG(-2.5)
PRINT "INT halves:"; INT(-0.5); INT(-1.5); INT(0.5); INT(1.5); " FIX halves:"; FIX(-0.5); FIX(-1.5); FIX(0.5); FIX(1.5)
i = -32768: l = -2147483648: q = -9223372036854775807 - 1
PRINT "ABS of minima:"; ABS(i); ABS(l); ABS(q)
f = 0: d = 0: d = -d
PRINT "SGN 0 -0 -5 5:"; SGN(0); SGN(f); SGN(d); SGN(-5); SGN(5); SGN(-0.5)
PRINT "_ROUND halves:"; _ROUND(0.5); _ROUND(1.5); _ROUND(2.5); _ROUND(-0.5); _ROUND(-1.5); _ROUND(7)
q = 7: q = _ROUND(1E+30): PRINT "_ROUND(1E30):"; q
f = 7: f = CSNG(1D+300): PRINT "CSNG(1D300):"; f
DIM x AS _FLOAT: x = 1D+300: x = x * x: d = 7: d = CDBL(x): PRINT "CDBL(1E600 as _FLOAT):"; d
PRINT "TAN(_PI / 2):"; TAN(_PI / 2); " ATN(1E300):"; ATN(1E+300); " SIN(1E30):"; SIN(1E+30)
PRINT "_ATAN2(0, 0):"; _ATAN2(0, 0); " _ATAN2(0, -1):"; _ATAN2(0, -1); " _HYPOT(3, 4):"; _HYPOT(3, 4)
PRINT "_PI(0):"; _PI(0); " _PI(-1):"; _PI(-1)
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
