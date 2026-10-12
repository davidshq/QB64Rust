$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): ERASE, REDIM and REDIM _PRESERVE of a member array whose
' base has indexes: whether the indexes are evaluated, and what one outside its bounds does.
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    s(0 TO 1) AS STRING
END TYPE
TYPE t3
    AS t1 n(1), m
END TYPE
ON ERROR GOTO h
DIM a(3) AS t1, x(1) AS t3, i AS LONG
FOR i = 0 TO 3: a(i).v(1) = i + 10: a(i).s(1) = "s" + STR$(i): NEXT
PRINT "1. ERASE a(one(1)).v:";
ERASE a(one(1)).v
PRINT ":"; a(0).v(1); a(1).v(1); a(2).v(1)
PRINT "2. ERASE a(9).v:";
ERASE a(9).v
PRINT ":"; a(0).v(1); a(2).v(1)
PRINT "3. REDIM a(one(2)).v(0 TO 3):";
REDIM a(one(2)).v(0 TO 3)
PRINT ":"; a(1).v(1); a(2).v(1); a(3).v(1)
PRINT "4. REDIM a(9).s(0 TO 1):";
REDIM a(9).s(0 TO 1)
PRINT ":["; a(0).s(1); "]["; a(1).s(1); "]"
PRINT "5. REDIM _PRESERVE a(one(3)).v(0 TO 3):";
REDIM _PRESERVE a(one(3)).v(0 TO 3)
PRINT ":"; a(3).v(1)
PRINT "6. REDIM _PRESERVE a(9).v(0 TO 3):";
REDIM _PRESERVE a(9).v(0 TO 3)
PRINT ":"; a(3).v(1)
x(1).n(1).v(2) = 5: x(1).n(0).v(2) = 6: x(0).n(0).v(2) = 7
PRINT "7. ERASE x(one(1)).n(one(2)).v:";
ERASE x(one(1)).n(one(2)).v
PRINT ":"; x(1).n(1).v(2); x(1).n(0).v(2)
PRINT "8. ERASE x(1).n(9).v:";
ERASE x(1).n(9).v
PRINT ":"; x(1).n(0).v(2); x(0).n(0).v(2)
PRINT "9. ERASE a(1).s, ERASE a(2).v, a(3).s on one line:";
ERASE a(1).s, a(2).v: ERASE a(3).s
PRINT ":["; a(1).s(1); "]"; a(2).v(1); "["; a(2).s(1); "]["; a(3).s(1); "]"
PRINT "10. REDIM a(2).v(0 TO 3), a(2).s(0 TO 1) in one list:";
a(2).v(1) = 4
REDIM a(2).v(0 TO 3), a(2).s(0 TO 1)
PRINT ":"; a(2).v(1); "["; a(2).s(1); "]"
PRINT "11. REDIM a(ASC(empty)).v(0 TO 3), then a(0).v(2):";
a(0).v(2) = 8
e$ = ""
REDIM a(ASC(e$)).v(0 TO 3)
PRINT ":"; a(0).v(2)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

FUNCTION one (n AS LONG)
    PRINT n;
    one = n
END FUNCTION
