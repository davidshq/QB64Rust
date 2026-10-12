$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, review of group 5): an array used without DIM first named in a bound of
' REDIM where $DYNAMIC is not in effect, and in a SUB: static or dynamic, and made anew by each round or not.
ON ERROR GOTO h
DIM i AS LONG
DIM SHARED sh(1 TO 2)
PRINT "1. main, REDIM rb(rc(1) + i) twice:";
FOR i = 1 TO 2
    REDIM rb(rc(1) + i)
    PRINT UBOUND(rb);
    rc(1) = 5
NEXT
PRINT rc(1); UBOUND(rc)
PRINT "2. REDIM rc(20) after it:";
REDIM rc(20)
PRINT UBOUND(rc)
PRINT "3. main, REDIM _PRESERVE rp(rq(1) + i) twice:";
FOR i = 1 TO 2
    REDIM _PRESERVE rp(rq(1) + i)
    rq(1) = 5
NEXT
PRINT UBOUND(rp); rq(1)
PRINT "4. main, DIM dv(dw(1) + 3) twice, then REDIM dw(20):";
FOR i = 1 TO 2
    DIM dv(dw(1) + 3)
    dw(1) = 5
NEXT
PRINT UBOUND(dv); dw(1);
REDIM dw(20)
PRINT UBOUND(dw)
PRINT "5. a SUB, REDIM of a shared array with a bound that names a new one, called twice:";
REDIM SHARED sd(1)
p
p
PRINT UBOUND(sd)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB p
    REDIM sd(lc(1) + 4)
    PRINT " UBOUND(lc):";
    PRINT UBOUND(lc);
    lc(1) = 5
    PRINT lc(1);
END SUB
