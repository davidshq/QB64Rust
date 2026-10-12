$CONSOLE:ONLY
' A DIM or REDIM of several dynamic arrays is one statement (spec language/arrays-and-types, "Dynamic arrays"):
' run a second time it raises error 10 once, an array of the list that is not allocated is still allocated after
' the error, and RESUME NEXT goes on after the whole list. Error cases go to a handler that prints ERR and
' resumes next.
ON ERROR GOTO h
DIM i AS LONG, n AS LONG
PRINT "1. DIM du(i + 1), dv(i + 3), run twice:";
FOR i = 1 TO 2
    DIM du(i + 1), dv(i + 3)
NEXT
PRINT UBOUND(du); UBOUND(dv)
PRINT "2. the second is new in the second round:";
REDIM dy(1)
FOR i = 1 TO 2
    ERASE dy
    DIM dx(i + 1), dy(i + 3)
NEXT
PRINT UBOUND(dx); UBOUND(dy)
PRINT "3. a statement after it on the line:";
FOR i = 1 TO 2
    DIM dz(i + 1), ea(i + 3): n = n + 1
NEXT
PRINT n
PRINT "4. REDIM of a static and a dynamic array:";
DIM st(3)
REDIM st(5), eb(4)
PRINT UBOUND(st); UBOUND(eb)
PRINT "5. three arrays, the second allocated before:";
REDIM m2(1)
DIM m1(i), m2(i + 1), m3(i + 2)
PRINT UBOUND(m1); UBOUND(m2); UBOUND(m3)
PRINT "6. in a SUB, DIM a(3), b(n) run twice in a call:";
local 4
PRINT "7. with strings:";
FOR i = 1 TO 2
    DIM s1(i) AS STRING, s2(i) AS STRING
    s1(1) = s1(1) + "a": s2(1) = s2(1) + "b"
NEXT
PRINT " "; s1(1); s2(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB local (n AS LONG)
    DIM r AS LONG, k AS LONG
    FOR r = 1 TO 2
        DIM a(3) AS LONG, b(n + r) AS LONG: k = k + 1
    NEXT
    PRINT UBOUND(a); UBOUND(b); k
END SUB
