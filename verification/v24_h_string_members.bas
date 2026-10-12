$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): STRING members: first value in a variable, a local, a static and
' a dynamic array, a nested type; stores and concatenation; REDIM, REDIM _PRESERVE and ERASE of arrays of such
' a type; passed by reference.
TYPE ts
    n AS STRING
    v AS LONG
    f AS STRING * 2
END TYPE
TYPE outer
    i AS ts
    t AS STRING
END TYPE
ON ERROR GOTO h
DIM p AS ts, o AS outer, a(2) AS ts, i AS LONG
p.n = "abc": p.n = p.n + "!"
PRINT "1. "; p.n; LEN(p.n); p.v; ASC(p.f)
o.i.n = "in": o.t = "out"
PRINT "2. nested: "; o.i.n; " "; o.t; LEN(o.i.n)
a(1).n = "one": a(2).n = "two": a(1).v = 1
PRINT "3. a static array: ["; a(0).n; "] "; a(1).n; " "; a(2).n
ERASE a
PRINT "4. after ERASE: ["; a(1).n; "]"; a(1).v; UBOUND(a)
REDIM d(1) AS ts
d(1).n = "keep": d(1).v = 5
REDIM _PRESERVE d(3) AS ts
PRINT "5. REDIM _PRESERVE of a dynamic array: "; d(1).n; d(1).v; " ["; d(3).n; "]"
REDIM d(2) AS ts
PRINT "6. REDIM: ["; d(1).n; "]"; d(1).v
d(1).n = "x"
ERASE d
PRINT "7. ERASE of the dynamic array, UBOUND:";
PRINT UBOUND(d)
PRINT "8. a local, twice:";
local: local
PRINT
PRINT "9. a STATIC one, twice:";
keep: keep
PRINT
change p.n
PRINT "10. the member passed by reference: "; p.n
PRINT "11. a long string and an empty one: ";
p.n = STRING$(1000, "z"): PRINT LEN(p.n);
p.n = "": PRINT LEN(p.n)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB local
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
