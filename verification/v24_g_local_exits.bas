$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a local array through the ways out of a procedure: EXIT SUB, an
' error inside with RESUME NEXT, a recursive FUNCTION with a local array per level, a local array of strings
' in a FUNCTION that returns a string.
ON ERROR GOTO h
DIM v AS LONG
PRINT "1. EXIT SUB with a local array, called twice:";
early 1: early 1
PRINT
PRINT "2. an error inside, then the SUB goes on:";
fails: fails
PRINT
PRINT "3. a recursive FUNCTION, each level with DIM a(n):";
v = down(3)
PRINT " result"; v
PRINT "4. a FUNCTION with a local string array:";
PRINT words("a"); words("b")

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB early (n AS LONG)
    DIM a(3) AS LONG
    a(1) = a(1) + 1
    PRINT a(1);
    IF n = 1 THEN EXIT SUB
    PRINT "not reached"
END SUB
SUB fails
    DIM a(3) AS LONG
    a(1) = a(1) + 1
    a(9) = 1
    ERROR 5
    PRINT a(1);
END SUB
FUNCTION down (n AS LONG)
    DIM a(n) AS LONG
    a(n) = n
    IF n > 1 THEN down = down(n - 1) + a(n) ELSE down = a(n)
    PRINT UBOUND(a); a(n);
END FUNCTION
FUNCTION words$ (s AS STRING)
    DIM w(2) AS STRING
    w(1) = w(1) + s + s
    words$ = "[" + w(1) + w(2) + "]"
END FUNCTION
