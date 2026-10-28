$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 16, _CAST of a DOUBLE and a SINGLE variable to each integer
' type, in range and out of it, NaN and infinity: the values a constant's _CAST is to give.
DIM x AS DOUBLE, z AS DOUBLE, f AS SINGLE
FOR i = 1 TO 8
    READ x
    IF i = 7 THEN x = z / z
    IF i = 8 THEN x = 1 / z
    PRINT i; ":"; _CAST(_BYTE, x); _CAST(_UNSIGNED _BYTE, x); _CAST(INTEGER, x); _CAST(_UNSIGNED INTEGER, x)
    PRINT "  "; _CAST(LONG, x); _CAST(_UNSIGNED LONG, x); _CAST(_INTEGER64, x); _CAST(_UNSIGNED _INTEGER64, x)
    PRINT "  "; _CAST(_OFFSET, x); _CAST(_UNSIGNED _OFFSET, x)
NEXT
DATA 1000.7,3E9,1E20,-1.5,-3E9,1E19,0,0
f = 3E9
PRINT "single:"; _CAST(LONG, f); _CAST(_BYTE, f); _CAST(_UNSIGNED LONG, f)
SYSTEM
