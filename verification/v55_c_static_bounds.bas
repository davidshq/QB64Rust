$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): STATIC arrays whose bounds are expressions of constants: a
' CONST of the main module and of the SUB, operators, a range; STATIC v(2 + 3) erased and then allocated again by
' a handled error.
ON ERROR GOTO h
CONST k = 3
s1: s1
s2: s2
s3: s3
s4: s4

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s1
    STATIC v(k + 1) AS LONG
    v(1) = v(1) + 1
    PRINT "s1:"; v(1); UBOUND(v)
END SUB

SUB s2
    CONST j = 2
    STATIC v(j * 3 - 1, 10 \ 3) AS LONG
    v(1, 1) = v(1, 1) + 1
    PRINT "s2:"; v(1, 1); UBOUND(v, 1); UBOUND(v, 2)
END SUB

SUB s3
    STATIC v(-1 TO &H4) AS LONG
    v(1) = v(1) + 1
    PRINT "s3:"; v(1); LBOUND(v); UBOUND(v)
END SUB

SUB s4
    STATIC v(2 + 3) AS LONG
    v(1) = v(1) + 1
    PRINT "s4:"; v(1); UBOUND(v);
    ERASE v
    PRINT v(1);
    PRINT " again:"; v(1); UBOUND(v)
END SUB
