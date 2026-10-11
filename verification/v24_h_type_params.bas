$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a user-type parameter: a variable, an array element, a member of
' a user type, a value in parentheses; STRING members changed in the procedure; a FUNCTION; an element with an
' index outside.
TYPE pt
    x AS LONG
    n AS STRING
END TYPE
TYPE box
    a AS pt
END TYPE
ON ERROR GOTO h
DIM q AS pt, a(2) AS pt, bx AS box
move q
move a(2)
move bx.a
PRINT "1. a variable, an element, a member:"; q.x; q.n; a(2).x; a(2).n; bx.a.x; bx.a.n
PRINT "2. a FUNCTION:"; twice(q); q.x
PRINT "3. an element with an index outside, move a(9):";
move a(9)
PRINT " a(0).x"; a(0).x
passon q
PRINT "4. passed on to a second SUB:"; q.x

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB move (p AS pt)
    p.x = p.x + 1
    p.n = p.n + "m"
END SUB
FUNCTION twice (p AS pt)
    twice = p.x * 2
    p.x = p.x + 10
END FUNCTION
SUB passon (p AS pt)
    move p
END SUB
