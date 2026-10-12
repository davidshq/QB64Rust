$CONSOLE:ONLY
' The type before the names (spec language/arrays-and-types, "DIM AS type lists"): DIM AS type with scalars and
' static arrays in one list, DIM SHARED AS, STATIC AS and SHARED AS in a SUB; _UNSIGNED, STRING, STRING * n and
' user types; every name typed as `name AS type` types it (found by its plain name under DEFINT A-Z, and by the
' type's suffix); a list that starts with an array.
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
keep
PRINT
keep
PRINT
PRINT "8. the names with their suffix: a&, u1~%%, s1$:"; a&; u1~%%; " "; s1$
DIM AS INTEGER c(1 TO 2), d(2, 2), e
c(2) = 2.5: d(2, 2) = 8: e = 3.5
PRINT "9. DIM AS INTEGER c(1 TO 2), d(2, 2), e:"; c(2); d(2, 2); e; LEN(c(1)); LBOUND(c)
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
