$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, review of group 5; the question is group 4's): a DIM of several dynamic
' arrays run twice: how many errors 10, and what RESUME NEXT goes on with.
ON ERROR GOTO h
DIM i AS LONG, n AS LONG
PRINT "1. DIM du(i + 1), dv(i + 3):";
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

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
