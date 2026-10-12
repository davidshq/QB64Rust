$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): the type of each built-in's result, by printing.
' A float type shows in the digits PRINT gives (SINGLE 7, DOUBLE 16, _FLOAT more). An integer type shows in HEX$
' of a negative result (INTEGER 4 digits, LONG 8, _INTEGER64 16), measured for variables first.
ON ERROR GOTO h
DIM a(5) AS LONG
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, f AS SINGLE, d AS DOUBLE, x AS _FLOAT
i = -1: l = -1: q = -1: f = -1: d = -1: x = -1
PRINT "HEX$ of -1 vars: "; HEX$(i); " "; HEX$(l); " "; HEX$(q)
PRINT "HEX$ of -1 exprs: "; HEX$(i + i); " "; HEX$(l + l); " "; HEX$(q + q)
' ABS: the minimum of each integer type (wraps if typed as the argument).
i = -32768: l = -2147483648: q = -9223372036854775807 - 1
PRINT "ABS min INTEGER:"; ABS(i); " HEX$: "; HEX$(ABS(i))
PRINT "ABS min LONG:"; ABS(l); " HEX$: "; HEX$(ABS(l))
PRINT "ABS min _INTEGER64:"; ABS(q); " HEX$: "; HEX$(ABS(q))
i = -1: l = -1: q = -1
PRINT "ABS(-1 vars) / 3:"; ABS(i) / 3; ABS(l) / 3; ABS(q) / 3
f = -2: d = -2: x = -2
PRINT "SQR(ABS(-2 floats)):"; SQR(ABS(f)); SQR(ABS(d)); SQR(ABS(x))
' INT and FIX of integers and floats.
PRINT "INT(-1 int vars) HEX$: "; HEX$(INT(i)); " "; HEX$(INT(l)); " "; HEX$(INT(q))
PRINT "FIX(-1 int vars) HEX$: "; HEX$(FIX(i)); " "; HEX$(FIX(l)); " "; HEX$(FIX(q))
f = -2.5: d = -2.5: x = -2.5
PRINT "INT(-2.5):"; INT(f); INT(d); INT(x); " FIX(-2.5):"; FIX(f); FIX(d); FIX(x)
f = 16777217: d = 9007199254740993#
PRINT "INT(f)+1/3:"; INT(f) + 1 / 3; " INT(d)/3:"; INT(d) / 3
' SGN.
PRINT "SGN HEX$: "; HEX$(SGN(i)); " "; HEX$(SGN(l)); " "; HEX$(SGN(q)); " "; HEX$(SGN(f)); " "; HEX$(SGN(d))
' The float functions by argument type.
i = 2: l = 2: q = 2: f = 2: d = 2: x = 2
PRINT "SQR i l q:"; SQR(i); SQR(l); SQR(q)
PRINT "SQR f d x:"; SQR(f); SQR(d); SQR(x)
PRINT "SIN i l q f d x:"; SIN(i); SIN(l); SIN(q); SIN(f); SIN(d); SIN(x)
PRINT "COS i l f d:"; COS(i); COS(l); COS(f); COS(d)
PRINT "TAN i l f d:"; TAN(i); TAN(l); TAN(f); TAN(d)
PRINT "ATN i l f d:"; ATN(i); ATN(l); ATN(f); ATN(d)
PRINT "LOG i l q f d x:"; LOG(i); LOG(l); LOG(q); LOG(f); LOG(d); LOG(x)
PRINT "EXP i l q f d x:"; EXP(i); EXP(l); EXP(q); EXP(f); EXP(d); EXP(x)
DIM b AS _BYTE
b = 2
PRINT "SQR EXP byte:"; SQR(b); EXP(b)
' Conversions.
PRINT "CINT(2.5) CINT(3.5) HEX$(CINT(-1.2)):"; CINT(2.5); CINT(3.5); " "; HEX$(CINT(-1.2))
PRINT "CLNG HEX$(CLNG(-1.2)): "; HEX$(CLNG(-1.2)); " CLNG(70000.5):"; CLNG(70000.5); CLNG(70001.5)
l = 16777217
PRINT "CSNG(l) - 16777216:"; CSNG(l) - 16777216; " CSNG(l):"; CSNG(l)
PRINT "CSNG(2#/3):"; CSNG(2# / 3); " CDBL(2!/3):"; CDBL(2! / 3); " CDBL(2/3):"; CDBL(2 / 3)
PRINT "CDBL(l) / 3:"; CDBL(l) / 3
PRINT "_ROUND(2.5) (3.5) (-2.5):"; _ROUND(2.5); _ROUND(3.5); _ROUND(-2.5); " HEX$: "; HEX$(_ROUND(-1.2))
PRINT "_PI _PI(2) _PI/3:"; _PI; _PI(2); _PI / 3
PRINT "_ATAN2(1, 1) _HYPOT(1, 2):"; _ATAN2(1, 1); _HYPOT(1, 2)
PRINT "_ATAN2(1#, 1#) _HYPOT(1#, 2#):"; _ATAN2(1#, 1#); _HYPOT(1#, 2#)
' String functions with numeric results.
PRINT "ASC HEX$: "; HEX$(-ASC("a")); " LEN HEX$: "; HEX$(-LEN("ab")); " INSTR HEX$: "; HEX$(-INSTR("ab", "b"))
PRINT "VAL(1/3 text):"; VAL("0.33333333333333333333"); " VAL/3:"; VAL("1") / 3
PRINT "VAL SINGLE/3:"; VAL("1", SINGLE) / 3; " VAL DOUBLE/3:"; VAL("1", DOUBLE) / 3
PRINT "VAL INTEGER HEX$: "; HEX$(VAL("-1", INTEGER)); " LONG: "; HEX$(VAL("-1", LONG)); " _INTEGER64: "; HEX$(VAL("-1", _INTEGER64))
PRINT "VAL(16777217, SINGLE) = 16777216&&:"; VAL("16777217", SINGLE) = 16777216&&
PRINT "VAL(40000, INTEGER):"; VAL("40000", INTEGER); " VAL(3000000000, LONG):"; VAL("3000000000", LONG)
PRINT "LBOUND HEX$: "; HEX$(-LBOUND(a)); " UBOUND/3:"; UBOUND(a) / 3
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
