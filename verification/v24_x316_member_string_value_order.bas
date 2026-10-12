$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): a string variable stored into a member of an array element
' whose index calls a FUNCTION that changes the variable: which value is stored. The same for a STRING * n
' member, for a concatenation, and for a number.
TYPE t1
    s AS STRING
    f AS STRING * 6
    k AS LONG
    v(0 TO 1) AS STRING
END TYPE
DIM a(3) AS t1
DIM SHARED z$, n AS LONG
z$ = "before"
a(setz(1)).s = z$
PRINT "1. a(setz(1)).s = z$: ["; a(1).s; "]"
z$ = "before"
a(setz(1)).f = z$
PRINT "2. a(setz(1)).f = z$: ["; a(1).f; "]"
z$ = "before"
a(setz(1)).s = z$ + "!"
PRINT "3. a(setz(1)).s = z$ + excl: ["; a(1).s; "]"
n = 1
a(setz(1)).k = n
PRINT "4. a(setz(1)).k = n:"; a(1).k
z$ = "before"
a(setz(1)).v(setz(1)) = z$
PRINT "5. a(setz(1)).v(setz(1)) = z$: ["; a(1).v(1); "]"
SYSTEM

FUNCTION setz (i AS LONG)
    z$ = "after"
    n = 2
    setz = i
END FUNCTION
