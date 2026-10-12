$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, review of group 5): below $DYNAMIC, an array used without DIM first
' named in a bound of REDIM or DIM; each statement runs twice, and an element is set after the first round to see
' whether the second round makes the array anew.
ON ERROR GOTO h
DIM i AS LONG
'$DYNAMIC
PRINT "1. REDIM rb(rc(1) + i):";
FOR i = 1 TO 2
    REDIM rb(rc(1) + i)
    PRINT UBOUND(rb);
    rc(1) = 5: rc(2) = 9
NEXT
PRINT rc(1); rc(2); UBOUND(rc)
PRINT "2. REDIM list, the second bound names it:";
FOR i = 1 TO 2
    REDIM ra(i + 1), rd(re(1) + i)
    re(1) = 5
NEXT
PRINT UBOUND(ra); UBOUND(rd); re(1)
PRINT "3. DIM dv(dw(1) + 3):";
FOR i = 1 TO 2
    DIM dv(dw(1) + 3)
    dw(1) = 5
NEXT
PRINT UBOUND(dv); dw(1)
PRINT "4. REDIM _PRESERVE rp(rq(1) + i):";
FOR i = 1 TO 2
    REDIM _PRESERVE rp(rq(1) + i)
    rq(1) = 5
NEXT
PRINT UBOUND(rp); rq(1)
PRINT "5. a use after the REDIM line that named it, in a loop:";
FOR i = 1 TO 2
    REDIM rf(rg(1) + 1)
    rg(2) = rg(2) + 1
NEXT
PRINT rg(2)
PRINT "6. REDIM of the array itself after its use in the same loop:";
FOR i = 1 TO 2
    rh(1) = rh(1) + 1
    REDIM ri(rh(1) + 1)
NEXT
PRINT rh(1); UBOUND(ri)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
