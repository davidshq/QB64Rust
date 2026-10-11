$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 5.1): below $DYNAMIC, where the allocation of an array used without
' DIM lands when its first use stands in the header of a block, in a loop, or in a one-line IF.
ON ERROR GOTO h
'$DYNAMIC
PRINT "1. IF header:";
IF a(1) = 0 THEN PRINT UBOUND(a)
PRINT "2. one-line IF, in THEN:";
IF 1 THEN b(1) = 2
PRINT b(1); UBOUND(b)
PRINT "3. one-line IF not taken:";
IF 0 THEN bb(1) = 2
PRINT UBOUND(bb)
PRINT "4. in a FOR body run twice:";
FOR i = 1 TO 2
    c(1) = c(1) + 1
NEXT
PRINT c(1)
PRINT "5. DO WHILE header, body run twice:";
DO WHILE d(1) < 2
    d(1) = d(1) + 1
LOOP
PRINT d(1)
PRINT "6. LOOP UNTIL, body run twice:";
k = 0
DO
    k = k + 1
LOOP UNTIL k >= 2 + e(1)
PRINT k; UBOUND(e)
PRINT "7. WHILE header, body run twice:";
WHILE f(1) < 2
    f(1) = f(1) + 1
WEND
PRINT f(1)
PRINT "8. FOR bound:";
FOR i = g(1) TO 1
NEXT
PRINT UBOUND(g)
PRINT "9. SELECT CASE selector:";
SELECT CASE s(1)
    CASE 0
        PRINT UBOUND(s)
END SELECT
PRINT "10. CASE value, second CASE:";
SELECT CASE 5
    CASE 1
        PRINT "one"
    CASE 5 + t(1)
        PRINT UBOUND(t)
END SELECT
PRINT "11. ELSEIF header:";
IF 0 THEN
    PRINT "no"
ELSEIF j(1) = 0 THEN
    PRINT UBOUND(j)
END IF
PRINT "12. ELSEIF header not reached:";
IF 1 THEN
    PRINT " first";
ELSEIF jj(1) = 0 THEN
    PRINT "no"
END IF
PRINT UBOUND(jj)
PRINT "13. LBOUND first:"; LBOUND(l); UBOUND(l)
PRINT "14. two in one statement:"; m(1) + n(1); UBOUND(m); UBOUND(n)
PRINT "15. after a colon:";: o(1) = 3: PRINT o(1)
PRINT "16. FOR STEP and block IF body:";
FOR i = 1 TO 3 STEP 1 + q(1)
    IF i = 2 THEN
        r(1) = r(1) + 1
    END IF
NEXT
PRINT UBOUND(q); r(1)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
