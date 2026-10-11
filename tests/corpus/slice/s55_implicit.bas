$CONSOLE:ONLY
' Arrays used without DIM (spec language/arrays-and-types, "Implicit arrays"): in the main module a static array,
' OPTION BASE to 10 per index written, of the suffix's type or the default type; LBOUND and UBOUND of a name that
' is no array make one; the names that stay free; ERASE and REDIM of one; below $DYNAMIC a dynamic array that the
' statement naming it first allocates; in a SUB or FUNCTION an array that nothing allocates (every use raises
' error 9). Error cases go to a handler that prints ERR and resumes next.
ON ERROR GOTO h
DIM i AS LONG
z(10) = 4
PRINT "1. z(10) = 4: z(10), LBOUND, UBOUND"; z(10); LBOUND(z); UBOUND(z)
PRINT "2. z(11) = 1, z(-1):";
z(11) = 1
PRINT z(-1)
m(2, 3) = 5
PRINT "3. two indexes: UBOUND 1, UBOUND 2, LBOUND 2, m(2, 3), m(10, 10)"; UBOUND(m, 1); UBOUND(m, 2); LBOUND(m, 2); m(2, 3); m(10, 10)
t(1, 2, 3) = 1
PRINT "4. three indexes: UBOUND 3, t(1, 2, 3), t(10, 10, 10)"; UBOUND(t, 3); t(1, 2, 3); t(10, 10, 10)
s$(2) = "ab": i%(3) = 2.5: d#(1) = 1# / 3: q&&(10) = 5000000000: u~%%(4) = 200: b`(5) = 1
PRINT "5. suffixes: "; s$(2); UBOUND(s$); i%(3); d#(1); q&&(10); u~%%(4); b`(5); LEN(i%(3)); LEN(d#(1))
PRINT "6. the first use is a read:"; r(4); UBOUND(r); "["; rs$(1); "]"
PRINT "7. LBOUND(lb) first, then lb(10) = 3:"; LBOUND(lb); UBOUND(lb);
lb(10) = 3: PRINT lb(10);
PRINT " UBOUND(lb, 2):";
PRINT UBOUND(lb, 2)
PRINT "8. the first use skipped by GOTO:";
GOTO sk
w(1) = 1
sk:
PRINT UBOUND(w); w(1)
PRINT "9. REDIM z(20):";
REDIM z(20)
PRINT UBOUND(z); z(10)
DIM c(3) AS LONG
c!(7) = 1.5: c(1) = 2
PRINT "10. c!(7) beside DIM c(3) AS LONG: c!(7), c(1), UBOUND(c), UBOUND(c!)"; c!(7); c(1); UBOUND(c); UBOUND(c!)
a = 5: a(2) = 3
PRINT "11. a scalar and an array of one name: a, a(2)"; a; a(2)
PRINT "12. an index in a float and in a string of digits:"; z(9.6);
z(VAL("3")) = 8: PRINT z(3)
DIM x AS LONG
x = 7: x(3) = 2.5
PRINT "13. beside DIM x AS LONG: x, x(3), LEN(x(3))"; x; x(3); LEN(x(3))
CONST k = 1
k(1) = 2
PRINT "14. beside CONST k: k, k(1)"; k; k(1)
chr(1) = 5: abc(2) = left(3) + 1: __foo(1) = 2
PRINT "15. chr(1), left(3), __foo(1), UBOUND(left):"; chr(1); left(3); __foo(1); UBOUND(left)
e(1) = 5: es$(2) = "ab"
ERASE e, es$
PRINT "16. ERASE: e(1), UBOUND(e), es$(2):"; e(1); UBOUND(e); "["; es$(2); "]"
FOR i = 0 TO 10: f(i) = i * i: NEXT
SWAP f(2), f(3)
bump f(4)
PRINT "17. filled in a loop, SWAP, an element by reference:"; f(2); f(3); f(4); f(10)
ss$(0) = "x": ss$(10) = ss$(0) + "y"
PRINT "18. strings: "; ss$(10); LEN(ss$(5))
DEFINT A-Z
OPTION BASE 1
di(1) = 2.5
PRINT "19. DEFINT A-Z and OPTION BASE 1: di(1), LEN, LBOUND, UBOUND, UBOUND(di%)"; di(1); LEN(di(1)); LBOUND(di); UBOUND(di); UBOUND(di%)
PRINT "20. di(0):";
PRINT di(0)
dm(1, 1) = 1
PRINT "21. two indexes under base 1:"; LBOUND(dm, 1); LBOUND(dm, 2); UBOUND(dm, 2)
DEFSNG A-Z
OPTION BASE 0
'$DYNAMIC
dy(3) = 4
PRINT "22. below $DYNAMIC: dy(3), LBOUND, UBOUND"; dy(3); LBOUND(dy); UBOUND(dy)
PRINT "23. the first use skipped by GOTO:";
GOTO sk2
dw(1) = 1
sk2:
PRINT UBOUND(dw);
dw(1) = 2
PRINT
PRINT "24. the first use run three times:";
FOR i = 1 TO 3
    dl(3) = dl(3) + 1
NEXT
PRINT dl(3); UBOUND(dl)
PRINT "25. REDIM of it:";
REDIM dy(20)
PRINT UBOUND(dy); dy(3)
PRINT "26. a one-line IF taken, and one not taken:";
IF i > 0 THEN it(1) = 2
IF i < 0 THEN nt(1) = 2
PRINT it(1); UBOUND(it);
PRINT UBOUND(nt)
PRINT "27. LBOUND first:";
PRINT LBOUND(dlb); UBOUND(dlb)
PRINT "28. two in one statement:";
PRINT da(1) + db(1, 2); UBOUND(da); UBOUND(db, 2)
PRINT "29. ERASE of it, then a use:";
ERASE da
PRINT da(1)
ds$(1) = "dyn"
PRINT "30. strings: "; ds$(1); UBOUND(ds$)
'$STATIC
st(1) = 6
PRINT "31. below $STATIC again:"; st(1); UBOUND(st)
REDIM st(3)
PRINT
PRINT "32. a SUB, called twice:";
plain
plain
PRINT
PRINT "33. a FUNCTION:";
i = fn(1)
PRINT
PRINT "34. beside main arrays, a parameter and a DIM SHARED scalar of the name:";
DIM SHARED g AS LONG
g = 3
names 4
PRINT z(10); g
PRINT "35. ERASE in a SUB:";
wipe
PRINT

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB bump (v)
    v = v + 100
END SUB
SUB plain
    x(2) = x(2) + 1
    PRINT " x(2):";
    PRINT x(2);
END SUB
FUNCTION fn (n)
    x(2) = 5
    PRINT " UBOUND(x), LBOUND(x):";
    PRINT UBOUND(x); LBOUND(x);
    s$(1) = "a"
    PRINT " ["; s$(1); "]";
END FUNCTION
SUB names (n)
    PRINT " z(10):";
    PRINT z(10);
    g(1) = 2.5
    PRINT " g(1):";
    PRINT g(1);
    n(1) = 1
    PRINT " n(1), n:";
    PRINT n(1); n
END SUB
SUB wipe
    w2(2) = 5
    ERASE w2
    PRINT " UBOUND(w2):";
    PRINT UBOUND(w2);
END SUB
