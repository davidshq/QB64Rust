$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an array used without DIM inside a procedure: a plain SUB, a
' FUNCTION, a SUB ... STATIC, after STATIC y(), and LBOUND of such a name.
ON ERROR GOTO h
DIM v AS LONG
plain: plain
v = fn(1)
st: st
listed: listed
bounds

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB plain
    PRINT "plain SUB, x(2) = x(2) + 1:";
    x(2) = x(2) + 1
    PRINT " x(2):";
    PRINT x(2)
END SUB
FUNCTION fn (n)
    PRINT "FUNCTION, x(2) = 5:";
    x(2) = 5
    PRINT " UBOUND(x):";
    PRINT UBOUND(x)
END FUNCTION
SUB st STATIC
    PRINT "SUB ... STATIC, x(2) = x(2) + 1:";
    x(2) = x(2) + 1
    PRINT " x(2), UBOUND(x):";
    PRINT x(2); UBOUND(x)
END SUB
SUB listed
    STATIC y()
    PRINT "after STATIC y(), y(2) = y(2) + 1:";
    y(2) = y(2) + 1
    PRINT " y(2):";
    PRINT y(2)
END SUB
SUB bounds
    PRINT "LBOUND(nb), UBOUND(nb) in a plain SUB:";
    PRINT LBOUND(nb); UBOUND(nb)
END SUB
