$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): below $DYNAMIC, an array used without DIM in a SUB and in a SUB
' ... STATIC.
ON ERROR GOTO h
'$DYNAMIC
plain: st: st

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB plain
    PRINT "plain SUB:";
    x(2) = 5
    PRINT x(2)
END SUB
SUB st STATIC
    PRINT "SUB ... STATIC:";
    x(2) = x(2) + 1
    PRINT x(2)
END SUB
