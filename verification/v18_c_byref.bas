$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): an element and a member passed by reference, and the order in which the
' arguments of a call are evaluated when two of them raise.
TYPE t
    m AS LONG
END TYPE
DIM x(5) AS LONG
DIM u AS t
DIM a(3) AS t
ON ERROR GOTO h
x(3) = 10: bump x(3)
PRINT "bump x(3):"; x(3)
x(3) = 10: bump (x(3))
PRINT "bump (x(3)):"; x(3)
x(3) = 10: CALL bump(x(3))
PRINT "CALL bump(x(3)):"; x(3)
x(0) = 50: bump x(9)
PRINT "bump x(9):"; x(0); x(3)
u.m = 20: bump u.m
PRINT "bump u.m:"; u.m
a(2).m = 30: bump a(2).m
PRINT "bump a(2).m:"; a(2).m
a(0).m = 40: bump a(9).m
PRINT "bump a(9).m:"; a(0).m
two x(9), ASC("")
PRINT "after two x(9), ASC(empty)"
two ASC(""), x(9)
PRINT "after two ASC(empty), x(9)"
two a(9).m, ASC("")
PRINT "after two a(9).m, ASC(empty)"
two ASC(""), a(9).m
PRINT "after two ASC(empty), a(9).m"
x(1) = 1: x(2) = 2
swapper x(1), x(2)
PRINT "swapper x(1), x(2):"; x(1); x(2)
s! = 1.5: x(4) = 4
v& = 7: bump v&
PRINT "bump v&:"; v&
PRINT "f(x(3)):"; f(x(3)); x(3)
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT

SUB bump (v AS LONG)
    PRINT "in bump"; v
    v = v + 1
END SUB

SUB two (p AS LONG, q AS LONG)
    PRINT "in two"; p; q
END SUB

SUB swapper (p AS LONG, q AS LONG)
    tmp& = p: p = q: q = tmp&
END SUB

FUNCTION f& (v AS LONG)
    v = v * 2
    f& = v
END FUNCTION
