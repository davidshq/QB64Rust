$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): STATIC a(3) in a SUB below $DYNAMIC, called three times with
' no REDIM: kept, made anew, or error 10; then ERASE of it and a further call.
ON ERROR GOTO h
'$DYNAMIC
s 0: s 0: s 1: s 0

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s (e AS LONG)
    STATIC a(3) AS LONG
    a(1) = a(1) + 1
    PRINT a(1); UBOUND(a);
    IF e THEN ERASE a: PRINT "erased";
END SUB
