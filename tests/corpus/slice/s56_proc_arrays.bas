$CONSOLE:ONLY
' Arrays in procedures (spec language/arrays-and-types, "Arrays in procedures"): DIM and REDIM in a SUB or
' FUNCTION make a dynamic array that is new on every call; STATIC a(n) keeps its elements; STATIC a() lists a
' dynamic array that a DIM, a REDIM or a REDIM _PRESERVE below it allocates; SHARED a() names the main module's
' array; a shared main array is seen below its declaration in the file only; a procedure with STATIC after its
' header keeps its variables and arrays down to its first STATIC statement. Error cases go to a handler that
' prints ERR and resumes next.
SUB above
    PRINT "13. a SUB above the DIM SHARED: sa(1)";
    PRINT sa(1)
END SUB
ON ERROR GOTO h
DIM v AS LONG
DIM g(3) AS LONG, hd(3), ns(3) AS LONG, bs(3) AS STRING
REDIM d(3) AS LONG
DIM SHARED sa(3) AS LONG
REDIM SHARED sr(3) AS LONG
PRINT "1. DIM t(5), new on each call:"
fresh 5
fresh 5
PRINT "2. one DIM run twice in a call:";
twice
PRINT "3. STATIC c(3):";
keep
keep
keep
PRINT
PRINT "4. STATIC l() then DIM l(3):";
listdim
listdim
PRINT
PRINT "5. STATIC r() then REDIM r(3):";
listredim
listredim
PRINT
PRINT "6. STATIC k() then REDIM _PRESERVE k(2, 3):";
listpres
listpres
PRINT
PRINT "7. SUB ... STATIC, DIM a(2):";
substatic
substatic
PRINT
PRINT "8. SUB ... STATIC, DIM b(n):";
substaticdyn 2
substaticdyn 2
PRINT
PRINT "9. REDIM in a SUB, twice in a call:";
local 3
local 4
PRINT
g(2) = 8: hd(2) = 1.5: d(2) = 4: ns(1) = 5: bs(1) = "main"
show
PRINT "11. main after it: g(2), hd(2), UBOUND(d), d(5), ns(1), ns(2), bs(1): "; g(2); hd(2); UBOUND(d); d(5); ns(1); ns(2); bs(1)
sa(1) = 1: sr(1) = 2
PRINT "12. a SUB below DIM SHARED and REDIM SHARED:";
below
PRINT sa(1); sr(1); UBOUND(sr)
above
PRINT
PRINT "14. EXIT SUB with a local array, called twice:";
early 1
early 1
PRINT
PRINT "15. an error inside, then the SUB goes on:";
fails
fails
PRINT
PRINT "16. a recursive FUNCTION, each level with DIM a(n):";
v = down(3)
PRINT " result"; v
PRINT "17. a FUNCTION with a local string array: "; words("a"); words("b")
PRINT "18. used without DIM, then DIM of it:";
later
later
PRINT
PRINT "19. STATIC y(), used with no DIM:";
listed
listed
PRINT
PRINT "20. DIM of a shared main array in a SUB:";
again
PRINT sa(1); UBOUND(sa); UBOUND(sr)
PRINT "21. SUB ... STATIC: what is kept above and below its STATIC statement:"
scope
scope
scope
PRINT "22. FUNCTION ... STATIC:";
v = count(2)
v = count(2)
PRINT v
PRINT "23. a used array in a SUB ... STATIC:";
auto
auto
PRINT
PRINT "24. ERASE of a local, a STATIC and a listed array:";
wipe
wipe
PRINT
PRINT "25. DIM a() and REDIM b() in a SUB are scalars:";
empty
empty
PRINT
PRINT "26. two dimensions, a FOR over a local array:";
grid 3

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
    STATIC c(3) AS LONG, w(1 TO 2) AS STRING
    c(1) = c(1) + 1: w(1) = w(1) + "x": PRINT c(1); w(1);
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
    REDIM _PRESERVE k(2, 3) AS LONG
    k(2, 3) = k(2, 3) + 1: PRINT k(2, 3);
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
SUB show
    SHARED g() AS LONG
    SHARED hd(), bs() AS STRING
    SHARED d() AS LONG
    PRINT "10. SHARED g(), hd(), d(), bs(): "; g(2); hd(2); d(2); bs(1)
    g(2) = 9: hd(2) = 2.5: bs(1) = "sub"
    REDIM d(5) AS LONG
    d(5) = 6
    PRINT "    ns(1) of a main array that is not shared:";
    PRINT ns(1)
    DIM ns(8) AS LONG
    ns(2) = 3
    PRINT "    after a local DIM ns(8): UBOUND, ns(1), ns(2)"; UBOUND(ns); ns(1); ns(2)
END SUB
SUB below
    PRINT sa(1); sr(1);
    sa(1) = 3
    REDIM sr(6) AS LONG
    sr(1) = 4
END SUB
SUB early (n AS LONG)
    DIM a(3) AS LONG, s(3) AS STRING
    a(1) = a(1) + 1: s(1) = s(1) + "e"
    PRINT a(1); s(1);
    IF n = 1 THEN EXIT SUB
    PRINT "not reached"
END SUB
SUB fails
    DIM a(3) AS LONG
    a(1) = a(1) + 1
    a(9) = 1
    ERROR 5
    PRINT a(1);
END SUB
FUNCTION down (n AS LONG)
    DIM a(n) AS LONG
    a(n) = n
    IF n > 1 THEN down = down(n - 1) + a(n) ELSE down = a(n)
    PRINT UBOUND(a); a(n);
END FUNCTION
FUNCTION words$ (s AS STRING)
    DIM w(2) AS STRING
    w(1) = w(1) + s + s
    words$ = "[" + w(1) + w(2) + "]"
END FUNCTION
SUB later
    a(1) = 5
    DIM a(3)
    a(1) = a(1) + 1
    PRINT a(1); UBOUND(a);
END SUB
SUB listed
    STATIC y() AS DOUBLE
    y(2) = y(2) + 1
    PRINT LEN(y(2));
END SUB
SUB again
    DIM sa(5) AS LONG
    DIM sr(9) AS LONG
END SUB
SUB scope STATIC
    DIM d AS LONG
    DIM a(2) AS LONG
    n = n + 1: d = d + 2: s$ = s$ + "a": a(1) = a(1) + 3
    PRINT "    above:"; n; d; s$; a(1);
    STATIC x(2) AS LONG
    DIM e AS LONG
    DIM w(2) AS LONG
    m = m + 1: e = e + 2: t$ = t$ + "b": x(1) = x(1) + 4: w(1) = w(1) + 5: n = n + 10
    PRINT " below:"; m; e; t$; x(1); w(1); n
END SUB
FUNCTION count (a AS LONG) STATIC
    c = c + a
    count = c
    PRINT c;
END FUNCTION
SUB auto STATIC
    u(2) = u(2) + 1
    PRINT u(2); UBOUND(u);
END SUB
SUB wipe
    DIM a(2) AS LONG
    STATIC c(2) AS LONG, l() AS LONG
    REDIM _PRESERVE l(2) AS LONG
    a(1) = a(1) + 1: c(1) = c(1) + 1: l(1) = l(1) + 1
    PRINT a(1); c(1); l(1);
    ERASE a, c, l
    PRINT c(1);
    PRINT a(1);
    PRINT l(1);
END SUB
SUB empty
    DIM a() AS LONG
    REDIM b() AS LONG
    a = a + 1: b = b + 2
    PRINT a; b;
END SUB
SUB grid (n AS LONG)
    DIM m(1 TO n, 1 TO n) AS LONG, i AS LONG, j AS LONG, total AS LONG
    FOR i = 1 TO n
        FOR j = 1 TO n
            m(i, j) = i * j
        NEXT
    NEXT
    FOR i = 1 TO n
        total = total + m(i, n)
    NEXT
    PRINT total; LBOUND(m, 2); UBOUND(m, 2)
END SUB
