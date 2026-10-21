$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): REDIM name [AS type] without parentheses in a SUB or FUNCTION.
ON ERROR GOTO h
PRINT "1. in a list after an array:"
s1 "abc"
s1 "abcde"
PRINT "2. run twice in one call:"
s2
s2
PRINT "3. in a SUB ... STATIC:"
s3
s3
PRINT "4. in a FUNCTION:"
PRINT f1(3); f1(4)
PRINT "5. main's i untouched:"; i
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s1 (s AS STRING)
    REDIM v(LEN(s)) AS LONG, i AS LONG
    FOR i = 1 TO UBOUND(v): v(i) = i * 100000: NEXT
    PRINT v(UBOUND(v)); i; LEN(i)
END SUB

SUB s2
    FOR k = 1 TO 2
        REDIM i AS LONG
        PRINT i;
        i = i + 5
        REDIM j
        PRINT j;
        j = j + 1.5
        REDIM t AS STRING
        PRINT "[" + t + "]";
        t = t + "x"
    NEXT
    PRINT LEN(i); LEN(j)
END SUB

SUB s3 STATIC
    REDIM i AS LONG
    REDIM j
    i = i + 1: j = j + 0.5
    PRINT i; j
END SUB

FUNCTION f1 (n)
    REDIM r AS INTEGER
    r = r + n
    f1 = r + LEN(r) / 10
END FUNCTION
