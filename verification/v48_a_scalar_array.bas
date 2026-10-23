$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): a scalar and an array of one name, one typed by DIM ... AS.
DIM SHARED sh(3) AS INTEGER
ON ERROR GOTO h
PRINT "1. plain scalar beside a typed array:";
DIM a(3) AS INTEGER
a = 1.5: a(1) = 7
PRINT a; a(1); LEN(a); LEN(a(1))
PRINT "2. a! against a:";
a! = a! + 1
PRINT a; a!
PRINT "3. beside a STRING array:";
DIM s(2) AS STRING
s = 2.5: s(1) = "x"
PRINT s; s(1); LEN(s)
PRINT "4. read before any store:";
DIM z(2) AS LONG
PRINT z; LEN(z)
PRINT "5. scalar used above the typed array:";
q = 1.25
DIM q(3) AS INTEGER
q(1) = 3
PRINT q; q(1); LEN(q); LEN(q(1))
PRINT "6. array without AS beside a typed scalar:";
DIM b AS STRING
b = "str"
DIM b(2)
b(1) = 1.25
PRINT b; b(1); LEN(b(1))
PRINT "7. array without AS beside a typed array:";
DIM c(2) AS DOUBLE
c(1) = 1 / 3
PRINT c(1); LEN(c(1));
DIM c(3)
c(1) = c(1) + 1
PRINT c(1); LEN(c(1)); UBOUND(c); c#(1)
PRINT "8. typed array after an array without AS:";
DIM g(2)
g(1) = 1.5
DIM g(4) AS LONG
g(1) = 2
PRINT g(1); g!(1); LEN(g(1)); UBOUND(g)
PRINT "9. typed array beside a scalar typed otherwise:";
DIM d AS STRING
DIM d(3) AS LONG
d = "dd": d(1) = 5
PRINT d; d(1); LEN(d); LEN(d(1))
PRINT "10. PUT and GET of the plain name:";
DIM p(1 TO 3) AS INTEGER
p = 2.25
OPEN "v48_a.tmp" FOR BINARY AS #1
PUT #1, 1, p
PRINT LOF(1);
p = 0
GET #1, 1, p
PRINT p
CLOSE #1: KILL "v48_a.tmp"
PRINT "11. in a SUB:";
sa
PRINT "12. a shared array seen in a SUB:";
ss
PRINT "    main's sh:"; sh; sh(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB sa
    DIM a(3) AS INTEGER
    a = 2.5: a(1) = 9
    PRINT a; a(1); LEN(a);
    DIM b AS LONG
    b = 7
    DIM b(2)
    b(1) = 0.5
    PRINT b; b(1); LEN(b(1));
    STATIC c() AS DOUBLE
    REDIM c(2)
    c(1) = 0.25
    PRINT c(1); LEN(c(1))
END SUB

SUB ss
    sh = 3.5: sh(1) = 4
    PRINT sh; sh(1); LEN(sh)
END SUB
