$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON and COMMON SHARED: scalars, an array named with () and
' given bounds by DIM and by REDIM, several names in one statement, a fixed-length string and a user type;
' what a SUB sees.
TYPE rec
    n AS LONG
END TYPE
COMMON SHARED total AS LONG
COMMON v AS LONG
COMMON SHARED cs() AS LONG
COMMON SHARED cr() AS LONG
COMMON pa() AS LONG
COMMON b1 AS LONG, b2 AS STRING
COMMON SHARED b3 AS LONG, fx AS STRING * 3, ur AS rec
ON ERROR GOTO h
total = 5: v = 5
DIM cs(3) AS LONG: cs(2) = 6
REDIM cr(4) AS LONG: cr(2) = 7
DIM pa(3) AS LONG: pa(1) = 8
b1 = 1: b2 = "x": b3 = 3: fx = "abcdef": ur.n = 9
show
PRINT "main: b1, b2, pa(1), UBOUND(cr), cr(6)"; b1; b2; pa(1); UBOUND(cr); cr(6)
PRINT "main: REDIM cs(9) of the array given bounds by DIM:";
REDIM cs(9) AS LONG
PRINT UBOUND(cs)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB show
    PRINT "SUB: total, v, cs(2), cr(2), b3, fx, ur.n"; total; v; cs(2); cr(2); b3; fx; ur.n
    REDIM cr(6) AS LONG
    cr(6) = 2
    PRINT "SUB: pa(1) of a COMMON array that is not shared:";
    PRINT pa(1)
END SUB
