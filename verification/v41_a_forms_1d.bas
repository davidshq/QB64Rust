$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): the four forms on one-dimensional LONG arrays with lower
' bounds other than 0 (whole to whole, range to a start, range to whole, whole to a start), what each copies and
' where, the elements it leaves; one element; overlap forward and backward; an array copied to itself.
DIM i AS LONG
DIM s(3 TO 8) AS LONG, d(-2 TO 12) AS LONG
FOR i = 3 TO 8: s(i) = i * 10: NEXT
GOSUB clr: _ARRAYCOPY s() TO d(): PRINT "1. whole to whole:": GOSUB show
GOSUB clr: _ARRAYCOPY s(4 TO 6) TO d(-1): PRINT "2. range to start:": GOSUB show
GOSUB clr: _ARRAYCOPY s(4 TO 6) TO d(): PRINT "3. range to whole:": GOSUB show
GOSUB clr: _ARRAYCOPY s() TO d(0): PRINT "4. whole to start:": GOSUB show
GOSUB clr: _ARRAYCOPY s(5 TO 5) TO d(12): PRINT "5. one element to the last:": GOSUB show
GOSUB clr: _ARRAYCOPY s() TO d(-2): PRINT "6. whole to the first:": GOSUB show
PRINT "7. overlap forward:";
GOSUB fill: _ARRAYCOPY s(3 TO 6) TO s(5): GOSUB shows
PRINT "8. overlap backward:";
GOSUB fill: _ARRAYCOPY s(5 TO 8) TO s(3): GOSUB shows
PRINT "9. itself whole:";
GOSUB fill: _ARRAYCOPY s() TO s(): GOSUB shows
PRINT "10. itself range to whole:";
GOSUB fill: _ARRAYCOPY s(4 TO 7) TO s(): GOSUB shows
PRINT "11. lower case and spaces:";
GOSUB fill: _arraycopy   s( 3  to  4 )   to   s( 7 ): GOSUB shows
SYSTEM
clr:
FOR i = -2 TO 12: d(i) = -1: NEXT
RETURN
show:
FOR i = -2 TO 12: PRINT d(i);: NEXT
PRINT
RETURN
fill:
FOR i = 3 TO 8: s(i) = i * 10: NEXT
RETURN
shows:
FOR i = 3 TO 8: PRINT s(i);: NEXT
PRINT
RETURN
