$CONSOLE:ONLY
' Default types and procedures (spec language/default-types): the table follows the file. A SUB and a FUNCTION
' above a statement keep the earlier default for parameter, variables and result; those below get the new one.
' A FUNCTION is called by its plain name whatever the default where the call stands, and with the suffix of its
' type. Parameters and results typed by a default, STRING and STRING * n included; STATIC and SHARED typed by it.
' The value 100000 + 1/3 shows a result's type: 100000 LONG, 100000.3359375 SINGLE, 100000.3333333333 DOUBLE.
SUB sa (p)
    v = 2.5
    PRINT "1. a SUB above every statement: LEN of its parameter and variable"; LEN(p); LEN(v)
END SUB
FUNCTION fa (x)
    fa = 100000 + 1# / 3
END FUNCTION
DEFDBL A-Z
DIM r AS DOUBLE
m = 2.5
PRINT "0. main below DEFDBL A-Z: LEN(m)"; LEN(m)
sa 1
r = fa(3): PRINT "2. a FUNCTION above it, called plain and as fa!:"; r;
r = fa!(3): PRINT r
sb 1
r = fb(3): PRINT "4. a FUNCTION below DEFDBL A-Z, called plain and as fb#:"; r;
r = fb#(3): PRINT r
DEFLNG F
PRINT "5. FUNCTION f2 below DEFLNG F, as f2(3) and f2&(3):"; f2(3); f2&(3)
DEFSNG F
PRINT "6. f2(3) where the default for F is SINGLE again:"; f2(3)
DEFINT A-Z
gv = 7
sub1
half 5
fx "abcdefgh"
PRINT "10. a FUNCTION typed STRING * 5 by _DEFINE: ["; fs; "]"
PRINT "11. a FUNCTION typed STRING by DEFSTR: "; sf(2); " "; sf$(3)
sv$ = "x"
show sv$
PRINT "    and what the SUB stored in its parameter: "; sv$
count
PRINT
count
PRINT
PRINT "14. gv after SHARED gv in the SUB:"; gv
SYSTEM
DEFDBL A-Z
SUB sb (p)
    v = 2.5
    PRINT "3. a SUB below DEFDBL A-Z: LEN of its parameter and variable"; LEN(p); LEN(v)
END SUB
FUNCTION fb (x)
    fb = 100000 + 1# / 3
END FUNCTION
DEFLNG F
FUNCTION f2 (x)
    f2 = x * 100000
END FUNCTION
DEFINT A-Z
SUB sub1
    v = 2.5
    PRINT "7. a SUB below DEFINT A-Z: v = 2.5 is"; v
END SUB
SUB half (n)
    PRINT "8. its parameter n, called with 5: n / 2 is"; n / 2
END SUB
_DEFINE f AS STRING * 5
SUB fx (f1)
    PRINT "9. a parameter typed STRING * 5 by _DEFINE: ["; f1; "]"; LEN(f1)
END SUB
FUNCTION fs
    fs = "xy"
END FUNCTION
DEFSTR S
FUNCTION sf (n AS LONG)
    sf = STRING$(n, "r")
END FUNCTION
SUB show (s)
    PRINT "12. a parameter typed STRING by DEFSTR: "; s + "!"
    s = "changed"
END SUB
DEFINT A-Z
SUB count
    STATIC st
    SHARED gv
    st = st + 1.5
    gv = gv + 1
    PRINT "13. STATIC st and SHARED gv typed INTEGER:"; st; gv; LEN(st);
END SUB
