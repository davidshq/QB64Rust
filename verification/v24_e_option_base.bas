$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): OPTION BASE 0 and 1: arrays above and below the statement,
' bounds with TO, the literal 0, REDIM, a variable bound, an array used without DIM, two dimensions, a SUB
' above and a SUB below the statement in the file, a statement skipped by GOTO.
SUB above
    DIM l(3) AS LONG
    PRINT "SUB above OPTION BASE 1 in the file: LBOUND of DIM l(3), of an array used without DIM";
    q(2) = 1
    PRINT LBOUND(l); LBOUND(q)
END SUB
ON ERROR GOTO h
DIM n AS LONG
DIM a0(3)
OPTION BASE 1
DIM a(3): DIM b(2 TO 4)
PRINT "1. DIM a0(3) above, DIM a(3) and DIM b(2 TO 4) below: LBOUND(a0), LBOUND(a), UBOUND(a), LBOUND(b)";
PRINT LBOUND(a0); LBOUND(a); UBOUND(a); LBOUND(b)
DIM z(0)
PRINT "2. DIM z(0):"; LBOUND(z); UBOUND(z)
DIM z2(0, 3)
PRINT "3. DIM z2(0, 3):"; LBOUND(z2, 1); UBOUND(z2, 1); LBOUND(z2, 2); UBOUND(z2, 2)
DIM z3(3, 0)
PRINT "4. DIM z3(3, 0):"; LBOUND(z3, 1); UBOUND(z3, 1); LBOUND(z3, 2); UBOUND(z3, 2)
REDIM r(3)
PRINT "5. REDIM r(3):"; LBOUND(r); UBOUND(r)
REDIM r0(0)
PRINT "6. REDIM r0(0):"; LBOUND(r0); UBOUND(r0)
n = 3
DIM dy(n)
PRINT "7. DIM dy(n), n = 3:"; LBOUND(dy); UBOUND(dy)
PRINT "8. DIM d0(n), n = 0:";
n = 0
DIM d0(n)
PRINT " LBOUND:";
PRINT LBOUND(d0)
im(2) = 1
PRINT "9. an array used without DIM: LBOUND, UBOUND"; LBOUND(im); UBOUND(im);
PRINT " im(0) = 1:";
im(0) = 1
PRINT
im2(2, 3) = 1
PRINT "10. the same with two indexes:"; LBOUND(im2, 1); LBOUND(im2, 2); UBOUND(im2, 2)
PRINT "11. LBOUND(nn) of a name that is no array:"; LBOUND(nn); UBOUND(nn)
above
below
OPTION BASE 0
DIM c(3)
PRINT "12. DIM c(3) below OPTION BASE 0:"; LBOUND(c)
GOTO skip
OPTION BASE 1
skip:
DIM e(3)
PRINT "13. DIM e(3) below an OPTION BASE 1 skipped by GOTO:"; LBOUND(e)
OPTION BASE 1

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB below
    DIM l(3) AS LONG
    PRINT "SUB below the last OPTION BASE 1: LBOUND of DIM l(3), of an array used without DIM";
    q(2) = 1
    PRINT LBOUND(l); LBOUND(q)
END SUB
