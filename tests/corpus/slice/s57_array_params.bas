$CONSOLE:ONLY
' Array parameters (spec language/procedures, "Array parameters"): a parameter written name() takes a whole array
' by reference, static or dynamic, allocated or not, the procedure's own parameter passed on too. The procedure
' reads, stores, DIMs, REDIMs and ERASEs the caller's array, with the errors that array's kind gives. Signedness
' is not compared. The number of dimensions comes from the indexes in the procedure and from the arrays passed,
' above or below it. Error cases go to a handler that prints ERR and resumes next.
TYPE pt
    x AS LONG
    y AS INTEGER
END TYPE
SUB above (m() AS LONG)
    PRINT "9. a SUB above the call, UBOUND(m, 2):"; UBOUND(m, 2)
END SUB
ON ERROR GOTO h
DIM i AS LONG
DIM x(1 TO 3) AS LONG, u(3) AS _UNSIGNED LONG, y(2, 4) AS LONG, e(3) AS LONG
DIM s(1 TO 2) AS STRING, f(2) AS STRING * 3, p(2) AS pt, b(3) AS _BIT * 4, d2(2 TO 5, -1 TO 1) AS DOUBLE
DIM SHARED sh(3) AS LONG
REDIM d(2) AS LONG, r(3) AS LONG, never(0) AS LONG, gone(0) AS LONG
COMMON c() AS LONG
ERASE never, gone
fill x()
PRINT "1. elements changed in the SUB:"; x(1); x(2); x(3)
d(2) = 5
grow d()
PRINT "2. REDIM _PRESERVE in the SUB:"; UBOUND(d); d(2)
outer x()
PRINT "3. passed on:"; x(1)
signed u()
PRINT "4. an _UNSIGNED LONG array as a LONG parameter:"; u(1)
PRINT "5. REDIM of a static argument:";
grow x()
PRINT UBOUND(x)
e(1) = 7: r(1) = 7
wipe e()
wipe r()
PRINT "6. ERASE in the SUB, static:"; UBOUND(e); e(1); " dynamic:";
PRINT UBOUND(r);
PRINT
PRINT "7. a SUB below the call, UBOUND(m, 2):";
below y()
PRINT
chain1 y()
above y()
PRINT "10. DIM of the parameter:";
dimmed x()
dimmed never()
PRINT UBOUND(x); UBOUND(never)
tell s(), f(), p(), b()
PRINT "11. strings, fixed strings, a TYPE, _BIT: "; s(1); s(2); "["; f(1); "]"; p(2).x; p(2).y; b(1)
PRINT "12. a FUNCTION:"; total#(d2()); d2(5, 1); pick(x(), 2)
CALL fill(x())
PRINT "13. CALL fill(x()):"; x(3)
fill fresh&()
PRINT "14. an array never declared:"; fresh&(1); UBOUND(fresh&)
make c()
PRINT "15. a COMMON array given its bounds in the SUB:"; UBOUND(c, 1); UBOUND(c, 2); c(1, 2)
PRINT "16. a STATIC k() array passed on:";
keeper
keeper
PRINT
countdown x(), 3
PRINT "17. a recursive SUB:"; x(1); x(2); x(3)
REDIM d(1 TO 4) AS LONG
pair d(), d()
PRINT "18. one array as two arguments:"; d(1); UBOUND(d)
PRINT "19. an index outside, in the SUB:";
outside x()
PRINT x(1)
sh(1) = 4
PRINT "20. a shared and a local array passed from a SUB:";
inner
PRINT sh(1)
PRINT "21. a parameter that is not allocated:";
PRINT bounds&(gone())
REDIM gone(6) AS LONG
PRINT "22. and allocated:"; bounds&(gone())
FOR i = 1 TO 2
    unsign x()
NEXT
PRINT "23. a LONG array as an _UNSIGNED LONG parameter:"; x(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB fill (a() AS LONG)
    DIM i AS LONG
    FOR i = LBOUND(a) TO UBOUND(a)
        a(i) = i * 2
    NEXT
END SUB
SUB grow (a() AS LONG)
    REDIM _PRESERVE a(10) AS LONG
END SUB
SUB outer (a() AS LONG)
    middle a()
END SUB
SUB middle (b() AS LONG)
    b(1) = 4
END SUB
SUB signed (a() AS LONG)
    a(1) = -1
END SUB
SUB unsign (a() AS _UNSIGNED LONG)
    a(1) = a(1) + 4294967295
END SUB
SUB wipe (a() AS LONG)
    ERASE a
END SUB
SUB below (m() AS LONG)
    PRINT UBOUND(m, 2);
END SUB
SUB chain1 (a() AS LONG)
    chain2 a()
END SUB
SUB chain2 (b() AS LONG)
    chain3 b()
END SUB
SUB chain3 (c() AS LONG)
    PRINT "8. down a chain of three:"; UBOUND(c, 1); UBOUND(c, 2)
END SUB
SUB dimmed (a() AS LONG)
    DIM a(5) AS LONG
END SUB
SUB tell (a() AS STRING, b() AS STRING * 3, c() AS pt, e() AS _BIT * 4)
    a(1) = "one": a(2) = a(1) + "+"
    b(1) = "abcdef"
    c(2).x = 70000: c(2).y = LEN(b(1))
    e(1) = 5
END SUB
FUNCTION total# (m() AS DOUBLE)
    DIM i AS LONG, j AS LONG, t AS DOUBLE
    FOR i = LBOUND(m, 1) TO UBOUND(m, 1)
        FOR j = LBOUND(m, 2) TO UBOUND(m, 2)
            m(i, j) = i * j
            t = t + m(i, j)
        NEXT
    NEXT
    total# = t
END FUNCTION
FUNCTION pick (a() AS LONG, i AS LONG)
    pick = a(i)
END FUNCTION
SUB make (a() AS LONG)
    REDIM a(3, 4) AS LONG
    a(1, 2) = 7
END SUB
SUB keeper
    STATIC k() AS LONG
    grow3 k()
    k(1) = k(1) + 1
    PRINT UBOUND(k); k(1);
END SUB
SUB grow3 (a() AS LONG)
    REDIM _PRESERVE a(3) AS LONG
END SUB
SUB countdown (a() AS LONG, n AS LONG)
    IF n = 0 THEN EXIT SUB
    a(n) = n * 10
    countdown a(), n - 1
END SUB
SUB pair (p() AS LONG, q() AS LONG)
    p(1) = 1: q(1) = q(1) + 1
    REDIM q(1 TO 2) AS LONG
END SUB
SUB outside (a() AS LONG)
    a(99) = 1
    a(1) = 3
END SUB
SUB inner
    DIM mine(2) AS LONG
    fill sh()
    fill mine()
    PRINT mine(2);
END SUB
FUNCTION bounds& (a() AS LONG)
    bounds& = UBOUND(a)
END FUNCTION
