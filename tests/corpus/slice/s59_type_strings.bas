$CONSOLE:ONLY
' STRING members (spec language/arrays-and-types, "User types": the two "String member" scenarios; "ERASE":
' "Array of a user type with a string member"): a member AS STRING is a string of its own in every value of the
' type: a variable, a nested member, each element of a static and of a dynamic array, a local (empty on each
' call) and a STATIC variable (kept). ERASE of a static array empties it, REDIM _PRESERVE keeps it, REDIM and
' ERASE of a dynamic array give it up. It is stored, read, compared and passed like any string. Error cases go
' to a handler that prints ERR and resumes next.
TYPE ts
    n AS STRING
    v AS LONG
    f AS STRING * 2
END TYPE
TYPE outer
    k AS INTEGER
    i AS ts
    t AS STRING
END TYPE
ON ERROR GOTO h
DIM p AS ts, q AS ts, o AS outer, a(2) AS ts, i AS LONG
p.n = "abc": p.n = p.n + "!": PRINT "1. "; p.n; LEN(p.n)
PRINT "2. the members beside it:"; p.v; ASC(p.f); " ["; q.n; "]"
o.i.n = "in": o.t = "out": o.k = 7
PRINT "3. nested: "; o.i.n; " "; o.t; o.k; LEN(o.i.n)
a(1).n = "one": a(2).n = "two": a(1).v = 1
PRINT "4. a static array: ["; a(0).n; "] "; a(1).n; " "; a(2).n
i = 2
a(i).n = a(i - 1).n + "+"
PRINT "5. from element to element: "; a(2).n; " "; a(1).n
PRINT "6. a read with an index outside:";
PRINT " ["; a(9).n; "]";
a(0).n = a(1).n + a(2).n
PRINT " "; a(0).n
ERASE a
PRINT "7. after ERASE: ["; a(1).n; "]"; a(1).v; UBOUND(a)
REDIM d(1) AS outer
d(1).i.n = "keep": d(1).t = "too": d(1).k = 5
REDIM _PRESERVE d(3) AS outer
PRINT "8. REDIM _PRESERVE: "; d(1).i.n; " "; d(1).t; d(1).k; " ["; d(3).t; "]"
REDIM _PRESERVE d(0) AS outer
PRINT "9. REDIM _PRESERVE to fewer elements: ["; d(0).t; "]"; UBOUND(d)
REDIM d(2) AS outer
PRINT "10. REDIM: ["; d(1).i.n; "]"; d(1).k
d(1).t = "x"
ERASE d
PRINT "11. ERASE of the dynamic array:";
PRINT UBOUND(d);
PRINT
PRINT "12. a local, twice: ";
fresh
fresh
PRINT
PRINT "13. a STATIC one, twice: ";
keep
keep
PRINT
change p.n
PRINT "14. passed by reference: "; p.n
p.n = STRING$(1000, "z"): PRINT "15. a long string and an empty one:"; LEN(p.n);
p.n = "": PRINT LEN(p.n)
READ p.n, q.n$
PRINT "16. READ, and the member named with its suffix: "; p.n; " "; q.n$
MID$(p.n, 2, 1) = "#"
SWAP p.n, q.n
PRINT "17. MID$ and SWAP: "; p.n; " "; q.n
IF q.n = "d#ta" AND p.n > q.n THEN PRINT "18. compared"
PRINT "19. a local array, twice: ";
locals 2
locals 3
PRINT
SYSTEM
DATA data,more
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB fresh
    DIM l AS ts
    PRINT "["; l.n; "]";
    l.n = "x"
END SUB
SUB keep
    STATIC k AS ts
    k.n = k.n + "k"
    PRINT k.n; " ";
END SUB
SUB change (s AS STRING)
    s = s + "?"
END SUB
SUB locals (n AS LONG)
    DIM la(n) AS ts
    PRINT "["; la(n).n; "]";
    la(n).n = "y"
    PRINT la(n).n; UBOUND(la);
END SUB
