$CONSOLE:ONLY
' Slice program (m2-arrays-and-types, D6, D9): the store rule of each place, the order parts of a statement are
' evaluated in, and which error ERR reports. Errors are raised by a bad index (9) and by CHR$(-1) inside INSTR
' (5, placeholder 0). Where the new compiler differs on purpose (DIVERGENCES.md D-004: a member store into an
' element with a bad index writes element 0 in the old compiler) this program does not look: element 0 of `bad`
' is never printed. No PRINT comma.
TYPE t
    m AS LONG
    n AS LONG
END TYPE
DIM x(10) AS LONG
DIM s(3) AS STRING
DIM u AS t
DIM a(3) AS t
DIM bad(3) AS t
DIM mm(3, 3) AS LONG
DIM k AS LONG
k = -1
ON ERROR GOTO h

' Element stores: the index first; with an error pending after it nothing is stored.
x(9) = 5: x(9) = INSTR(CHR$(k), "x")
PRINT "x(9) = raising value:"; x(9)
x(0) = 77: x(10) = 88
x(11) = 5
PRINT "after x(11) = 5:"; x(0); x(10)
x(-1) = 6
PRINT "after x(-1) = 6:"; x(0); x(10)
x(11) = INSTR(CHR$(k), "x")
PRINT "after x(11) = raising value:"; x(0); x(10)
x(INSTR(CHR$(k), "x")) = 5
PRINT "after x(raising index) = 5:"; x(0)
s(0) = "zero"
s(4) = "four"
PRINT "after s(4) = four: ["; s(0); "] ["; s(3); "]"
s(1) = "a": s(1) = CHR$(k)
PRINT "s(1) = CHR$(-1): ["; s(1); "]"

' A read with a bad index gives element 0's value.
y = 5: y = x(11)
PRINT "y = x(11):"; y
x(2) = 4: x(2) = x(-1) + 1
PRINT "x(2) = x(-1) + 1:"; x(2)
x(3) = 4: x(x(11) + 3) = 9
PRINT "x(x(11) + 3) = 9:"; x(0); x(3)
t$ = "t": t$ = s(9)
PRINT "t$ = s(9): ["; t$; "]"
PRINT "print"; x(11); "rest"
PRINT "after print"

' Member stores are not guarded.
u.m = 5: u.m = INSTR(CHR$(k), "x")
PRINT "u.m = raising value:"; u.m
a(2).n = 3: a(2).n = INSTR(CHR$(k), "x")
PRINT "a(2).n = raising value:"; a(2).n
a(0).n = 60
a(1).n = 4: a(1).n = a(9).n
PRINT "a(1).n = a(9).n:"; a(1).n

' A member store into an element: the value before the index; the first error wins.
bad(1).m = 71: bad(2).m = 72: bad(3).m = 73
bad(9).m = 5
PRINT "after bad(9).m = 5:"; bad(1).m; bad(2).m; bad(3).m
bad(9).m = INSTR(CHR$(k), "x")
PRINT "after bad(9).m = raising value:"; bad(1).m; bad(2).m; bad(3).m
a(ix&(1)).m = va&(5)
PRINT "a(1).m:"; a(1).m
x(ix&(2)) = va&(6)
PRINT "x(2):"; x(2)
y& = x(ix&(2)) + va&(0)
PRINT "y&:"; y&

' Several indexes: left to right, each checked on its own, first error wins.
mm(ix&(1), ix&(2)) = 5
PRINT "mm(1, 2):"; mm(1, 2)
PRINT "mm(1, 4):"; mm(1, 4)
mm(9, INSTR(CHR$(k), "x")) = 1
PRINT "after mm(9, raising) = 1"
mm(INSTR(CHR$(k), "x"), 9) = 1
PRINT "after mm(raising, 9) = 1"

' By reference: a bad index never reaches the SUB; arguments left to right.
x(0) = 50: bump x(11)
PRINT "bump x(11):"; x(0)
bump bad(9).m
PRINT "after bump bad(9).m:"; bad(1).m
two x(11), INSTR(CHR$(k), "x")
PRINT "after two x(11), raising"
two INSTR(CHR$(k), "x"), x(11)
PRINT "after two raising, x(11)"

' LBOUND and UBOUND with a dimension out of range.
PRINT "LBOUND(mm, 3):"; LBOUND(mm, 3)
PRINT "UBOUND(x, 0):"; UBOUND(x, 0)
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

FUNCTION ix& (n AS LONG)
    PRINT "index"; n
    ix& = n
END FUNCTION

FUNCTION va& (n AS LONG)
    PRINT "value"; n
    va& = n
END FUNCTION
