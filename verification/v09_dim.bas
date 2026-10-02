$CONSOLE:ONLY
' v09: DIM / REDIM / STATIC / COMMON / ERASE statement semantics (study\10 section 1)
' Every runtime error is trapped, so no message box appears.
COMMON SHARED cs()
ON ERROR GOTO eh

PRINT "1. OPTION BASE 1 + DIM z(0):";
OPTION BASE 1
DIM z(0)
PRINT LBOUND(z); UBOUND(z)
DIM y(3)
PRINT "   DIM y(3) under base 1:"; LBOUND(y); UBOUND(y)
OPTION BASE 0

PRINT "2. static array skipped by GOTO:";
GOTO skip
DIM s(5)
skip:
s(3) = 7: PRINT s(3)

PRINT "3. REDIM without type adopts REDIM AS type:";
REDIM x(5) AS LONG
REDIM x(10)
x(1) = 3.7: PRINT x(1)

PRINT "4. REDIM of a static array:";
DIM st(5)
REDIM st(8)
PRINT " ubound"; UBOUND(st)

PRINT "5. ERASE static keeps bounds, clears data:";
st(2) = 9: ERASE st: PRINT UBOUND(st); st(2)
PRINT "   ERASE dynamic, then UBOUND:";
REDIM dy(4): ERASE dy: PRINT UBOUND(dy)

PRINT "6. DIM twice in a SUB loop:"
dimtwice
PRINT "7. STATIC a(3) keeps values across calls:";
keep: keep: PRINT
PRINT "8. STATIC a() list array, DIM a(3) on each call:";
listarr: listarr: PRINT
PRINT "9. SUB ... STATIC keeps a constant-bound array:";
substatic: substatic: PRINT
PRINT "10. COMMON SHARED cs() then DIM CS(3) (different case), seen in SUB:";
DIM CS(3): CS(1) = 5
showcs
PRINT "11. DIM in SUB runs on each call (fresh zeroed array):";
fresh: fresh: PRINT
SYSTEM

eh:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB dimtwice
    FOR i = 1 TO 2
        DIM d(3)
        PRINT "   pass"; i; "ubound"; UBOUND(d)
    NEXT
END SUB

SUB keep
    STATIC a(3)
    a(1) = a(1) + 1: PRINT a(1);
END SUB

SUB listarr
    STATIC b()
    DIM b(3)
    b(1) = b(1) + 1: PRINT b(1);
END SUB

SUB substatic STATIC
    DIM c(3)
    c(1) = c(1) + 1: PRINT c(1);
END SUB

SUB showcs
    PRINT cs(1)
END SUB

SUB fresh
    DIM f(3)
    f(1) = f(1) + 1: PRINT f(1);
END SUB
