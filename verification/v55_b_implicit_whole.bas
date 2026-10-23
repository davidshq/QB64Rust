$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): the whole array x() of a name that is no array: LEN and the
' value of a scalar's name and of a name never used, of other types, then an element of it; in a SUB.
ON ERROR GOTO h
x = 1
PRINT "1. a scalar x, LEN(x()) and x():"; LEN(x()); x(); x
PRINT "2. a name never used, LEN(nothing()) and nothing():"; LEN(nothing()); nothing()
PRINT "3. xs$() (LEN of it is an error, v55_x70):"; "["; xs$(); "]"
PRINT "4. xd#() and xl&(), LEN:"; LEN(xd#()); LEN(xl&())
PRINT "5. y() then y(5) = 1:"; LEN(y());
y(5) = 1
PRINT y(5); UBOUND(y); LBOUND(y)
PRINT "6. w(3) = 2 then LEN(w()) and w():";
w(3) = 2
PRINT LEN(w()); w()
PRINT "7. v(0) = 9 then v():";
v(0) = 9
PRINT v()
s
PRINT "10. the SUB again:"
s

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    PRINT "8. in a SUB, LEN(u()):"; LEN(u())
    PRINT "9. in a SUB, u():"; u()
END SUB
