$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): a STATIC a() array passed on before anything gave it bounds.
ON ERROR GOTO h
outer
outer
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
SUB outer
    STATIC k() AS LONG
    make k()
    PRINT UBOUND(k);
    k(1) = k(1) + 1
    PRINT k(1)
END SUB
SUB make (a() AS LONG)
    REDIM _PRESERVE a(3) AS LONG
END SUB
