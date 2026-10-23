$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): below $DYNAMIC, an array used without DIM first named in the
' line of a block, each line run twice under RESUME NEXT, so that the second allocation raises error 10: what
' runs after the error for an IF, an ELSEIF, a LOOP UNTIL, a LOOP WHILE, a later CASE, a SELECT CASE, a FOR, a
' WHILE and a DO WHILE, also where the test is false or no CASE matches; a CASE after one that matched, then
' UBOUND of its array.
ON ERROR GOTO h
DIM i AS LONG, k AS LONG
'$DYNAMIC
PRINT "1. IF a(1) = 0 THEN, a block, twice:";
FOR i = 1 TO 2
    IF a(1) = 0 THEN
        PRINT " then";
    ELSE
        PRINT " else";
    END IF
    PRINT " after";
NEXT
PRINT
PRINT "2. one-line IF b(1) = 0 THEN ... ELSE ..., twice:";
FOR i = 1 TO 2
    IF b(1) = 0 THEN PRINT " then"; ELSE PRINT " else";
    PRINT " after";
NEXT
PRINT
PRINT "3. ELSEIF c(1) = 0, twice:";
FOR i = 1 TO 2
    IF i = 0 THEN
        PRINT " if";
    ELSEIF c(1) = 0 THEN
        PRINT " elseif";
    ELSE
        PRINT " else";
    END IF
    PRINT " after";
NEXT
PRINT
PRINT "4. ELSEIF not reached, then reached:";
FOR i = 1 TO 2
    IF i = 1 THEN
        PRINT " if";
    ELSEIF cc(1) = 0 THEN
        PRINT " elseif";
    END IF
    PRINT " after";
NEXT
PRINT UBOUND(cc)
PRINT "5. LOOP UNTIL d(1) = 0, the loop run twice:";
FOR i = 1 TO 2
    k = 0
    DO
        k = k + 1
        PRINT " body"; k;
        IF k > 3 THEN EXIT DO
    LOOP UNTIL d(1) = 0
    PRINT " after";
NEXT
PRINT
PRINT "6. LOOP WHILE dd(1) <> 0, the loop run twice:";
FOR i = 1 TO 2
    k = 0
    DO
        k = k + 1
        PRINT " body"; k;
        IF k > 3 THEN EXIT DO
    LOOP WHILE dd(1) <> 0
    PRINT " after";
NEXT
PRINT
PRINT "7. a later CASE e(1) + 5, twice:";
FOR i = 1 TO 2
    SELECT CASE 5
        CASE 1
            PRINT " one";
        CASE e(1) + 5
            PRINT " five";
        CASE ELSE
            PRINT " else";
    END SELECT
    PRINT " after";
NEXT
PRINT
PRINT "8. a CASE after one that matched:";
SELECT CASE 1
    CASE 1
        PRINT " one";
    CASE f(1)
        PRINT " f";
END SELECT
PRINT UBOUND(f)
PRINT "9. SELECT CASE g(1), twice:";
FOR i = 1 TO 2
    SELECT CASE g(1)
        CASE 0
            PRINT " zero";
        CASE ELSE
            PRINT " else";
    END SELECT
    PRINT " after";
NEXT
PRINT
PRINT "10. FOR k = m(1) TO 1, twice:";
FOR i = 1 TO 2
    FOR k = m(1) TO 1
        PRINT " body"; k;
    NEXT
    PRINT " after";
NEXT
PRINT
PRINT "11. WHILE n(1) < 1, twice:";
FOR i = 1 TO 2
    WHILE n(1) < 1
        PRINT " body";
        n(1) = 1
    WEND
    PRINT " after";
    n(1) = 0
NEXT
PRINT
PRINT "12. DO WHILE o(1) < 1, twice:";
FOR i = 1 TO 2
    DO WHILE o(1) < 1
        PRINT " body";
        o(1) = 1
    LOOP
    PRINT " after";
    o(1) = 0
NEXT
PRINT
PRINT "13. CASE IS > p(1), CASE p2(1) TO 9, twice:";
FOR i = 1 TO 2
    SELECT CASE 5
        CASE IS < p(1)
            PRINT " less";
        CASE p2(1) TO 9
            PRINT " range";
    END SELECT
    PRINT " after";
NEXT
PRINT
PRINT "14. WHILE q(1) > 0, false, twice:";
FOR i = 1 TO 2
    WHILE q(1) > 0
        PRINT " body";
        q(1) = 0
    WEND
    PRINT " after";
NEXT
PRINT
PRINT "15. DO UNTIL r(1) = 0, true, twice:";
FOR i = 1 TO 2
    DO UNTIL r(1) = 0
        PRINT " body";
        r(1) = 0
    LOOP
    PRINT " after";
NEXT
PRINT
PRINT "16. IF s(1) <> 0, false, twice:";
FOR i = 1 TO 2
    IF s(1) <> 0 THEN PRINT " then";
    PRINT " after";
NEXT
PRINT
PRINT "17. FOR k = t(1) + 5 TO 1, no pass, twice:";
k = 7
FOR i = 1 TO 2
    FOR k = t(1) + 5 TO 1
        PRINT " body"; k;
    NEXT
    PRINT " after"; k;
NEXT
PRINT
PRINT "18. SELECT CASE u(1) + 3, no CASE matches, twice:";
FOR i = 1 TO 2
    SELECT CASE u(1) + 3
        CASE 0
            PRINT " zero";
    END SELECT
    PRINT " after";
NEXT
PRINT
PRINT "19. ELSEIF v(1) <> 0, false, twice:";
FOR i = 1 TO 2
    IF i = 0 THEN
        PRINT " if";
    ELSEIF v(1) <> 0 THEN
        PRINT " elseif";
    ELSE
        PRINT " else";
    END IF
    PRINT " after";
NEXT
PRINT

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
