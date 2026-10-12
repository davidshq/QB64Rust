$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): arrays declared in procedures: DIM new on each call with
' constant and variable bounds and for strings, one DIM run twice, STATIC c(3), STATIC l() then DIM, then
' REDIM, then REDIM _PRESERVE, SUB ... STATIC with a constant and a variable bound, REDIM in a SUB.
ON ERROR GOTO h
PRINT "1. DIM t(5), new on each call:"
fresh 5: fresh 5
PRINT "2. one DIM run twice in a call:";
twice
PRINT "3. STATIC c(3):";
keep: keep: keep
PRINT
PRINT "4. STATIC l() then DIM l(3):";
listdim: listdim
PRINT
PRINT "5. STATIC r() then REDIM r(3):";
listredim: listredim
PRINT
PRINT "6. STATIC k() then REDIM _PRESERVE k(3):";
listpres: listpres
PRINT
PRINT "7. SUB ... STATIC, DIM a(2):";
substatic: substatic
PRINT
PRINT "8. SUB ... STATIC, DIM b(n):";
substaticdyn 2: substaticdyn 2
PRINT
PRINT "9. REDIM in a SUB, twice in a call:";
local 3: local 4
PRINT

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB fresh (n AS LONG)
    DIM t(5) AS LONG, v(n) AS LONG, s(2) AS STRING, f(2) AS STRING * 2
    PRINT "   t(1), v(n), s(1), ASC(f(1)), UBOUND(v):"; t(1); v(n); "["; s(1); "]"; ASC(f(1)); UBOUND(v)
    t(1) = 7: v(n) = 8: s(1) = "old": f(1) = "zz"
END SUB
SUB twice
    DIM i AS LONG
    FOR i = 1 TO 2
        DIM d(3) AS LONG
        d(1) = d(1) + 1
    NEXT
    PRINT d(1)
END SUB
SUB keep
    STATIC c(3) AS LONG
    c(1) = c(1) + 1: PRINT c(1);
END SUB
SUB listdim
    STATIC l() AS LONG
    DIM l(3) AS LONG
    l(1) = l(1) + 1: PRINT l(1);
END SUB
SUB listredim
    STATIC r() AS LONG
    REDIM r(3) AS LONG
    r(1) = r(1) + 1: PRINT r(1);
END SUB
SUB listpres
    STATIC k() AS LONG
    REDIM _PRESERVE k(3) AS LONG
    k(1) = k(1) + 1: PRINT k(1);
END SUB
SUB substatic STATIC
    DIM a(2) AS LONG
    a(1) = a(1) + 1: PRINT a(1);
END SUB
SUB substaticdyn (n AS LONG) STATIC
    DIM b(n) AS LONG
    b(1) = b(1) + 1: PRINT b(1);
END SUB
SUB local (n AS LONG)
    REDIM q(n) AS LONG
    q(1) = q(1) + 1
    REDIM q(n + 1) AS LONG
    PRINT UBOUND(q); q(1);
END SUB
