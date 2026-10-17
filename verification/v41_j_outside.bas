$CONSOLE:ONLY
' Verification (m3-arraycopy, while its proposal was written): an index outside its bounds on the way to a member
' array, as destination and as source, numbers and strings: what is copied, and into which element.
TYPE t
    b(1 TO 3) AS LONG
    s(1 TO 2) AS STRING * 2
END TYPE
ON ERROR GOTO h
DIM i AS LONG, j AS LONG
DIM tt(1 TO 2) AS t, src(1 TO 3) AS LONG, d(1 TO 3) AS LONG, fs(1 TO 2) AS STRING * 2
FOR i = 1 TO 3: src(i) = i * 7: NEXT
fs(1) = "ab": fs(2) = "cd"
tt(1).b(1) = 51: tt(1).b(2) = 52: tt(1).b(3) = 53
PRINT "1. destination tt(3).b():";: _ARRAYCOPY src() TO tt(3).b(): PRINT: GOSUB show
PRINT "2. destination tt(0).b(1):";: _ARRAYCOPY src(1 TO 2) TO tt(0).b(1): PRINT: GOSUB show
PRINT "3. destination tt(-5).s():";: _ARRAYCOPY fs() TO tt(-5).s(): PRINT: GOSUB show
PRINT "4. source tt(3).b():";: _ARRAYCOPY tt(3).b() TO d(): PRINT d(1); d(2); d(3)
PRINT "5. source tt(0).b(1 TO 2):";: _ARRAYCOPY tt(0).b(1 TO 2) TO d(2): PRINT d(1); d(2); d(3)
SYSTEM
show:
FOR i = 1 TO 2
    PRINT "   tt("; LTRIM$(STR$(i)); "):";
    FOR j = 1 TO 3: PRINT tt(i).b(j);: NEXT
    PRINT " ["; tt(i).s(1); tt(i).s(2); "]"
NEXT
RETURN
h:
PRINT " error"; ERR;
RESUME NEXT
