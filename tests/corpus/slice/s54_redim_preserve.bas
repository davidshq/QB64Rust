$CONSOLE:ONLY
' REDIM _PRESERVE (spec language/arrays-and-types, "REDIM _PRESERVE"): the elements kept by flat position, as many
' as the smaller of the old and the new number: growing and shrinking, strings, fixed-length strings, a user type,
' _BIT elements, two dimensions (positions, not coordinates), another lower bound; of an array that is not
' allocated (as REDIM), as the first statement that names the array, of a static array (error 10); with SHARED;
' of a shared array in a SUB; in a loop that grows an array. Error cases go to a handler that prints ERR and
' resumes next.
TYPE rec
    n AS LONG
    t AS STRING * 2
END TYPE
ON ERROR GOTO h
DIM i AS LONG, n AS LONG
REDIM p(3) AS LONG
FOR i = 0 TO 3: p(i) = i + 1: NEXT
REDIM _PRESERVE p(5) AS LONG
PRINT "1. 0 to 3 holding 1 2 3 4, REDIM _PRESERVE p(5): p(3), p(5), UBOUND"; p(3); p(5); UBOUND(p)
REDIM _PRESERVE p(1) AS LONG
PRINT "   then REDIM _PRESERVE p(1): UBOUND, p(0), p(1)"; UBOUND(p); p(0); p(1)
REDIM _PRESERVE p(3) AS LONG
PRINT "   then REDIM _PRESERVE p(3): p(1), p(2), p(3)"; p(1); p(2); p(3)
REDIM t(1) AS STRING
t(0) = "zero": t(1) = "keep"
REDIM _PRESERVE t(4) AS STRING
PRINT "2. strings: "; t(0); " "; t(1); LEN(t(4))
REDIM _PRESERVE t(0) AS STRING
REDIM _PRESERVE t(2) AS STRING
PRINT "   shrunk to one and grown again: "; t(0); " ["; t(1); "]"
REDIM g(1, 1) AS LONG
g(0, 0) = 1: g(1, 0) = 2: g(0, 1) = 3: g(1, 1) = 4
REDIM _PRESERVE g(2, 1) AS LONG
PRINT "3. two dimensions, (1, 1) holding 1 2 3 4 to (2, 1):"; g(0, 0); g(1, 0); g(2, 0); g(0, 1); g(1, 1); g(2, 1)
REDIM b(3) AS LONG
FOR i = 0 TO 3: b(i) = i + 1: NEXT
REDIM _PRESERVE b(10 TO 15) AS LONG
PRINT "4. another lower bound, 0 to 3 to 10 to 15:";
FOR i = 10 TO 15: PRINT b(i);: NEXT
PRINT
DIM s(5) AS LONG
s(1) = 3
PRINT "5. of a static array:";
REDIM _PRESERVE s(9) AS LONG
PRINT UBOUND(s); s(1)
REDIM _PRESERVE nw(3) AS LONG
nw(3) = 5
PRINT "6. as the first statement that names the array:"; UBOUND(nw); nw(3)
ERASE nw
REDIM _PRESERVE nw(2) AS LONG
PRINT "   of an array that is not allocated:"; UBOUND(nw); nw(2)
REDIM fx(1) AS STRING * 3, ud(1) AS rec
fx(1) = "abc": ud(1).n = 7: ud(1).t = "xy"
REDIM _PRESERVE fx(3) AS STRING * 3, ud(3) AS rec
PRINT "7. STRING * 3 and a user type: "; fx(1); ASC(fx(3)); ud(1).n; ud(1).t; ud(3).n; ASC(ud(3).t)
REDIM bt(4) AS _BIT * 5
FOR i = 0 TO 4: bt(i) = i + 1: NEXT
REDIM _PRESERVE bt(9) AS _BIT * 5
PRINT "8. _BIT * 5 from 4 to 9:";
FOR i = 0 TO 9: PRINT bt(i);: NEXT
PRINT
REDIM _PRESERVE bt(1) AS _BIT * 5
REDIM _PRESERVE bt(3) AS _BIT * 5
PRINT "   shrunk to 1 and grown to 3:"; bt(0); bt(1); bt(2); bt(3)
REDIM _PRESERVE SHARED sh(2) AS LONG
sh(2) = 6
PRINT "9. REDIM _PRESERVE SHARED sh(2), grown in a SUB:";
grow
PRINT UBOUND(sh); sh(2); sh(4)
REDIM lst(0) AS STRING
n = 0
FOR i = 1 TO 5
    n = n + 1
    REDIM _PRESERVE lst(n) AS STRING
    lst(n) = STR$(i * i)
NEXT
PRINT "10. a list grown in a loop:"; UBOUND(lst); lst(1); lst(3); lst(5); " ["; lst(0); "]"
n = 2
REDIM _PRESERVE lst(n TO n + 2) AS STRING
PRINT "11. the same, cut to three elements from 2:"; LBOUND(lst); UBOUND(lst); " ["; lst(2); "]"; lst(3); lst(4)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB grow
    REDIM _PRESERVE sh(4)
    sh(4) = sh(2) + 1
END SUB
