$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): an index that raises but gives an in-range placeholder (ASC("") is 0):
' is the store made, for an element and for a member of an element? And when value and index both raise?
TYPE t
    m AS LONG
END TYPE
DIM x(3) AS LONG
DIM a(3) AS t
ON ERROR GOTO h
x(0) = 50
x(ASC("")) = 5
PRINT "x(ASC(empty)) = 5:"; x(0)
a(0).m = 60
a(ASC("")).m = 6
PRINT "a(ASC(empty)).m = 6:"; a(0).m
a(0).m = 60
a(ASC("")).m = LEN(CHR$(-1)) + 7
PRINT "a(ASC(empty)).m = LEN(CHR$(-1)) + 7:"; a(0).m
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT
