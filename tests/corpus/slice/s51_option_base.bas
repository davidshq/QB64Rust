$CONSOLE:ONLY
' OPTION BASE with static arrays (spec language/arrays-and-types, "OPTION BASE"): the lower bound of a dimension
' written without TO, from the statement on in the file; bounds with TO unchanged; the number 0 in plain digits
' staying 0 to 0, per dimension; base 0 again; a statement skipped by GOTO and one inside a SUB still applying
' below them; what follows the word after BASE ignored; an index below the base (error 9); arrays of strings and
' of a user type. The other kinds of array come with their own slice programs.
TYPE rec
    n AS LONG
END TYPE
ON ERROR GOTO h
DIM a0(3)
OPTION BASE 1
DIM a(3): DIM b(2 TO 4)
PRINT "1. base 1, DIM a(3), DIM b(2 TO 4): LBOUND(a), UBOUND(a), LBOUND(b)"; LBOUND(a); UBOUND(a); LBOUND(b)
PRINT "2. DIM a0(3) above the statement:"; LBOUND(a0); UBOUND(a0)
DIM z(0)
PRINT "3. DIM z(0):"; LBOUND(z); UBOUND(z)
DIM z2(0, 3), z3(3, 0), z4(00)
PRINT "4. DIM z2(0, 3):"; LBOUND(z2, 1); UBOUND(z2, 1); LBOUND(z2, 2); UBOUND(z2, 2)
PRINT "   DIM z3(3, 0):"; LBOUND(z3, 1); UBOUND(z3, 1); LBOUND(z3, 2); UBOUND(z3, 2)
PRINT "   DIM z4(00):"; LBOUND(z4); UBOUND(z4)
DIM s(2) AS STRING, r(2) AS rec, m(2, 1 TO 2) AS INTEGER
s(1) = "one": s(2) = "two": r(1).n = 5: m(1, 1) = 7: m(2, 2) = 8
PRINT "5. strings, a user type, two dimensions: "; s(1); " "; s(2); r(1).n; r(2).n; m(1, 1); m(2, 2); LBOUND(m, 1); LBOUND(m, 2)
PRINT "6. an index below the base, a(0) = 1:";
a(0) = 1
PRINT " and PRINT s(0):";
PRINT s(0)
a(1) = 1.5: a(3) = 3.5
PRINT "7. a(1), a(3):"; a(1); a(3)
OPTION BASE 0
DIM c(3)
PRINT "8. base 0 again, DIM c(3):"; LBOUND(c); UBOUND(c)
GOTO skip
OPTION BASE 1
skip:
DIM e(3)
PRINT "9. DIM e(3) below an OPTION BASE 1 that GOTO skipped:"; LBOUND(e)
OPTION BASE 0 + 1
DIM f(3)
PRINT "10. DIM f(3) below OPTION BASE 0 + 1:"; LBOUND(f)
OPTION BASE 1 x
DIM g(3)
PRINT "11. DIM g(3) below OPTION BASE 1 x:"; LBOUND(g)
OPTION BASE 0
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
