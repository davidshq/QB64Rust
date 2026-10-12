$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, review of group 8): STRING members where group 8 had no program: a
' member of an element passed by reference; an array of such a type as an array parameter, with REDIM _PRESERVE
' and ERASE in the SUB; STATIC a(n) of it; a DIM SHARED variable of it; a recursive SUB with a local of it; a
' member of a parameter passed on as a user-type parameter; a whole value copied between elements of two arrays
' in a SUB; SWAP of two elements; EXIT SUB with locals; a FUNCTION result stored into a member.
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
TYPE box
    a AS ts
    b AS ts
END TYPE
ON ERROR GOTO h
DIM SHARED sh AS ts
DIM a(2) AS ts, bx AS box, i AS LONG
REDIM d(2) AS ts
a(1).n = "one"
change a(1).n
PRINT "1. a member of an element by reference: "; a(1).n
d(1).n = "d1": d(2).n = "d2"
grow d()
PRINT "2. REDIM _PRESERVE in a SUB: "; d(1).n; " "; d(2).n; " ["; d(4).n; "]"; UBOUND(d)
wipe a()
PRINT "3. ERASE of a static array parameter: ["; a(1).n; "]"; UBOUND(a)
wipe d()
PRINT "4. ERASE of a dynamic array parameter:";
PRINT UBOUND(d);
PRINT
PRINT "5. STATIC k(2) AS ts, twice: ";
keeps
keeps
PRINT
sh.n = "shared"
shows
PRINT "6. DIM SHARED: "; sh.n
PRINT "7. a recursive SUB: ";
deep 3
PRINT
bx.a.n = "a": bx.b.n = "b"
inner bx
PRINT "8. a member of a parameter passed on: "; bx.a.n; " "; bx.b.n
REDIM d(2) AS ts
a(2).n = "two": a(2).v = 2
across a(), d()
PRINT "9. a whole value between two array parameters: "; d(1).n; d(1).v
d(2).n = "last"
SWAP d(1), d(2)
PRINT "10. SWAP of two elements: "; d(1).n; " "; d(2).n; d(2).v
early
PRINT "11. EXIT SUB with a local"
bx.a.n = word$(3)
PRINT "12. a FUNCTION result in a member: "; bx.a.n
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB change (s AS STRING)
    s = s + "?"
END SUB
SUB grow (x() AS ts)
    REDIM _PRESERVE x(4) AS ts
END SUB
SUB wipe (x() AS ts)
    ERASE x
END SUB
SUB keeps
    STATIC k(2) AS ts
    k(1).n = k(1).n + "k"
    PRINT k(1).n; " ";
END SUB
SUB shows
    sh.n = sh.n + "!"
END SUB
SUB deep (n AS LONG)
    DIM l AS ts
    l.n = STR$(n)
    IF n > 1 THEN deep n - 1
    PRINT l.n;
END SUB
SUB inner (b AS box)
    tag b.a
    tag b.b
END SUB
SUB tag (t AS ts)
    t.n = t.n + "+"
END SUB
SUB across (x() AS ts, y() AS ts)
    y(1) = x(2)
END SUB
SUB early
    DIM l AS ts
    l.n = "x"
    IF LEN(l.n) = 1 THEN EXIT SUB
    PRINT "not reached"
END SUB
FUNCTION word$ (n AS LONG)
    word$ = STRING$(n, "w")
END FUNCTION
