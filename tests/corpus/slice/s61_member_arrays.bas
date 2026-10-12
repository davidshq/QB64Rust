$CONSOLE:ONLY
' Member arrays (spec language/arrays-and-types, "Member arrays", and the member-array scenario of "LBOUND and
' UBOUND"): a TYPE member declared as an array with constant bounds, in both declaration forms; its elements are
' part of each value of the type. An element is named through any chain of members and elements. A store
' evaluates a number before the indexes and a string after them. The bounds never change: REDIM with the
' declared bounds and ERASE clear, REDIM _PRESERVE does nothing. LBOUND and UBOUND give the member's bounds
' without finding the place. A whole member array is an array argument. Error cases go to a handler that prints
' ERR and resumes next. A store with an index outside its bounds stores nothing here and writes position 0 of
' that dimension in the old compiler (DIVERGENCES.md D-004): no line prints such a position after such a store.
TYPE below0
    w(3) AS LONG
END TYPE
OPTION BASE 1
TYPE below1
    w(3) AS LONG
    e(0 TO 2) AS LONG
END TYPE
OPTION BASE 0
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    tail AS INTEGER
END TYPE
TYPE inner
    AS INTEGER q(2), r
END TYPE
TYPE outer
    AS inner n(1), m
END TYPE
TYPE wide
    g(1 TO 2, 3) AS _BYTE
    f(2) AS STRING * 3
    d(1) AS DOUBLE
    s(0 TO 1) AS STRING
    n(1 TO 4) AS LONG
END TYPE
TYPE holder
    k AS LONG
    Values(0 TO 3) AS LONG
END TYPE
ON ERROR GOTO h
DIM SHARED z$
DIM t AS t1, a(2) AS t1, u AS wide, ua(1) AS wide, b0 AS below0, b1 AS below1, i AS LONG, k AS LONG
DIM item(1) AS holder, p AS wide, q AS wide
REDIM x(0) AS outer
t.v(2) = 7
PRINT "1. t.v(2) = 7:"; t.v(2); t.v(3)
x(0).n(1).q(2) = 77: x(0).m.r = 88
PRINT "2. a chain:"; x(0).n(1).q(2); x(0).m.r; x(0).n(0).q(2)
a(0).v(1) = 5
PRINT "3. each element has its own:"; a(0).v(1); a(1).v(1)
PRINT "4. t.v(4) = 1:";
t.v(4) = 1
PRINT
t.v(1) = 100: t.v(2) = 200: a(1).v(1) = 211: a(2).v(1) = 221
PRINT "5. stores with an index outside:";
t.v(9) = 5
a(9).v(1) = 6
a(1).v(9) = 7
PRINT t.v(1); t.v(2); a(1).v(1); a(2).v(1)
PRINT "6. a(one(1)).v(one(2)) = one(3):";
a(one(1)).v(one(2)) = one(3)
PRINT ":"; a(1).v(1)
PRINT "7. LEN of the variable, an element, a nested element and a type of two:"; LEN(t); LEN(t.v(1)); LEN(x(0).n(1)); LEN(x(0))
PRINT "8. OPTION BASE at the TYPE block:"; LBOUND(b0.w); UBOUND(b0.w); LBOUND(b1.w); UBOUND(b1.w); LBOUND(b1.e); LEN(b0); LEN(b1)
FOR i = 0 TO 3: item(0).Values(i) = i + 1: item(1).Values(i) = i + 11: NEXT
item(0).k = 9
REDIM item(0).Values(0 TO 3)
PRINT "9. REDIM item(0).Values(0 TO 3):"; item(0).Values(0); item(0).Values(1); item(0).Values(2); item(0).Values(3); item(1).Values(0); item(1).Values(3); item(0).k
t.id = 1: t.tail = 9
FOR i = 0 TO 3: t.v(i) = i + 1: NEXT
ERASE t.v
PRINT "10. ERASE t.v:"; t.v(0); t.v(3); UBOUND(t.v); t.id; t.tail
PRINT "11. LBOUND and UBOUND:"; LBOUND(u.n); UBOUND(u.n); LBOUND(x(0).n); UBOUND(x(0).n(1).q); LBOUND(u.g, 1); UBOUND(u.g, 1); UBOUND(u.g, 2); UBOUND(t.v()); UBOUND(t.v) / 2
PRINT "12. the indexes of the place are not evaluated: UBOUND(a(one(1)).v), LBOUND(a(9).v), LEN(a(one(2)).v(9)):"; UBOUND(a(one(1)).v); LBOUND(a(9).v); LEN(a(one(2)).v(9))
PRINT "13. a dimension outside: LBOUND(t.v, 0), UBOUND(t.v, 2), UBOUND(u.g, i) for i = 3:";
k = LBOUND(t.v, 0)
k = UBOUND(t.v, 2)
i = 3
k = UBOUND(u.g, i)
i = 2
PRINT UBOUND(u.g, i)
u.g(2, 3) = 100: u.g(1, 0) = -1: u.f(1) = "abcdef": u.d(1) = 1# / 3: u.s(1) = "str"
PRINT "14. two dimensions, STRING * 3, DOUBLE, STRING:"; u.g(2, 3); u.g(1, 0); u.g(2, 0); "["; u.f(1); "]"; ASC(u.f(2)); u.d(1); u.s(1); LEN(u.s(0))
PRINT "15. u.g(one(1), one(2)) = one(3):";
u.g(one(1), one(2)) = one(3)
PRINT ":"; u.g(1, 1)
PRINT "16. x(zero(1)).n(one(2)).q(one(3)) = one(4):";
x(zero(1)).n(one(2)).q(one(3)) = one(4)
PRINT ":"; x(0).n(1).q(1)
z$ = "before"
u.s(setz(1)) = z$
z$ = "before"
ua(setz(1)).s(setz(0)) = z$ + "!"
PRINT "17. a string is evaluated after the indexes: "; u.s(1); " "; ua(1).s(0)
t.v(1) = 11: a(1).v(0) = 10
PRINT "18. reads with an index outside:";
k = t.v(9)
k = a(9).v(1)
PRINT u.s(9);
PRINT
PRINT "19. a float index and a float stored:";
t.v(1.5) = 2.5
PRINT t.v(2)
PRINT "20. a local variable, twice:";
fresh
fresh
PRINT
t.v(2) = 1: a(1).v(2) = 1
bump t.v(2)
bump a(1).v(2)
bump (t.v(2))
bumpi t.v(2)
PRINT "21. by reference, in parentheses, to an INTEGER parameter:"; t.v(2); a(1).v(2)
PRINT "22. bump a(9).v(1) and bump t.v(9):";
bump a(9).v(1)
bump t.v(9)
PRINT
u.s(1) = "s": u.f(1) = "f"
adds u.s(1)
adds u.f(1)
PRINT "23. a STRING and a STRING * 3 element to a STRING parameter: ["; u.s(1); "]["; u.f(1); "]"
x(0).n(1).r = 5: x(0).n(1).q(2) = 6
show x(0).n(1)
x(0).n(0) = x(0).n(1)
PRINT "24. an element of a user type as an argument and as a whole value:"; x(0).n(1).r; x(0).n(0).r; x(0).n(0).q(2)
t.v(0) = 1: t.v(1) = 2: a(1).v(0) = 3: a(2).v(1) = 4: u.s(0) = "a": u.s(1) = "b": x(0).n(0).r = 1
SWAP t.v(0), t.v(1)
SWAP a(1).v(0), a(2).v(1)
SWAP u.s(0), u.s(1)
SWAP x(0).n(0), x(0).n(1)
PRINT "25. SWAP:"; t.v(0); t.v(1); a(1).v(0); a(2).v(1); u.s(0); u.s(1); x(0).n(0).r; x(0).n(1).r
PRINT "26. SWAP a(one(1)).v(one(2)), a(one(3)).v(one(4)):";
SWAP a(one(1)).v(one(2)), a(one(3)).v(one(4))
PRINT
t.id = 1: t.tail = 9
fill t.v()
fill a(1).v()
PRINT "27. a whole member array as an argument:"; t.v(0); t.v(3); t.id; t.tail; a(1).v(3); a(0).v(3); a(2).v(3)
PRINT "28. two dimensions, the bounds in the SUB:";
grid u.g()
PRINT ":"; u.g(2, 2)
u.s(0) = "": u.s(1) = "x"
names u.s()
PRINT "29. a STRING member array: ["; u.s(0); "]["; u.s(1); "]"
PRINT "30. ERASE and REDIM in the SUB:";
wipe t.v()
again a(1).v()
PRINT ":"; t.v(0); t.v(3); t.id; t.tail; a(1).v(3); UBOUND(a(1).v)
PRINT "31. fill a(9).v() and fill a(zero(1)).v():";
a(0).v(3) = 0
fill a(9).v()
PRINT a(0).v(3);
fill a(zero(1)).v()
PRINT a(0).v(3)
p.g(2, 3) = 1: p.f(2) = "fff": p.d(1) = 2.5: p.s(0) = "s0": p.s(1) = "s1": p.n(4) = 44
q = p
p.s(1) = "changed": p.n(4) = 0
PRINT "32. q = p with member arrays:"; q.g(2, 3); q.f(2); q.d(1); q.s(0); q.s(1); q.n(4)
PRINT "33. ua(one(1)) = ua(one(2)), six parts:";
ua(one(1)) = ua(one(2))
PRINT
REDIM a(2).v(3) AS LONG
REDIM x(0).n(1).q(0 TO 2)
PRINT "34. REDIM with AS and through a chain:"; a(2).v(1); x(0).n(1).q(2); x(0).n(1).r
t.v(1) = 8
REDIM _PRESERVE t.v(0 TO 3)
REDIM _PRESERVE a(one(1)).v(0 TO 3)
REDIM _PRESERVE a(9).v(0 TO 3)
PRINT "35. REDIM _PRESERVE does nothing:"; t.v(1); UBOUND(t.v)
a(0).v(1) = 1: a(1).v(1) = 2: a(2).v(1) = 3
PRINT "36. ERASE a(one(1)).v, then a(9).v:";
ERASE a(one(1)).v
ERASE a(9).v
PRINT ":"; a(0).v(1); a(1).v(1); a(2).v(1)
u.s(1) = "keep": ua(1).s(1) = "gone": u.f(1) = "fff": u.d(1) = 1
ERASE ua(1).s
ERASE u.f, u.d
PRINT "37. ERASE of STRING, STRING * 3 and DOUBLE member arrays: ["; ua(1).s(1); "]["; u.s(1); "]"; ASC(u.f(1)); u.d(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB fresh
    DIM l AS t1
    PRINT l.v(2);
    l.v(2) = 5
END SUB
SUB bump (n AS LONG)
    PRINT " in bump";
    n = n + 1
END SUB
SUB bumpi (n AS INTEGER)
    n = n + 1
END SUB
SUB adds (s AS STRING)
    s = s + "!"
END SUB
SUB show (p AS inner)
    p.r = p.r + 1
END SUB
SUB fill (b() AS LONG)
    DIM i AS LONG
    FOR i = LBOUND(b) TO UBOUND(b): b(i) = i * 2 + 1: NEXT
END SUB
SUB grid (b() AS _BYTE)
    PRINT LBOUND(b, 1); UBOUND(b, 1); LBOUND(b, 2); UBOUND(b, 2);
    b(2, 2) = 22
END SUB
SUB names (b() AS STRING)
    b(0) = "zero": b(1) = b(1) + "!"
END SUB
SUB wipe (b() AS LONG)
    ERASE b
END SUB
SUB again (b() AS LONG)
    REDIM b(0 TO 3) AS LONG
    PRINT " after REDIM";
END SUB
FUNCTION one (n AS LONG)
    PRINT n;
    one = 1
END FUNCTION
FUNCTION zero (n AS LONG)
    PRINT n;
    zero = 0
END FUNCTION
FUNCTION setz (n AS LONG)
    z$ = "after"
    setz = n
END FUNCTION
