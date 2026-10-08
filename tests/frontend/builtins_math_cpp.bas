' TEST: cpp
$CONSOLE:ONLY
' The math built-ins as emitted (m2-core-builtins task 5.1): func_abs and func_sgn with the argument cast, INT and
' FIX as std::floor, func_fix_double/_float or the value, the float functions uncast, func_exp_single/_float, the
' conversion entries chosen by the argument's believed type, _PI's passed mask
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, f AS SINGLE, d AS DOUBLE, x AS _FLOAT
PRINT ABS(-32768%); ABS(l); SGN(i); INT(i); INT(f); FIX(d); FIX(x)
PRINT SIN(i); SQR(l); LOG(x); EXP(i); EXP(d); _ATAN2(i, l); _HYPOT(d, x); _PI; _PI(2)
PRINT CINT(d); CINT(x); CINT(l); CINT(q); CINT(i); CLNG(f); CLNG(x); CLNG(q); CLNG(i)
PRINT CSNG(l); CSNG(d); CSNG(x); CSNG(f); CDBL(i); CDBL(f); CDBL(x); _ROUND(d); _ROUND(x); _ROUND(l)
