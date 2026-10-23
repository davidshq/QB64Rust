' TEST: cpp
$CONSOLE:ONLY
' STATIC a(n) in a SUB beside a shared main-module array of that name and type (m4-names-implicit-arrays task 5.2,
' design D4): it names the shared array, and its allocation when the program starts is a second allocation of it,
' written in maindata.txt after the arrays' own start code; a STATIC of another type is an array of its own
' (measured, verification\v24_x272, v55_d1 to d4).
DIM SHARED a(3) AS LONG
'$DYNAMIC
DIM SHARED b(3) AS LONG
a(1) = 7
s
SYSTEM

SUB s
    STATIC a(5) AS LONG
    STATIC b(4) AS LONG
    STATIC c(2) AS INTEGER
    PRINT a(1); UBOUND(a); UBOUND(b)
END SUB
