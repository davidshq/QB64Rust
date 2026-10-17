$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-arraycopy, before its proposal): _DYNAMIC member arrays (numbers in 1 and 2 dimensions,
' variable-length strings) as source and destination, after a REDIM of the member, of an element of an array of
' the owner type; a member not allocated.
TYPE dy
    _DYNAMIC n(1 TO 4) AS LONG
    _DYNAMIC g(1 TO 2, 1 TO 3) AS INTEGER
    _DYNAMIC s(0 TO 2) AS STRING
END TYPE
DIM i AS LONG, j AS LONG
DIM v AS dy, w(1 TO 2) AS dy
DIM src(1 TO 4) AS LONG, back(1 TO 10) AS LONG
FOR i = 1 TO 4: src(i) = i * 100: NEXT

_ARRAYCOPY src() TO v.n()
PRINT "1. to a dynamic member:";: FOR i = 1 TO 4: PRINT v.n(i);: NEXT: PRINT
REDIM v.n(1 TO 10) AS LONG
_ARRAYCOPY src() TO v.n(7)
PRINT "2. after REDIM:";: FOR i = 1 TO 10: PRINT v.n(i);: NEXT: PRINT
_ARRAYCOPY v.n() TO back()
PRINT "3. member to array:";: FOR i = 1 TO 10: PRINT back(i);: NEXT: PRINT
_ARRAYCOPY v.n(7 TO 9) TO w(2).n(1)
PRINT "4. member to member of an element:";: FOR i = 1 TO 4: PRINT w(2).n(i);: NEXT: PRINT
DIM gs(1 TO 2, 1 TO 3) AS INTEGER
FOR i = 1 TO 2: FOR j = 1 TO 3: gs(i, j) = i * 10 + j: NEXT: NEXT
_ARRAYCOPY gs() TO w(1).g()
PRINT "5. 2D dynamic member:";: FOR i = 1 TO 2: FOR j = 1 TO 3: PRINT w(1).g(i, j);: NEXT: NEXT: PRINT
_ARRAYCOPY w(1).g(1 TO 2, 2 TO 3) TO v.g(1, 1)
PRINT "6. 2D member range:";: FOR i = 1 TO 2: FOR j = 1 TO 3: PRINT v.g(i, j);: NEXT: NEXT: PRINT
DIM ss(0 TO 2) AS STRING
ss(0) = "x": ss(1) = "yy": ss(2) = "zzz"
_ARRAYCOPY ss() TO v.s()
PRINT "7. strings to a dynamic member:";: FOR i = 0 TO 2: PRINT " ["; v.s(i); "]";: NEXT: PRINT
_ARRAYCOPY v.s(1 TO 2) TO w(2).s(0)
PRINT "8. string member to member:";: FOR i = 0 TO 2: PRINT " ["; w(2).s(i); "]";: NEXT: PRINT
ss(1) = "changed"
_ARRAYCOPY v.s() TO ss()
PRINT "9. string member back:";: FOR i = 0 TO 2: PRINT " ["; ss(i); "]";: NEXT: PRINT
ON ERROR GOTO h
ERASE v.n
PRINT "10. erased member as source:";
_ARRAYCOPY v.n() TO back()
PRINT " done"
SYSTEM
h:
PRINT " error"; ERR;
RESUME NEXT
