' TEST: typed
$CONSOLE:ONLY
' The math built-ins by rule and argument type (m2-core-builtins task 5.1): ABS cast to the argument's type and
' typed by it; INT, FIX as they are; SIN ... LOG believed SINGLE, DOUBLE or _FLOAT by the argument, held as C++
' computes them (std:: by overloading, func_sqr and func_log a double); EXP its own typing; the conversions held as
' their libqb entries return (CSNG of an integer not narrowed); _ATAN2, _HYPOT with _FLOAT slots passed as they
' are; _PI with a DOUBLE slot, bare and with an argument; SGN with an any-numeric slot, LONG
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, f AS SINGLE, d AS DOUBLE, x AS _FLOAT
PRINT ABS(i); ABS(-7%); ABS(l); ABS(q); ABS(f); ABS(2.5); ABS(x)
PRINT INT(i); INT(f); INT(2.5); FIX(d); FIX(x); FIX(q)
PRINT SIN(i); SIN(l); SIN(q); SIN(f); SIN(d); SIN(x); COS(2.5); TAN(d); ATN(l)
PRINT SQR(i); SQR(l); SQR(f); SQR(x); LOG(i); LOG(q); LOG(d)
PRINT EXP(i); EXP(l); EXP(f); EXP(d); EXP(2.5)
PRINT CINT(d); CINT(x); CINT(l); CINT(q); CINT(i); CLNG(f); CLNG(x); CLNG(q); CLNG(i)
PRINT CSNG(l); CSNG(d); CSNG(f); CSNG(2.5); CDBL(i); CDBL(f); CDBL(x); _ROUND(d); _ROUND(x); _ROUND(l)
PRINT _ATAN2(i, l); _ATAN2(f, f); _HYPOT(d, x); _PI; _PI(2); _PI(f); SGN(i); SGN(d)
