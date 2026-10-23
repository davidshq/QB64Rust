' TEST: typed
$CONSOLE:ONLY
' STATIC arrays whose bounds are expressions of constants (m4-names-implicit-arrays task 5.1, design D4): each is
' allocated when the program starts with the value its bounds have (measured, verification\v24_x280,
' v55_c_static_bounds, v55_x73).
CONST k = 3
s
SYSTEM

SUB s
    CONST j = 2
    STATIC a(k + 1) AS LONG
    STATIC b(j * 3 - 1, 10 \ 3) AS LONG
    STATIC c(-1 TO &H4) AS LONG
    STATIC d(2 + 3) AS LONG
    STATIC e(LEN("abc")) AS LONG
    PRINT UBOUND(a); UBOUND(b, 2); LBOUND(c); UBOUND(d); UBOUND(e)
END SUB
