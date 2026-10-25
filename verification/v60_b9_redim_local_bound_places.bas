$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): in a SUB, REDIM of a shared array where the array used without DIM is
' first named in the lower bound, in an upper bound after a lower one, and in the second dimension.
ON ERROR GOTO h
REDIM SHARED sa(1), sb(1), sc(1, 1)
PRINT "1.";: q1
PRINT " main:"; LBOUND(sa); UBOUND(sa)
PRINT "2.";: q2
PRINT " main:"; LBOUND(sb); UBOUND(sb)
PRINT "3.";: q3
PRINT " main:"; UBOUND(sc, 1); UBOUND(sc, 2)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB q1
    REDIM sa(la(1) TO 4)
    PRINT " q1:"; LBOUND(sa); UBOUND(sa);
END SUB

SUB q2
    REDIM sb(2 TO lb(1) + 5)
    PRINT " q2:"; LBOUND(sb); UBOUND(sb);
END SUB

SUB q3
    REDIM sc(3, lq(1) + 6)
    PRINT " q3:"; UBOUND(sc, 1); UBOUND(sc, 2);
END SUB
