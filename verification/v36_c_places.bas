$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): where the arrays of a() = b() may be: SHARED,
' local to a SUB, STATIC, array parameters (both sides, one side, an array of the caller), COMMON SHARED (in
' conventional memory), $DYNAMIC, used without DIM, a FUNCTION.
COMMON SHARED cm() AS INTEGER
DIM SHARED cm(1 TO 3) AS INTEGER
DIM SHARED g(1 TO 3) AS INTEGER
DIM SHARED gs(1 TO 2) AS STRING
DIM a(1 TO 3) AS INTEGER, b(1 TO 3) AS INTEGER
DIM i AS LONG
FOR i = 1 TO 3: g(i) = i: a(i) = 10 * i: NEXT
gs(1) = "g1": gs(2) = "g2"
PRINT "1. SHARED into COMMON (conventional memory), from a SUB:";
FromShared
FOR i = 1 TO 3: PRINT cm(i);: NEXT: PRINT
PRINT "2. local and STATIC arrays of a SUB:";
Locals: Locals: PRINT
PRINT "3. array parameters, both sides:";
Params a(), b()
FOR i = 1 TO 3: PRINT b(i);: NEXT: PRINT
PRINT "4. a parameter and a SHARED array:";
ToParam b()
FOR i = 1 TO 3: PRINT b(i);: NEXT: PRINT
PRINT "5. a parameter of strings:";
DIM t(1 TO 2) AS STRING
StrParam t()
PRINT " "; t(1); " "; t(2)
'$DYNAMIC
DIM dy(1 TO 3) AS INTEGER
'$STATIC
PRINT "6. $DYNAMIC:";
dy() = a()
FOR i = 1 TO 3: PRINT dy(i);: NEXT: PRINT
PRINT "7. used without DIM:";
u(4) = 44
DIM w(10) AS SINGLE
w() = u()
PRINT w(4); UBOUND(w)
PRINT "8. in a FUNCTION:"; F(2)
SYSTEM

SUB FromShared
    cm() = g()
END SUB

SUB Locals
    STATIC n AS INTEGER
    DIM l(2) AS LONG
    STATIC st(2) AS LONG
    n = n + 1
    l(1) = 100 + n
    PRINT st(1);
    st() = l()
    PRINT st(1);
END SUB

SUB Params (p() AS INTEGER, q() AS INTEGER)
    q() = p()
END SUB

SUB ToParam (q() AS INTEGER)
    g(2) = -2
    q() = g()
END SUB

SUB StrParam (q() AS STRING)
    q() = gs()
END SUB

FUNCTION F (k AS LONG)
    DIM x(3) AS LONG, y(3) AS LONG
    x(k) = k * 3
    y() = x()
    F = y(k)
END FUNCTION
