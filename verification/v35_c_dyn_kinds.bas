$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): _DYNAMIC members of every kind of element: each numeric
' type, STRING * n, STRING, a user type without and with a string, a user type that has a _DYNAMIC member
' itself; _RETAIN of each, and a whole-value copy of a type that holds them all.
TYPE inner
    k AS INTEGER
    f AS STRING * 2
END TYPE
TYPE innerstr
    k AS INTEGER
    s AS STRING
END TYPE
TYPE innerdyn
    k AS INTEGER
    d(1) _DYNAMIC AS INTEGER
END TYPE
TYPE t
    b(1) _DYNAMIC AS _BYTE
    ub(1) _DYNAMIC AS _UNSIGNED _BYTE
    i(1) _DYNAMIC AS INTEGER
    l(1) _DYNAMIC AS LONG
    q(1) _DYNAMIC AS _INTEGER64
    uq(1) _DYNAMIC AS _UNSIGNED _INTEGER64
    s(1) _DYNAMIC AS SINGLE
    d(1) _DYNAMIC AS DOUBLE
    f(1) _DYNAMIC AS _FLOAT
    o(1) _DYNAMIC AS _OFFSET
    fs(1) _DYNAMIC AS STRING * 3
    vs(1) _DYNAMIC AS STRING
    u(1) _DYNAMIC AS inner
    us(1) _DYNAMIC AS innerstr
    ud(1) _DYNAMIC AS innerdyn
END TYPE
ON ERROR GOTO h
DIM x AS t, y AS t
x.b(1) = -5: x.ub(1) = 250: x.i(1) = -30000: x.l(1) = 100000: x.q(1) = 2 ^ 40: x.uq(1) = 18446744073709551614~&&
x.s(1) = 1.5: x.d(1) = 2.25: x.f(1) = 3.125: x.o(1) = 77
x.fs(0) = "abc": x.fs(1) = "xy"
x.vs(0) = "first": x.vs(1) = "second string"
x.u(1).k = 9: x.u(1).f = "zz"
x.us(1).k = 8: x.us(1).s = "inner string"
x.ud(1).k = 7: x.ud(1).d(1) = 70
REDIM x.ud(1).d(2 TO 4) AS INTEGER: x.ud(1).d(4) = 44
PRINT "1."; x.b(1); x.ub(1); x.i(1); x.l(1); x.q(1); x.uq(1); x.s(1); x.d(1); x.f(1); x.o(1)
PRINT "2. ["; x.fs(0); "] ["; x.fs(1); "] ["; x.vs(0); "] ["; x.vs(1); "]"
PRINT "3."; x.u(1).k; x.u(1).f; x.us(1).k; x.us(1).s; x.ud(1).k; LBOUND(x.ud(1).d); UBOUND(x.ud(1).d); x.ud(1).d(4)
y = x
x.vs(1) = "changed": x.us(1).s = "changed": x.ud(1).d(4) = -1: x.fs(0) = "QQQ"
PRINT "4. copy: ["; y.vs(1); "] ["; y.us(1).s; "]"; y.ud(1).d(4); "["; y.fs(0); "]"
REDIM _RETAIN x.vs(1 TO 3) AS STRING
PRINT "5. _RETAIN vs(1 TO 3): ["; x.vs(1); "] ["; x.vs(2); "] ["; x.vs(3); "]"
REDIM _RETAIN x.fs(-1 TO 1) AS STRING * 3
PRINT "6. _RETAIN fs(-1 TO 1): ["; x.fs(-1); "] ["; x.fs(0); "] ["; x.fs(1); "]"
REDIM _PRESERVE x.us(0 TO 2) AS innerstr
PRINT "7. _PRESERVE us(0 TO 2):"; x.us(1).k; "["; x.us(1).s; "]"; x.us(2).k; "["; x.us(2).s; "]"
REDIM _RETAIN x.ud(1 TO 2) AS innerdyn
PRINT "8. _RETAIN ud(1 TO 2):"; x.ud(1).k; LBOUND(x.ud(1).d); UBOUND(x.ud(1).d); x.ud(1).d(4); LBOUND(x.ud(2).d); UBOUND(x.ud(2).d)
REDIM x.vs(0 TO 1) AS STRING
PRINT "9. plain REDIM vs: ["; x.vs(0); "] ["; x.vs(1); "]"; LEN(x.vs(1))
ERASE x.us
PRINT "10. ERASE us, then y.us(1).s: ["; y.us(1).s; "]"
SYSTEM
h:
PRINT "[error"; ERR; "at"; _ERRORLINE; "]";
RESUME NEXT
