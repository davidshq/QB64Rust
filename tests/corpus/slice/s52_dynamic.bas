$CONSOLE:ONLY
' Dynamic arrays in the main module (spec language/arrays-and-types, "Dynamic arrays", and the scenarios of
' "Static arrays" that came with them): DIM with a bound that is not constant, also by its spelling only; DIM run
' twice; $DYNAMIC and $STATIC; a dimension with no element; a bound that raises; an array used before its DIM has
' run; every element type, _BIT included (also as READ targets); float bounds; OPTION BASE; DIM SHARED seen in a
' SUB and an element passed by reference; DIM with empty parentheses. Error cases go to a handler that prints ERR and resumes next.
TYPE rec
    n AS LONG
END TYPE
ON ERROR GOTO h
DIM i AS LONG, n AS LONG, m AS LONG
n = 4
DIM d(n) AS LONG: d(4) = 9
PRINT "1. a bound from a variable, DIM d(n): LBOUND, UBOUND, d(4), d(0)"; LBOUND(d); UBOUND(d); d(4); d(0)
d(1) = 5
PRINT "2. another DIM of it, DIM d(n + 1):";
DIM d(n + 1) AS LONG
PRINT UBOUND(d); d(1)
'$DYNAMIC
DIM a(5) AS LONG
'$STATIC
DIM a0(5) AS LONG
REDIM a(8) AS LONG
PRINT "3. $DYNAMIC above DIM a(5), then REDIM a(8):"; UBOUND(a)
PRINT "   $STATIC above DIM a0(5), then REDIM a0(8):";
REDIM a0(8) AS LONG
PRINT UBOUND(a0)
m = -1
PRINT "4. no element, DIM e(m) with m = -1:";
DIM e(m) AS LONG
PRINT LBOUND(e); UBOUND(e);
e(0) = 1
PRINT
PRINT "   and DIM e(n) after it:";
DIM e(n) AS LONG
PRINT UBOUND(e)
PRINT "5. a bound with an operator, DIM a3(2 + 3) then REDIM a3(20):";
DIM a3(2 + 3) AS LONG
REDIM a3(20) AS LONG
PRINT UBOUND(a3)
PRINT "   one number, DIM a1(5) then REDIM a1(20):";
DIM a1(5) AS LONG
REDIM a1(20) AS LONG
PRINT UBOUND(a1)
PRINT "6. two DIM statements, DIM s(5) then DIM s(n):";
DIM s(5) AS LONG
DIM s(n) AS LONG
PRINT UBOUND(s)
PRINT "7. a bound that raises, DIM r(ASC of an empty string):";
DIM r(ASC("")) AS LONG
PRINT UBOUND(r)
PRINT "8. a DIM that GOTO skips: late(1) = 1";
GOTO skip
DIM late(n) AS LONG
skip:
late(1) = 1
PRINT "   LBOUND(late)";
PRINT LBOUND(late)
PRINT "   UBOUND(late, 1)";
PRINT UBOUND(late, 1)
PRINT "   PRINT late(1)";
PRINT late(1)
PRINT "9. one DIM in a loop that runs twice:";
FOR i = 1 TO 2
    DIM lp(i + 2) AS LONG
NEXT
PRINT UBOUND(lp)
DIM st(n) AS STRING, fx(n) AS STRING * 3, ud(n) AS rec, db(1 TO n, n) AS DOUBLE
st(1) = "one": fx(1) = "abcdef": ud(2).n = 7: db(2, 3) = 1.5
PRINT "10. STRING, STRING * 3, a user type, DOUBLE in two dimensions: "; st(1); "["; st(2); "] "; fx(1); LEN(fx(2)); ASC(fx(2)); ud(2).n; ud(1).n; db(2, 3); db(1, 0); LBOUND(db, 1); UBOUND(db, 2)
x = 2.5
DIM q(x), q2(1.5 TO x + 1)
PRINT "11. float bounds, DIM q(x) with x = 2.5, DIM q2(1.5 TO x + 1):"; UBOUND(q); LBOUND(q2); UBOUND(q2)
PRINT "12. an index outside, d(5) = 1 and PRINT d(-1):";
d(5) = 1
PRINT d(-1)
DIM bt(n) AS _BIT * 4, bu(1 TO n) AS _UNSIGNED _BIT * 3
bt(0) = 7: bt(1) = 8: bt(3) = 15: bu(1) = 9: bu(4) = -1
PRINT "13. dynamic _BIT * 4 and _UNSIGNED _BIT * 3:"; bt(0); bt(1); bt(2); bt(3); bu(1); bu(2); bu(4)
DIM b4(3) AS _BIT * 4
b4(0) = 7: b4(1) = 8: b4(3) = 15
PRINT "14. static _BIT * 4, stores 7, 8, none, 15:"; b4(0); b4(1); b4(2); b4(3)
DIM SHARED sh(n) AS LONG
sh(2) = 6
PRINT "15. DIM SHARED sh(n) in a SUB:";
show
bump sh(2)
bump d(4)
PRINT "    elements passed by reference:"; sh(2); d(4)
DIM ep() AS LONG
ep = 100000.5
PRINT "16. DIM ep() AS LONG, ep = 100000.5: ep, LEN(ep)"; ep; LEN(ep)
OPTION BASE 1
DIM ob(n) AS LONG, ob0(0, n) AS LONG, ob2(0 TO n) AS LONG
PRINT "17. OPTION BASE 1, DIM ob(n), ob0(0, n), ob2(0 TO n):"; LBOUND(ob); UBOUND(ob); LBOUND(ob0, 1); UBOUND(ob0, 1); LBOUND(ob0, 2); LBOUND(ob2)
m = 0
PRINT "18. OPTION BASE 1, DIM z(m) with m = 0:";
DIM z(m) AS LONG
PRINT LBOUND(z); UBOUND(z)
PRINT "    DIM z2(0 + 0):";
DIM z2(0 + 0) AS LONG
PRINT LBOUND(z2); UBOUND(z2)
READ bt(2), bu(2), b4(2)
PRINT "19. READ into _BIT elements, dynamic and static:"; bt(2); bu(2); b4(2)
DATA 5, 7, -8
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB show
    PRINT sh(2); UBOUND(sh)
END SUB

SUB bump (v AS LONG)
    v = v + 1
END SUB
