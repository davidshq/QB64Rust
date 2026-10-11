$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): array parameters: a static and a dynamic argument, LBOUND and
' UBOUND, REDIM _PRESERVE and ERASE in the procedure, an array passed on, a FUNCTION, CALL, a parameter with a
' suffix and with the default type, two dimensions, strings, fixed-length strings, a user type, a dynamic
' array that is not allocated, a DIM SHARED array and a local array as arguments, an index outside.
ON ERROR GOTO h
TYPE rec
    n AS LONG
    s AS STRING * 2
END TYPE
DIM x(1 TO 3) AS LONG
fill x()
PRINT "1. a static array filled in the SUB:"; x(1); x(2); x(3)
REDIM d(2) AS LONG
fill d()
PRINT "2. a dynamic array:"; d(0); d(1); d(2)
d(2) = 5
grow d()
PRINT "3. REDIM _PRESERVE a(10) in the SUB: UBOUND(d), d(2)"; UBOUND(d); d(2)
PRINT "4. the same SUB with the static array:";
grow x()
PRINT UBOUND(x); x(3)
wipe x()
PRINT "5. ERASE in the SUB, static: UBOUND(x), x(3)"; UBOUND(x); x(3)
wipe d()
PRINT "6. ERASE in the SUB, dynamic: UBOUND(d):";
PRINT UBOUND(d)
PRINT "7. the dynamic array, not allocated, passed to a SUB that prints UBOUND and to one that does REDIM:";
bounds d()
grow d()
PRINT UBOUND(d)
x(1) = 0
outer x()
PRINT "8. passed on to a second SUB:"; x(1)
x(1) = 1: x(2) = 2: x(3) = 3
PRINT "9. a FUNCTION:"; total(x())
CALL fill(x())
PRINT "10. CALL fill(x()):"; x(1); x(2); x(3)
DIM q(3) AS INTEGER, w(3)
sfx q()
df w()
PRINT "11. a parameter with a suffix and one with the default type:"; q(1); w(1)
DIM m(1 TO 2, 1 TO 3) AS LONG
two m()
PRINT "12. two dimensions:"; m(2, 3)
DIM s(2) AS STRING, f(2) AS STRING * 3, u(2) AS rec
s(1) = "ab": f(1) = "abc": u(1).n = 4
strs s(), f(), u()
PRINT "13. STRING, STRING * 3 and a user type: "; s(1); " "; f(1); u(1).n; u(1).s
DIM SHARED sh(3) AS LONG
fill sh()
PRINT "14. a DIM SHARED array as argument:"; sh(3)
PRINT "15. a local array as argument:";
inner
PRINT "16. an index outside, in the SUB:";
past x()
PRINT
PRINT "17. one array as two arguments:";
both x(), x()
PRINT x(1); x(2)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB fill (a() AS LONG)
    DIM i AS LONG
    FOR i = LBOUND(a) TO UBOUND(a): a(i) = i * 2: NEXT
END SUB
SUB grow (a() AS LONG)
    REDIM _PRESERVE a(10) AS LONG
END SUB
SUB wipe (a() AS LONG)
    ERASE a
END SUB
SUB bounds (a() AS LONG)
    PRINT UBOUND(a);
END SUB
SUB outer (a() AS LONG)
    second a()
END SUB
SUB second (b() AS LONG)
    b(1) = 4
END SUB
FUNCTION total (a() AS LONG)
    DIM i AS LONG, t AS LONG
    FOR i = LBOUND(a) TO UBOUND(a): t = t + a(i): NEXT
    total = t
END FUNCTION
SUB sfx (a%())
    a%(1) = 2.5
END SUB
SUB df (a())
    a(1) = 2.5
END SUB
SUB two (m() AS LONG)
    m(2, 3) = LBOUND(m, 2) * 10 + UBOUND(m, 2)
END SUB
SUB strs (s() AS STRING, f() AS STRING * 3, u() AS rec)
    s(1) = s(1) + "!"
    f(1) = "xy"
    u(1).n = u(1).n + 1: u(1).s = "uv"
END SUB
SUB inner
    DIM l(1 TO 2) AS LONG
    fill l()
    PRINT l(1); l(2)
END SUB
SUB past (a() AS LONG)
    a(99) = 1
    PRINT a(-5);
END SUB
SUB both (a() AS LONG, b() AS LONG)
    a(1) = 7
    b(2) = b(1) + 1
END SUB
