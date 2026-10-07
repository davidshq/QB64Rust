$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): the store rule of a TYPE member, also of an element of a TYPE array.
' Under RESUME NEXT: is a member store made after a raising value, a raising index, both? Which element is
' written when the index is out of range? Which ERR is reported when both raise?
TYPE t
    m AS LONG
    n AS LONG
END TYPE
DIM u AS t
DIM a(3) AS t
ON ERROR GOTO h
u.m = 5: u.m = ASC("")
PRINT "u.m = ASC(empty):"; u.m
a(0).m = 70: a(1).m = 71: a(2).m = 72: a(3).m = 73
a(9).m = 5
PRINT "after a(9).m = 5:"; a(0).m; a(1).m; a(2).m; a(3).m
a(-1).m = 6
PRINT "after a(-1).m = 6:"; a(0).m; a(1).m; a(2).m; a(3).m
a(0).m = 70
a(9).m = ASC("")
PRINT "after a(9).m = ASC(empty):"; a(0).m; a(1).m; a(2).m; a(3).m
a(0).n = 60
a(1).n = 4: a(1).n = a(9).n
PRINT "a(1).n = a(9).n:"; a(1).n
y = 5: y = a(9).m
PRINT "y = a(9).m:"; y
a(2).n = 3: a(2).n = ASC("")
PRINT "a(2).n = ASC(empty):"; a(2).n
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT
