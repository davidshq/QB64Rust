$CONSOLE:ONLY
' REDIM (spec language/arrays-and-types, "REDIM"): the first REDIM as the declaration; new bounds and new elements
' each time; the type kept without AS, of an array made by REDIM and by DIM; the same array by its suffix and
' another one by another suffix; REDIM of a static array (error 10); several arrays in one statement and the type
' before the names; a dimension with no element and a bound that raises; REDIM of a name without bounds (a
' scalar); a scalar of the array's name; strings, fixed-length strings, a user type; REDIM SHARED, and REDIM of a
' shared array in a SUB. Error cases go to a handler that prints ERR and resumes next.
TYPE rec
    n AS LONG
    t AS STRING * 2
END TYPE
ON ERROR GOTO h
DIM i AS LONG, n AS LONG, m AS LONG
n = 3
REDIM r(5) AS LONG
r(2) = 7
REDIM r(1 TO 20) AS LONG
PRINT "1. REDIM r(5), r(2) = 7, REDIM r(1 TO 20): LBOUND, UBOUND, r(2)"; LBOUND(r); UBOUND(r); r(2)
REDIM x(5) AS LONG
REDIM x(10)
x(1) = 100000
PRINT "2. REDIM x(5) AS LONG, REDIM x(10), x(1) = 100000:"; x(1); UBOUND(x)
DIM s(5) AS LONG
s(1) = 3
PRINT "3. REDIM of a static array:";
REDIM s(9) AS LONG
PRINT UBOUND(s); s(1)
REDIM x&(7)
REDIM x!(2)
x!(1) = 1.5
PRINT "4. REDIM x&(7) and REDIM x!(2): UBOUND(x), UBOUND(x&), UBOUND(x!), x!(1)"; UBOUND(x); UBOUND(x&); UBOUND(x!); x!(1)
DIM dm(n) AS LONG
REDIM dm(7)
dm(1) = 100000
PRINT "5. DIM dm(n) AS LONG, REDIM dm(7), dm(1) = 100000:"; UBOUND(dm); dm(1)
REDIM p1(2) AS LONG, p2(n, 1 TO 2) AS STRING, p3%(4)
p2(3, 2) = "last"
PRINT "6. three arrays in one statement:"; UBOUND(p1); UBOUND(p2, 1); UBOUND(p2, 2); UBOUND(p3%); " "; p2(3, 2); LEN(p2(0, 1))
REDIM AS DOUBLE q1(2), q2(1 TO n)
q2(n) = .5
PRINT "7. the type before the names:"; UBOUND(q1); LBOUND(q2); q2(n); LEN(q1(0))
m = -1
r(1) = 4
PRINT "8. no element, REDIM r(m) with m = -1:";
REDIM r(m) AS LONG
PRINT LBOUND(r); UBOUND(r);
PRINT r(1)
PRINT
PRINT "   and REDIM r(2) after it:";
REDIM r(2) AS LONG
PRINT UBOUND(r); r(1)
r(1) = 4
PRINT "9. a bound that raises, REDIM r(ASC of an empty string TO 5):";
REDIM r(ASC("") TO 5) AS LONG
PRINT LBOUND(r); UBOUND(r); r(1)
REDIM e1 AS LONG
REDIM e2() AS LONG
e1 = 100000.5: e2 = 2.5
PRINT "10. REDIM e1 AS LONG and REDIM e2() AS LONG are scalars:"; e1; e2; LEN(e1)
sc& = 5
REDIM sc&(3)
sc&(1) = 2
REDIM sc&(4)
PRINT "11. a scalar of the array's name:"; sc&; sc&(1); UBOUND(sc&)
REDIM st(n) AS STRING, fx(n) AS STRING * 3, ud(n) AS rec
st(1) = "one": fx(1) = "abcdef": ud(2).n = 7: ud(2).t = "xy"
PRINT "12. STRING, STRING * 3, a user type: "; st(1); "["; st(2); "] "; fx(1); ud(2).n; ud(2).t
REDIM st(n + 1) AS STRING, fx(n + 1) AS STRING * 3, ud(n + 1) AS rec
PRINT "    after REDIM: ["; st(1); "]"; LEN(fx(1)); ASC(fx(1)); ud(2).n; ASC(ud(2).t); UBOUND(st)
FOR i = 1 TO 3
    REDIM lp(i * 2) AS LONG
    lp(i) = lp(i) + i
NEXT
PRINT "13. one REDIM in a loop that runs three times:"; UBOUND(lp); lp(3); lp(2)
REDIM SHARED g(2) AS LONG
g(1) = 8
PRINT "14. REDIM SHARED g(2), in a SUB:";
grow
PRINT "    after the SUB's REDIM g(5):"; UBOUND(g); g(1); g(5)
DIM SHARED fixed(3) AS LONG
fixed(1) = 9
PRINT "15. REDIM of a shared static array in a SUB:";
nogrow
PRINT UBOUND(fixed); fixed(1)
OPTION BASE 1
REDIM ob(n) AS LONG, ob0(0) AS LONG
PRINT "16. OPTION BASE 1, REDIM ob(n), ob0(0):"; LBOUND(ob); UBOUND(ob); LBOUND(ob0); UBOUND(ob0)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB grow
    PRINT g(1); UBOUND(g)
    REDIM g(5)
    g(5) = 1
END SUB

SUB nogrow
    REDIM fixed(7) AS LONG
END SUB
