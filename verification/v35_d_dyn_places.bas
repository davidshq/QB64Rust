$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): where a variable with a _DYNAMIC member can stand: a local
' of a SUB (new on each call?), a STATIC local (kept between calls?), SHARED, a parameter of the type (a REDIM
' of its member seen by the caller), the member passed as an array argument, a FUNCTION's parameter.
TYPE t
    n AS LONG
    v(1) _DYNAMIC AS LONG
END TYPE
ON ERROR GOTO h
DIM SHARED g AS t
DIM x AS t
PRINT "1. a local, twice:": loc: loc
PRINT "2. a STATIC local, twice:": sta: sta
PRINT "3. SHARED: before"; LBOUND(g.v); UBOUND(g.v);: sh: PRINT " after"; LBOUND(g.v); UBOUND(g.v); g.v(9)
PRINT "4. a parameter:";: par x: PRINT " caller sees"; LBOUND(x.v); UBOUND(x.v); x.v(3)
PRINT "5. the member as an array argument:";: arr x.v(): PRINT " caller sees"; LBOUND(x.v); UBOUND(x.v); x.v(7)
PRINT "6. a FUNCTION's parameter:"; total(x)
REDIM e(1) AS t
REDIM e(1).v(2 TO 3) AS LONG: e(1).v(3) = 33
PRINT "7. an element as a parameter:";: par e(1): PRINT " caller sees"; LBOUND(e(1).v); UBOUND(e(1).v); e(1).v(3)
PRINT "8. the parent array as an argument:";: parr e(): PRINT " caller sees"; UBOUND(e(0).v); e(0).v(4)
SYSTEM
h:
PRINT "[error"; ERR; "at"; _ERRORLINE; "]";
RESUME NEXT

SUB loc
    DIM a AS t
    PRINT "   "; LBOUND(a.v); UBOUND(a.v); a.v(1); a.n
    REDIM a.v(0 TO 4) AS LONG: a.v(1) = 5: a.n = 5
END SUB

SUB sta
    STATIC a AS t
    PRINT "   "; LBOUND(a.v); UBOUND(a.v); a.v(1); a.n
    REDIM a.v(0 TO 4) AS LONG: a.v(1) = 5: a.n = 5
END SUB

SUB sh
    REDIM g.v(8 TO 9) AS LONG: g.v(9) = 99
END SUB

SUB par (p AS t)
    PRINT " callee sees"; LBOUND(p.v); UBOUND(p.v);
    REDIM p.v(3 TO 3) AS LONG: p.v(3) = 3
END SUB

SUB arr (a() AS LONG)
    PRINT " callee sees"; LBOUND(a); UBOUND(a);
    REDIM a(6 TO 7) AS LONG: a(7) = 7
END SUB

SUB parr (a() AS t)
    PRINT " callee sees"; UBOUND(a); UBOUND(a(1).v);
    REDIM a(0).v(4) AS LONG: a(0).v(4) = 4
END SUB

FUNCTION total& (p AS t)
    DIM i AS LONG, s AS LONG
    FOR i = LBOUND(p.v) TO UBOUND(p.v): s = s + p.v(i): NEXT
    total& = s
END FUNCTION
