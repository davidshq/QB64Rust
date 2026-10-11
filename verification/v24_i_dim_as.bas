$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1, for task 3.3): the type before the names, in DIM, DIM SHARED, STATIC
' and SHARED: scalars and arrays in one list, _UNSIGNED and STRING * n types, a user type, the plain name typed
' as by DIM name AS type, a default type in effect, and the same list declared again name by name.
TYPE rec
    n AS LONG
END TYPE
DEFINT A-Z
DIM AS LONG a, b(3)
a = 100000: b(3) = a
PRINT "1. DIM AS LONG a, b(3):"; a; b(3); LEN(a); LEN(b(1)); UBOUND(b)
DIM AS _UNSIGNED _BYTE u1, u2
u1 = -1: u2 = 300
PRINT "2. DIM AS _UNSIGNED _BYTE u1, u2:"; u1; u2
DIM AS STRING * 4 f1, f2(2)
f1 = "abcdef": f2(1) = "x"
PRINT "3. DIM AS STRING * 4 f1, f2(2): ["; f1; "] ["; f2(1); "]"; LEN(f2(2))
DIM AS STRING s1, s2
s1 = "one": s2 = s1 + "!"
PRINT "4. DIM AS STRING s1, s2: "; s1; " "; s2
DIM AS rec r1, r2(2)
r1.n = 5: r2(2).n = 6
PRINT "5. DIM AS rec r1, r2(2):"; r1.n; r2(2).n
DIM SHARED AS DOUBLE g1, g2(1 TO 2)
g1 = 1# / 3: g2(2) = 2.5
PRINT "6. DIM SHARED AS DOUBLE g1, g2(1 TO 2), in a SUB:";
show
keep: PRINT
keep: PRINT
PRINT "8. the names with their suffix: a&, u1~%%, s1$:"; a&; u1~%%; " "; s1$
DIM AS INTEGER one
one = 2.5
PRINT "9. one name:"; one
SYSTEM
SUB show
    PRINT g1; g2(2)
END SUB
SUB keep
    STATIC AS LONG k1, k2
    SHARED AS LONG a
    k1 = k1 + 1: k2 = k2 + a
    PRINT "7. STATIC AS LONG k1, k2 and SHARED AS LONG a:"; k1; k2;
END SUB
